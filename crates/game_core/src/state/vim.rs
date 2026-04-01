use bevy::prelude::*;
use vim_engine::VimParser;

#[derive(Resource, Default)]
pub struct VimState {
    pub parser: VimParser,
}
