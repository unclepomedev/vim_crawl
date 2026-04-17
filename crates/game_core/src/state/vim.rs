use bevy::prelude::*;
use vim_engine::parser::VimParser;
use vim_engine::state::Mode;

#[derive(Resource)]
pub struct VimState {
    pub parser: VimParser,
    pub buffer: String,
}

impl Default for VimState {
    fn default() -> Self {
        Self {
            parser: VimParser::new(),
            buffer: String::new(),
        }
    }
}

/// A UI-facing summary of the current editor mode.
/// This type intentionally mirrors `vim_engine::state::Mode`
/// without exposing the underlying crate to downstream consumers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeKind {
    Normal,
    Insert,
    Visual,
    OperatorPending(String),
}

impl VimState {
    pub fn mode_kind(&self) -> ModeKind {
        match self.parser.state.mode {
            Mode::Normal => ModeKind::Normal,
            Mode::Insert => ModeKind::Insert,
            Mode::Visual => ModeKind::Visual,
            Mode::OperatorPending(op) => ModeKind::OperatorPending(format!("{op:?}")),
        }
    }
}
