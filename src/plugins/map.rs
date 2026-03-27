// Tower Defense Game - Map Plugin
// Initializes the grid map and renders tiles.

use bevy::prelude::*;

use crate::resources::{AppState, MapGrid, TileKind};
use crate::systems::map::setup_map_system;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_default_map())
            .add_systems(OnEnter(AppState::Playing), setup_map_system);
    }
}

/// Builds a default 12×12 map with a winding enemy path.
fn create_default_map() -> MapGrid {
    let w = 12;
    let h = 12;
    let mut map = MapGrid::new(w, h);

    // Create a winding path from top-left to bottom-right
    // Spawn at (0, 0)
    map.set(0, 0, TileKind::Spawn);

    // Path across the top
    for x in 1..=5 {
        map.set(x, 0, TileKind::Path);
    }
    // Path down the right side
    for y in 1..=4 {
        map.set(5, y, TileKind::Path);
    }
    // Path back to the left
    for x in (2..=4).rev() {
        map.set(x, 4, TileKind::Path);
    }
    // Path down
    for y in 5..=8 {
        map.set(2, y, TileKind::Path);
    }
    // Path to the right
    for x in 3..=9 {
        map.set(x, 8, TileKind::Path);
    }
    // Path down to base
    for y in 9..=10 {
        map.set(9, y, TileKind::Path);
    }

    // Base at (9, 11)
    map.set(9, 11, TileKind::Base);

    // Add some blocked tiles for variety
    map.set(3, 2, TileKind::Blocked);
    map.set(4, 2, TileKind::Blocked);
    map.set(7, 5, TileKind::Blocked);
    map.set(8, 5, TileKind::Blocked);

    map
}
