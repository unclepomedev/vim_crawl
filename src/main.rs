use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use game_core::GameCorePlugin;
use game_ui::GameUiPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Vim Crawl".into(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GameCorePlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(GameUiPlugin)
        .run();
}
