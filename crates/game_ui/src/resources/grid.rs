use bevy::prelude::Resource;
use game_core::resources::map::MapBounds;

#[derive(Resource)]
pub struct GridRenderConfig {
    pub tile_size: f32,
    pub offset_x: f32,
    pub offset_z: f32,
    pub max_col: i32,
    pub max_row: i32,
    pub enemy_spawn_cols: i32,
}

impl Default for GridRenderConfig {
    fn default() -> Self {
        let map_bound = MapBounds::default();
        Self {
            tile_size: 80.0,
            offset_x: -400.0,
            offset_z: -240.0,
            max_col: map_bound.max_col,
            max_row: map_bound.max_row,
            enemy_spawn_cols: map_bound.enemy_spawn_cols,
        }
    }
}

impl GridRenderConfig {
    /// Recalculate grid settings based on window size.
    pub fn recalculate(&mut self, window_width: f32, window_height: f32) {
        let cols = (self.max_col + 1) as f32;
        let rows = (self.max_row + 1) as f32;

        let margin_x = 40.0;
        let margin_y = 60.0;

        let available_w = window_width - margin_x * 2.0;
        let available_h = window_height - margin_y * 2.0;

        // Tile size: Choose the smaller size to match either the length or width.
        let tile_by_w = available_w / cols;
        let tile_by_h = available_h / rows;
        self.tile_size = tile_by_w.min(tile_by_h).floor();

        // Overall width and height of the grid
        let grid_w = self.tile_size * cols;
        let grid_h = self.tile_size * rows;

        // The 3D camera is set to have an offset where top-left is the origin.
        // (Center of the screen) - (half the grid width/height)
        self.offset_x = -(grid_w * 0.5) + self.tile_size * 0.5;
        self.offset_z = -(grid_h * 0.5) + self.tile_size * 0.5;
    }
}
