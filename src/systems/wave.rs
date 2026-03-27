// Tower Defense Game - Wave Systems
// Handles wave progression, enemy spawning, and difficulty scaling.

use bevy::prelude::*;

use crate::components::*;
use crate::config::{EnemiesConfig, WavesConfig};
use crate::resources::*;

/// Resource that tracks the current spawn queue for the active wave.
#[derive(Resource, Debug, Clone, Default)]
pub struct SpawnQueue {
    pub entries: Vec<SpawnEntry>,
    pub current_index: usize,
    pub timer: f32,
}

#[derive(Debug, Clone)]
pub struct SpawnEntry {
    pub enemy_type: EnemyType,
    pub health_scale: f32,
    pub spawn_interval: f32,
}

/// Prepares the spawn queue when a new wave begins.
pub fn wave_start_system(
    mut wave_state: ResMut<WaveState>,
    waves_config: Res<WavesConfig>,
    mut spawn_queue: ResMut<SpawnQueue>,
    time: Res<Time>,
) {
    if wave_state.wave_active {
        return;
    }

    // Wait between waves
    wave_state.between_wave_timer -= time.delta_secs();
    if wave_state.between_wave_timer > 0.0 {
        return;
    }

    let next_wave = wave_state.current_wave + 1;
    if let Some(def) = waves_config.waves.iter().find(|w| w.wave_number == next_wave) {
        let mut entries = Vec::new();
        for group in &def.groups {
            for _ in 0..group.count {
                entries.push(SpawnEntry {
                    enemy_type: group.enemy_type,
                    health_scale: def.difficulty_scale,
                    spawn_interval: group.spawn_interval,
                });
            }
        }
        *spawn_queue = SpawnQueue {
            entries,
            current_index: 0,
            timer: 0.0,
        };
        wave_state.current_wave = next_wave;
        wave_state.enemies_spawned = 0;
        wave_state.wave_active = true;
        wave_state.wave_complete = false;
    }
}

/// Spawns enemies from the spawn queue at timed intervals.
pub fn wave_spawn_system(
    time: Res<Time>,
    mut commands: Commands,
    mut wave_state: ResMut<WaveState>,
    mut spawn_queue: ResMut<SpawnQueue>,
    enemies_config: Res<EnemiesConfig>,
    map: Res<MapGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !wave_state.wave_active {
        return;
    }

    if spawn_queue.current_index >= spawn_queue.entries.len() {
        // All enemies spawned – wave will end when all are dead
        return;
    }

    spawn_queue.timer -= time.delta_secs();
    if spawn_queue.timer > 0.0 {
        return;
    }

    let entry = &spawn_queue.entries[spawn_queue.current_index];
    let interval = entry.spawn_interval;

    if let Some(cfg) = enemies_config.get(entry.enemy_type) {
        let path = map.build_path();
        let start_pos = path.first().copied().unwrap_or(Vec3::ZERO);

        let color = match cfg.enemy_type {
            EnemyType::Infantry => Color::srgb(0.9, 0.2, 0.2),
            EnemyType::Tank => Color::srgb(0.4, 0.4, 0.4),
            EnemyType::AirUnit => Color::srgb(0.2, 0.6, 0.9),
        };

        let y_offset = if cfg.movement == MovementMode::Air { 2.0 } else { 0.5 };

        commands.spawn((
            EnemyTag,
            EnemyData {
                enemy_type: cfg.enemy_type,
                armor: cfg.armor,
                movement: cfg.movement,
                reward: cfg.reward,
                speed: cfg.speed,
            },
            Health::new(cfg.health * entry.health_scale),
            PathFollower {
                waypoints: path,
                current_index: 0,
            },
            Mesh3d(meshes.add(Cuboid::new(0.4, 0.4, 0.4))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            Transform::from_translation(start_pos + Vec3::Y * y_offset),
        ));

        wave_state.enemies_spawned += 1;
        wave_state.enemies_alive += 1;
    }

    spawn_queue.current_index += 1;
    spawn_queue.timer = interval;
}

/// Checks if all enemies in the current wave have been defeated.
pub fn wave_completion_system(
    mut wave_state: ResMut<WaveState>,
    spawn_queue: Res<SpawnQueue>,
) {
    if !wave_state.wave_active {
        return;
    }

    let all_spawned = spawn_queue.current_index >= spawn_queue.entries.len();
    if all_spawned && wave_state.enemies_alive == 0 {
        wave_state.wave_active = false;
        wave_state.wave_complete = true;
        wave_state.between_wave_timer = 5.0;
    }
}
