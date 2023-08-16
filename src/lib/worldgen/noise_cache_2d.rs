use noise::NoiseFn;

use crate::world::CHUNK_SIZE;
use crate::types::*;

/// Caches noise from the given 2d noise source for use with a 3d chunk
#[derive(Debug, Default)]
pub struct NoiseCache2d<> {
    cached_data: [[Option<f64>; CHUNK_SIZE]; CHUNK_SIZE],
}

impl NoiseCache2d {
    pub fn get<T: NoiseFn<f64, 2>>(&mut self, noise: &T, block_pos: BlockPos) -> f64 {
        let block_pos_local = block_pos.as_chunk_local();
        if let Some(data) = self.cached_data[block_pos_local.x as usize][block_pos_local.z as usize] {
            data
        } else {
            let sample_pos = [
                block_pos.x as f64 + 0.5,
                block_pos.z as f64 + 0.5,
            ];

            let data = noise.get(sample_pos);
            self.cached_data[block_pos_local.x as usize][block_pos_local.z as usize] = Some(data);
            data
        }
    }
}