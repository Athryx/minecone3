use bevy::prelude::*;

mod systems;
pub use systems::*;

mod debug_miner;
use debug_miner::DebugMiner;

use crate::GameSet;

#[derive(Debug)]
pub struct ItemStack {
    pub item: ItemType,
    pub stack_size: usize,
}

#[derive(Debug)]
struct ItemProperties {
    use_time: usize,
}

/// Items should implement this trait to work with the register_items macro
trait Item: Component + Default {
    fn properties() -> ItemProperties;
    fn add_systems(app: &mut App);

    fn spawn_bundle(commands: &mut Commands) -> Entity {
        commands.spawn((
            Self::default(),
            WeaponUseTime::from_use_time(Self::properties().use_time),
            TransformBundle::default(),
        )).id()
    }
}

macro_rules! register_items {
    ($( $items:ident ),*,) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ItemType {
            $(
                $items,
            )*
        }

        impl ItemType {
            pub fn spawn_bundle(&self, commands: &mut Commands) -> Entity {
                match self {
                    $(
                        Self::$items => $items::spawn_bundle(commands),
                    )*
                }
            }
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
pub struct ItemUseSet;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, ItemUseSet.in_set(GameSet::Main));
        add_item_systems(app);
    }
}