use crate::components::RenderConfig;
use crate::components::{GridPosition, PlayerFilter};
use bevy::prelude::*;

pub fn sync_grid_to_transform(
    config: Res<RenderConfig>,
    mut query: Query<(&GridPosition, &mut Transform), PlayerFilter>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = config.offset_x + (pos.col as f32 * config.tile_size);
        transform.translation.y = config.offset_y - (pos.row as f32 * config.tile_size);
    }
}
