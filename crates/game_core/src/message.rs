use bevy::prelude::*;
use vim_engine::parser::key::Key;

#[derive(Event, Debug, Clone, Message)]
pub struct VimInputMessage {
    pub key: Key,
}
