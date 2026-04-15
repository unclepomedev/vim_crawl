use bevy::core_pipeline::fullscreen_material::FullscreenMaterialPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game_core::systems::actions::movement::process_movement_intention;
use game_core::systems::vim::process_vim_input;

pub mod components;
pub mod material;
pub mod render;
pub mod resources;
pub mod setup;
pub mod ui;

use crate::material::{ElectronSeaMaterial, update_world_material};
use crate::resources::grid::GridRenderConfig;
use setup::setup_cameras_and_player;
use ui::render_editor_ui;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridRenderConfig>();
        app.add_plugins(FullscreenMaterialPlugin::<ElectronSeaMaterial>::default());
        app.add_plugins(WorldInspectorPlugin::new()); // TODO: toggle with debug flag
        app.add_systems(Startup, setup_cameras_and_player);
        app.add_systems(
            Update,
            (
                update_world_material,
                render_editor_ui.after(process_vim_input),
            ),
        );
        app.add_systems(
            Update,
            render::sync_grid_to_transform.after(process_movement_intention),
        );
    }
}
