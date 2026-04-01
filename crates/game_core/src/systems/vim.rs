use crate::message::RawCharMessage;
use crate::state::vim::VimState;
use bevy::prelude::*;
use vim_engine::ParseResult;

pub fn process_vim_input(
    mut raw_char_reader: MessageReader<RawCharMessage>,
    mut vim_state: ResMut<VimState>,
) {
    for msg in raw_char_reader.read() {
        let result = vim_state.parser.feed(msg.char);

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
