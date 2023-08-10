use crate::world::{CHUNK_SIZE, CHUNK_BLOCK_COUNT};
use crate::types::BlockPos;

use super::*;

#[derive(Debug, Clone, Default)]
struct BlockStorageInner {
    blocks: Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl BlockStorageInner {
    fn new(block: Block) -> Self {
        BlockStorageInner {
            blocks: Box::new([[[block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
        }
    }

    fn get(&self, block_pos: BlockPos) -> &Block {
        &self.blocks[block_pos.x as usize][block_pos.y as usize][block_pos.z as usize]
    }

    fn get_mut(&mut self, block_pos: BlockPos) -> &mut Block {
        &mut self.blocks[block_pos.x as usize][block_pos.y as usize][block_pos.z as usize]
    }
}

#[derive(Debug, Clone)]
pub struct BlockStorage {
    inner: Option<BlockStorageInner>,
    non_air_count: usize,
}

impl Default for BlockStorage {
    fn default() -> Self {
        BlockStorage {
            inner: None,
            non_air_count: 0,
        }
    }
}

impl BlockStorage {
    /// Creates a new chunk filled with the given block type
    /// 
    /// Only works on inline blocks
    pub fn new_filled(block_type: BlockType) -> Self {
        if block_type == BlockType::Air {
            BlockStorage::default()
        } else {
            let block = Block::new_from_type(block_type);

            BlockStorage {
                inner: Some(BlockStorageInner::new(block)),
                non_air_count: CHUNK_BLOCK_COUNT,
            }
        }
    }

    /// Returns true if this block storage is only holding air
    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }

    /// Replaces the old block at the given position with a block of the new type, and returns a reference to the block
    pub fn new_block(&mut self, pos: BlockPos, block_type: BlockType) -> Block {
        let block = if block_type.is_inline() {
            Block::new_from_type(block_type)
        } else {
            todo!()
        };

        self.set(pos, block);
        block
    }

    // handles cleaning up any extended data
    fn set(&mut self, block_pos: BlockPos, block: Block) {
        if let Some(ref mut blocks) = self.inner {
            let old_block = *blocks.get(block_pos);
            if !old_block.block_type().is_inline() {
                todo!("clean up extended data")
            }

            *blocks.get_mut(block_pos) = block;
            // TODO: set up extended data

            if block.is_air() && !old_block.is_air() {
                self.non_air_count -= 1;
                if self.non_air_count == 0 {
                    // there are no more non air blocks left so get ride of block storage
                    self.inner = None;
                }
            } else if old_block.is_air() && !block.is_air() {
                self.non_air_count += 1;
            }
        } else if !block.is_air() {
            // there is currently no block storage and one must be created
            let mut block_storage = BlockStorageInner::default();

            // TODO: set up extended data
            *block_storage.get_mut(block_pos) = block;

            self.inner = Some(block_storage);
            self.non_air_count = 1;
        }
    }

    pub fn get(&self, block_pos: BlockPos) -> Block {
        if let Some(ref blocks) = self.inner {
            *blocks.get(block_pos)
        } else {
            Block::default()
        }
    }
}