use bevy::prelude::*;
use std::collections::HashSet;

/// Closed intervals, including min and max.
#[derive(Resource)]
pub struct MapBounds {
    pub min_col: i32,
    pub max_col: i32,
    pub min_row: i32,
    pub max_row: i32,
    /// num of right cols where enemies spawn
    pub enemy_spawn_cols: i32,
    pub obstacles: HashSet<(i32, i32)>,
}

impl Default for MapBounds {
    fn default() -> Self {
        Self {
            min_col: 0,
            max_col: 9,
            min_row: 0,
            max_row: 5,
            enemy_spawn_cols: 2,
            obstacles: HashSet::default(),
        }
    }
}

impl MapBounds {
    pub fn is_passable(&self, col: i32, row: i32) -> bool {
        if col < self.min_col
            || col > self.max_col - self.enemy_spawn_cols
            || row < self.min_row
            || row > self.max_row
        {
            return false;
        }
        !self.obstacles.contains(&(col, row))
    }
}
