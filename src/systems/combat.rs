// Tower Defense Game - Combat Systems
// Projectile movement, hit detection, damage resolution with counter system.

use bevy::prelude::*;

use crate::components::*;
use crate::config::damage_multiplier;
use crate::resources::*;

/// Moves projectiles toward their targets.
pub fn projectile_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Transform, &ProjectileData, &Velocity), With<ProjectileTag>>,
    targets: Query<&Transform, (With<EnemyTag>, Without<ProjectileTag>)>,
) {
    for (entity, mut proj_tf, data, velocity) in projectiles.iter_mut() {
        // If target is gone, despawn projectile
        if targets.get(data.target).is_err() {
            commands.entity(entity).despawn();
            continue;
        }

        let target_tf = targets.get(data.target).unwrap();
        let direction = (target_tf.translation - proj_tf.translation).normalize();
        let step = data.speed * time.delta_secs();
        proj_tf.translation += direction * step;

        // Check if projectile reached the target
        let dist = proj_tf.translation.distance(target_tf.translation);
        if dist < 0.3 {
            // Hit – apply damage (handled by damage_resolution_system via event or direct)
            // We despawn the projectile here; damage is applied in damage_resolution
            commands.entity(entity).despawn();
        }
    }
}

/// Applies damage when a projectile reaches its target, using the counter system.
pub fn damage_resolution_system(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &ProjectileData), With<ProjectileTag>>,
    mut enemies: Query<(Entity, &Transform, &mut Health, &EnemyData), With<EnemyTag>>,
    day_night: Res<DayNightState>,
) {
    for (proj_entity, proj_tf, data) in projectiles.iter() {
        // Check if projectile is close enough to its target to hit
        if let Ok((_enemy_entity, enemy_tf, mut health, enemy_data)) = enemies.get_mut(data.target) {
            let dist = proj_tf.translation.distance(enemy_tf.translation);
            if dist < 0.3 {
                let multiplier = damage_multiplier(data.damage_type, enemy_data.armor);
                let final_damage = data.damage * multiplier;
                health.take_damage(final_damage);
                commands.entity(proj_entity).despawn();

                // Splash damage
                if data.splash_radius > 0.0 {
                    let center = enemy_tf.translation;
                    for (_other_entity, other_tf, mut other_health, other_data) in enemies.iter_mut() {
                        let splash_dist = center.distance(other_tf.translation);
                        if splash_dist <= data.splash_radius && splash_dist > 0.1 {
                            let splash_mult = damage_multiplier(data.damage_type, other_data.armor);
                            let splash_dmg = data.damage * splash_mult * 0.5; // 50% splash
                            other_health.take_damage(splash_dmg);
                        }
                    }
                }
            }
        }
    }
}
