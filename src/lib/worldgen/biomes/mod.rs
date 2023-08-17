use bevy::prelude::Vec4;
use noise::{OpenSimplex, NoiseFn};
use strum::EnumIter;
use rand::{RngCore, rngs::StdRng};
use derive_more::Sub;
use std::fmt::Debug;

use crate::types::BlockPos;

use super::NoiseCache2d;

mod block_layers;
use block_layers::{BiomeLayers, BlockLayer};
mod grasslands;
use grasslands::*;

/// Biome conditions specify spawn conditions of a biome
/// 
/// These are randomly generated using noise maps for each block,
/// and the biome which is closest to the generated conditions is selected as the biome for that block
/// 
/// All fields are on a range of -1 to 1
#[derive(Debug, Clone, Copy, Sub)]
pub struct BiomeConditions {
    pub temperature: f64,
    pub humidity: f64,
    pub special_factor: f64,
    pub biome_height: f64,
}

#[derive(Debug, Default)]
pub struct BiomeMap(Vec<(BiomeType, BiomeConditions)>);

impl BiomeMap {
    // TODO: maybe look into kd-trees if vec performance is not good enough
    // for low number of biomes, a vec is probably faster though
    pub fn get_biome(&self, conditions: BiomeConditions) -> BiomeType {
        let (biome_type, _) = self.0.iter()
            .map(|(biome_type, this_conditions)| {
                let diff = *this_conditions - conditions;
                let dist_squared = diff.temperature * diff.temperature
                    + diff.humidity * diff.humidity
                    + diff.special_factor * diff.special_factor
                    + diff.biome_height * diff.biome_height;

                (biome_type, dist_squared)
            })
            .reduce(|a, b| {
                if a.1 < b.1 {
                    a
                } else {
                    b
                }
            }).unwrap();
        
        *biome_type
    }

    pub fn insert_biome(&mut self, biome: &Biome) {
        self.0.push((biome.biome_type(), biome.biome_conditions()))
    }
}

/// Contains a noise cache 2d for all biome noise
#[derive(Default)]
pub struct BiomeNoiseCache {
    temperature: NoiseCache2d,
    humidity: NoiseCache2d,
    special_factor: NoiseCache2d,
    biome_height: NoiseCache2d,
}

/// Generates random biome conditions
#[derive(Debug)]
pub struct BiomeNoiseMap {
    temperature: OpenSimplex,
    humidity: OpenSimplex,
    special_factor: OpenSimplex,
    biome_height: OpenSimplex,
}

impl BiomeNoiseMap {
    pub fn new(seed_rng: &mut StdRng) -> Self {
        BiomeNoiseMap {
            temperature: OpenSimplex::new(seed_rng.next_u32()),
            humidity: OpenSimplex::new(seed_rng.next_u32()),
            special_factor: OpenSimplex::new(seed_rng.next_u32()),
            biome_height: OpenSimplex::new(seed_rng.next_u32()),
        }
    }

    pub fn get(&self, block: BlockPos, cache: &mut BiomeNoiseCache) -> BiomeConditions {
        BiomeConditions {
            temperature: cache.temperature.get(&self.temperature, block),
            humidity: cache.humidity.get(&self.humidity, block),
            special_factor: cache.special_factor.get(&self.special_factor, block),
            biome_height: cache.biome_height.get(&self.biome_height, block),
        }
    }
}

pub trait BiomeGen: Debug {
    fn from_seed(seed_rng: &mut StdRng) -> Self;

    fn biome_conditions() -> BiomeConditions;
    fn layers() -> BiomeLayers;

    /// Gets the height of the surface at the given block position
    fn get_height(&self, block: BlockPos, cache: &mut NoiseCache2d) -> i32;
}

macro_rules! register_biome {
    ($( $biomes:ident ),*,) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
        pub enum BiomeType {
            $(
                $biomes,
            )*
        }

        #[derive(Debug)]
        pub enum Biome {
            $(
                $biomes($biomes),
            )*
        }

        impl Biome {
            pub fn biome_type(&self) -> BiomeType {
                match self {
                    $(
                        Biome::$biomes(_) => BiomeType::$biomes,
                    )*
                }
            }

            pub fn biome_conditions(&self) -> BiomeConditions {
                match self {
                    $(
                        Biome::$biomes(_) => $biomes::biome_conditions(),
                    )*
                }
            }

            pub fn layers(&self) -> BiomeLayers {
                match self {
                    $(
                        Biome::$biomes(_) => $biomes::layers(),
                    )*
                }
            }

            pub fn get_height(&self, block: BlockPos, cache: &mut NoiseCache2d) -> i32 {
                match self {
                    $(
                        Biome::$biomes(biome) => biome.get_height(block, cache),
                    )*
                }
            }
        }

        /// Creates an array containing all the biomes
        /// 
        /// This array is indexed by biome type to get the corresponding biome
        pub fn construct_biomes(seed_rng: &mut StdRng) -> Vec<Biome> {
            let mut out = Vec::new();

            $(
                out.push(Biome::$biomes($biomes::from_seed(seed_rng)));
            )*

            out
        }
    };
}

register_biome! {
    Grasslands,
}