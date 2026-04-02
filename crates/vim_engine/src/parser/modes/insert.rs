use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::error::ParseError;
use crate::parser::VimParser;
use crate::parser::key::Key;
use crate::parser::result::ParseResult;
use crate::state::CommandContext;

pub fn handle(_parser: &mut VimParser, key: Key) -> ParseResult {
    if let Key::Char(c) = key {
        ParseResult::Success(ParsedCommand {
            context: CommandContext::default(),
            action: Action::Insert(c.to_string()),
        })
    } else {
        // Unsupported keys in insert mode should signal an error
        // TODO: Add support for Backspace, arrow keys, etc.
        ParseResult::Invalid(ParseError::UnsupportedKey)
    }
}
