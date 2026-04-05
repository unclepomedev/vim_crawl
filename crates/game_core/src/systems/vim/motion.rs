use crate::events::actions::movement::MoveIntentionEvent;
use bevy::prelude::MessageWriter;
use vim_engine::ast::motion::Motion;

pub fn handle_motion(motion: Motion, move_writer: &mut MessageWriter<MoveIntentionEvent>) {
    let (d_col, d_row) = match motion {
        Motion::Left => (-1, 0),
        Motion::Down => (0, 1),
        Motion::Up => (0, -1),
        Motion::Right => (1, 0),
        _ => (0, 0),
    };

    if d_col != 0 || d_row != 0 {
        move_writer.write(MoveIntentionEvent { d_col, d_row });
    }
}
