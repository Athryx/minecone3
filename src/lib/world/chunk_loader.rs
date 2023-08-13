use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering, AtomicBool};

use bevy::prelude::*;
use bevy::math::Vec3A;
use bevy::render::primitives::Aabb;
use parking_lot::RwLock;

use crate::task::{Task, TaskPool};
use crate::{types::*, render::GlobalBlockMaterial, worldgen::Worldgen};
use super::CHUNK_SIZE;
use super::{World, EcsChunk, Chunk, chunk::ChunkData, ChunkRegion};

/// Something which loads in chunks in a certain distance around it
#[derive(Debug, Clone, Copy, Component)]
pub struct ChunkLoader {
    pub position: ChunkPos,
    pub load_distance: UVec3,
    last_position: ChunkPos,
    last_load_distance: UVec3,
}

impl ChunkLoader {
    pub fn new(position: ChunkPos, load_distance: UVec3) -> Self {
        ChunkLoader {
            position,
            load_distance,
            last_position: ChunkPos::new(0, 0, 0),
            last_load_distance: UVec3::new(0, 0, 0)
        }
    }

    pub fn move_to(&mut self, position: Vec3) {
        self.position = position.into();
    }

    fn chunk_region_inner(position: ChunkPos, load_distance: UVec3) -> ChunkRegion {
        let min_chunk_offset = load_distance.map(|n| if n == 0 { 0 } else { n - 1 }).as_ivec3();
        let min_chunk = ChunkPos(position.0 - min_chunk_offset);

        let size = load_distance.map(|n| if n == 0 { 0 } else { n * 2 - 1 });

        ChunkRegion {
            min_chunk,
            size,
        }
    }

    fn current_loaded_region(&self) -> ChunkRegion {
        Self::chunk_region_inner(self.position, self.load_distance)
    }

    fn last_loaded_region(&self) -> ChunkRegion {
        Self::chunk_region_inner(self.last_position, self.last_load_distance)
    }

    fn has_changed(&self) -> bool {
        self.position != self.last_position
            || self.load_distance != self.last_load_distance
    }
}

/// moves the chunk loaders to the position of the transform
pub fn move_chunk_loader(
    mut query: Query<(&mut ChunkLoader, &Transform)>,
) {
    for (mut loader, transform) in query.iter_mut() {
        loader.move_to(transform.translation);
    }
}

/// A task that is currently loading a chunk
#[derive(Component)]
pub struct ChunkLoadTask(Task<ChunkData>);

/// Loads and unloads chunks based on whre chunk loaders are
pub fn queue_generate_chunks(
    mut world: ResMut<World>,
    block_material: Res<GlobalBlockMaterial>,
    mut loaders: Query<&mut ChunkLoader>,
    mut commands: Commands,
) {
    let task_pool = TaskPool::get();

    for mut loader in loaders.iter_mut() {
        if loader.has_changed() {
            let current_region = loader.current_loaded_region();
            let last_region = loader.last_loaded_region();

            // FIXME: don't use this naive implementation
            for chunk_pos in current_region.iter_chunks() {
                if !last_region.contains_chunk(chunk_pos) {
                    // load chunks
                    let load_task = task_pool.spawn(move || {
                        Worldgen::get().generate_chunk(chunk_pos)
                    });

                    let half_chunk_size = CHUNK_SIZE as f32 * BLOCK_SIZE * 0.5;
                    let half_chunk_size = Vec3A::new(half_chunk_size, half_chunk_size, half_chunk_size);

                    let chunk_entity = commands.spawn((
                        EcsChunk(chunk_pos),
                        ChunkLoadTask(load_task),
                        // All these are all parts of material mesh bundle except the mesh, which will be generated later
                        TransformBundle {
                            local: chunk_pos.into(),
                            ..Default::default()
                        },
                        Aabb {
                            center: half_chunk_size,
                            half_extents: half_chunk_size,
                        },
                        Visibility::default(),
                        ComputedVisibility::default(),
                        block_material.0.clone(),
                    )).id();

                    let chunk = Chunk {
                        data: RwLock::new(ChunkData::default()),
                        chunk_pos,
                        entity: chunk_entity,
                        load_count: AtomicU32::new(1),
                        // chunk is not dirty because it has no blocks and has not been generated yet,
                        // so having no mesh is up to date with blocks
                        dirty: AtomicBool::new(false),
                    };

                    world.chunks.insert(chunk_pos, Arc::new(chunk));
                }
            }

            for chunk_pos in last_region.iter_chunks() {
                if !current_region.contains_chunk(chunk_pos) {
                    // unload chunks
                    let chunk = world.chunks.get(&chunk_pos).unwrap();

                    // TODO: figure out if this is right ordering
                    let load_count = chunk.load_count.fetch_sub(1, Ordering::AcqRel);
                    if load_count == 1 {
                        commands.entity(chunk.entity).despawn();
                        world.chunks.remove(&chunk_pos);
                    }
                }
            }

            loader.last_position = loader.position;
            loader.last_load_distance = loader.load_distance;
        }
    }
}

pub fn poll_chunk_load_tasks(
    world: Res<World>,
    mut query: Query<(Entity, &EcsChunk, &ChunkLoadTask)>,
    mut commands: Commands,
) {
    for (entity, ecs_chunk, load_task) in query.iter_mut() {
        if let Some(chunk_data) = load_task.0.poll() {
            commands.entity(entity).remove::<ChunkLoadTask>();

            let chunk = world.chunks.get(&ecs_chunk.0).unwrap();
            *chunk.data.write() = chunk_data;
            chunk.mark_dirty(&world);

            let adjacent_region = ChunkRegion {
                min_chunk: ChunkPos::new(-1, -1, -1),
                size: UVec3::new(3, 3, 3),
            };

            for chunk_pos in adjacent_region.iter_chunks() {
                if chunk_pos != ChunkPos::ZERO {
                    if let Some(chunk) = world.chunks.get(&(ecs_chunk.0 + chunk_pos)) {
                        chunk.mark_dirty(&world);
                    }
                }
            }
        }
    }
}