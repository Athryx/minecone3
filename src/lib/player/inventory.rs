use bevy::prelude::*;

use crate::{items::{WeaponUseTime, ItemUseSet, ItemStack}, GameSet};

use super::ControlledPlayer;

#[derive(Debug, Default, Component)]
pub struct Inventory {
    /// All the items in the inventory, with row 0 being the hotbar
    items: [[Option<ItemStack>; 10]; 5],
    selected_hotbar_index: usize,
    pub(super) selected_item: Option<Entity>,
}

fn mark_item_for_use(
    players: Query<&Inventory, With<ControlledPlayer>>,
    mut items: Query<&mut WeaponUseTime>,
    buttons: Res<Input<MouseButton>>,
) {
    for inventory in players.iter() {
        if let Some(selected_item) = inventory.selected_item {
            if let Ok(mut item) = items.get_mut(selected_item) {
                item.currently_using = buttons.pressed(MouseButton::Left);
            }
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            mark_item_for_use
                .in_set(GameSet::Main)
                .before(ItemUseSet)
        );
    }
}