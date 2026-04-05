use crate::events::actions::movement::MoveIntentionEvent;
use crate::message::VimInputMessage;
use crate::resources::map::MapBounds;
use crate::state::vim::VimState;
use crate::systems::actions::movement::process_movement_intention;
use crate::systems::input::route_keyboard_input;
use crate::systems::vim::process_vim_input;
use bevy::prelude::*;

pub mod components;
pub mod events;
mod message;
pub mod resources;
pub mod state;
pub mod systems;
pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VimState>()
            .init_resource::<MapBounds>()
            .add_message::<MoveIntentionEvent>()
            .add_message::<VimInputMessage>()
            .add_systems(
                Update,
                (
                    route_keyboard_input,
                    process_vim_input,
                    process_movement_intention,
                )
                    .chain(),
            );
    }
}
