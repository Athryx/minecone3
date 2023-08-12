use bevy::prelude::*;

use crate::types::ChunkPos;

/// A rectangular Region of chunks
#[derive(Debug, Clone, Copy)]
pub struct ChunkRegion {
    // inclusive
    pub min_chunk: ChunkPos,
    pub size: UVec3,
}

impl ChunkRegion {
    /// Position of maximum chunk, exclusive
    pub fn max_chunk(&self) -> ChunkPos {
        ChunkPos(self.min_chunk.0 + self.size.as_ivec3())
    }

    pub fn chunk_count(&self) -> usize {
        self.size.x as usize * self.size.y as usize * self.size.z as usize
    }

    pub fn contains_chunk(&self, chunk_pos: ChunkPos) -> bool {
        let max_chunk = self.max_chunk();

        self.min_chunk.x() <= chunk_pos.x()
            && self.min_chunk.y() <= chunk_pos.y()
            && self.min_chunk.z() <= chunk_pos.z()
            && max_chunk.x() > chunk_pos.x()
            && max_chunk.y() > chunk_pos.y()
            && max_chunk.z() > chunk_pos.z()
    }

    /// Iterates chunks in this region, going along z aixis, then y axis, then x axis
    pub fn iter_chunks(&self) -> ChunkRegionIterator {
        let current_chunk = if self.size == UVec3::ZERO {
            None
        } else {
            Some(self.min_chunk)
        };

        ChunkRegionIterator {
            min_chunk: self.min_chunk,
            max_chunk: self.max_chunk(),
            current_chunk,
        }
    }

    /// Converts the chunk position to an array index if there is a flat array of all the chunks in this chunk region
    pub fn get_array_index(&self, chunk_pos: ChunkPos) -> Option<usize> {
        self.get_array_index_relative(chunk_pos - self.min_chunk)
    }

    /// Converts the chunk position to an array index if there is a flat array of all the chunks in this chunk region
    /// 
    /// Treats the bottom left corner of this region as (0, 0, 0)
    pub fn get_array_index_relative(&self, chunk_pos: ChunkPos) -> Option<usize> {
        if chunk_pos.0.is_negative_bitmask() != 0 {
            return None;
        }

        if chunk_pos.x() >= self.size.x as i32
            || chunk_pos.y() >= self.size.y as i32
            || chunk_pos.z() >= self.size.z as i32 {
            return None;
        }

        let position = chunk_pos.0.as_uvec3();
        let index = (position.x * self.size.y * self.size.z)
            + (position.y * self.size.z)
            + position.z;

        Some(index as usize)
    }
}

pub struct ChunkRegionIterator {
    min_chunk: ChunkPos,
    max_chunk: ChunkPos,
    current_chunk: Option<ChunkPos>,
}

impl Iterator for ChunkRegionIterator {
    type Item = ChunkPos;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_chunk = self.current_chunk?;
        let out = current_chunk;

        current_chunk.0.z += 1;

        if current_chunk.z() == self.max_chunk.z() {
            current_chunk.0.z = self.min_chunk.z();
            current_chunk.0.y += 1;

            if current_chunk.y() == self.max_chunk.y() {
                current_chunk.0.y = self.min_chunk.y();
                current_chunk.0.x += 1;

                if current_chunk.x() == self.max_chunk.x() {
                    self.current_chunk = None;
                    return Some(out);
                }
            }
        }

        self.current_chunk = Some(current_chunk);
        Some(out)
    }
}