use crate::message::VimInputMessage;
use crate::state::vim::VimState;
use bevy::prelude::*;
use vim_engine::ParseResult;
use vim_engine::ast::action::Action;
use vim_engine::ast::command::ParsedCommand;

pub fn process_vim_input(
    mut vim_input_reader: MessageReader<VimInputMessage>,
    mut vim_state: ResMut<VimState>,
) {
    for msg in vim_input_reader.read() {
        let result = vim_state.parser.feed(msg.key);

        match result {
            ParseResult::Incomplete => {}
            ParseResult::Success(cmd) => {
                handle_command(&mut vim_state, cmd);
            }
            ParseResult::Invalid(err) => {
                trace!("Vim parse error: {}", err);
            }
        }
    }
}

fn handle_command(vim_state: &mut VimState, cmd: ParsedCommand) {
    match cmd.action {
        Action::Insert(text) => {
            vim_state.buffer.push_str(&text);
        }
        Action::Backspace => {
            vim_state.buffer.pop();
        }
        action => {
            debug!("Action parsed: {:?}", std::mem::discriminant(&action));
        }
    }
}
