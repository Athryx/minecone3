use bevy::prelude::*;

use crate::types::{FxDashMap, ChunkPos};
use chunk::Chunk;

mod chunk;
pub use chunk::BlockArray;

#[derive(Debug, Default, Resource)]
pub struct World {
    chunks: FxDashMap<ChunkPos, Chunk>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World>();
    }
}