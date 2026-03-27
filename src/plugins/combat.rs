// Tower Defense Game - Combat Plugin
// Projectile movement and damage resolution with counter system.

use bevy::prelude::*;

use crate::resources::AppState;
use crate::systems::combat::{damage_resolution_system, projectile_movement_system};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (projectile_movement_system, damage_resolution_system)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}
