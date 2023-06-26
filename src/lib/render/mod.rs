use bevy::prelude::*;

use crate::meshing::BlockModels;

mod shader;
mod texture_map;
use texture_map::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockModels>()
            .add_state::<TextureLoadState>()
            .add_system(load_textures.in_schedule(OnEnter(TextureLoadState::Loading)))
            .add_system(poll_load_status.in_set(OnUpdate(TextureLoadState::Loading)))
            .add_system(generate_texture_map.in_schedule(OnEnter(TextureLoadState::Done)));
    }
}