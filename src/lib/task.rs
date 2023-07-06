use std::sync::{Arc, atomic::{Ordering, AtomicU8}};
use std::marker::PhantomData;
use std::time::Duration;
use std::cell::UnsafeCell;
use std::any::Any;
use std::sync::LazyLock;
use std::thread;
use std::ptr;

use crossbeam::deque::{Injector, Steal};

static TASK_POOL: LazyLock<TaskPool> = LazyLock::new(|| {
    let cpu_count = std::cmp::max(1, num_cpus::get() - 2);
    TaskPool::new(cpu_count)
});

const SLEEP_DURATION: Duration = Duration::from_millis(2);

// numbers for task inner status
const TASK_PENDING: u8 = 0;
const TASK_COMPLETED: u8 = 1;
const TASK_POLLED: u8 = 2;

#[derive(Clone)]
pub struct TaskPool {
    queue: Arc<Injector<(
        Box<dyn FnOnce() -> Box<dyn Any + Send + 'static> + Send>,
        Arc<TaskInner>,
    )>>,
}

impl TaskPool {
    fn new(worker_count: usize) -> Self {
        let task_pool = TaskPool {
            queue: Arc::new(Injector::new()),
        };

        for _ in 0..worker_count {
            let pool = task_pool.clone();
            thread::spawn(move || pool.worker());
        }

        task_pool
    }

    pub fn get() -> &'static TaskPool {
        &*TASK_POOL
    }

    // function run on the worker threads
    fn worker(&self) {
        loop {
            let (task_fn, task_inner) = loop {
                match self.queue.steal() {
                    Steal::Success(val) => break val,
                    Steal::Empty => thread::sleep(SLEEP_DURATION),
                    Steal::Retry => continue,
                }
            };

            let result = task_fn();
            
            let data_pointer = task_inner.data.get();
            unsafe {
                ptr::replace(data_pointer, Some(result));
            }

            task_inner.status.store(TASK_COMPLETED, Ordering::Release);
        }
    }

    pub fn spawn<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(&self, f: F) -> Task<T> {
        let task_inner = Arc::new(TaskInner {
            status: AtomicU8::new(TASK_PENDING),
            data: UnsafeCell::new(None),
            marker: PhantomData,
        });

        let operation = Box::new(|| {
            let result: Box<dyn Any + Send + 'static> = Box::new(f());
            result
        });

        self.queue.push((operation, task_inner.clone()));

        Task {
            inner: task_inner,
            marker: PhantomData,
        }
    }
}

struct TaskInner {
    status: AtomicU8,
    data: UnsafeCell<Option<Box<dyn Any + Send + 'static>>>,
    marker: PhantomData<Option<Box<dyn Any + Send + 'static>>>,
}

unsafe impl Send for TaskInner {}
unsafe impl Sync for TaskInner {}

pub struct Task<T> {
    inner: Arc<TaskInner>,
    marker: PhantomData<Option<T>>
}

impl<T: Send + 'static> Task<T> {
    pub fn poll(&self) -> Option<T> {
        // temporary orederings
        match self.inner.status.compare_exchange(
            TASK_COMPLETED,
            TASK_POLLED,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                let ptr = self.inner.data.get();

                let data_boxed = unsafe {
                    ptr::replace(ptr, None)
                }.unwrap();

                Some(Box::into_inner(
                    data_boxed.downcast::<T>().unwrap(),
                ))
            },
            Err(_) => None,
        }
    }
}