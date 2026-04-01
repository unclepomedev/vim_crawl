use crate::parser::result::ParseResult;
use crate::state::EditorState;

pub mod result;

#[derive(Default)]
pub struct VimParser {
    pub state: EditorState,
}

impl VimParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn feed(&mut self, _c: char) -> ParseResult {
        ParseResult::Incomplete
    }
}
