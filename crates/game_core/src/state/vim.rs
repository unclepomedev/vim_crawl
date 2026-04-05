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

impl VimState {
    pub fn get_mode_display_string(&self) -> String {
        match self.parser.state.mode {
            Mode::Normal => "-- NORMAL --".to_string(),
            Mode::Insert => "-- INSERT --".to_string(),
            Mode::Visual => "-- VISUAL --".to_string(),
            Mode::OperatorPending(op) => format!("-- OPERATOR PENDING ({op:?}) --"),
        }
    }
}
