use bytemuck::{bytes_of, bytes_of_mut};

use crate::meshing::{BlockModel, BlockFace, BlockFaceType, TextureIdentifier};

pub mod utils;
mod dirt;
use dirt::Dirt;

const BLOCK_ID_MASK: u32 = 0xfff;
const INLINE_BLOCK_HP_MASK: u32 = 0xfff000;
const DATA_ID_MASK: u32 = 0xfffff000;

/// Block data that is stored in each chunk's and constructs block array
/// 
/// There are 2 types of blocks
/// 
/// - inline
///     - bits 0-12: block
///     - bits 12-24: block hp
///     - bits 24-32: state byte (used differenctly by different blocks)
/// - extended data (used when block needs more than 1 byte of state to be stored)
///     - bits 0-12: block
///     - bits 12-32: extended data id
/// 
/// each chunk and construct has an array of extended data, which the extended data id is an index for
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Block(u32);

impl Block {
    // returns a reference to the state byte of an inline block
    fn state(&self) -> &u8 {
        &bytes_of(&self.0)[3]
    }

    // returns a mutable reference to the state byte of an inline block
    fn state_mut(&mut self) -> &mut u8 {
        &mut bytes_of_mut(&mut self.0)[3]
    }
}

// Inline and extended blocks must implement this trait
trait BaseBlock {
    fn model() -> BlockModel;
}

/// Blocks which don't need any extra state should implement this trait
trait InlineBlock: BaseBlock {
}

/// Blocks which do need extra data should use this trait
trait ExtendedBlock: BaseBlock {
}

macro_rules! register_blocks {
    (
        inline {
            $( $inline_blocks:ident ),*,
        },
        extended {
            $( $extended_blocks:ident ),*,
        },
    ) => {
        use strum::{FromRepr, EnumIter};

        #[repr(u16)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter)]
        pub enum BlockType {
            $(
                $inline_blocks,
            )*
            $(
                $extended_blocks,
            )*
        }

        impl BlockType {
            pub fn model(&self) -> BlockModel {
                match self {
                    $(
                        Self::$inline_blocks => $inline_blocks::model(),
                    )*
                    $(
                        Self::$extended_blocks => $extended_blocks::model(),
                    )*
                }
            }
        }
    };
}

register_blocks! {
    inline {
        Dirt,
    },
    extended {
        ,
    },
}