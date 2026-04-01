use bevy::prelude::*;

#[derive(Message)]
pub struct RawCharMessage {
    pub char: char,
}
