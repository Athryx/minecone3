use crate::blocks::{BlockStorage, BlockType};
use crate::world::ChunkData;
use crate::types::ChunkPos;

// temp
pub fn generate_chunk(chunk_pos: ChunkPos) -> ChunkData {
    /*if chunk_pos == ChunkPos::new(0, 0, 0) {
        Some(BlockStorage::new_filled(BlockType::Dirt).into())
    } else {
        None
    }*/
    if chunk_pos.y() > 0 {
        BlockStorage::default().into()
    } else {
        BlockStorage::new_filled(BlockType::Dirt).into()
    }
}