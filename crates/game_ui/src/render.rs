use crate::resources::grid::GridRenderConfig;
use bevy::prelude::*;
use game_core::components::grid::GridPosition;
use game_core::components::player::Player;

pub type PlayerFilter = (With<Player>, Changed<GridPosition>);

pub fn sync_grid_to_transform(
    config: Res<GridRenderConfig>,
    mut query: Query<(&GridPosition, &mut Transform), PlayerFilter>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = config.offset_x + (pos.col as f32 * config.tile_size);
        transform.translation.y = 0.0;
        transform.translation.z = -(config.offset_y - (pos.row as f32 * config.tile_size));
    }
}
