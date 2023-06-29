use crate::blocks::{BlockStorage, BlockType};
use crate::world::ChunkData;
use crate::types::ChunkPos;

// temp
pub fn generate_chunk(chunk_pos: ChunkPos) -> Option<ChunkData> {
    if chunk_pos.y() > 0 {
        None
    } else {
        Some(BlockStorage::new_filled(BlockType::Dirt).into())
    }
}