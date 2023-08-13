use std::sync::OnceLock;

use bevy::prelude::*;

use crate::meshing::BlockModelUv;
use crate::GameSet;

mod material;
pub use material::*;
mod texture_map;
pub use texture_map::*;

use self::material::BlockMaterial;

const SKY_COLOR: Color = Color::Rgba {
    red: 0.4,
    green: 0.6,
    blue: 0.8,
    alpha: 1.0,
};

static BLOCK_MODELS: OnceLock<Vec<BlockModelUv>> = OnceLock::new();

/// Returns the list of block models, panics if block models are not yet initialized
/// 
/// The block model for a block type can be found by indexing this array with the block type id
pub fn block_models() -> &'static [BlockModelUv] {
    BLOCK_MODELS.get().expect("block models are not yet initialized")
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(SKY_COLOR))
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.5,
            })
            .add_plugins(MaterialPlugin::<BlockMaterial>::default())
            .add_state::<TextureLoadState>()
            .configure_set(Update, GameSet::Main.run_if(in_state(TextureLoadState::Done)))
            .add_systems(OnEnter(TextureLoadState::Loading), load_textures)
            .add_systems(Update, poll_load_status.run_if(in_state(TextureLoadState::Loading)))
            .add_systems(OnExit(TextureLoadState::Loading), generate_texture_map);
    }
}