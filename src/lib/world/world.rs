use std::sync::Arc;

use bevy::{prelude::*, utils::HashMap};
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use crossbeam::queue::SegQueue;

use crate::{types::*, vec3_map_many, blocks::{Block, BlockType}};
use super::{chunk::Chunk, ChunkData};

#[derive(Debug, Default, Resource)]
pub struct World {
    pub chunks: HashMap<ChunkPos, Arc<Chunk>>,
    /// A list of chunks which have changed and need to be remeshed
    pub(super) dirty_chunks: SegQueue<ChunkPos>,
}

#[derive(Debug)]
pub struct RayHitInfo {
    pub position: Vec3,
    pub block_pos: BlockPos,
}

impl World {
    /// Sets the block at the given position to the given block type
    /// 
    /// Returns a copy of the block on sucess, or `None` on failure
    /// 
    /// This should only be used for one off accessess
    /// If repeted accesses are needed, use [`ChunkLockCacheMut`] directly
    pub fn new_block(&self, block_pos: BlockPos, block_type: BlockType) -> Option<Block> {
        ChunkLockCacheMut::new(self)
            .new_block(block_pos, block_type)
    }

    pub fn raycast(&self, ray: Ray, max_length: f32) -> Option<RayHitInfo> {
        let mut block_pos = BlockPos::from(ray.origin);

        let direction = ray.direction.signum().as_ivec3();

        // distance it would take for each ray to travel 1 block for each axis
        let intercept_time_interval = ray.direction.map(|elem| {
            if elem != 0.0 {
                (BLOCK_SIZE / elem).abs()
            } else {
                f32::INFINITY
            }
        });

        // offset in block of starting position
        let ray_offset = ray.origin.map(|elem| {
            if elem >= 0.0 {
                elem % BLOCK_SIZE
            } else {
                BLOCK_SIZE + (elem % BLOCK_SIZE)
            }
        });

        let mut next_intercept_time = vec3_map_many!(|ray_elem, ray_offset_elem: f32| {
            if ray_elem > 0.0 {
                (BLOCK_SIZE - ray_offset_elem) / ray_elem
            } else if ray_elem < 0.0 {
                -ray_offset_elem / ray_elem
            } else {
                f32::INFINITY
            }
        }, Vec3, ray.direction, ray_offset);

        let mut chunk_lock = ChunkLockCache::new(self);

        loop {
            let next_intercept_axis = if next_intercept_time.x < next_intercept_time.y && next_intercept_time.x < next_intercept_time.z {
                VecAxis::X
            } else if next_intercept_time.y < next_intercept_time.z {
                VecAxis::Y
            } else {
                VecAxis::Z
            };

            block_pos[next_intercept_axis] += direction[next_intercept_axis];

            let current_time = next_intercept_time[next_intercept_axis];
            next_intercept_time[next_intercept_axis] += intercept_time_interval[next_intercept_axis];

            // ray has gone on for too long, no intercept
            if current_time > max_length {
                return None;
            }

            // ray has hit
            if let Some(block) = chunk_lock.get_block(block_pos) && !block.is_air() {
                return Some(RayHitInfo {
                    position: ray.get_point(current_time),
                    block_pos,
                });
            }
        }
    }
}

/// Caches the last lock chunk so block accessess around the same area do not need to repeatedly re lock the chunk
struct ChunkLockCache<'world> {
    world: &'world World,
    inner: Option<ChunkLockCacheInner<'world>>,
}

struct ChunkLockCacheInner<'world> {
    lock: RwLockReadGuard<'world, ChunkData>,
    chunk: &'world Chunk,
}

impl<'a> ChunkLockCache<'a> {
    fn new(world: &'a World) -> Self {
        ChunkLockCache {
            world,
            inner: None,
        }
    }

    fn get_chunk_data(&self) -> Option<&ChunkData> {
        Some(&self.inner.as_ref()?.lock)
    }

    fn lock_chunk(&mut self, chunk_pos: ChunkPos) {
        if let Some(ref inner) = self.inner && inner.chunk.chunk_pos == chunk_pos {
            // correct chunk is already locked
        } else {
            // force old lock to be dropped before acquiring new lock
            self.inner = None;

            let Some(chunk) = self.world.chunks.get(&chunk_pos) else {
                return;
            };

            self.inner = Some(ChunkLockCacheInner {
                lock: chunk.data.read(),
                chunk: &chunk,
            });
        }
    }

    fn get_block(&mut self, block_pos: BlockPos) -> Option<Block> {
        self.lock_chunk(ChunkPos::from(block_pos));
        let chunk_data = self.get_chunk_data()?;

        Some(chunk_data.blocks.get(block_pos.as_chunk_local()))
    }
}

/// Caches the last lock chunk so block accessess around the same area do not need to repeatedly re lock the chunk
struct ChunkLockCacheMut<'world> {
    world: &'world World,
    inner: Option<ChunkLockCacheInnerMut<'world>>,
}

struct ChunkLockCacheInnerMut<'world> {
    lock: RwLockWriteGuard<'world, ChunkData>,
    chunk: &'world Chunk,
}

impl<'a> ChunkLockCacheMut<'a> {
    fn new(world: &'a World) -> Self {
        ChunkLockCacheMut {
            world,
            inner: None,
        }
    }

    fn get_chunk_data_mut(&mut self) -> Option<&mut ChunkData> {
        Some(&mut self.inner.as_mut()?.lock)
    }

    fn lock_chunk(&mut self, chunk_pos: ChunkPos) {
        if let Some(ref inner) = self.inner && inner.chunk.chunk_pos == chunk_pos {
            // correct chunk is already locked
        } else {
            // force old lock to be dropped before acquiring new lock
            self.inner = None;

            let Some(chunk) = self.world.chunks.get(&chunk_pos) else {
                return;
            };

            self.inner = Some(ChunkLockCacheInnerMut {
                lock: chunk.data.write(),
                chunk: &chunk,
            });
        }
    }

    /// Marks the current chunk as dirty if it exists
    fn mark_dirty(&self) {
        if let Some(ref inner) = self.inner {
            inner.chunk.mark_dirty(self.world);
        }
    }

    fn get_block(&mut self, block_pos: BlockPos) -> Option<Block> {
        self.lock_chunk(ChunkPos::from(block_pos));
        let chunk_data = self.get_chunk_data_mut()?;

        Some(chunk_data.blocks.get(block_pos.as_chunk_local()))
    }

    // TODO: figure out how this will work with dirty chunk
    /*fn get_block_mut(&mut self, block_pos: BlockPos) -> Option<&mut Block> {
        self.lock_chunk(ChunkPos::from(block_pos));
        let chunk_data = self.get_chunk_data_mut()?;

        Some(chunk_data.blocks.get_mut(block_pos.as_chunk_local()))
    }*/

    fn new_block(&mut self, block_pos: BlockPos, block_type: BlockType) -> Option<Block> {
        self.lock_chunk(ChunkPos::from(block_pos));

        // do this before marking chunks as dirty to ensure lock chunk succeeded
        let inner = self.inner.as_mut()?;

        for chunk_pos in block_pos.adjacent_chunks().iter_chunks() {
            if let Some(chunk) = self.world.chunks.get(&chunk_pos) {
                chunk.mark_dirty(self.world);
            }
        }

        Some(inner.lock.blocks.new_block(block_pos.as_chunk_local(), block_type))
    }
}