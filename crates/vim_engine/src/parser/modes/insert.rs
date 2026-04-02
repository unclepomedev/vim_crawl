use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::error::ParseError;
use crate::parser::VimParser;
use crate::parser::key::Key;
use crate::parser::result::ParseResult;
use crate::state::CommandContext;

pub fn handle(_parser: &mut VimParser, key: Key) -> ParseResult {
    match key {
        Key::Char(c) => ParseResult::Success(ParsedCommand {
            context: CommandContext::default(),
            action: Action::Insert(c.to_string()),
        }),
        Key::Enter => ParseResult::Success(ParsedCommand {
            context: CommandContext::default(),
            action: Action::Insert("\n".to_string()),
        }),
        Key::Backspace => ParseResult::Success(ParsedCommand {
            context: CommandContext::default(),
            action: Action::Backspace,
        }),
        // TODO: Add support for arrow keys, etc.
        _ => ParseResult::Invalid(ParseError::UnsupportedKey),
    }
}
