use bevy::prelude::*;
use bevy::pbr::wireframe::WireframeConfig;

fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::P) {
        wireframe_config.global = !wireframe_config.global;
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_wireframe);
    }
}