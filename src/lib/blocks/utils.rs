//! Various utilities for different blocks to use

use crate::meshing::{BlockFace, BlockModel};

/// Rotation counterclockwise
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    Deg0 = 0,
    Deg90 = 1,
    Deg180 = 2,
    Deg270 = 3,
}

impl From<Rotation> for u8 {
    fn from(value: Rotation) -> Self {
        value as u8
    }
}