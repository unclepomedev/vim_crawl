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
                warn!("Vim parse error: {}", err);
            }
        }
    }
}

fn handle_command(_vim_state: &mut VimState, cmd: ParsedCommand) {
    info!("Vim parsed command: {:?}", cmd);

    match cmd.action {
        Action::EnterInsert => {
            info!("Entering Insert Mode");
        }
        Action::EnterVisual => {
            info!("Entering Visual Mode");
        }
        Action::Cancel => {
            info!("Command cancelled / Returned to Normal Mode");
        }
        Action::Insert(text) => {
            info!("Inserting text: {}", text);
        }
        Action::Move(motion) => {
            info!("Moving cursor: {:?}", motion);
        }
        Action::Operate(op, target) => {
            info!("Operating: {:?} on {:?}", op, target);
        }
        Action::Undo => {
            info!("Undo action");
        }
        Action::Redo => {
            info!("Redo action");
        }
        _ => {}
    }
}
