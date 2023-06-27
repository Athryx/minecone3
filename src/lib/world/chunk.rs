use bevy::prelude::*;

use crate::blocks::Block;

pub const CHUNK_SIZE: usize = 32;

#[derive(Debug)]
pub struct Chunk {
    // will be None if the chunk is air or has not finished loading yet
    pub data: Option<ChunkData>,
    pub entity: Entity,
    pub load_count: u32,
}

pub type BlockArray = [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

#[derive(Debug)]
pub struct ChunkData {
    pub blocks: Box<BlockArray>,
}