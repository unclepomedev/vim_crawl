use bevy::core_pipeline::fullscreen_material::FullscreenMaterialPlugin;
use bevy::prelude::*;
use game_core::systems::vim::process_vim_input;

pub mod components;
pub mod material;
pub mod setup;
pub mod ui;

use material::{ElectronSeaMaterial, update_world_material};
use setup::setup_cameras_and_player;
use ui::render_editor_ui;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FullscreenMaterialPlugin::<ElectronSeaMaterial>::default());
        app.add_systems(Startup, setup_cameras_and_player);
        app.add_systems(
            Update,
            (
                update_world_material,
                render_editor_ui.after(process_vim_input),
            ),
        );
    }
}
