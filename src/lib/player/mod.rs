use bevy::prelude::*;

use crate::items::ItemType;
use crate::{world::ChunkLoader, types::ChunkPos};

mod camera_controller;
mod inventory;

const RENDER_DISTANCE: UVec3 = UVec3::new(10, 5, 10);

/// Marks the player that is currently being controlled
#[derive(Component)]
struct ControlledPlayer;

fn setup_player(mut commands: Commands) {
    let debug_miner = ItemType::DebugMiner.spawn_bundle(&mut commands);

    let mut inventory = inventory::Inventory::default();
    inventory.selected_item = Some(debug_miner);

    let player = commands.spawn((
        ControlledPlayer,
        Camera3dBundle::default(),
        camera_controller::Controller::default(),
        ChunkLoader::new(ChunkPos::new(0, 0, 0), RENDER_DISTANCE),
        inventory,
    )).id();

    commands.entity(player).push_children(&[debug_miner]);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_plugins((
                camera_controller::ControllerPlugin,
                inventory::InventoryPlugin,
            ));
    }
}