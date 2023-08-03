use bevy::prelude::*;

use crate::items::ItemStack;

#[derive(Debug, Default, Component)]
struct Inventory {
    // 2d array of items in the inventory
    // first row in the array is the hotbar
    items: [[Option<ItemStack>; 10]; 6],
    selected_hotbar_item: usize,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        
    }
}