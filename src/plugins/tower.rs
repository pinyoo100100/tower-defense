// Tower Defense Game - Tower Plugin
// Registers tower targeting and attack systems.

use bevy::prelude::*;

use crate::config::TowersConfig;
use crate::resources::AppState;
use crate::systems::tower::{tower_attack_system, tower_targeting_system};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TowersConfig::default())
            .add_systems(
                Update,
                (tower_targeting_system, tower_attack_system)
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
