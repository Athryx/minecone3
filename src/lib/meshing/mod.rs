use bevy::asset::AssetPath;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use strum::{FromRepr, EnumIter};

use crate::blocks::utils::Rotation;
use crate::render::{ATTRIBUTE_UV_BASE, ATTRIBUTE_FACE_COUNT};
use crate::world::ChunkData;

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
enum FaceDirection {
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

pub fn generate_mesh(chunk: Option<&ChunkData>, models: &[BlockModel]) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut positions = Vec::new();
    let mut uv_base = Vec::new();
    let mut face_count = Vec::new();
    let mut index_buffer = Vec::new();

    let mut insert_face = |face_data, face_direction| {
        
    };

    if let Some(chunk_data) = chunk {

    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(ATTRIBUTE_UV_BASE, uv_base);
    mesh.insert_attribute(ATTRIBUTE_FACE_COUNT, face_count);

    mesh.set_indices(Some(Indices::U32(index_buffer)));

    mesh
}