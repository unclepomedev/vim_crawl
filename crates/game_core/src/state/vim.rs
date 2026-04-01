use bevy::prelude::*;
use vim_engine::parser::VimParser;

#[derive(Resource, Default)]
pub struct VimState {
    pub parser: VimParser,
}
