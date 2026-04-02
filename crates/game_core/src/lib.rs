use crate::message::VimInputMessage;
use bevy::prelude::*;

mod message;
pub mod state;
pub mod systems;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<state::vim::VimState>()
            .add_message::<VimInputMessage>()
            .add_systems(
                Update,
                (
                    systems::input::route_keyboard_input,
                    systems::vim::process_vim_input,
                )
                    .chain(),
            );
    }
}
