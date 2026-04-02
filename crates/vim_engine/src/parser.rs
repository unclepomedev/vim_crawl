use crate::ast::motion::Motion;
use crate::parser::result::ParseResult;
use crate::state::{EditorState, Mode, PendingAction};

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
        let result = if let Some(pending) = self.state.context.pending_action.take() {
            self.resolve_pending(pending, c)
        } else {
            self.route_input(c)
        };

        match result {
            ParseResult::Success(_) | ParseResult::Invalid(_) => {
                self.state.context.reset();
            }
            ParseResult::Incomplete => {}
        }

        result
    }

    fn resolve_pending(&mut self, pending: PendingAction, c: char) -> ParseResult {
        let motion = match pending {
            PendingAction::FindForward => Motion::FindForward(c),
            PendingAction::FindBackward => Motion::FindBackward(c),
            PendingAction::TillForward => Motion::TillForward(c),
            PendingAction::TillBackward => Motion::TillBackward(c),
        };
        match self.state.mode {
            Mode::Normal => normal::handle_motion(self, motion),
            Mode::OperatorPending(op) => operator_pending::handle_motion(self, op, motion),
            _ => ParseResult::Incomplete,
        }
    }

    fn route_input(&mut self, c: char) -> ParseResult {
        match self.state.mode {
            Mode::Normal => normal::handle(self, c),
            Mode::Insert => insert::handle(self, c),
            Mode::Visual => visual::handle(self, c),
            Mode::OperatorPending(op) => operator_pending::handle(self, op, c),
        }
    }
}
