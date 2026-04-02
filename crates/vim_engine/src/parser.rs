use crate::parser::result::ParseResult;
use crate::state::{EditorState, Mode};

pub mod insert;
pub mod mapping;
pub mod normal;
pub mod operator_pending;
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
        let result = match self.state.mode {
            Mode::Normal => normal::handle(self, c),
            Mode::Insert => insert::handle(self, c),
            Mode::Visual => visual::handle(self, c),
            Mode::OperatorPending(op) => operator_pending::handle(self, op, c),
        };

        match result {
            ParseResult::Success(_) | ParseResult::Invalid(_) => {
                self.state.context.count = None;
            }
            ParseResult::Incomplete => {}
        }

        result
    }
}
