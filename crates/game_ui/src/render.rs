use bevy::prelude::*;
use game_core::components::grid::GridPosition;
use game_core::components::player::Player;

type PlayerFilter = (With<Player>, Changed<GridPosition>);

pub fn sync_grid_to_transform(mut query: Query<(&GridPosition, &mut Transform), PlayerFilter>) {
    // TODO: dynamically calculate based on the screen resolution and camera position.
    let tile_size = 40.0;
    let offset_x = -200.0;
    let offset_y = 100.0;

    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = offset_x + (pos.col as f32 * tile_size);
        transform.translation.y = offset_y - (pos.row as f32 * tile_size);
    }
}
