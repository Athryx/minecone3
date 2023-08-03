
use bevy::{prelude::*, utils::HashMap};

use crate::{types::*, vec3_map_many, blocks::Block};
use super::chunk::Chunk;

#[derive(Debug, Default, Resource)]
pub struct World {
    pub chunks: HashMap<ChunkPos, Chunk>,
}

#[derive(Debug)]
pub struct RayHitInfo {
    position: Vec3,
    block_pos: BlockPos,
}

impl World {
    pub fn get_block(&self, block_pos: BlockPos) -> Option<&Block> {
        let chunk = self.chunks.get(&ChunkPos::from(block_pos))?;
        Some(chunk.data.as_ref()?.blocks.get(block_pos.as_chunk_local()))
    }

    pub fn raycast(&self, ray_start_position: Vec3, ray: Vec3) -> Option<RayHitInfo> {
        let max_length = ray.length();
        let ray = ray.normalize();

        let mut block_pos = BlockPos::from(ray_start_position);

        let direction = ray.signum().as_ivec3();

        // distance it would take for each ray to travel 1 block for each axis
        let intercept_time_interval = ray.map(|elem| {
            if elem != 0.0 {
                (BLOCK_SIZE / elem).abs()
            } else {
                f32::INFINITY
            }
        });

        // offset in block of starting position
        let ray_offset = ray_start_position.map(|elem| {
            if elem > 0.0 {
                elem % BLOCK_SIZE
            } else {
                BLOCK_SIZE - (elem % BLOCK_SIZE)
            }
        });

        let mut next_intercept_time = vec3_map_many!(|ray_elem, ray_offset_elem: f32| {
            if ray_elem > 0.0 {
                (BLOCK_SIZE - ray_offset_elem) / ray_elem
            } else if ray_elem < 0.0 {
                -ray_offset_elem / ray_elem
            } else {
                f32::INFINITY
            }
        }, Vec3, ray, ray_offset);

        loop {
            let next_intercept_axis = if next_intercept_time.x < next_intercept_time.y && next_intercept_time.x < next_intercept_time.z {
                VecAxis::X
            } else if next_intercept_time.y < next_intercept_time.z {
                VecAxis::Y
            } else {
                VecAxis::Z
            };

            block_pos[next_intercept_axis] += direction[next_intercept_axis];

            let current_time = next_intercept_time[next_intercept_axis];
            next_intercept_time[next_intercept_axis] += intercept_time_interval[next_intercept_axis];

            // ray has gone on for too long, no intercept
            if current_time > max_length {
                return None;
            }

            // ray has hit
            if let Some(block) = self.get_block(block_pos) && !block.is_air() {
                return Some(RayHitInfo {
                    position: ray_start_position + current_time * ray,
                    block_pos,
                });
            }
        }
    }
}