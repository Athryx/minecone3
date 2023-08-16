use std::sync::OnceLock;

use noise::OpenSimplex;
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::blocks::{BlockStorage, BlockType};
use crate::world::{ChunkData, CHUNK_SIZE};
use crate::types::*;

mod biomes;
use biomes::Biome;
mod noise_cache_2d;
use noise_cache_2d::NoiseCache2d;

use self::biomes::{BiomeMap, BiomeNoiseMap, BiomeNoiseCache, BiomeConditions};

static WORLDGEN: OnceLock<Worldgen> = OnceLock::new();

#[derive(Debug)]
pub struct Worldgen {
    seed: u64,
    biomes: Vec<Biome>,
    biome_map: BiomeMap,
    biome_noise: BiomeNoiseMap,
}

impl Worldgen {
    pub fn init(seed: u64) {
        WORLDGEN.set(Self::new(seed)).expect("worldgen already initialized");
    }

    pub fn get() -> &'static Worldgen {
        WORLDGEN.get().expect("worldgen not initialized")
    }

    fn new(seed: u64) -> Self {
        // used to generate the seeds for all noise maps
        let mut seed_rng = StdRng::seed_from_u64(seed);

        let biomes = biomes::construct_biomes(&mut seed_rng);

        let mut biome_map = BiomeMap::default();
        for biome in biomes.iter() {
            biome_map.insert_biome(biome);
        }

        Worldgen {
            seed,
            biomes,
            biome_map,
            biome_noise: BiomeNoiseMap::new(&mut seed_rng),
        }
    }

    /// Gets the biome for the given conditions
    fn get_biome(&self, biome_conditions: BiomeConditions) -> &Biome {
        &self.biomes[self.biome_map.get_biome(biome_conditions) as usize]
    }

    pub fn generate_chunk(&self, chunk_pos: ChunkPos) -> ChunkData {
        let mut blocks = BlockStorage::default();

        let mut biome_noise_cache = BiomeNoiseCache::default();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let local_block_pos = BlockPos::new(x as i32, y as i32, z as i32);
                    let block_pos = BlockPos::from(chunk_pos) + local_block_pos;

                    let biome_conditions = self.biome_noise.get(block_pos, &mut biome_noise_cache);
                    let biome = self.get_biome(biome_conditions);

                    let height = biome.get_height(block_pos);
                    let depth = block_pos.y - height;

                    blocks.new_block(local_block_pos, biome.layers().get_block_at_depth(depth));
                }
            }
        }

        blocks.into()
    }
}