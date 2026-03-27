// Tower Defense Game - UI Plugin
// HUD overlay showing score, combo, wave info, base HP, and economy.

use bevy::prelude::*;

use crate::resources::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu)
            .add_systems(OnEnter(AppState::Playing), setup_hud)
            .add_systems(
                Update,
                (update_hud, handle_menu_interaction).run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                Update,
                (update_hud, handle_tower_placement_input).run_if(in_state(AppState::Playing)),
            )
            .add_systems(OnEnter(AppState::GameOver), setup_game_over_screen)
            .add_systems(OnEnter(AppState::LevelComplete), setup_level_complete_screen);
    }
}

// ---------------------------------------------------------------------------
// Marker Components for UI entities
// ---------------------------------------------------------------------------

#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct HudText;

#[derive(Component)]
struct PlayButton;

// ---------------------------------------------------------------------------
// Main Menu
// ---------------------------------------------------------------------------

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("TOWER DEFENSE"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.7, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("Red Alert Inspired"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Play button
            parent
                .spawn((
                    PlayButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("PLAY"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn handle_menu_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::Playing);
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// ---------------------------------------------------------------------------
// In-Game HUD
// ---------------------------------------------------------------------------

fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                HudText,
                Text::new("HUD Loading..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn update_hud(
    mut query: Query<&mut Text, With<HudText>>,
    economy: Option<Res<Economy>>,
    base_hp: Option<Res<BaseHealth>>,
    wave_state: Option<Res<WaveState>>,
    combo: Option<Res<ComboState>>,
    score: Option<Res<ScoreState>>,
    day_night: Option<Res<DayNightState>>,
) {
    for mut text in query.iter_mut() {
        let money = economy.as_ref().map(|e| e.money).unwrap_or(0);
        let hp = base_hp.as_ref().map(|h| h.current).unwrap_or(0.0);
        let wave = wave_state.as_ref().map(|w| w.current_wave).unwrap_or(0);
        let total_waves = wave_state.as_ref().map(|w| w.total_waves).unwrap_or(0);
        let combo_val = combo.as_ref().map(|c| c.current).unwrap_or(0);
        let score_val = score.as_ref().map(|s| s.points).unwrap_or(0);
        let time_str = if day_night.as_ref().is_some_and(|d| d.is_night) {
            "Night"
        } else {
            "Day"
        };

        **text = format!(
            "Money: {} | Base HP: {:.0} | Wave: {}/{} | Combo: x{} | Score: {} | {}",
            money, hp, wave, total_waves, combo_val, score_val, time_str
        );
    }
}

// ---------------------------------------------------------------------------
// Tower Placement Input (keyboard shortcut)
// ---------------------------------------------------------------------------

/// Simple keyboard-based tower placement for demonstration.
/// Press 1/2/3 to select tower type, then click would place (simplified).
fn handle_tower_placement_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    map: Res<crate::resources::MapGrid>,
    mut economy: ResMut<Economy>,
    towers_config: Res<crate::config::TowersConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut stats: ResMut<LevelStats>,
) {
    // For demo: press 1 to place MachineGun at a predefined buildable spot
    // In a full game, this would use mouse picking / raycasting
    let placements = [
        (KeyCode::Digit1, crate::components::TowerType::MachineGun, 1, 1),
        (KeyCode::Digit2, crate::components::TowerType::TeslaCoil, 3, 1),
        (KeyCode::Digit3, crate::components::TowerType::MissileLauncher, 6, 6),
        (KeyCode::Digit4, crate::components::TowerType::MachineGun, 4, 6),
        (KeyCode::Digit5, crate::components::TowerType::TeslaCoil, 1, 5),
        (KeyCode::Digit6, crate::components::TowerType::MissileLauncher, 8, 9),
    ];

    for (key, tower_type, gx, gy) in placements {
        if keys.just_pressed(key) {
            crate::systems::map::try_place_tower(
                &mut commands,
                &map,
                &mut economy,
                &towers_config,
                tower_type,
                gx,
                gy,
                &mut meshes,
                &mut materials,
                &mut stats,
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Game Over / Level Complete Screens
// ---------------------------------------------------------------------------

fn setup_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.0, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.2)),
            ));
        });
}

fn setup_level_complete_screen(
    mut commands: Commands,
    score: Res<ScoreState>,
    base_hp: Res<BaseHealth>,
    economy: Res<Economy>,
) {
    let rating = crate::systems::score::compute_star_rating(&base_hp, &score, &economy);
    let stars = match rating {
        StarRating::Zero => "No Stars",
        StarRating::One => "★",
        StarRating::Two => "★★",
        StarRating::Three => "★★★",
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.1, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("LEVEL COMPLETE!"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 1.0, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new(format!("Score: {} | Rating: {}", score.points, stars)),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}
