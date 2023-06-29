use crate::world::CHUNK_SIZE;
use crate::types::BlockPos;

use super::*;

#[derive(Debug, Clone, Default)]
pub struct BlockStorage {
    blocks: Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl BlockStorage {
    /// Creates a new chunk filled with the given block type
    /// 
    /// Only works on inline blocks
    pub fn new_filled(block_type: BlockType) -> Self {
        let block = Block::new_from_type(block_type);

        BlockStorage {
            blocks: Box::new([[[block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE])
        }
    }

    /// Replaces the old block at the given position with a block of the new type
    pub fn new_block(&mut self, pos: BlockPos, block_type: BlockType) {
        let block = if block_type.is_inline() {
            Block::new_from_type(block_type)
        } else {
            todo!()
        };

        self.set(pos, block);
    }

    // handles cleaning up any extended data
    fn set(&mut self, block_pos: BlockPos, block: Block) {
        let old_block = self.get_mut(block_pos);
        if !old_block.block_type().is_inline() {
            todo!("clean up extended data")
        }

        *old_block = block;
    } 

    pub fn get_mut(&mut self, block_pos: BlockPos) -> &mut Block {
        &mut self.blocks[block_pos.x as usize][block_pos.y as usize][block_pos.z as usize]
    }
}