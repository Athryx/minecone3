use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

use bevy::prelude::*;
use parking_lot::RwLock;

use crate::blocks::BlockStorage;
use crate::meshing::{generate_mesh, ChunkMeshData};
use crate::render::block_models;
use crate::task::{TaskPool, Task};
use crate::types::ChunkPos;
use super::{World, ChunkRegion, OwnedChunkArea};

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
        let remesh_region = ChunkRegion {
            min_chunk: dirty_chunk_pos - ChunkPos::new(1, 1, 1),
            size: UVec3::new(3, 3, 3),
        };

        let Some(owned_chunk_area) = OwnedChunkArea::new(&world, remesh_region) else {
            // adjacent chunks did not exist, so the chunk was skipped, but since it was removed from the dirty list,
            // it needs to be marked dirty so it can be remeshed when its neigbors are loaded in the future
            if let Some(chunk) = world.chunks.get(&dirty_chunk_pos) {
                chunk.dirty.store(false, Ordering::Release);
            }

            continue;
        };

        let chunk_entity = owned_chunk_area
            .get_chunk_relative(ChunkPos::new(1, 1, 1))
            .unwrap()
            .entity;

        let task = task_pool.spawn(move || {
            let mesh_data = ChunkMeshData::new(owned_chunk_area.read());

            owned_chunk_area
                .get_chunk_relative(ChunkPos::new(1, 1, 1))
                .unwrap()
                .dirty
                .store(false, Ordering::Release);

            generate_mesh(&mesh_data, block_models())
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