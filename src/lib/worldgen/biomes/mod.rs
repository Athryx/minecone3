use bevy::prelude::Vec4;

/// A biome point represents a point in 4d space, with each axis representing a different biome characteristic
/// 
/// These are randomly generated using noise maps for each block,
/// and the biome which is closest to the generated point is selected as the biome for that block
/// 
/// All fields except height are on a range of -1 to 1
pub struct BiomePoint {
    pub temperature: f32,
    pub humidity: f32,
    pub height: f32,
    pub special_factor: f32,
}

pub struct BiomeMap {

}