use crate::message::VimInputMessage;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use vim_engine::parser::key::Key as VimKey;

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
                VimKey::Ctrl(char_val.to_ascii_lowercase())
            } else {
                VimKey::Char(char_val)
            };

            vim_input_writer.write(VimInputMessage { key: vim_key });
        }
    }
}
