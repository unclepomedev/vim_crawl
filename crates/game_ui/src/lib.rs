use bevy::core_pipeline::fullscreen_material::FullscreenMaterialPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game_core::systems::actions::movement::process_movement_intention;

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

/// A SystemSet to separate the update and consumption phases of `GridRenderConfig`.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GridConfigSet {
    /// Systems that rewrite `GridRenderConfig`
    Update,
    /// A group of systems that read `GridRenderConfig` and apply it to something.
    Consume,
}

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridRenderConfig>();
        app.add_plugins(FullscreenMaterialPlugin::<ElectronSeaMaterial>::default());
        #[cfg(feature = "dev")]
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_systems(Startup, setup_cameras_and_player);

        // Declare the order between SystemSets
        app.configure_sets(
            Update,
            (GridConfigSet::Update, GridConfigSet::Consume).chain(),
        );

        // Update phase
        app.add_systems(
            Update,
            recalculate_grid_on_window_resize.in_set(GridConfigSet::Update),
        );

        // Consume phase
        app.add_systems(
            Update,
            (
                update_world_material,
                sync_player_scale_on_grid_config_change,
                render::sync_grid_to_transform.after(process_movement_intention),
            )
                .in_set(GridConfigSet::Consume),
        );

        // Alternative schedule
        app.add_systems(EguiPrimaryContextPass, render_editor_ui);
    }
}
