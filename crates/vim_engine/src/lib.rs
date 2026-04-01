use bevy::prelude::*;

pub struct VimEnginePlugin;

impl Plugin for VimEnginePlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Default)]
pub struct VimParser;

impl VimParser {
    pub fn feed(&mut self, _p0: char) {
        todo!()
    }
}
// TODO: Implement
