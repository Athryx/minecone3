use crate::world::CHUNK_SIZE;
use crate::types::BlockPos;

use super::*;

#[derive(Debug, Clone, Default)]
pub struct BlockStorage {
    blocks: Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    /// Used to indicate if blocks have been changed but chunk has not yet been remeshed
    dirty: bool,
}

impl BlockStorage {
    /// Creates a new chunk filled with the given block type
    /// 
    /// Only works on inline blocks
    pub fn new_filled(block_type: BlockType) -> Self {
        let block = Block::new_from_type(block_type);

        BlockStorage {
            blocks: Box::new([[[block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
            dirty: false,
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Replaces the old block at the given position with a block of the new type, and returns a reference to the block
    pub fn new_block(&mut self, pos: BlockPos, block_type: BlockType) -> &mut Block {
        let block = if block_type.is_inline() {
            Block::new_from_type(block_type)
        } else {
            todo!()
        };

        self.set(pos, block)
    }

    // handles cleaning up any extended data
    fn set(&mut self, block_pos: BlockPos, block: Block) -> &mut Block {
        // mark as dirty since blocks have changed
        // TODO: maybe only set this if blocks are actually changing,
        // but it shouldn't happen often that set is called but no blocks are changed
        self.dirty = true;

        let old_block = self.get_mut(block_pos);
        if !old_block.block_type().is_inline() {
            todo!("clean up extended data")
        }

        *old_block = block;
        old_block
    }

    pub fn get(&self, block_pos: BlockPos) -> &Block {
        &self.blocks[block_pos.x as usize][block_pos.y as usize][block_pos.z as usize]
    }

    pub fn get_mut(&mut self, block_pos: BlockPos) -> &mut Block {
        &mut self.blocks[block_pos.x as usize][block_pos.y as usize][block_pos.z as usize]
    }
}