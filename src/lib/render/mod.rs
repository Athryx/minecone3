use bevy::prelude::*;

use crate::meshing::BlockModels;

mod material;
mod texture_map;
use texture_map::*;

use self::material::BlockMaterial;

const SKY_COLOR: Color = Color::Rgba {
    red: 0.1,
    green: 0.2,
    blue: 0.3,
    alpha: 1.0,
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockModels>()
            .insert_resource(ClearColor(SKY_COLOR))
            .add_asset::<BlockMaterial>()
            .add_startup_system(material::initialize_block_material)
            .add_state::<TextureLoadState>()
            .add_system(load_textures.in_schedule(OnEnter(TextureLoadState::Loading)))
            .add_system(poll_load_status.in_set(OnUpdate(TextureLoadState::Loading)))
            .add_system(generate_texture_map.in_schedule(OnEnter(TextureLoadState::Done)));
    }
}