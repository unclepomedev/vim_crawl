use crate::parser::VimParser;
use crate::parser::result::ParseResult;

pub fn handle(_parser: &mut VimParser, _c: char) -> ParseResult {
    ParseResult::Incomplete
}
