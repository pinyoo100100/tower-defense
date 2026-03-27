// Tower Defense Game - Score Plugin
// Manages combo tracking, score computation, and star ratings.

use bevy::prelude::*;

use crate::resources::*;
use crate::systems::score::{combo_tick_system, level_timer_system};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ComboState>()
            .init_resource::<ScoreState>()
            .init_resource::<LevelStats>()
            .init_resource::<LevelResults>()
            .add_systems(
                Update,
                (combo_tick_system, level_timer_system).run_if(in_state(AppState::Playing)),
            );
    }
}
