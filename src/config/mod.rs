// Tower Defense Game - Configuration Types
// Structs for loading tower / wave / enemy definitions from config files or code.

use serde::{Deserialize, Serialize};

use crate::components::*;

// ---------------------------------------------------------------------------
// Tower Configuration
// ---------------------------------------------------------------------------

/// Static definition of a tower at a specific upgrade level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerLevelConfig {
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub cost: u32,
}

/// Full tower definition with up to 3 upgrade levels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerConfig {
    pub tower_type: TowerType,
    pub name: String,
    pub damage_type: DamageType,
    pub can_target_air: bool,
    /// Splash radius (0.0 = single target).
    pub splash_radius: f32,
    /// Upgrade levels (index 0 = base, 1 = upgrade 1, 2 = upgrade 2).
    pub levels: Vec<TowerLevelConfig>,
}

/// Complete collection of tower configs, indexable by `TowerType`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowersConfig {
    pub towers: Vec<TowerConfig>,
}

impl TowersConfig {
    pub fn get(&self, tt: TowerType) -> Option<&TowerConfig> {
        self.towers.iter().find(|t| t.tower_type == tt)
    }
}

impl Default for TowersConfig {
    fn default() -> Self {
        Self {
            towers: vec![
                TowerConfig {
                    tower_type: TowerType::MachineGun,
                    name: "Machine Gun Tower".into(),
                    damage_type: DamageType::Bullet,
                    can_target_air: true,
                    splash_radius: 0.0,
                    levels: vec![
                        TowerLevelConfig { damage: 10.0, range: 5.0, attack_speed: 0.3, cost: 50 },
                        TowerLevelConfig { damage: 15.0, range: 5.5, attack_speed: 0.25, cost: 40 },
                        TowerLevelConfig { damage: 22.0, range: 6.0, attack_speed: 0.2, cost: 60 },
                    ],
                },
                TowerConfig {
                    tower_type: TowerType::TeslaCoil,
                    name: "Tesla Coil".into(),
                    damage_type: DamageType::Electric,
                    can_target_air: false,
                    splash_radius: 0.0,
                    levels: vec![
                        TowerLevelConfig { damage: 40.0, range: 4.0, attack_speed: 1.5, cost: 100 },
                        TowerLevelConfig { damage: 60.0, range: 4.5, attack_speed: 1.3, cost: 75 },
                        TowerLevelConfig { damage: 90.0, range: 5.0, attack_speed: 1.0, cost: 100 },
                    ],
                },
                TowerConfig {
                    tower_type: TowerType::MissileLauncher,
                    name: "Missile Launcher".into(),
                    damage_type: DamageType::Explosive,
                    can_target_air: true,
                    splash_radius: 2.5,
                    levels: vec![
                        TowerLevelConfig { damage: 25.0, range: 6.0, attack_speed: 2.0, cost: 80 },
                        TowerLevelConfig { damage: 40.0, range: 6.5, attack_speed: 1.8, cost: 60 },
                        TowerLevelConfig { damage: 60.0, range: 7.0, attack_speed: 1.5, cost: 80 },
                    ],
                },
            ],
        }
    }
}

// ---------------------------------------------------------------------------
// Enemy Configuration
// ---------------------------------------------------------------------------

/// Static definition of an enemy variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyConfig {
    pub enemy_type: EnemyType,
    pub name: String,
    pub health: f32,
    pub speed: f32,
    pub armor: ArmorType,
    pub movement: MovementMode,
    pub reward: u32,
}

/// Collection of enemy configs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemiesConfig {
    pub enemies: Vec<EnemyConfig>,
}

impl EnemiesConfig {
    pub fn get(&self, et: EnemyType) -> Option<&EnemyConfig> {
        self.enemies.iter().find(|e| e.enemy_type == et)
    }
}

impl Default for EnemiesConfig {
    fn default() -> Self {
        Self {
            enemies: vec![
                EnemyConfig {
                    enemy_type: EnemyType::Infantry,
                    name: "Infantry".into(),
                    health: 50.0,
                    speed: 2.0,
                    armor: ArmorType::Light,
                    movement: MovementMode::Ground,
                    reward: 10,
                },
                EnemyConfig {
                    enemy_type: EnemyType::Tank,
                    name: "Tank".into(),
                    health: 200.0,
                    speed: 1.0,
                    armor: ArmorType::Heavy,
                    movement: MovementMode::Ground,
                    reward: 30,
                },
                EnemyConfig {
                    enemy_type: EnemyType::AirUnit,
                    name: "Air Unit".into(),
                    health: 80.0,
                    speed: 3.0,
                    armor: ArmorType::Air,
                    movement: MovementMode::Air,
                    reward: 20,
                },
            ],
        }
    }
}

// ---------------------------------------------------------------------------
// Wave Configuration
// ---------------------------------------------------------------------------

/// A single spawn group within a wave.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveSpawnGroup {
    pub enemy_type: EnemyType,
    pub count: u32,
    /// Seconds between each spawn in this group.
    pub spawn_interval: f32,
}

/// Definition of a single wave.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveDefinition {
    pub wave_number: u32,
    pub groups: Vec<WaveSpawnGroup>,
    pub is_boss_wave: bool,
    /// Difficulty multiplier applied to enemy HP and reward.
    pub difficulty_scale: f32,
}

/// All wave definitions for a level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WavesConfig {
    pub waves: Vec<WaveDefinition>,
}

impl Default for WavesConfig {
    fn default() -> Self {
        let mut waves = Vec::new();
        for i in 1..=10 {
            let scale = 1.0 + (i as f32 - 1.0) * 0.15;
            let is_boss = i % 5 == 0;
            let mut groups = vec![WaveSpawnGroup {
                enemy_type: EnemyType::Infantry,
                count: 3 + i,
                spawn_interval: 1.0,
            }];
            if i >= 3 {
                groups.push(WaveSpawnGroup {
                    enemy_type: EnemyType::Tank,
                    count: 1 + i / 3,
                    spawn_interval: 2.0,
                });
            }
            if i >= 5 {
                groups.push(WaveSpawnGroup {
                    enemy_type: EnemyType::AirUnit,
                    count: i / 2,
                    spawn_interval: 1.5,
                });
            }
            waves.push(WaveDefinition {
                wave_number: i,
                groups,
                is_boss_wave: is_boss,
                difficulty_scale: scale,
            });
        }
        Self { waves }
    }
}

// ---------------------------------------------------------------------------
// Counter System Matrix
// ---------------------------------------------------------------------------

/// Look up the damage multiplier based on attack DamageType vs target ArmorType.
pub fn damage_multiplier(damage_type: DamageType, armor: ArmorType) -> f32 {
    match (damage_type, armor) {
        // Bullet: strong vs Light (infantry), weak vs Heavy (tanks)
        (DamageType::Bullet, ArmorType::Light) => 1.5,
        (DamageType::Bullet, ArmorType::Heavy) => 0.5,
        (DamageType::Bullet, ArmorType::Air) => 1.0,

        // Explosive: strong vs groups (Light), weak vs Air
        (DamageType::Explosive, ArmorType::Light) => 1.25,
        (DamageType::Explosive, ArmorType::Heavy) => 1.0,
        (DamageType::Explosive, ArmorType::Air) => 0.5,

        // Electric: strong vs Armored (Heavy), weak vs fast units (Light/Air)
        (DamageType::Electric, ArmorType::Heavy) => 1.5,
        (DamageType::Electric, ArmorType::Light) => 0.75,
        (DamageType::Electric, ArmorType::Air) => 0.75,
    }
}
