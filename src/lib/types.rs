use std::hash::BuildHasherDefault;

use bevy::math::IVec3;
use dashmap::DashMap;
use rustc_hash::FxHasher;

pub type FxDashMap<K, V> = DashMap<K, V, BuildHasherDefault<FxHasher>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos(IVec3);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos(IVec3);