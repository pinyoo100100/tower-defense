// Tower Defense Game - Resource Definitions
// Global resources shared across systems.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::*;

// ---------------------------------------------------------------------------
// Application State
// ---------------------------------------------------------------------------

/// Top-level application state machine.
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    LevelComplete,
    GameOver,
}

// ---------------------------------------------------------------------------
// Economy
// ---------------------------------------------------------------------------

/// Player currency used to build and upgrade towers.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct Economy {
    pub money: u32,
    pub total_earned: u32,
    pub total_spent: u32,
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            money: 200,
            total_earned: 200,
            total_spent: 0,
        }
    }
}

impl Economy {
    pub fn can_afford(&self, cost: u32) -> bool {
        self.money >= cost
    }

    pub fn spend(&mut self, cost: u32) -> bool {
        if self.can_afford(cost) {
            self.money -= cost;
            self.total_spent += cost;
            true
        } else {
            false
        }
    }

    pub fn earn(&mut self, amount: u32) {
        self.money += amount;
        self.total_earned += amount;
    }
}

// ---------------------------------------------------------------------------
// Combo System
// ---------------------------------------------------------------------------

/// Tracks the current kill combo and its decay timer.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct ComboState {
    pub current: u32,
    pub timer: f32,
    pub max_combo: u32,
    /// Seconds before the combo resets after the last kill
    pub decay_time: f32,
}

impl Default for ComboState {
    fn default() -> Self {
        Self {
            current: 0,
            timer: 0.0,
            max_combo: 0,
            decay_time: 3.0,
        }
    }
}

impl ComboState {
    /// Returns the score multiplier from the current combo.
    pub fn multiplier(&self) -> f32 {
        1.0 + (self.current as f32) * 0.1
    }

    pub fn register_kill(&mut self) {
        self.current += 1;
        self.timer = self.decay_time;
        if self.current > self.max_combo {
            self.max_combo = self.current;
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if self.current > 0 {
            self.timer -= dt;
            if self.timer <= 0.0 {
                self.current = 0;
                self.timer = 0.0;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Score
// ---------------------------------------------------------------------------

/// Accumulated score for the current level.
#[derive(Resource, Debug, Clone, Default, Reflect)]
pub struct ScoreState {
    pub points: u32,
}

// ---------------------------------------------------------------------------
// Wave State
// ---------------------------------------------------------------------------

/// Tracks the progress of enemy waves.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct WaveState {
    pub current_wave: u32,
    pub total_waves: u32,
    pub enemies_spawned: u32,
    pub enemies_alive: u32,
    pub wave_active: bool,
    pub spawn_timer: f32,
    pub between_wave_timer: f32,
    pub wave_complete: bool,
}

impl Default for WaveState {
    fn default() -> Self {
        Self {
            current_wave: 0,
            total_waves: 10,
            enemies_spawned: 0,
            enemies_alive: 0,
            wave_active: false,
            spawn_timer: 0.0,
            between_wave_timer: 5.0,
            wave_complete: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Day / Night Cycle
// ---------------------------------------------------------------------------

/// Global day/night state affecting gameplay.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct DayNightState {
    /// Normalized time-of-day in `[0, 1)`. 0 = dawn, 0.5 = dusk.
    pub time_of_day: f32,
    /// Speed multiplier for the cycle (higher = faster).
    pub cycle_speed: f32,
    /// Whether it is currently "night" (reduced visibility, enemy buffs).
    pub is_night: bool,
    /// Vision-range multiplier applied during night.
    pub night_vision_factor: f32,
    /// Damage buff multiplier applied to enemies at night.
    pub night_enemy_buff: f32,
}

impl Default for DayNightState {
    fn default() -> Self {
        Self {
            time_of_day: 0.25, // Start at morning
            cycle_speed: 0.02,
            is_night: false,
            night_vision_factor: 0.6,
            night_enemy_buff: 1.25,
        }
    }
}

// ---------------------------------------------------------------------------
// Level Statistics
// ---------------------------------------------------------------------------

/// Per-level statistics tracked for the star-rating system.
#[derive(Resource, Debug, Clone, Default, Reflect)]
pub struct LevelStats {
    pub total_kills: u32,
    pub towers_built: u32,
    pub damage_dealt: f32,
    pub max_combo: u32,
    pub time_elapsed: f32,
}

// ---------------------------------------------------------------------------
// Base HP (player life)
// ---------------------------------------------------------------------------

/// The player's base health – when this reaches zero, the game is lost.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct BaseHealth {
    pub current: f32,
    pub max: f32,
}

impl Default for BaseHealth {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

impl BaseHealth {
    pub fn fraction(&self) -> f32 {
        if self.max == 0.0 {
            0.0
        } else {
            self.current / self.max
        }
    }
}

// ---------------------------------------------------------------------------
// Game Level
// ---------------------------------------------------------------------------

/// Current level / progression information.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct GameLevel {
    pub current_level: u32,
    pub levels_unlocked: u32,
    pub total_levels: u32,
}

impl Default for GameLevel {
    fn default() -> Self {
        Self {
            current_level: 1,
            levels_unlocked: 1,
            total_levels: 10,
        }
    }
}

// ---------------------------------------------------------------------------
// Map Grid
// ---------------------------------------------------------------------------

/// Tile variants on the grid map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum TileKind {
    /// Can place towers here.
    Buildable,
    /// Enemy path.
    Path,
    /// Blocked / impassable.
    Blocked,
    /// Spawn point for enemies.
    Spawn,
    /// The player base to defend.
    Base,
}

/// The grid-based map used for tower placement and enemy pathing.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct MapGrid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileKind>,
}

impl MapGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileKind::Buildable; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<TileKind> {
        if x < self.width && y < self.height {
            Some(self.tiles[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, kind: TileKind) {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x] = kind;
        }
    }

    /// Returns the world-space center of the tile at grid `(x, y)`.
    /// Uses a simple isometric mapping for 2.5D projection.
    pub fn grid_to_world(x: i32, y: i32) -> Vec3 {
        let tile_size = 1.0_f32;
        Vec3::new(
            (x as f32 - y as f32) * tile_size * 0.5,
            0.0,
            (x as f32 + y as f32) * tile_size * 0.5,
        )
    }

    /// Find path tiles from spawn to base (simple ordered collection).
    pub fn build_path(&self) -> Vec<Vec3> {
        // Collect all Path tiles in row-major order as a simple path
        // A real game would use A* but for the demo we rely on map layout
        let mut path = Vec::new();
        // Find spawn first
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y * self.width + x] == TileKind::Spawn {
                    path.push(Self::grid_to_world(x as i32, y as i32));
                }
            }
        }
        // Then path tiles
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y * self.width + x] == TileKind::Path {
                    path.push(Self::grid_to_world(x as i32, y as i32));
                }
            }
        }
        // Then base
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y * self.width + x] == TileKind::Base {
                    path.push(Self::grid_to_world(x as i32, y as i32));
                }
            }
        }
        path
    }
}

// ---------------------------------------------------------------------------
// Level Progression / Star Ratings
// ---------------------------------------------------------------------------

/// Star rating result for a completed level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum StarRating {
    Zero,
    One,
    Two,
    Three,
}

/// Stored results for each completed level.
#[derive(Resource, Debug, Clone, Default, Reflect)]
pub struct LevelResults {
    pub ratings: Vec<(u32, StarRating)>,
}

impl LevelResults {
    pub fn record(&mut self, level: u32, rating: StarRating) {
        if let Some(entry) = self.ratings.iter_mut().find(|(l, _)| *l == level) {
            // Keep best rating
            if rating_value(rating) > rating_value(entry.1) {
                entry.1 = rating;
            }
        } else {
            self.ratings.push((level, rating));
        }
    }
}

fn rating_value(r: StarRating) -> u8 {
    match r {
        StarRating::Zero => 0,
        StarRating::One => 1,
        StarRating::Two => 2,
        StarRating::Three => 3,
    }
}
