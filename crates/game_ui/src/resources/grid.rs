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

// TODO: dynamically calculate based on the screen resolution and camera position.
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
