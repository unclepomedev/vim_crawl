use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GridRenderConfig {
    pub tile_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

// TODO: dynamically calculate based on the screen resolution and camera position.
impl Default for GridRenderConfig {
    fn default() -> Self {
        Self {
            tile_size: 80.0,
            offset_x: -200.0,
            offset_y: 100.0,
        }
    }
}
