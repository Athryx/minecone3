use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

use bevy::prelude::*;
use parking_lot::RwLock;

use crate::blocks::BlockStorage;
use crate::meshing::generate_mesh;
use crate::render::block_models;
use crate::task::{TaskPool, Task};
use crate::types::ChunkPos;
use super::World;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_BLOCK_COUNT: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[derive(Debug)]
pub struct Chunk {
    // will be None if the chunk is air or has not finished loading yet
    pub data: RwLock<ChunkData>,
    pub chunk_pos: ChunkPos,
    pub entity: Entity,
    pub load_count: AtomicU32,
    /// Used to indicate if blocks have been changed but chunk has not yet been remeshed
    pub dirty: AtomicBool,
}

impl Chunk {
    /// Marks the chunk as dirty and queues a remesh job for the chunk
    pub fn mark_dirty(&self, world: &World) {
        // TODO: make sure ordering is correct
        if !self.dirty.swap(true, Ordering::AcqRel) {
            // old dirty bit was false, so push to remesh list
            // panic safety: if current chunk is dirty, current chunk should be loaded
            world.dirty_chunks.push(self.chunk_pos);
        }
    }
}

#[derive(Debug, Default)]
pub struct ChunkData {
    pub blocks: BlockStorage,
}

impl From<BlockStorage> for ChunkData {
    fn from(blocks: BlockStorage) -> Self {
        ChunkData {
            blocks,
        }
    }
}

#[derive(Component)]
pub struct ChunkMeshTask(Task<Option<Mesh>>);

pub(super) fn remesh_dirty_chunks(world: Res<World>, mut commands: Commands) {
    let task_pool = TaskPool::get();

    while let Some(dirty_chunk_pos) = world.dirty_chunks.pop() {
        let Some(chunk) = world.chunks.get(&dirty_chunk_pos).cloned() else {
            // TODO: print warning
            continue;
        };

        let chunk_entity = chunk.entity;

        let task = task_pool.spawn(move || {
            let chunk_data = chunk.data.read();
            chunk.dirty.store(false, Ordering::Release);
            generate_mesh(&chunk_data.blocks, block_models())
        });

        commands.entity(chunk_entity)
            .insert(ChunkMeshTask(task));
    }
}

pub(super) fn poll_chunk_mesh_tasks(
    mut meshes: ResMut<Assets<Mesh>>,
    tasks: Query<(Entity, &ChunkMeshTask)>,
    mut commands: Commands,
) {
    for (entity, task) in tasks.iter() {
        if let Some(mesh) = task.0.poll() {
            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<ChunkMeshTask>();

            if let Some(mesh) = mesh {
                let mesh_handle = meshes.add(mesh);
                entity_commands.insert(mesh_handle);
            }
        }
    }
}