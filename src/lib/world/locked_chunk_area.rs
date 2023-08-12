use std::sync::Arc;

use bevy::prelude::*;
use parking_lot::RwLockReadGuard;

use super::{ChunkData, ChunkRegion, World, Chunk};
use crate::types::ChunkPos;

pub struct OwnedChunkArea {
    chunks: Vec<Arc<Chunk>>,
    region: ChunkRegion,
}

impl OwnedChunkArea {
    pub fn new(world: &World, region: ChunkRegion) -> Option<Self> {
        let mut chunks = Vec::with_capacity(region.chunk_count());
        for chunk in region.iter_chunks() {
            chunks.push(world.chunks.get(&chunk)?.clone());
        }

        Some(OwnedChunkArea {
            chunks,
            region,
        })
    }

    pub fn read(&self) -> LockedChunkArea {
        let chunks = self.chunks.iter()
            .map(|chunk| chunk.data.read())
            .collect::<Vec<_>>();

        LockedChunkArea {
            chunks,
            region: self.region,
        }
    }

    pub fn region(&self) -> ChunkRegion {
        self.region
    }

    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<&Chunk> {
        Some(&self.chunks[self.region.get_array_index(chunk_pos)?])
    }

    pub fn get_chunk_relative(&self, chunk_pos: ChunkPos) -> Option<&Chunk> {
        Some(&self.chunks[self.region.get_array_index_relative(chunk_pos)?])
    }
}

pub struct LockedChunkArea<'chunks> {
    chunks: Vec<RwLockReadGuard<'chunks, ChunkData>>,
    region: ChunkRegion,
}

impl<'chunks> LockedChunkArea<'chunks> {
    pub fn new(world: &'chunks World, region: ChunkRegion) -> Option<Self> {
        let mut chunks = Vec::with_capacity(region.chunk_count());
        for chunk in region.iter_chunks() {
            chunks.push(world.chunks.get(&chunk)?.data.read());
        }

        Some(LockedChunkArea {
            chunks,
            region,
        })
    }

    pub fn region(&self) -> ChunkRegion {
        self.region
    }

    pub fn get_chunk_data(&self, chunk_pos: ChunkPos) -> Option<&ChunkData> {
        Some(&self.chunks[self.region.get_array_index(chunk_pos)?])
    }

    pub fn get_chunk_data_relative(&self, chunk_pos: ChunkPos) -> Option<&ChunkData> {
        Some(&self.chunks[self.region.get_array_index_relative(chunk_pos)?])
    }
}