use crate::blocks::Block;

pub const CHUNK_SIZE: usize = 32;

#[derive(Debug)]
pub enum Chunk {
    /// used for chunks that are just full of air
    Empty,
    /// a normal chunk
    Filled(FilledChunk),
}

pub type BlockArray = [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

#[derive(Debug)]
pub struct FilledChunk {
    blocks: Box<BlockArray>,
}