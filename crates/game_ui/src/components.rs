use bevy::prelude::*;
pub use game_core::components::grid::GridPosition;
pub use game_core::components::player::Player;

pub type PlayerFilter = (With<Player>, Changed<GridPosition>);

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct RenderConfig {
    pub tile_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

// TODO: dynamically calculate based on the screen resolution and camera position.
impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            tile_size: 40.0,
            offset_x: -200.0,
            offset_y: 100.0,
        }
    }
}
