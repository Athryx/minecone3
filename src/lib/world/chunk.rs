use bevy::prelude::*;
use parking_lot::Mutex;

use crate::blocks::BlockStorage;

pub const CHUNK_SIZE: usize = 32;

#[derive(Debug)]
pub struct Chunk {
    // will be None if the chunk is air or has not finished loading yet
    pub data: Mutex<Option<ChunkData>>,
    pub entity: Entity,
    pub load_count: u32,
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