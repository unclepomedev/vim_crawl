pub mod edit;
pub mod motion;

use crate::events::actions::movement::MoveIntentionEvent;
use crate::message::VimInputMessage;
use crate::state::vim::VimState;
use bevy::prelude::*;
use vim_engine::ParseResult;
use vim_engine::ast::action::Action;
use vim_engine::ast::command::ParsedCommand;

pub fn process_vim_input(
    mut vim_input_reader: MessageReader<VimInputMessage>,
    mut vim_state: ResMut<VimState>,
    mut move_writer: MessageWriter<MoveIntentionEvent>,
) {
    for msg in vim_input_reader.read() {
        let result = vim_state.parser.feed(msg.key);

        match result {
            ParseResult::Incomplete => {}
            ParseResult::Success(cmd) => {
                dispatch_command(&mut vim_state, cmd, &mut move_writer);
            }
            ParseResult::Invalid(err) => {
                trace!("Vim parse error: {}", err);
            }
        }
    }
}

fn dispatch_command(
    vim_state: &mut VimState,
    cmd: ParsedCommand,
    move_writer: &mut MessageWriter<MoveIntentionEvent>,
) {
    match cmd.action {
        Action::Insert(text) => {
            edit::handle_insert(vim_state, text);
        }
        Action::Backspace => {
            edit::handle_backspace(vim_state);
        }
        Action::Move(motion) => {
            motion::handle_motion(motion, move_writer);
        }
        action => {
            debug!("Action parsed: {:?}", std::mem::discriminant(&action));
        }
    }
}
