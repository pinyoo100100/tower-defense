// Tower Defense Game – Entry Point
//
// A Tower Defense game inspired by Command & Conquer: Red Alert, built with
// the Bevy ECS engine. Features an isometric camera, counter-based combat,
// wave system, combo scoring, day/night cycle, and star rating system.
//
// Architecture follows clean ECS patterns: every subsystem lives in its own
// Plugin so that features can be added, removed, or tested independently.

use bevy::prelude::*;

use tower_defense::plugins::{
    camera::CameraPlugin,
    combat::CombatPlugin,
    day_night::DayNightPlugin,
    enemy::EnemyPlugin,
    level::LevelPlugin,
    map::MapPlugin,
    score::ScorePlugin,
    tower::TowerPlugin,
    ui::UIPlugin,
    wave::WavePlugin,
};
use tower_defense::resources::AppState;

fn main() {
    App::new()
        // Bevy built-in plugins (rendering, input, windowing, etc.)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tower Defense – Red Alert Inspired".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        // Application state machine
        .init_state::<AppState>()
        // Game plugins (each encapsulates a logical subsystem)
        .add_plugins((
            CameraPlugin,
            MapPlugin,
            TowerPlugin,
            EnemyPlugin,
            WavePlugin,
            CombatPlugin,
            DayNightPlugin,
            ScorePlugin,
            LevelPlugin,
            UIPlugin,
        ))
        .run();
}
