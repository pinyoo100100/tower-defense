// Tower Defense Game - Wave Plugin
// Manages wave progression and enemy spawning.

use bevy::prelude::*;

use crate::config::WavesConfig;
use crate::resources::AppState;
use crate::systems::wave::{wave_completion_system, wave_spawn_system, wave_start_system, SpawnQueue};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WavesConfig::default())
            .init_resource::<SpawnQueue>()
            .add_systems(
                Update,
                (wave_start_system, wave_spawn_system, wave_completion_system)
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
