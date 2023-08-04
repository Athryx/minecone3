use bevy::prelude::*;

use crate::types::*;
use crate::world::World;
use super::*;

const DEBUG_MINER_REACH: f32 = 50.0;

#[derive(Default, Component)]
pub struct DebugMiner;

fn use_item(
    mut query: Query<(&GlobalTransform, &mut WeaponUseTime), With<DebugMiner>>,
    world: Res<World>,
) {
    for (transform, mut use_time) in query.iter_mut() {
        if use_time.try_use() {
            let Some(hit_result) = world.raycast(transform.to_ray(), DEBUG_MINER_REACH) else {
                continue;
            };
        }
    }
}

impl Item for DebugMiner {
    fn properties() -> ItemProperties {
        ItemProperties {
            use_time: 0,
        }
    }

    fn add_systems(app: &mut App) {
        app.add_systems(Update, use_item.in_set(ItemUseSet));
    }
}