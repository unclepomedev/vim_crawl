use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use game_core::GameCorePlugin;
use game_ui::GameUiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameCorePlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(GameUiPlugin)
        .run();
}
