use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use game_core::state::vim::VimState;

// To avoid initialization lag between macOS and Metal, the first 10 frames will skip the rendering process.
const WARMUP_FRAMES: u32 = 10;

pub fn render_editor_ui(
    mut contexts: EguiContexts,
    vim_state: Res<VimState>,
    mut frame_counter: Local<u32>,
) {
    if *frame_counter < WARMUP_FRAMES {
        *frame_counter += 1;
        return;
    }

    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let mut frame = egui::Frame::central_panel(&ctx.style());
    frame.fill = egui::Color32::from_rgba_premultiplied(10, 10, 15, 200);

    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new("Vim Engine E2E Test").color(egui::Color32::GREEN));

            if let Some(hash) = option_env!("COMMIT_HASH") {
                ui.label(egui::RichText::new(hash).small().color(egui::Color32::GRAY));
            }
        });

        ui.separator();

        let mode_str = format!("{:?}", vim_state.parser.state.mode);
        ui.label(
            egui::RichText::new(mode_str)
                .strong()
                .size(16.0)
                .color(egui::Color32::WHITE),
        );

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(
                egui::RichText::new(&vim_state.buffer)
                    .monospace()
                    .color(egui::Color32::LIGHT_GREEN),
            );
        });
    });
}
