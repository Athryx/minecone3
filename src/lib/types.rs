use std::ops::{Index, IndexMut};

use bevy::prelude::*;
use derive_more::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use crate::world::{CHUNK_SIZE, ChunkRegion};

/// Size of block in meters
pub const BLOCK_SIZE: f32 = 0.5;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum VecAxis {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deref, DerefMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign)]
pub struct ChunkPos(pub IVec3);

impl ChunkPos {
    pub const ZERO: Self = ChunkPos::new(0, 0, 0);
    pub const X: Self = ChunkPos::new(1, 0, 0);
    pub const NEG_X: Self = ChunkPos::new(-1, 0, 0);
    pub const Y: Self = ChunkPos::new(0, 1, 0);
    pub const NEG_Y: Self = ChunkPos::new(0, -1, 0);
    pub const Z: Self = ChunkPos::new(0, 0, 1);
    pub const NEG_Z: Self = ChunkPos::new(0, 0, -1);

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
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

impl Index<VecAxis> for ChunkPos {
    type Output = i32;

    fn index(&self, index: VecAxis) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<VecAxis> for ChunkPos {
    fn index_mut(&mut self, index: VecAxis) -> &mut Self::Output {
        &mut self.0[index as usize]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deref, DerefMut, Add, Sub, Mul, Div)]
pub struct BlockPos(pub IVec3);

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        BlockPos(IVec3::new(x, y, z))
    }

    pub fn is_chunk_local(&self) -> bool {
        const CSIZE: i32 = CHUNK_SIZE as i32;
        self.x >= 0 && self.x < CSIZE
            && self.y >= 0 && self.y < CSIZE
            && self.z >= 0 && self.z < CSIZE
    }

    pub fn as_chunk_local(&self) -> Self {
        BlockPos(self.0.map(|elem| {
            if elem >= 0 {
                elem % CHUNK_SIZE as i32
            } else {
                CHUNK_SIZE as i32 + ((elem + 1) % CHUNK_SIZE as i32) - 1
            }
        }))
    }

    // Gets a chunk region of all the chunks which this block is adjcent to (including diagonally)
    pub fn adjacent_chunks(&self) -> ChunkRegion {
        let local_pos = self.as_chunk_local();

        let mut chunk_pos = ChunkPos::from(*self);
        let mut size = UVec3::new(1, 1, 1);

        if local_pos.x == 0 {
            chunk_pos -= ChunkPos::X;
            size += UVec3::X;
        } else if local_pos.x == CHUNK_SIZE as i32 - 1 {
            size += UVec3::X;
        }

        if local_pos.y == 0 {
            chunk_pos -= ChunkPos::Y;
            size += UVec3::Y;
        } else if local_pos.y == CHUNK_SIZE as i32 - 1 {
            size += UVec3::Y;
        }

        if local_pos.z == 0 {
            chunk_pos -= ChunkPos::Z;
            size += UVec3::Z;
        } else if local_pos.z == CHUNK_SIZE as i32 - 1 {
            size += UVec3::Z;
        }

        ChunkRegion {
            min_chunk: chunk_pos,
            size,
        }
    }

    pub fn as_noise_point_2d(&self) -> [f64; 2] {
        [self.0.x as f64, self.0.z as f64]
    }

    pub fn as_noise_point_3d(&self) -> [f64; 3] {
        [self.0.x as f64, self.0.y as f64, self.0.z as f64]
    }
}

impl Index<VecAxis> for BlockPos {
    type Output = i32;

    fn index(&self, index: VecAxis) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<VecAxis> for BlockPos {
    fn index_mut(&mut self, index: VecAxis) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

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
        block_pos.0.as_vec3() * BLOCK_SIZE
    }
}

pub trait VecExt {
    type NumType;

    fn map<F: FnMut(Self::NumType) -> Self::NumType>(&self, f: F) -> Self;
}

impl VecExt for Vec3 {
    type NumType = f32;

    fn map<F: FnMut(Self::NumType) -> Self::NumType>(&self, mut f: F) -> Self {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }
}

impl Index<VecAxis> for Vec3 {
    type Output = f32;

    fn index(&self, index: VecAxis) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<VecAxis> for Vec3 {
    fn index_mut(&mut self, index: VecAxis) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl VecExt for IVec3 {
    type NumType = i32;

    fn map<F: FnMut(Self::NumType) -> Self::NumType>(&self, mut f: F) -> Self {
        IVec3::new(f(self.x), f(self.y), f(self.z))
    }
}

impl Index<VecAxis> for IVec3 {
    type Output = i32;

    fn index(&self, index: VecAxis) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<VecAxis> for IVec3 {
    fn index_mut(&mut self, index: VecAxis) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl VecExt for UVec3 {
    type NumType = u32;

    fn map<F: FnMut(Self::NumType) -> Self::NumType>(&self, mut f: F) -> Self {
        UVec3::new(f(self.x), f(self.y), f(self.z))
    }
}

impl Index<VecAxis> for UVec3 {
    type Output = u32;

    fn index(&self, index: VecAxis) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<VecAxis> for UVec3 {
    fn index_mut(&mut self, index: VecAxis) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[macro_export]
macro_rules! vec3_map_many {
    ($f:expr, $out_vec:ident, $( $vecs:expr ),+) => {
        $out_vec::new($f(
            $(
                $vecs.x,
            )*
        ),
        $f(
            $(
                $vecs.y,
            )*
        ),
        $f(
            $(
                $vecs.z,
            )*
        ))
    };
}

pub trait TransformExt {
    fn to_ray(&self) -> Ray;
}

impl TransformExt for Transform {
    fn to_ray(&self) -> Ray {
        Ray {
            origin: self.translation,
            direction: self.forward(),
        }
    }
}

impl TransformExt for GlobalTransform {
    fn to_ray(&self) -> Ray {
        Ray {
            origin: self.translation(),
            direction: self.forward(),
        }
    }
}