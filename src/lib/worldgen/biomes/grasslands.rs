use crate::blocks::BlockType;

use super::*;

#[derive(Debug)]
pub struct Grasslands;

const LAYERS: BiomeLayers = BiomeLayers {
    layers: &[BlockLayer {
        block: BlockType::Grass,
        thickness: 1,
    }],
    bottom: BlockType::Dirt,
};

impl BiomeGen for Grasslands {
    fn from_seed(seed: u64) -> Self {
        Grasslands
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

    fn get_height(&self, block: BlockPos) -> i32 {
        0
    }
}