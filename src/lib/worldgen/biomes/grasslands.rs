use noise::{Fbm, MultiFractal};

use crate::blocks::BlockType;

use super::*;

#[derive(Debug)]
pub struct Grasslands {
    height_noise: Fbm<OpenSimplex>,
}

const LAYERS: BiomeLayers = BiomeLayers {
    layers: &[
        BlockLayer {
            block: BlockType::Grass,
            thickness: 1,
        },
        BlockLayer {
            block: BlockType::Dirt,
            thickness: 4,
        },
    ],
    bottom: BlockType::Stone,
};

impl BiomeGen for Grasslands {
    fn from_seed(seed_rng: &mut StdRng) -> Self {
        Grasslands {
            height_noise: Fbm::new(seed_rng.next_u32())
                .set_octaves(5)
                .set_frequency(0.01),
        }
    }

    fn biome_conditions() -> BiomeConditions {
        BiomeConditions {
            temperature: 0.0,
            humidity: -0.2,
            special_factor: 0.0,
            biome_height: 0.0,
        }
    }

    fn layers() -> BiomeLayers {
        LAYERS
    }

    fn get_height(&self, block: BlockPos, cache: &mut NoiseCache2d) -> i32 {
        (5.0 * self.height_noise.get(block.as_noise_point_2d())) as i32
    }
}