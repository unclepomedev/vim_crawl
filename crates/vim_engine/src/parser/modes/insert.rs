use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::parser::VimParser;
use crate::parser::key::Key;
use crate::parser::result::ParseResult;

pub fn handle(_parser: &mut VimParser, key: Key) -> ParseResult {
    if let Key::Char(c) = key {
        ParseResult::Success(ParsedCommand {
            context: crate::state::CommandContext::default(),
            action: Action::Insert(c.to_string()),
        })
    } else {
        ParseResult::Incomplete
    }
}
