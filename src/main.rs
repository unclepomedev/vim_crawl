use bevy::prelude::*;
use game_core::GameCorePlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameCorePlugin)
        .run();
}
