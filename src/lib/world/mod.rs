use bevy::prelude::*;

use crate::{types::ChunkPos, GameSet};

mod chunk;
pub use chunk::{Chunk, ChunkData, CHUNK_SIZE, CHUNK_BLOCK_COUNT};
mod chunk_loader;
pub use chunk_loader::ChunkLoader;
mod chunk_region;
pub use chunk_region::*;
mod locked_chunk_area;
pub use locked_chunk_area::*;
mod world;
pub use world::World;

/// A chunk component stored for the chunk entity in the EcsChunk
/// 
/// It references the position of a chunk stored in the world
#[derive(Debug, Clone, Copy, Component)]
pub struct EcsChunk(pub ChunkPos);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World>()
            .add_systems(
                Update,
                (
                    chunk_loader::move_chunk_loader,
                    // run this before queue generate chunks so it will run next frame, which will give command buffer time to flush
                    chunk_loader::poll_chunk_load_tasks.before(chunk_loader::queue_generate_chunks),
                    chunk_loader::queue_generate_chunks,
                    chunk::poll_chunk_mesh_tasks,
                ).in_set(GameSet::Main)
            )
            // do this after everything else has run
            .add_systems(PostUpdate, chunk::remesh_dirty_chunks);
    }
}