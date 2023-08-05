#![feature(lazy_cell)]
#![feature(box_into_inner)]
#![feature(let_chains)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::pbr::wireframe::WireframePlugin;

mod blocks;
mod items;
mod meshing;
mod physics;
mod player;
mod render;
mod task;
mod types;
mod ui;
mod world;
mod worldgen;

pub struct MineconePlugin;

impl Plugin for MineconePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4)
            .add_plugins((
                WireframePlugin::default(),
                LogDiagnosticsPlugin::default(),
                FrameTimeDiagnosticsPlugin::default(),
            ))
            .add_plugins((
                items::ItemPlugin,
                player::PlayerPlugin,
                physics::PhysicsPlugin,
                world::WorldPlugin,
                render::RenderPlugin,
                ui::UiPlugin,
            ));
    }
}

/// All the system sets used when in the game
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum GameSet {
    Main,
}