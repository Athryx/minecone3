use bevy::prelude::*;

use crate::{world::ChunkLoader, types::ChunkPos};

mod camera_controller;

const RENDER_DISTANCE: UVec3 = UVec3::new(10, 5, 10);

#[derive(Component)]
struct Player;

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Camera3dBundle::default(),
        camera_controller::Controller::default(),
        ChunkLoader::new(ChunkPos::new(0, 0, 0), RENDER_DISTANCE),
    ));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_plugins((
                camera_controller::ControllerPlugin,
            ));
    }
}