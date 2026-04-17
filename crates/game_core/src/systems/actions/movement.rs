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
    let Ok(mut pos) = player_q.single_mut() else {
        return;
    };

    for intention in move_reader.read() {
        let new_col = pos.col + intention.d_col;
        let new_row = pos.row + intention.d_row;

        if map_bounds.is_passable(new_col, new_row) {
            pos.col = new_col;
            pos.row = new_row;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_app() -> App {
        let mut app = App::new();

        app.insert_resource(MapBounds::default());

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

    #[test]
    fn move_blocks_at_enemy_spawn_boundary() {
        let mut app = setup_test_app();

        // Passable area is 0-7. Col 8 and 9 are spawn areas.
        let player_entity = app
            .world_mut()
            .spawn((Player, GridPosition { col: 7, row: 0 }))
            .id();

        app.world_mut()
            .write_message(MoveIntentionEvent { d_col: 1, d_row: 0 });
        app.update();

        let pos = app.world().get::<GridPosition>(player_entity).unwrap();
        assert_eq!(pos.col, 7); // Blocked at col 8
    }

    #[test]
    fn move_multiple_events_processed_in_one_frame() {
        let mut app = setup_test_app();

        let player_entity = app
            .world_mut()
            .spawn((Player, GridPosition { col: 5, row: 2 }))
            .id();

        app.world_mut()
            .write_message(MoveIntentionEvent { d_col: 1, d_row: 0 });
        app.world_mut()
            .write_message(MoveIntentionEvent { d_col: 1, d_row: 0 });
        app.world_mut().write_message(MoveIntentionEvent {
            d_col: 0,
            d_row: -1,
        });
        app.update();

        let pos = app.world().get::<GridPosition>(player_entity).unwrap();
        assert_eq!(pos.col, 7);
        assert_eq!(pos.row, 1);
    }

    #[test]
    fn move_ignores_invalid_intentions_and_keeps_valid_ones() {
        let mut app = setup_test_app();

        let player_entity = app
            .world_mut()
            .spawn((Player, GridPosition { col: 0, row: 0 }))
            .id();

        app.world_mut().write_message(MoveIntentionEvent {
            d_col: -1,
            d_row: 0,
        });
        app.world_mut()
            .write_message(MoveIntentionEvent { d_col: 0, d_row: 1 });
        app.update();

        let pos = app.world().get::<GridPosition>(player_entity).unwrap();
        assert_eq!(pos.col, 0);
        assert_eq!(pos.row, 1);
    }
}
