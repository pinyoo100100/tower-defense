// Tower Defense Game - Map Systems
// Grid rendering and tower placement helpers.

use bevy::prelude::*;

use crate::components::*;
use crate::config::TowersConfig;
use crate::resources::*;

/// Spawns visual tiles for the map grid during level setup.
pub fn setup_map_system(
    mut commands: Commands,
    map: Res<MapGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for y in 0..map.height {
        for x in 0..map.width {
            let kind = map.tiles[y * map.width + x];
            let world_pos = MapGrid::grid_to_world(x as i32, y as i32);

            let color = match kind {
                TileKind::Buildable => Color::srgb(0.3, 0.6, 0.3),
                TileKind::Path => Color::srgb(0.6, 0.5, 0.3),
                TileKind::Blocked => Color::srgb(0.2, 0.2, 0.2),
                TileKind::Spawn => Color::srgb(0.8, 0.2, 0.2),
                TileKind::Base => Color::srgb(0.2, 0.2, 0.8),
            };

            // Flat tile mesh
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.9, 0.1, 0.9))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                })),
                Transform::from_translation(world_pos),
            ));
        }
    }
}

/// Spawns a tower at the given grid position if affordable and the tile is buildable.
pub fn try_place_tower(
    commands: &mut Commands,
    map: &MapGrid,
    economy: &mut Economy,
    towers_config: &TowersConfig,
    tower_type: TowerType,
    grid_x: usize,
    grid_y: usize,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    stats: &mut LevelStats,
) -> bool {
    // Validate tile
    if let Some(tile) = map.get(grid_x, grid_y) {
        if tile != TileKind::Buildable {
            return false;
        }
    } else {
        return false;
    }

    // Look up config
    let cfg = match towers_config.get(tower_type) {
        Some(c) => c,
        None => return false,
    };
    let level_cfg = &cfg.levels[0]; // Base level

    // Check cost
    if !economy.spend(level_cfg.cost) {
        return false;
    }

    let world_pos = MapGrid::grid_to_world(grid_x as i32, grid_y as i32);

    let color = match tower_type {
        TowerType::MachineGun => Color::srgb(0.1, 0.7, 0.1),
        TowerType::TeslaCoil => Color::srgb(0.3, 0.3, 0.9),
        TowerType::MissileLauncher => Color::srgb(0.9, 0.5, 0.1),
    };

    commands.spawn((
        TowerTag,
        TowerData {
            tower_type,
            level: 1,
            can_target_air: cfg.can_target_air,
        },
        GridPosition {
            x: grid_x as i32,
            y: grid_y as i32,
        },
        Damage {
            amount: level_cfg.damage,
            damage_type: cfg.damage_type,
        },
        Range(level_cfg.range),
        AttackSpeed::from_secs(level_cfg.attack_speed),
        TowerTarget(None),
        Mesh3d(meshes.add(Cuboid::new(0.5, 1.0, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            ..default()
        })),
        Transform::from_translation(world_pos + Vec3::Y * 0.5),
    ));

    stats.towers_built += 1;
    true
}
