use bevy::asset::AssetPath;
use bevy::prelude::*;
use strum::{FromRepr, EnumIter};

use crate::blocks::utils::Rotation;
use crate::world::{BlockArray, ChunkData};

#[derive(Debug, Clone, Copy)]
pub struct TexCoords(pub Vec2);

/// Represents a certain texture for a block
/// 
/// Blocks should use the Path variant, the texture atlas will be stitched together at runtime,
/// and the texture identifier will be converted to coordinates
#[derive(Debug, Clone, Copy)]
pub enum TextureIdentifier {
    Path(&'static str),
    Coordinates(TexCoords),
}

/// Represents the type of a block face
#[derive(Debug, Clone, Copy)]
pub enum BlockFaceType {
    /// A square side of a block textured with the given texture
    Full(TextureIdentifier),
    /// The side of a sloped block
    HalfSlope,
    /// The block has now block face, either if it is air or has a custom model
    Empty,
}

/// A face of a block
#[derive(Debug, Clone, Copy)]
pub struct BlockFace {
    pub rotation: Rotation,
    pub face_type: BlockFaceType,
}

/// The model of a block
#[derive(Debug, Clone, Copy)]
pub struct BlockModel {
    pub faces: [BlockFace; 6],
    //custom_model: Option<TODO>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter)]
enum Face {
    Front,
    Back,
    Top,
    Bottom,
    Left,
    Right,
}


/// All the models used by blocks, index the block model vec with the blocks id to get it model
#[derive(Debug, Default, Resource)]
pub struct BlockModels(pub Vec<BlockModel>);

pub fn generate_mesh(blocks: Option<&ChunkData>, models: &[BlockModel]) -> Mesh {
    todo!()
}