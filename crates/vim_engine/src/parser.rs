use crate::parser::result::ParseResult;
use crate::state::{EditorState, Mode};

pub mod insert;
pub mod normal;
pub mod result;
pub mod visual;

#[derive(Default)]
pub struct VimParser {
    pub state: EditorState,
}

impl VimParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn feed(&mut self, c: char) -> ParseResult {
        match self.state.mode {
            Mode::Normal => normal::handle(self, c),
            Mode::Insert => insert::handle(self, c),
            Mode::Visual => visual::handle(self, c),
        }
    }
}
