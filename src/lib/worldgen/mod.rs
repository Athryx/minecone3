use std::sync::OnceLock;

use crate::blocks::{BlockStorage, BlockType};
use crate::world::{ChunkData, CHUNK_SIZE};
use crate::types::*;

static WORLDGEN: OnceLock<Worldgen> = OnceLock::new();

#[derive(Debug)]
pub struct Worldgen {
    seed: u64,
}

impl Worldgen {
    pub fn init(seed: u64) {
        WORLDGEN.set(Self::new(seed)).expect("worldgen already initialized");
    }

    pub fn get() -> &'static Worldgen {
        WORLDGEN.get().expect("worldgen not initialized")
    }

    fn new(seed: u64) -> Self {
        Worldgen {
            seed,
        }
    }

    pub fn generate_chunk(&self, chunk_pos: ChunkPos) -> ChunkData {
        if chunk_pos.y() >= 0 {
            BlockStorage::default().into()
        } else if chunk_pos.y() == -1 {
            let mut out = BlockStorage::new_filled(BlockType::Dirt);

            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_pos = BlockPos::new(x as i32, CHUNK_SIZE as i32 - 1, z as i32);
                    out.new_block(block_pos, BlockType::Grass);
                }
            }

            out.into()
        } else {
            BlockStorage::new_filled(BlockType::Dirt).into()
        }
    }
}