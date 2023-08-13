#![feature(lazy_cell)]
#![feature(box_into_inner)]
#![feature(let_chains)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::pbr::wireframe::WireframePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod debug;
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
        worldgen::Worldgen::init(128947);

        app.insert_resource(Msaa::Sample4)
            .add_plugins((
                WireframePlugin::default(),
                LogDiagnosticsPlugin::default(),
                FrameTimeDiagnosticsPlugin::default(),
                // NOTE: this plugin causes a lot of lag with larger render distances
                //WorldInspectorPlugin::new(),
            ))
            .add_plugins((
                debug::DebugPlugin,
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