use crate::message::VimInputMessage;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

pub fn route_keyboard_input(
    mut kbd_events: MessageReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut vim_input_writer: MessageWriter<VimInputMessage>,
) {
    let ctrl_pressed = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    for ev in kbd_events.read() {
        if ev.state == ButtonState::Pressed
            && let Key::Character(c) = &ev.logical_key
            && let Some(char_val) = c.chars().next()
        {
            let vim_key = if ctrl_pressed {
                vim_engine::parser::key::Key::Ctrl(char_val.to_ascii_lowercase())
            } else {
                vim_engine::parser::key::Key::Char(char_val)
            };

            vim_input_writer.write(VimInputMessage { key: vim_key });
        }
    }
}
