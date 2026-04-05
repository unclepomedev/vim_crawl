use bevy::prelude::*;

#[derive(Event, Debug, Clone, Message)]
pub struct MoveIntentionEvent {
    pub d_col: i32,
    pub d_row: i32,
}
