use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use game_core::state::vim::VimState;
use game_core::systems::vim::process_vim_input;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, render_editor_ui.after(process_vim_input));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn render_editor_ui(
    mut contexts: EguiContexts,
    vim_state: Res<VimState>,
    mut frame_counter: Local<u32>,
) {
    // To avoid initialization lag between macOS and Metal, the first 10 frames will skip the rendering process.
    if *frame_counter < 10 {
        *frame_counter += 1;
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Vim Engine E2E Test");
        ui.separator();

        let mode_str = format!("{:?}", vim_state.parser.state.mode);
        ui.label(egui::RichText::new(mode_str).strong().size(16.0));

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(egui::RichText::new(&vim_state.buffer).monospace());
        });
    });
}
