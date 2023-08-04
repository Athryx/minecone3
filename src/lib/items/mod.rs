use bevy::prelude::*;

mod systems;
use systems::*;

mod debug_miner;
use debug_miner::DebugMiner;

#[derive(Debug)]
struct ItemProperties {
    use_time: usize,
}

/// Items should implement this trait to work with the register_items macro
trait Item: Component + Default {
    fn properties() -> ItemProperties;
    fn add_systems(app: &mut App);
}

macro_rules! register_items {
    ($( $items:ident ),*,) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ItemType {
            $(
                $items,
            )*
        }

        $(
            impl $items {
                pub fn item_type() -> ItemType {
                    ItemType::$items
                }
            }
        )*

        fn add_item_systems(app: &mut App) {
            $(
                $items::add_systems(app);
            )*
        }
    };
}

register_items! {
    DebugMiner,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
struct ItemUseSet;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        add_item_systems(app);
    }
}