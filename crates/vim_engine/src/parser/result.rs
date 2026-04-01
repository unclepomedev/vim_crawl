use crate::ast::command::ParsedCommand;
use crate::error::ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseResult {
    Incomplete,
    Success(ParsedCommand),
    Invalid(ParseError),
}
