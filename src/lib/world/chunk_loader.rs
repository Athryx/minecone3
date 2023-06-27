use bevy::{prelude::*, tasks::{Task, AsyncComputeTaskPool}};
use futures_lite::future;

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
pub struct ChunkLoadTask(Task<(Option<ChunkData>, Mesh)>);

/// Loads and meshes the chunk at the given position
fn load_chunk(chunk_pos: ChunkPos) -> (Option<ChunkData>, Mesh) {
    let chunk_data = generate_chunk(chunk_pos);

    let mesh = generate_mesh(chunk_data.as_ref(), block_models());

    (chunk_data, mesh)
}

/// Loads and unloads chunks based on whre chunk loaders are
pub fn queue_generate_chunks(
    mut world: ResMut<World>,
    mut loaders: Query<&mut ChunkLoader>,
    mut commands: Commands,
) {
    let compute_pool = AsyncComputeTaskPool::get();

    for mut loader in loaders.iter_mut() {
        if loader.has_changed() {
            let current_region = loader.current_loaded_region();
            let last_region = loader.last_loaded_region();

            // FIXME: don't use this naive implementation
            for chunk_pos in current_region.iter_chunks() {
                if !last_region.contains_chunk(chunk_pos) {
                    // load chunks
                    let load_task = compute_pool.spawn(async move {
                        load_chunk(chunk_pos)
                    });

                    let chunk_entity = commands.spawn((
                        EcsChunk(chunk_pos),
                        ChunkLoadTask(load_task),
                    )).id();

                    let chunk = Chunk {
                        data: None,
                        entity: chunk_entity,
                        load_count: 1,
                    };

                    world.chunks.insert(chunk_pos, chunk);
                }
            }

            for chunk_pos in last_region.iter_chunks() {
                if !current_region.contains_chunk(chunk_pos) {
                    // unload chunks
                    let mut chunk = world.chunks.get_mut(&chunk_pos).unwrap();

                    if chunk.load_count == 1 {
                        commands.entity(chunk.entity).despawn();
                        world.chunks.remove(&chunk_pos);
                    } else {
                        chunk.load_count -= 1;
                    }
                }
            }

            loader.last_position = loader.position;
            loader.last_load_distance = loader.load_distance;
        }
    }
}

pub fn poll_chunk_load_tasks(
    mut world: ResMut<World>,
    block_material: Res<GlobalBlockMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &EcsChunk, &mut ChunkLoadTask)>,
    mut commands: Commands,
) {
    for (entity, ecs_chunk, mut load_task) in query.iter_mut() {
        if let Some((chunk_data, mesh)) = future::block_on(future::poll_once(&mut load_task.0)) {
            let mesh_handle = meshes.add(mesh);

            commands.entity(entity)
                .insert(MaterialMeshBundle {
                    mesh: mesh_handle,
                    material: block_material.0.clone(),
                    transform: ecs_chunk.0.into(),
                    ..Default::default()
                })
                .remove::<ChunkLoadTask>();

            world.chunks.get_mut(&ecs_chunk.0).unwrap().data = chunk_data;
        }
    }
}