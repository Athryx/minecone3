use std::sync::OnceLock;

use bevy::prelude::*;

use crate::{meshing::{BlockModels, BlockModel}, GameSet};

mod material;
pub use material::*;
mod texture_map;
use texture_map::*;

use self::material::BlockMaterial;

const SKY_COLOR: Color = Color::Rgba {
    red: 0.4,
    green: 0.6,
    blue: 0.8,
    alpha: 1.0,
};

static BLOCK_MODELS: OnceLock<Vec<BlockModel>> = OnceLock::new();

/// Returns the list of block models, may block at start while textures are being loaded
pub fn block_models() -> &'static [BlockModel] {
    loop {
        // TODO: don't use a spinlock here
        if let Some(block_models) = BLOCK_MODELS.get() {
            return block_models;
        }

        std::hint::spin_loop();
    }
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockModels>()
            .insert_resource(ClearColor(SKY_COLOR))
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.5,
            })
            .add_plugin(MaterialPlugin::<BlockMaterial>::default())
            .add_state::<TextureLoadState>()
            .configure_set(GameSet::Main.run_if(in_state(TextureLoadState::Done)))
            .add_system(load_textures.in_schedule(OnEnter(TextureLoadState::Loading)))
            .add_system(poll_load_status.in_set(OnUpdate(TextureLoadState::Loading)))
            .add_system(generate_texture_map.in_schedule(OnExit(TextureLoadState::Loading)));
    }
}