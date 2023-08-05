use std::sync::atomic::AtomicU32;

use bevy::prelude::*;
use parking_lot::Mutex;

use crate::blocks::BlockStorage;
use crate::meshing::generate_mesh;
use crate::render::block_models;
use crate::task::{TaskPool, Task};
use super::World;

pub const CHUNK_SIZE: usize = 32;

#[derive(Debug)]
pub struct Chunk {
    // will be None if the chunk is air or has not finished loading yet
    pub data: Mutex<Option<ChunkData>>,
    pub entity: Entity,
    pub load_count: AtomicU32,
}

#[derive(Debug)]
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
            let mut chunk_data = chunk.data.lock();

            if let Some(chunk_data) = &mut *chunk_data {
                chunk_data.blocks.clear_dirty();

                Some(generate_mesh(&chunk_data.blocks, block_models()))
            } else {
                None
            }
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