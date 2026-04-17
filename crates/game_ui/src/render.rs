use crate::resources::grid::GridRenderConfig;
use bevy::prelude::*;
use game_core::components::grid::GridPosition;
use game_core::components::player::Player;

pub fn sync_grid_to_transform(
    config: Res<GridRenderConfig>,
    mut query: Query<(Ref<GridPosition>, &mut Transform), With<Player>>,
) {
    let config_changed = config.is_changed();

    for (pos, mut transform) in query.iter_mut() {
        if config_changed || pos.is_changed() {
            transform.translation.x = config.offset_x + (pos.col as f32 * config.tile_w);
            transform.translation.y = 0.0;
            transform.translation.z = config.offset_z + (pos.row as f32 * config.tile_h);
        }
    }
}
