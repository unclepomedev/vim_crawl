use crate::message::VimInputMessage;
use crate::state::vim::VimState;
use bevy::prelude::*;
use vim_engine::ParseResult;

pub fn process_vim_input(
    mut vim_input_reader: MessageReader<VimInputMessage>,
    mut vim_state: ResMut<VimState>,
) {
    for msg in vim_input_reader.read() {
        let result = vim_state.parser.feed(msg.key);

        match result {
            ParseResult::Incomplete => {}
            ParseResult::Success(cmd) => {
                info!("Vim parse success: {:?}", cmd);
            }
            ParseResult::Invalid(err) => {
                warn!("Vim parse error: {}", err);
            }
        }
    }
}
