// Tower Defense Game - Level Plugin
// Handles level progression, win/lose conditions, and game-over detection.

use bevy::prelude::*;

use crate::resources::*;
use crate::systems::score::{compute_final_score, compute_star_rating};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameLevel>()
            .init_resource::<BaseHealth>()
            .init_resource::<Economy>()
            .add_systems(
                Update,
                (check_game_over, check_level_complete).run_if(in_state(AppState::Playing)),
            );
    }
}

/// Transitions to `GameOver` state if the base HP reaches zero.
fn check_game_over(
    base_hp: Res<BaseHealth>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if base_hp.current <= 0.0 {
        next_state.set(AppState::GameOver);
    }
}

/// Transitions to `LevelComplete` when all waves are done and no enemies remain.
fn check_level_complete(
    wave_state: Res<WaveState>,
    mut next_state: ResMut<NextState<AppState>>,
    base_hp: Res<BaseHealth>,
    score: Res<ScoreState>,
    economy: Res<Economy>,
    stats: Res<LevelStats>,
    game_level: Res<GameLevel>,
    mut results: ResMut<LevelResults>,
) {
    if wave_state.current_wave >= wave_state.total_waves
        && wave_state.wave_complete
        && wave_state.enemies_alive == 0
    {
        // Compute star rating
        let rating = compute_star_rating(&base_hp, &score, &economy);
        let _final_score = compute_final_score(&score, &base_hp, &stats);
        results.record(game_level.current_level, rating);
        next_state.set(AppState::LevelComplete);
    }
}
