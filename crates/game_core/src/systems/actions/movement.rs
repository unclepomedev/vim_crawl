use crate::components::grid::GridPosition;
use crate::components::player::Player;
use crate::events::actions::movement::MoveIntentionEvent;
use crate::resources::map::MapBounds;
use bevy::prelude::*;

pub fn process_movement_intention(
    mut move_reader: MessageReader<MoveIntentionEvent>,
    map_bounds: Res<MapBounds>,
    mut player_q: Query<&mut GridPosition, With<Player>>,
) {
    if let Ok(mut pos) = player_q.single_mut() {
        for intention in move_reader.read() {
            let new_col = pos.col + intention.d_col;
            let new_row = pos.row + intention.d_row;

            if new_col >= map_bounds.min_col
                && new_col <= map_bounds.max_col
                && new_row >= map_bounds.min_row
                && new_row <= map_bounds.max_row
            {
                pos.col = new_col;
                pos.row = new_row;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_app() -> App {
        let mut app = App::new();

        app.insert_resource(MapBounds {
            min_col: 0,
            max_col: 9,
            min_row: 0,
            max_row: 4,
        });

        app.add_message::<MoveIntentionEvent>();
        app.add_systems(Update, process_movement_intention);

        app
    }

    #[test]
    fn move_within_bounds() {
        let mut app = setup_test_app();

        let player_entity = app
            .world_mut()
            .spawn((Player, GridPosition { col: 5, row: 2 }))
            .id();

        app.world_mut().write_message(MoveIntentionEvent {
            d_col: 1,
            d_row: -1,
        });
        app.update();

        let pos = app.world().get::<GridPosition>(player_entity).unwrap();
        assert_eq!(pos.col, 6);
        assert_eq!(pos.row, 1);
    }

    #[test]
    fn move_blocks_at_boundaries() {
        let mut app = setup_test_app();

        let player_entity = app
            .world_mut()
            .spawn((Player, GridPosition { col: 0, row: 0 }))
            .id();

        app.world_mut().write_message(MoveIntentionEvent {
            d_col: -1,
            d_row: -1,
        });
        app.update();

        let pos = app.world().get::<GridPosition>(player_entity).unwrap();
        assert_eq!(pos.col, 0);
        assert_eq!(pos.row, 0);
    }
}
