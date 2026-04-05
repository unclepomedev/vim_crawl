use crate::events::actions::movement::MoveIntentionEvent;
use bevy::prelude::MessageWriter;
use vim_engine::ast::motion::Motion;

pub fn handle_motion(motion: Motion, move_writer: &mut MessageWriter<MoveIntentionEvent>) {
    let delta = match motion {
        Motion::Left => Some((-1, 0)),
        Motion::Down => Some((0, 1)),
        Motion::Up => Some((0, -1)),
        Motion::Right => Some((1, 0)),
        _ => None,
    };

    if let Some((d_col, d_row)) = delta {
        move_writer.write(MoveIntentionEvent { d_col, d_row });
    }
}
