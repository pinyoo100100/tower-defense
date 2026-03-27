// Tower Defense Game - Enemy Plugin
// Registers enemy movement and death systems.

use bevy::prelude::*;

use crate::config::EnemiesConfig;
use crate::resources::AppState;
use crate::systems::enemy::{enemy_death_system, enemy_movement_system};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemiesConfig::default())
            .add_systems(
                Update,
                (enemy_movement_system, enemy_death_system)
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
