// Tower Defense Game - Tower Systems
// Targeting, shooting, and upgrade logic for towers.

use bevy::prelude::*;

use crate::components::*;
use crate::config::TowersConfig;
use crate::resources::*;

/// Each frame, each tower picks the closest in-range enemy as its target.
pub fn tower_targeting_system(
    mut towers: Query<(&Transform, &Range, &TowerData, &mut TowerTarget), With<TowerTag>>,
    enemies: Query<(Entity, &Transform, &EnemyData), With<EnemyTag>>,
    day_night: Res<DayNightState>,
) {
    for (tower_tf, range, tower_data, mut target) in towers.iter_mut() {
        let effective_range = if day_night.is_night {
            range.0 * day_night.night_vision_factor
        } else {
            range.0
        };

        let mut best: Option<(Entity, f32)> = None;
        for (enemy_entity, enemy_tf, enemy_data) in enemies.iter() {
            // Air targeting check
            if enemy_data.movement == MovementMode::Air && !tower_data.can_target_air {
                continue;
            }
            let dist = tower_tf.translation.distance(enemy_tf.translation);
            if dist <= effective_range {
                if best.is_none() || dist < best.unwrap().1 {
                    best = Some((enemy_entity, dist));
                }
            }
        }
        target.0 = best.map(|(e, _)| e);
    }
}

/// Towers fire projectiles at their current target when the attack timer is ready.
pub fn tower_attack_system(
    time: Res<Time>,
    mut commands: Commands,
    mut towers: Query<
        (&Transform, &TowerTarget, &Damage, &mut AttackSpeed, &TowerData),
        With<TowerTag>,
    >,
    enemies: Query<&Transform, With<EnemyTag>>,
    towers_config: Res<TowersConfig>,
    mut stats: ResMut<LevelStats>,
) {
    for (tower_tf, target, damage, mut speed, tower_data) in towers.iter_mut() {
        speed.timer.tick(time.delta());

        if let Some(target_entity) = target.0 {
            if speed.timer.just_finished() {
                // Verify target is still alive
                if let Ok(enemy_tf) = enemies.get(target_entity) {
                    let cfg = towers_config.get(tower_data.tower_type);
                    let splash = cfg.map(|c| c.splash_radius).unwrap_or(0.0);
                    let direction = (enemy_tf.translation - tower_tf.translation).normalize();

                    commands.spawn((
                        ProjectileTag,
                        ProjectileData {
                            target: target_entity,
                            damage: damage.amount,
                            damage_type: damage.damage_type,
                            speed: 12.0,
                            splash_radius: splash,
                        },
                        Transform::from_translation(tower_tf.translation + direction * 0.5),
                        Velocity(direction * 12.0),
                    ));

                    stats.damage_dealt += damage.amount;
                }
            }
        }
    }
}
