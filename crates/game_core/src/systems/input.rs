use crate::message::VimInputMessage;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key as BevyKey, KeyboardInput};
use bevy::prelude::*;
use vim_engine::parser::key::Key as VimKey;

pub fn route_keyboard_input(
    mut kbd_events: MessageReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut vim_input_writer: MessageWriter<VimInputMessage>,
) {
    let ctrl_pressed = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    for ev in kbd_events.read() {
        if ev.state == ButtonState::Pressed {
            let vim_key = match &ev.logical_key {
                BevyKey::Character(c) => {
                    if let Some(char_val) = c.chars().next() {
                        if ctrl_pressed {
                            Some(VimKey::Ctrl(char_val.to_ascii_lowercase()))
                        } else {
                            Some(VimKey::Char(char_val))
                        }
                    } else {
                        None
                    }
                }
                BevyKey::Escape => Some(VimKey::Esc),
                BevyKey::Enter => Some(VimKey::Enter),
                BevyKey::Backspace => Some(VimKey::Backspace),
                _ => None,
            };

            if let Some(key) = vim_key {
                vim_input_writer.write(VimInputMessage { key });
            }
        }
    }
}
