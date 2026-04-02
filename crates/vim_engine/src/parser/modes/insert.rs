use crate::parser::VimParser;
use crate::parser::key::Key;
use crate::parser::result::ParseResult;

pub fn handle(_parser: &mut VimParser, _key: Key) -> ParseResult {
    ParseResult::Incomplete
}
