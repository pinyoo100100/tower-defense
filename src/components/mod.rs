// Tower Defense Game - Component Definitions
// All ECS components used across the game are defined here for centralized access.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Damage types form the core of the counter system.
/// Each type has strengths and weaknesses against specific armor types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum DamageType {
    /// Fast-firing rounds – strong vs Infantry, weak vs Armored
    Bullet,
    /// Area-of-effect blasts – strong vs groups, weak vs Air
    Explosive,
    /// High-voltage discharge – strong vs Armored, weak vs fast units
    Electric,
}

/// Armor categories carried by enemies. Combined with DamageType to compute
/// the damage multiplier via the counter system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum ArmorType {
    Light,   // Infantry-class
    Heavy,   // Tank-class
    Air,     // Aerial units
}

/// Identifies the tower variant (used to look up config data).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum TowerType {
    MachineGun,
    TeslaCoil,
    MissileLauncher,
}

/// Identifies the enemy variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum EnemyType {
    Infantry,
    Tank,
    AirUnit,
}

/// The movement mode of an enemy (ground or air).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum MovementMode {
    Ground,
    Air,
}

// ---------------------------------------------------------------------------
// Core Components
// ---------------------------------------------------------------------------

/// Health pool shared by enemies and the player base.
#[derive(Component, Debug, Clone, Reflect)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn fraction(&self) -> f32 {
        if self.max == 0.0 {
            return 0.0;
        }
        self.current / self.max
    }
}

/// Damage descriptor attached to projectiles or towers.
#[derive(Component, Debug, Clone, Reflect)]
pub struct Damage {
    pub amount: f32,
    pub damage_type: DamageType,
}

/// Circular attack range (world-units radius).
#[derive(Component, Debug, Clone, Reflect)]
pub struct Range(pub f32);

/// Cooldown between attacks, driven by a Bevy `Timer`.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(from_reflect = false)]
pub struct AttackSpeed {
    #[reflect(ignore)]
    pub timer: Timer,
}

impl AttackSpeed {
    pub fn from_secs(secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(secs, TimerMode::Repeating),
        }
    }
}

/// Movement velocity for enemies and projectiles.
#[derive(Component, Debug, Clone, Reflect)]
pub struct Velocity(pub Vec3);

// ---------------------------------------------------------------------------
// Tag / Marker Components
// ---------------------------------------------------------------------------

/// Marks an entity as an enemy.
#[derive(Component, Debug, Default, Reflect)]
pub struct EnemyTag;

/// Marks an entity as a tower.
#[derive(Component, Debug, Default, Reflect)]
pub struct TowerTag;

/// Marks an entity as a projectile.
#[derive(Component, Debug, Default, Reflect)]
pub struct ProjectileTag;

/// Marks an entity as the player base (the thing enemies attack).
#[derive(Component, Debug, Default, Reflect)]
pub struct BaseTag;

// ---------------------------------------------------------------------------
// Tower-specific Components
// ---------------------------------------------------------------------------

/// Metadata attached to tower entities.
#[derive(Component, Debug, Clone, Reflect)]
pub struct TowerData {
    pub tower_type: TowerType,
    pub level: u32,
    pub can_target_air: bool,
}

/// Grid coordinate where a tower is placed.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

/// The entity this tower is currently aiming at.
#[derive(Component, Debug, Reflect)]
pub struct TowerTarget(pub Option<Entity>);

// ---------------------------------------------------------------------------
// Enemy-specific Components
// ---------------------------------------------------------------------------

/// Metadata attached to enemy entities.
#[derive(Component, Debug, Clone, Reflect)]
pub struct EnemyData {
    pub enemy_type: EnemyType,
    pub armor: ArmorType,
    pub movement: MovementMode,
    pub reward: u32,
    pub speed: f32,
}

/// Guides an enemy along a sequence of waypoints.
#[derive(Component, Debug, Clone, Reflect)]
pub struct PathFollower {
    pub waypoints: Vec<Vec3>,
    pub current_index: usize,
}

impl PathFollower {
    pub fn current_target(&self) -> Option<Vec3> {
        self.waypoints.get(self.current_index).copied()
    }

    pub fn advance(&mut self) -> bool {
        if self.current_index + 1 < self.waypoints.len() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    pub fn finished(&self) -> bool {
        self.current_index >= self.waypoints.len()
    }
}

// ---------------------------------------------------------------------------
// Projectile Component
// ---------------------------------------------------------------------------

/// Tracks the projectile's intended target and damage payload.
#[derive(Component, Debug, Reflect)]
pub struct ProjectileData {
    pub target: Entity,
    pub damage: f32,
    pub damage_type: DamageType,
    pub speed: f32,
    /// Splash radius (0 = single target)
    pub splash_radius: f32,
}
