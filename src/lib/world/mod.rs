use bevy::prelude::*;

use crate::{types::ChunkPos};

mod chunk;
pub use chunk::{Chunk, ChunkData, BlockArray, CHUNK_SIZE};
mod chunk_loader;
pub use chunk_loader::ChunkLoader;
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
            .add_system(chunk_loader::move_chunk_loader)
            .add_system(chunk_loader::queue_generate_chunks)
            .add_system(chunk_loader::poll_chunk_load_tasks);
    }
}