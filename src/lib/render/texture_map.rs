use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::render::render_resource::{SamplerDescriptor, AddressMode, FilterMode};
use bevy::render::texture::ImageSampler;
use bevy::utils::HashMap;
use image::imageops::overlay;
use image::{DynamicImage, GenericImage};
use strum::IntoEnumIterator;

use crate::blocks::BlockType;
use crate::blocks::utils::Rotation;
use crate::meshing::{BlockFaceType, BlockFaceUv, TextureUvData, FaceDirection, BlockModelUv};

use super::BLOCK_MODELS;
use super::material::{GlobalBlockMaterial, BlockMaterial};

/// Number of pixels in each block texture
const TEXTURE_SIZE: u32 = 16;

/// Number of blocks along each axis of the texture map
// make sure to keep this up to date with the shaders texture size
const TEXTURE_MAP_SIZE: u32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BlockFaceTexture {
    Image(&'static str),
    Overlay {
        top: TextureKey,
        bottom: TextureKey,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureKey(usize);

/// Passed in to the `block_model` function of each block to create their textures for each face
#[derive(Debug, Default)]
pub struct TextureBuilder {
    key_to_texture_map: HashMap<BlockFaceTexture, TextureKey>,
    textures: Vec<BlockFaceTexture>,
}

impl TextureBuilder {
    fn insert_block_face_texture(&mut self, texture: BlockFaceTexture) -> TextureKey {
        if let Some(texture_key) = self.key_to_texture_map.get(&texture) {
            *texture_key
        } else {
            let texture_key = TextureKey(self.textures.len());
            self.textures.push(texture);
            self.key_to_texture_map.insert(texture, texture_key);
            texture_key
        }
    }

    pub fn image(&mut self, image_path: &'static str) -> TextureKey {
        self.insert_block_face_texture(BlockFaceTexture::Image(image_path))
    }

    pub fn overlay(&mut self, top: TextureKey, bottom: TextureKey) -> TextureKey {
        self.insert_block_face_texture(BlockFaceTexture::Overlay {
            top,
            bottom,
        })
    }

    /// Returns a hashmap of the actual data for all texture keys given a mapping from loaded image paths to image data
    fn generate_image_data(&self, images: &HashMap<&'static str, Handle<Image>>, image_data: &Assets<Image>) -> HashMap<TextureKey, DynamicImage> {
        let mut out: HashMap<TextureKey, DynamicImage> = HashMap::new();

        for (i, texture) in self.textures.iter().enumerate() {
            // a texture key is always the textures index in the array
            let texture_key = TextureKey(i);
    
            let image_data = match texture {
                BlockFaceTexture::Image(image_path) => {
                    // panic safety: if this texture exists, it should have been loaded
                    let image_handle = images.get(image_path).unwrap();
                    // panic safety: all images have been checked to make sure they are finished loading
                    let image = image_data.get(image_handle).unwrap();
                    image.clone().try_into_dynamic().expect("unsupported texture format")
                },
                BlockFaceTexture::Overlay {
                    top,
                    bottom,
                } => {
                    // panic safety: texture key can only reference an image that has already been processed, and thus is already inserted in the map
                    let top_image = out.get(top).unwrap();
                    let mut bottom_image = out.get(bottom).unwrap().clone();
    
                    overlay(&mut bottom_image, top_image, 0, 0);
    
                    bottom_image
                }
            };
    
            out.insert(texture_key, image_data);
        }

        out
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BlockFace {
    Full(TextureKey),
    Empty,
}

/// The model of a block
#[derive(Debug, Clone, Copy)]
pub struct BlockModel {
    pub faces: [BlockFace; 6],
    //custom_model: Option<TODO>,
}

impl BlockModel {
    pub fn new(face: BlockFace) -> Self {
        BlockModel {
            faces: [face; 6],
        }
    }

    pub fn set_front(mut self, face: BlockFace) -> Self {
        self.faces[FaceDirection::Front as usize] = face;
        self
    }

    pub fn set_back(mut self, face: BlockFace) -> Self {
        self.faces[FaceDirection::Back as usize] = face;
        self
    }

    pub fn set_top(mut self, face: BlockFace) -> Self {
        self.faces[FaceDirection::Top as usize] = face;
        self
    }

    pub fn set_bottom(mut self, face: BlockFace) -> Self {
        self.faces[FaceDirection::Bottom as usize] = face;
        self
    }

    pub fn set_left(mut self, face: BlockFace) -> Self {
        self.faces[FaceDirection::Left as usize] = face;
        self
    }

    pub fn set_right(mut self, face: BlockFace) -> Self {
        self.faces[FaceDirection::Right as usize] = face;
        self
    }
}

#[derive(Debug, Component)]
pub struct TextureLoadJob {
    /// A map between the path and the image
    images: HashMap<&'static str, Handle<Image>>,
    texture_builder: TextureBuilder,
    block_models: Vec<BlockModel>,
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut texture_builder = TextureBuilder::default();
    let mut block_models = Vec::new();

    for block_type in BlockType::iter() {
        // this iterates BlockType in the correct order they will need to be to index the block model array,
        // so push here will have correct order
        block_models.push(block_type.model(&mut texture_builder));
    }

    let mut images = HashMap::new();
    for texture in texture_builder.textures.iter() {
        if let BlockFaceTexture::Image(image_path) = texture {
            images.insert(*image_path, asset_server.load(*image_path));
        }
    }

    commands.spawn(TextureLoadJob {
        images,
        texture_builder,
        block_models,
    });
}

pub fn poll_load_status(
    query: Query<&TextureLoadJob>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<TextureLoadState>>,
) {
    let handle_id_iter = query.single().images
        .values()
        .map(Handle::id);

    match asset_server.get_group_load_state(handle_id_iter) {
        LoadState::Loaded => next_state.set(TextureLoadState::Done),
        LoadState::Loading => (),
        _ => panic!("failed to load textures for blocks"),
    }
}

pub fn generate_texture_map(
    query: Query<(Entity, &TextureLoadJob)>,
    mut textures: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<BlockMaterial>>,
    mut commands: Commands,
) {
    let (load_job_id, load_job) = query.single();

    assert!(
        load_job.texture_builder.textures.len() <= (TEXTURE_MAP_SIZE * TEXTURE_MAP_SIZE) as usize,
        "texture map size is too small",
    );

    let mut texture_map = DynamicImage::new_rgba8(
        TEXTURE_MAP_SIZE * TEXTURE_SIZE,
        TEXTURE_MAP_SIZE * TEXTURE_SIZE
    );

    // inserts the image iat the given index into the texture map
    // returns the texture uv data for the inserted image
    let mut insert_index = 0;
    let mut insert_texture = |image| {
        let x = insert_index as u32 % TEXTURE_MAP_SIZE;
        let y = insert_index as u32 / TEXTURE_MAP_SIZE;

        texture_map.copy_from(image, x * TEXTURE_SIZE, y * TEXTURE_SIZE)
            .expect("could not stitch texture map");

        insert_index += 1;

        let uv_base = Vec2::new(x as f32 / TEXTURE_MAP_SIZE as f32, y as f32 / TEXTURE_MAP_SIZE as f32);

        TextureUvData {
            uv_base,
            texture_map_index: insert_index,
        }
    };

    // a map of texture keys to the image data they represent
    let image_data_map= load_job.texture_builder
        .generate_image_data(&load_job.images, &textures);

    // a map of texture keys to their corresponding uv data
    let mut uv_data_map = HashMap::new();

    // generate block models uv
    let mut block_models_uv = Vec::new();
    for model in load_job.block_models.iter() {
        let mut uv_faces = [BlockFaceUv {
            rotation: Rotation::Deg0,
            face_type: BlockFaceType::Empty,
        }; 6];

        for (i, face) in model.faces.iter().enumerate() {
            match face {
                BlockFace::Full(texture_key) => {
                    let uv_data = uv_data_map.entry(*texture_key)
                        .or_insert_with(|| {
                            let image = image_data_map.get(texture_key).unwrap();
                            insert_texture(image)
                        });
    
                    uv_faces[i].face_type = BlockFaceType::Full(*uv_data);
                },
                // default uv face is already set to empty
                BlockFace::Empty => ()
            }
        }

        let uv_model = BlockModelUv {
            faces: uv_faces,
        };

        block_models_uv.push(uv_model);
    }

    BLOCK_MODELS.set(block_models_uv).expect("block models already initialized");

    // generate texture map from image
    let mut texture_map = Image::from_dynamic(texture_map, true);
    texture_map.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        ..Default::default()
    });

    let block_material = materials.add(BlockMaterial {
        texture_map: textures.add(texture_map),
    });
    commands.insert_resource(GlobalBlockMaterial(block_material));

    commands.entity(load_job_id).despawn();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum TextureLoadState {
    #[default]
    Loading,
    Done,
}