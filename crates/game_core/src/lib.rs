use crate::message::VimInputMessage;
use crate::state::vim::VimState;
use bevy::prelude::*;
use systems::input::route_keyboard_input;
use systems::vim::process_vim_input;

mod message;
pub mod state;
pub mod systems;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VimState>()
            .add_message::<VimInputMessage>()
            .add_systems(Update, (route_keyboard_input, process_vim_input).chain());
    }
}
