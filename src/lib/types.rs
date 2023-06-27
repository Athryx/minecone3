use std::hash::BuildHasherDefault;

use bevy::prelude::*;
use derive_more::{Deref, DerefMut, Add, Sub, Mul, Div};
use dashmap::DashMap;
use rustc_hash::FxHasher;

use crate::world::CHUNK_SIZE;

pub type FxDashMap<K, V> = DashMap<K, V, BuildHasherDefault<FxHasher>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Add, Sub, Mul, Div)]
pub struct ChunkPos(pub IVec3);

impl ChunkPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        ChunkPos(IVec3::new(x, y, z))
    }

    pub fn x(&self) -> i32 {
        self.0.x
    }

    pub fn y(&self) -> i32 {
        self.0.y
    }

    pub fn z(&self) -> i32 {
        self.0.z
    }
}

impl From<BlockPos> for ChunkPos {
    fn from(block_pos: BlockPos) -> Self {
        ChunkPos(IVec3::from_array(block_pos.0.to_array().map(|elem| {
            if elem >= 0 {
                elem / CHUNK_SIZE as i32
            } else {
                (elem - (CHUNK_SIZE as i32 - 1)) / CHUNK_SIZE as i32
            }
        })))
    }
}

impl From<Vec3> for ChunkPos {
    fn from(position: Vec3) -> Self {
        BlockPos::from(position).into()
    }
}

impl From<ChunkPos> for Vec3 {
    fn from(chunk_pos: ChunkPos) -> Self {
        BlockPos::from(chunk_pos).into()
    }
}

impl From<ChunkPos> for Transform {
    fn from(chunk_pos: ChunkPos) -> Self {
        Transform::from_translation(chunk_pos.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Add, Sub, Mul, Div)]
pub struct BlockPos(pub IVec3);

impl From<ChunkPos> for BlockPos {
    fn from(chunk_pos: ChunkPos) -> Self {
        BlockPos(chunk_pos.0 * CHUNK_SIZE as i32)
    }
}

impl From<Vec3> for BlockPos {
    fn from(position: Vec3) -> Self {
        BlockPos((2.0 * position).floor().as_ivec3())
    }
}

impl From<BlockPos> for Vec3 {
    fn from(block_pos: BlockPos) -> Self {
        block_pos.0.as_vec3() / 2.0
    }
}