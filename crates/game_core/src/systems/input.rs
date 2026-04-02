use crate::message::RawCharMessage;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

pub fn route_keyboard_input(
    mut keyboard_input_reader: MessageReader<KeyboardInput>,
    mut raw_char_writer: MessageWriter<RawCharMessage>,
) {
    for msg in keyboard_input_reader.read() {
        if msg.state == ButtonState::Pressed
            && let Key::Character(c) = &msg.logical_key
            && let Some(char_val) = c.chars().next()
        {
            raw_char_writer.write(RawCharMessage { char: char_val });
        }
    }
}
