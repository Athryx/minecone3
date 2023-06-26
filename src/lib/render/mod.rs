use bevy::prelude::*;

use crate::meshing::BlockModels;

mod material;
mod texture_map;
use texture_map::*;

use self::material::BlockMaterial;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockModels>()
            .add_asset::<BlockMaterial>()
            .add_startup_system(material::initialize_block_material)
            .add_state::<TextureLoadState>()
            .add_system(load_textures.in_schedule(OnEnter(TextureLoadState::Loading)))
            .add_system(poll_load_status.in_set(OnUpdate(TextureLoadState::Loading)))
            .add_system(generate_texture_map.in_schedule(OnEnter(TextureLoadState::Done)));
    }
}