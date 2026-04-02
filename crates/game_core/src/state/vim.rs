use bevy::prelude::*;
use vim_engine::parser::VimParser;

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
