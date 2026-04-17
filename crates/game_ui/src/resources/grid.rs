use bevy::prelude::Resource;
use game_core::resources::map::MapBounds;

/// Aspect ratio of a single grid cell (width : height).
const CELL_ASPECT_W: f32 = 4.0;
const CELL_ASPECT_H: f32 = 3.0;

const MARGIN_X: f32 = 40.0;
const MARGIN_Y: f32 = 60.0;

#[derive(Resource)]
pub struct GridRenderConfig {
    /// Width of a single tile in world units.
    pub tile_w: f32,
    /// Height of a single tile in world units.
    pub tile_h: f32,
    /// World-space X offset of the grid origin (column 0, row 0).
    pub offset_x: f32,
    /// World-space Z offset of the grid origin (column 0, row 0).
    pub offset_z: f32,
    pub max_col: i32,
    pub max_row: i32,
    pub enemy_spawn_cols: i32,
}

impl Default for GridRenderConfig {
    fn default() -> Self {
        let map_bound = MapBounds::default();
        let mut cfg = Self {
            tile_w: 0.0,
            tile_h: 0.0,
            offset_x: 0.0,
            offset_z: 0.0,
            max_col: map_bound.max_col,
            max_row: map_bound.max_row,
            enemy_spawn_cols: map_bound.enemy_spawn_cols,
        };
        // default for a 1280×720 window
        cfg.recalculate(1280.0, 720.0);
        cfg
    }
}

impl GridRenderConfig {
    pub fn candidate_tile_w(&self, window_width: f32, window_height: f32) -> f32 {
        let cols = (self.max_col + 1) as f32;
        let rows = (self.max_row + 1) as f32;

        let available_w = window_width - MARGIN_X * 2.0;
        let available_h = window_height - MARGIN_Y * 2.0;

        let ratio = CELL_ASPECT_H / CELL_ASPECT_W;

        let tile_w_from_w = available_w / cols;
        let tile_h_from_w = tile_w_from_w * ratio;

        let tile_h_from_h = available_h / rows;
        let tile_w_from_h = tile_h_from_h / ratio;

        let (tile_w, _) = if tile_h_from_w * rows <= available_h {
            (tile_w_from_w, tile_h_from_w)
        } else {
            (tile_w_from_h, tile_h_from_h)
        };

        tile_w.floor()
    }

    /// Recalculate tile dimensions and grid offset to fill the available
    /// window area while preserving the [`CELL_ASPECT_W`]:[`CELL_ASPECT_H`]
    /// cell aspect ratio.
    pub fn recalculate(&mut self, window_width: f32, window_height: f32) {
        let cols = (self.max_col + 1) as f32;
        let rows = (self.max_row + 1) as f32;

        let available_w = (window_width - MARGIN_X * 2.0).max(1.0);
        let available_h = (window_height - MARGIN_Y * 2.0).max(1.0);

        // Determine the largest cell size that fits the available area while
        // respecting the desired aspect ratio.
        let ratio = CELL_ASPECT_H / CELL_ASPECT_W;

        let tile_w_from_w = available_w / cols;
        let tile_h_from_w = tile_w_from_w * ratio;

        let tile_h_from_h = available_h / rows;
        let tile_w_from_h = tile_h_from_h / ratio;

        // Use the width-constrained candidate unless it overflows vertically.
        let (tile_w, tile_h) = if tile_h_from_w * rows <= available_h {
            (tile_w_from_w, tile_h_from_w)
        } else {
            (tile_w_from_h, tile_h_from_h)
        };

        self.tile_w = tile_w.floor().max(1.0);
        self.tile_h = tile_h.floor().max(1.0);

        let grid_w = self.tile_w * cols;
        let grid_h = self.tile_h * rows;

        self.offset_x = -(grid_w * 0.5) + self.tile_w * 0.5;
        self.offset_z = -(grid_h * 0.5) + self.tile_h * 0.5;
    }
}
