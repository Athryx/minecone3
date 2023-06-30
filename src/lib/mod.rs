#![feature(once_cell)]

use bevy::prelude::*;
use render::GlobalBlockMaterial;

mod blocks;
mod camera_controls;
mod meshing;
mod physics;
mod render;
mod types;
mod world;
mod worldgen;

fn test(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });
}

pub struct MineconePlugin;

impl Plugin for MineconePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4)
            .add_plugin(camera_controls::ControllerPlugin)
            .add_plugin(physics::PhysicsPlugin)
            .add_plugin(world::WorldPlugin)
            .add_plugin(render::RenderPlugin)
            .add_startup_system(test);
    }
}

/// All the system sets used when in the game
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum GameSet {
    Main,
}