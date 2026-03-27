// Tower Defense Game - Enemy Systems
// Movement, pathfinding, and death handling for enemy entities.

use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

/// Moves enemies along their assigned waypoint paths.
pub fn enemy_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &EnemyData, &mut PathFollower, &mut Transform), With<EnemyTag>>,
    mut base_hp: ResMut<BaseHealth>,
    mut wave_state: ResMut<WaveState>,
) {
    for (entity, data, mut follower, mut transform) in query.iter_mut() {
        if let Some(target) = follower.current_target() {
            let direction = target - transform.translation;
            let distance = direction.length();
            let step = data.speed * time.delta_secs();

            if distance <= step {
                transform.translation = target;
                if !follower.advance() {
                    // Reached the base – deal damage and despawn
                    base_hp.current = (base_hp.current - 10.0).max(0.0);
                    commands.entity(entity).despawn();
                    wave_state.enemies_alive = wave_state.enemies_alive.saturating_sub(1);
                }
            } else {
                let movement = direction.normalize() * step;
                transform.translation += movement;
            }
        }
    }
}

/// Removes dead enemies (health <= 0) and grants rewards.
pub fn enemy_death_system(
    mut commands: Commands,
    query: Query<(Entity, &Health, &EnemyData), With<EnemyTag>>,
    mut economy: ResMut<Economy>,
    mut combo: ResMut<ComboState>,
    mut score: ResMut<ScoreState>,
    mut stats: ResMut<LevelStats>,
    mut wave_state: ResMut<WaveState>,
) {
    for (entity, health, data) in query.iter() {
        if health.is_dead() {
            economy.earn(data.reward);
            combo.register_kill();
            let points = (data.reward as f32 * combo.multiplier()) as u32;
            score.points += points;
            stats.total_kills += 1;
            wave_state.enemies_alive = wave_state.enemies_alive.saturating_sub(1);
            commands.entity(entity).despawn();
        }
    }
}
