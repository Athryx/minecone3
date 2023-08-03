use bytemuck::{bytes_of, bytes_of_mut};

use crate::meshing::{BlockModel, BlockFace, BlockFaceType};

mod block_storage;
pub use block_storage::BlockStorage;
pub mod utils;
mod air;
use air::Air;
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
    fn new_inline(block_type: BlockType, hp: u16, state: u8) -> Self {
        assert!(block_type.is_inline());

        let block_type = block_type as u32;
        let hp = ((hp as u32) << 12) & INLINE_BLOCK_HP_MASK;
        let state = (state as u32) << 24;
        Block(block_type | hp | state)
    }

    fn new_from_type(block_type: BlockType) -> Self {
        Self::new_inline(block_type, block_type.properties().max_hp, 0)
    }

    // returns a reference to the state byte of an inline block
    fn state(&self) -> &u8 {
        &bytes_of(&self.0)[3]
    }

    // returns a mutable reference to the state byte of an inline block
    fn state_mut(&mut self) -> &mut u8 {
        &mut bytes_of_mut(&mut self.0)[3]
    }

    pub fn block_id(&self) -> u16 {
        (self.0 & BLOCK_ID_MASK) as u16
    }

    pub fn block_type(&self) -> BlockType {
        BlockType::from_repr(self.block_id())
            .expect("invalid block id")
    }

    pub fn is_air(&self) -> bool {
        self.block_type() == BlockType::Air
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new_inline(BlockType::Air, 0, 0)
    }
}

/// Describes the properties of the block
#[derive(Debug, Clone, Copy)]
pub struct BlockProperties {
    pub max_hp: u16,
}

// Inline and extended blocks must implement this trait
trait BaseBlock {
    fn model() -> BlockModel;
    fn properties() -> BlockProperties;
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

            pub fn properties(&self) -> BlockProperties {
                match self {
                    $(
                        Self::$inline_blocks => $inline_blocks::properties(),
                    )*
                    $(
                        Self::$extended_blocks => $extended_blocks::properties(),
                    )*
                }
            }

            pub fn is_inline(&self) -> bool {
                match self {
                    $(
                        Self::$inline_blocks => true,
                    )*
                    $(
                        Self::$extended_blocks => false,
                    )*
                }
            }
        }
    };
}

register_blocks! {
    inline {
        Air,
        Dirt,
    },
    extended {
        ,
    },
}