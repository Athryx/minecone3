use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering, AtomicBool};

use bevy::prelude::*;
use parking_lot::RwLock;

use crate::task::{Task, TaskPool};
use crate::{types::ChunkPos, render::{GlobalBlockMaterial, block_models}, worldgen::generate_chunk, meshing::generate_mesh};
use super::{World, EcsChunk, Chunk, chunk::ChunkData};

/// A rectangular Region of chunks
#[derive(Debug, Clone, Copy)]
struct ChunkRegion {
    // inclusive
    min_chunk: ChunkPos,
    // exclusive
    max_chunk: ChunkPos,
}

impl ChunkRegion {
    fn contains_chunk(&self, chunk_pos: ChunkPos) -> bool {
        self.min_chunk.x() <= chunk_pos.x()
            && self.min_chunk.y() <= chunk_pos.y()
            && self.min_chunk.z() <= chunk_pos.z()
            && self.max_chunk.x() > chunk_pos.x()
            && self.max_chunk.y() > chunk_pos.y()
            && self.max_chunk.z() > chunk_pos.z()
    }

    fn iter_chunks(&self) -> ChunkRegionIterator {
        let current_chunk = if self.min_chunk == self.max_chunk {
            None
        } else {
            Some(self.min_chunk)
        };

        ChunkRegionIterator {
            min_chunk: self.min_chunk,
            max_chunk: self.max_chunk,
            current_chunk,
        }
    }
}

struct ChunkRegionIterator {
    min_chunk: ChunkPos,
    max_chunk: ChunkPos,
    current_chunk: Option<ChunkPos>,
}

impl Iterator for ChunkRegionIterator {
    type Item = ChunkPos;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_chunk = self.current_chunk?;
        let out = current_chunk;

        current_chunk.0.x += 1;

        if current_chunk.x() == self.max_chunk.x() {
            current_chunk.0.x = self.min_chunk.x();
            current_chunk.0.y += 1;

            if current_chunk.y() == self.max_chunk.y() {
                current_chunk.0.y = self.min_chunk.y();
                current_chunk.0.z += 1;

                if current_chunk.z() == self.max_chunk.z() {
                    self.current_chunk = None;
                    return Some(out);
                }
            }
        }

        self.current_chunk = Some(current_chunk);
        Some(out)
    }
}

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
        if load_distance.x == 0 || load_distance.y == 0 || load_distance.z == 0 {
            ChunkRegion {
                min_chunk: position,
                max_chunk: position,
            }
        } else {
            ChunkRegion {
                min_chunk: ChunkPos::new(
                    position.x() - load_distance.x as i32 + 1,
                    position.y() - load_distance.x as i32 + 1,
                    position.z() - load_distance.x as i32 + 1,
                ),
                max_chunk: ChunkPos::new(
                    position.x() + load_distance.x as i32,
                    position.y() + load_distance.x as i32,
                    position.z() + load_distance.x as i32,
                )
            }
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
                        generate_chunk(chunk_pos)
                    });

                    let chunk_entity = commands.spawn((
                        EcsChunk(chunk_pos),
                        ChunkLoadTask(load_task),
                        // All these are all parts of material mesh bundle except the mesh, which will be generated later
                        TransformBundle {
                            local: chunk_pos.into(),
                            ..Default::default()
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
        }
    }
}