use bevy::core_pipeline::fullscreen_material::FullscreenMaterialPlugin;
use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game_core::systems::actions::movement::process_movement_intention;
use game_core::systems::vim::process_vim_input;

pub mod components;
pub mod editor;
pub mod material;
pub mod render;
pub mod resources;
pub mod setup;

use crate::material::{ElectronSeaMaterial, update_world_material};
use crate::resources::grid::GridRenderConfig;
use crate::setup::{recalculate_grid_on_window_resize, sync_player_scale_on_grid_config_change};
use editor::render_editor_ui;
use setup::setup_cameras_and_player;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridRenderConfig>();
        app.add_plugins(FullscreenMaterialPlugin::<ElectronSeaMaterial>::default());
        #[cfg(feature = "dev")]
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_systems(Startup, setup_cameras_and_player);
        app.add_systems(
            Update,
            (
                recalculate_grid_on_window_resize,
                update_world_material,
                render_editor_ui.after(process_vim_input),
                sync_player_scale_on_grid_config_change,
            ),
        );
        app.add_systems(
            Update,
            render::sync_grid_to_transform.after(process_movement_intention),
        );
    }
}
