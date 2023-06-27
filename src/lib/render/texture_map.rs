use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use image::{DynamicImage, GenericImage};
use strum::IntoEnumIterator;

use crate::blocks::BlockType;
use crate::meshing::{BlockFaceType, TextureIdentifier, TexCoords, BlockModels};

use super::BLOCK_MODELS;
use super::material::GlobalTextureMap;

/// Number of pixels in each block texture
const TEXTURE_SIZE: u32 = 16;

#[derive(Debug, Component)]
pub struct TextureLoadJob(Vec<Handle<Image>>);

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut block_models: ResMut<BlockModels>,
) {
    let mut load_jobs = Vec::new();

    for block_type in BlockType::iter() {
        let model = block_type.model();
        block_models.0.push(model.clone());

        for face in model.faces.iter() {
            if let BlockFaceType::Full(ref texture) = face.face_type {
                let TextureIdentifier::Path(asset_path) = texture else {
                    panic!("block model had texture coordinates when asset path was expected");
                };

                load_jobs.push(asset_server.load(asset_path.clone()));
            }
        }
    }

    commands.spawn(TextureLoadJob(load_jobs));
}

pub fn poll_load_status(
    query: Query<&TextureLoadJob>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<TextureLoadState>>,
) {
    let handle_id_iter = query.single().0
        .iter()
        .map(Handle::id);

    match asset_server.get_group_load_state(handle_id_iter) {
        LoadState::Loaded => next_state.set(TextureLoadState::Done),
        LoadState::Loading => (),
        _ => panic!("failed to load textures for blocks"),
    }
}

pub fn generate_texture_map(
    query: Query<(Entity, &TextureLoadJob)>,
    asset_server: Res<AssetServer>,
    mut global_texture_map: ResMut<GlobalTextureMap>,
    mut textures: ResMut<Assets<Image>>,
    mut block_models: ResMut<BlockModels>,
    mut commands: Commands,
) {
    let (load_job_id, loaded_textures) = query.single();

    // the length of the texture map in block textures
    // the texture map is a square
    let texture_map_size = (loaded_textures.0.len() as f32).sqrt().ceil() as u32;
    let mut texture_map = DynamicImage::new_rgba8(
        texture_map_size * TEXTURE_SIZE,
        texture_map_size * TEXTURE_SIZE
    );

    // the uv offset between each texture
    let texture_step_size = 1.0 / texture_map_size as f32;

    let mut asset_path_to_uv_map = HashMap::new();

    // stitch textures together
    for (i, texture_handle) in loaded_textures.0.iter().enumerate() {
        let x = i as u32 % texture_map_size;
        let y = i as u32 / texture_map_size;

        let uv = TexCoords(Vec2::new(x as f32, y as f32) * texture_step_size);

        // TODO: stitch textures
        let texture = textures
            .get(texture_handle)
            .unwrap()
            .clone()
            .try_into_dynamic()
            .unwrap();

        assert!(
            texture.width() == TEXTURE_SIZE && texture.height() == TEXTURE_SIZE,
            "invalid sized block texture",
        );

        texture_map.copy_from(&texture, x * TEXTURE_SIZE, y * TEXTURE_SIZE)
            .expect("could not stitch texture map");

        let asset_path = asset_server.get_handle_path(texture_handle).unwrap();

        asset_path_to_uv_map.insert(asset_path, uv);
    }

    // update block models with uv coordinates
    for model in block_models.0.iter_mut() {
        for face in model.faces.iter_mut() {
            if let BlockFaceType::Full(ref mut texture_id) = face.face_type {
                let TextureIdentifier::Path(asset_path) = *texture_id else {
                    panic!("block model had texture coordinates when asset path was expected");
                };

                let uv = asset_path_to_uv_map.get(&asset_path.into()).unwrap();

                *texture_id = TextureIdentifier::Coordinates(*uv);
            }
        }
    }
    // update block models global
    BLOCK_MODELS.set(block_models.0.clone()).unwrap();

    let texture_map = Image::from_dynamic(texture_map, false);

    // update temporary texture map to the real one
    // updating the handle is fine, since the old handles that may be in use still reference the same resource
    global_texture_map.0 = textures.set(&global_texture_map.0, texture_map);

    commands.entity(load_job_id).despawn();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum TextureLoadState {
    #[default]
    Loading,
    Done,
}