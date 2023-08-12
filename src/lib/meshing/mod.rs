use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use strum::{FromRepr, EnumIter, IntoEnumIterator};

use crate::blocks::{BlockStorage, BlockType, Block};
use crate::blocks::utils::Rotation;
use crate::render::{ATTRIBUTE_UV_BASE, ATTRIBUTE_FACE_COUNT};
use crate::world::{CHUNK_SIZE, LockedChunkArea};
use crate::types::*;

mod chunk_area;

/// All the models used by blocks, index the block model vec with the blocks id to get it model
#[derive(Debug, Default, Resource)]
pub struct BlockModels(pub Vec<BlockModel>);

/// Represents the type of a block face
#[derive(Debug, Clone, Copy)]
pub enum BlockFaceType {
    /// A square side of a block with the given texture
    Full(&'static str),
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
    // will be none if the block has no texture data, like air
    pub texture_data: Option<FaceTextureData>,
}

#[derive(Debug, Clone, Copy)]
pub struct FaceTextureData {
    /// position of top left corner of this faces texture in uv map
    pub uv_base: Vec2,
    /// index in the texture map, used to determine if 2 faces are the same
    pub texture_map_index: usize,
}

impl BlockFace {
    fn is_visible(&self) -> bool {
        matches!(self.face_type, BlockFaceType::Full(_))
    }

    /// Returns true if this face can be merged with the other face in the greedy meshing algorithm
    fn can_merge_with(&self, other: &BlockFace) -> bool {
        // if either of these do not have texture faces, they are air and cannot be merged
        let Some(this_texture_data) = self.texture_data else {
            return false;
        };

        let Some(other_texture_data) = other.texture_data else {
            return false;
        };

        this_texture_data.texture_map_index == other_texture_data.texture_map_index
            && self.rotation == other.rotation
    }

    /// True if this face will completely hid any faces behind it
    fn is_occluder(&self) -> bool {
        matches!(self.face_type, BlockFaceType::Full(_))
    }
}

/// The model of a block
#[derive(Debug, Clone, Copy)]
pub struct BlockModel {
    pub faces: [BlockFace; 6],
    //custom_model: Option<TODO>,
}

impl BlockModel {
    fn get_face(&self, face: FaceDirection) -> BlockFace {
        self.faces[face as usize]
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter)]
enum FaceDirection {
    /// Z positive
    Front,
    /// Z negative
    Back,
    /// Y positive
    Top,
    /// Y negative
    Bottom,
    /// X negative
    Left,
    /// X positive
    Right,
}

impl FaceDirection {
    fn opposite_face(&self) -> FaceDirection {
        match self {
            Self::Front => Self::Back,
            Self::Back => Self::Front,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left
        }
    }
}

/// Marks blocks that have been checked by the meshing algorithm
struct VisitedBlockMap(Box<[bool; CHUNK_SIZE * CHUNK_SIZE]>);

impl VisitedBlockMap {
    fn new() -> Self {
        VisitedBlockMap(Box::new([false; CHUNK_SIZE * CHUNK_SIZE]))
    }

    fn clear(&mut self) {
        self.0.fill(false);
    }

    fn is_visited(&self, x: i32, y: i32) -> bool {
        self.0[x as usize * CHUNK_SIZE + y as usize]
    }

    fn visit(&mut self, x: i32, y: i32) {
        self.0[x as usize * CHUNK_SIZE + y as usize] = true;
    }
}

#[derive(Debug, Default)]
struct MeshBuffers {
    position_buffer: Vec<[f32; 3]>,
    uv_base_buffer: Vec<[f32; 2]>,
    face_count_buffer: Vec<[f32; 2]>,
    index_buffer: Vec<u32>,
}

impl MeshBuffers {
    fn is_empty(&self) -> bool {
        self.index_buffer.is_empty()
    }
}

#[derive(Debug)]
struct FaceMeshData {
    tl_vertex: Vec3,
    tr_vertex: Vec3,
    bl_vertex: Vec3,
    br_vertex: Vec3,
    uv_base: Vec2,
    face_count: Vec2,
    rotation: Rotation,
}

impl FaceMeshData {
    fn new(face: BlockFace, position: BlockPos, face_count: Vec2, face_direction: FaceDirection) -> Self {
        let position = Vec3::from(position);

        let fx = face_count.x * BLOCK_SIZE;
        let fy = face_count.y * BLOCK_SIZE;

        let (tl_vertex, tr_vertex, bl_vertex, br_vertex) = match face_direction {
            FaceDirection::Front => (
                // when looking at front, up is direction of positive y axis
                Vec3::new(fx, fy, BLOCK_SIZE),
                Vec3::new(0.0, fy, BLOCK_SIZE),
                Vec3::new(fx, 0.0, BLOCK_SIZE),
                Vec3::new(0.0, 0.0, BLOCK_SIZE),
            ),
            FaceDirection::Back => (
                // when looking at back, up is direction of positive y axis
                Vec3::new(0.0, fy, 0.0),
                Vec3::new(fx, fy, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(fx, 0.0, 0.0),
            ),
            FaceDirection::Top => (
                // when looking at top, up is direction of positive z axis
                Vec3::new(0.0, BLOCK_SIZE, fy),
                Vec3::new(fx, BLOCK_SIZE, fy),
                Vec3::new(0.0, BLOCK_SIZE, 0.0),
                Vec3::new(fx, BLOCK_SIZE, 0.0),
            ),
            FaceDirection::Bottom => (
                // when looking at bottom, up is direction of positive z axis
                Vec3::new(fx, 0.0, fy),
                Vec3::new(0.0, 0.0, fy),
                Vec3::new(fx, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            ),
            FaceDirection::Left => (
                // when looking at laft, up is direction of positive y axis
                Vec3::new(0.0, fx, fy),
                Vec3::new(0.0, fx, 0.0),
                Vec3::new(0.0, 0.0, fy),
                Vec3::new(0.0, 0.0, 0.0),
            ),
            FaceDirection::Right => (
                // when looking at right, up is direction of positive y axis
                Vec3::new(BLOCK_SIZE, fx, 0.0),
                Vec3::new(BLOCK_SIZE, fx, fy),
                Vec3::new(BLOCK_SIZE, 0.0, 0.0),
                Vec3::new(BLOCK_SIZE, 0.0, fy),
            ),
        };

        // fix distorted textures because face count axis are in the wrong order for left and right sides
        let face_count = match face_direction {
            FaceDirection::Left
                | FaceDirection::Right => Vec2::new(face_count.y, face_count.x),
            FaceDirection::Top
                | FaceDirection::Bottom
                | FaceDirection::Front
                | FaceDirection::Back => face_count,
        };

        FaceMeshData {
            tl_vertex: tl_vertex + position,
            tr_vertex: tr_vertex + position,
            bl_vertex: bl_vertex + position,
            br_vertex: br_vertex + position,
            uv_base: face.texture_data.expect("uv not created for texture map yet").uv_base,
            face_count,
            rotation: face.rotation,
        }
    }

    fn insert_into_bufers(&self, buffers: &mut MeshBuffers) {
        let index_base = buffers.position_buffer.len() as u32;

        buffers.position_buffer.extend_from_slice(&[
            self.tl_vertex.into(),
            self.tr_vertex.into(),
            self.br_vertex.into(),
            self.bl_vertex.into(),
        ]);

        buffers.uv_base_buffer.extend_from_slice(&[self.uv_base.into(); 4]);

        let face_count = self.face_count;
        let face_count_verticies = match self.rotation {
            Rotation::Deg0 => [[0.0, 0.0], [face_count.x, 0.0], [face_count.x, face_count.y], [0.0, face_count.y]],
            Rotation::Deg90 => [[0.0, face_count.y], [0.0, 0.0], [face_count.x, 0.0], [face_count.x, face_count.y]],
            Rotation::Deg180 => [[face_count.x, face_count.y], [0.0, face_count.y], [0.0, 0.0], [face_count.x, 0.0]],
            Rotation::Deg270 => [[face_count.x, 0.0], [face_count.x, face_count.y], [0.0, face_count.y], [0.0, 0.0]],
        };

        buffers.face_count_buffer.extend_from_slice(&face_count_verticies);

        buffers.index_buffer.extend_from_slice(&[0, 1, 2, 2, 3, 0].map(|n| n + index_base));
    }
}

pub struct ChunkMeshData<'a>(LockedChunkArea<'a>);

impl<'a> ChunkMeshData<'a> {
    pub fn new(chunk_area: LockedChunkArea<'a>) -> Self {
        assert!(chunk_area.region().size == UVec3::new(3, 3, 3));

        ChunkMeshData(chunk_area)
    }

    fn is_empty(&self) -> bool {
        self.0.get_chunk_data_relative(ChunkPos::new(1, 1, 1)).unwrap().blocks.is_empty()
    }

    fn get(&self, block_pos: BlockPos) -> Block {
        let chunk_pos = ChunkPos::from(block_pos) + ChunkPos::new(1, 1, 1);

        self.0.get_chunk_data_relative(chunk_pos).unwrap().blocks.get(block_pos.as_chunk_local())
    }
}

/// Generates a mesh for the given chunk, or returns None if the mesh has no faces
// An empty mesh cannot be used here because the custom shader needs all the attributes to exist,
// and if an attribute exists but it has an empty array, this causes a ton of lag in bevy for some reason
pub fn generate_mesh(blocks: &ChunkMeshData, models: &[BlockModel]) -> Option<Mesh> {
    if blocks.is_empty() {
        return None;
    }

    let mut buffers = MeshBuffers::default();
    let mut visit_map = VisitedBlockMap::new();

    for face in FaceDirection::iter() {
        for layer in 0..(CHUNK_SIZE as i32) {
            mesh_layer(blocks, models, &mut buffers, &mut visit_map, face, layer);
        }
    }

    if buffers.is_empty() {
        return None;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, buffers.position_buffer);
    mesh.insert_attribute(ATTRIBUTE_UV_BASE, buffers.uv_base_buffer);
    mesh.insert_attribute(ATTRIBUTE_FACE_COUNT, buffers.face_count_buffer);
    mesh.set_indices(Some(Indices::U32(buffers.index_buffer)));

    Some(mesh)
}

fn block_pos_for_layer(face: FaceDirection, layer: i32, x: i32, y: i32) -> BlockPos {
    match face {
        FaceDirection::Front | FaceDirection::Back => BlockPos::new(x, y, layer),
        FaceDirection::Top | FaceDirection::Bottom => BlockPos::new(x, layer, y),
        FaceDirection::Left | FaceDirection::Right => BlockPos::new(layer, x, y),
    }
}

fn mesh_layer(
    blocks: &ChunkMeshData,
    models: &[BlockModel],
    buffers: &mut MeshBuffers,
    visit_map: &mut VisitedBlockMap,
    face: FaceDirection,
    layer: i32,
) {
    visit_map.clear();

    let get_block_pos = move |x, y| {
        match face {
            FaceDirection::Front | FaceDirection::Back => BlockPos::new(x, y, layer),
            FaceDirection::Top | FaceDirection::Bottom => BlockPos::new(x, layer, y),
            FaceDirection::Left | FaceDirection::Right => BlockPos::new(layer, x, y),
        }
    };

    let occluding_block_pos = move |x, y| {
        let occluding_layer = match face {
            FaceDirection::Top | FaceDirection::Front | FaceDirection::Right => layer + 1,
            FaceDirection::Bottom | FaceDirection::Back | FaceDirection::Left => layer - 1,
        };
    
        block_pos_for_layer(face, occluding_layer, x, y)
    };

    let get_model = |x, y| {
        let block_pos = block_pos_for_layer(face, layer, x, y);
        let block = blocks.get(block_pos);
        &models[block.block_id() as usize]
    };

    let is_occluded = |x, y| {
        let occluding_pos = occluding_block_pos(x, y);
        let block = blocks.get(occluding_pos);
        let model = &models[block.block_id() as usize];
        model.get_face(face.opposite_face()).is_occluder()
    };

    for x in 0..(CHUNK_SIZE as i32) {
        let mut y = 0;
        while y < CHUNK_SIZE as i32 {
            if visit_map.is_visited(x, y) {
                y += 1;
                continue;
            }

            visit_map.visit(x, y);

            let model = get_model(x, y);

            let block_face = model.get_face(face);
            if !block_face.is_visible() {
                y += 1;
                continue;
            }

            if is_occluded(x, y) {
                y += 1;
                continue;
            }

            // x and y length of greedy meshed region
            let mut x_len = 1;
            let mut y_len = 1;

            // first find how much we can greedy mesh in the y direction
            loop {
                let y_pos = y + y_len;
                if y_pos >= CHUNK_SIZE as i32 {
                    break;
                }

                if visit_map.is_visited(x, y_pos) {
                    break;
                }

                if !block_face.can_merge_with(&get_model(x, y_pos).get_face(face)) || is_occluded(x, y_pos) {
                    break;
                }

                visit_map.visit(x, y_pos);
                y_len += 1;
            }

            // then find out how much can be greedy meshed in the x direction
            'outer: loop {
                let x_pos = x + x_len;
                if x_pos >= CHUNK_SIZE as i32 {
                    break;
                }

                for y_pos in 0..y_len {
                    if visit_map.is_visited(x_pos, y_pos + y) {
                        break 'outer;
                    }

                    if is_occluded(x_pos, y_pos + y) {
                        // this can be marked as visited, because since it is occluded it will never generate a face
                        visit_map.visit(x_pos, y_pos + y);
                        break 'outer;
                    }

                    if !block_face.can_merge_with(&get_model(x_pos, y_pos + y).get_face(face)) {
                        // don't mark it as visited here, we still might generate face later
                        break 'outer;
                    }
                }

                // mark faces in x direction that could merge as visited
                for y_pos in 0..y_len {
                    visit_map.visit(x_pos, y_pos + y);
                }

                x_len += 1;
            }

            let face_count = Vec2::new(x_len as f32, y_len as f32);

            // TODO: make this cleaner
            let face_mesh_data = FaceMeshData::new(
                block_face,
                get_block_pos(x, y),
                face_count,
                face,
            );

            face_mesh_data.insert_into_bufers(buffers);

            y += y_len;
        }
    }
}