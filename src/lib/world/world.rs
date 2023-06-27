
use bevy::{prelude::*, utils::HashMap};

use crate::types::ChunkPos;
use super::chunk::Chunk;

#[derive(Debug, Default, Resource)]
pub struct World {
    pub chunks: HashMap<ChunkPos, Chunk>,
}