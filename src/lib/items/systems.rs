//! A collection of utility systems and components for items to use

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct WeaponUseTime {
    /// The use time of the tool / weapon
    use_time: usize,
    /// The current cooldown timer of the weapon
    /// 
    /// This is 0 when the weapon is ready to use, and is reset to the use time when the weapon is used
    remaining_cooldown_time: usize,
    /// True if the player is currently attemtping to use the weapon
    pub currently_using: bool,
}

impl WeaponUseTime {
    pub fn from_use_time(use_time: usize) -> Self {
        WeaponUseTime {
            use_time,
            remaining_cooldown_time: use_time,
            currently_using: false,
        }
    }

    pub fn try_use(&mut self) -> bool {
        if self.remaining_cooldown_time > 0 {
            self.remaining_cooldown_time -= 1;
            false
        } else if self.currently_using {
            self.remaining_cooldown_time = self.use_time;
            true
        } else {
            false
        }
    }
}