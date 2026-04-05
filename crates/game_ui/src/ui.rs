use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui::{CentralPanel, Color32, Frame, RichText, ScrollArea, TopBottomPanel, Ui};
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

    let bottom_frame = Frame {
        fill: Color32::from_rgba_premultiplied(30, 30, 40, 255),
        ..default()
    };

    TopBottomPanel::bottom("vim_status_line")
        .frame(bottom_frame)
        .show(ctx, |ui| render_status_line(ui, &vim_state));

    let frame = Frame {
        fill: Color32::from_rgba_premultiplied(10, 10, 15, 200),
        ..Frame::central_panel(&ctx.style())
    };

    CentralPanel::default().frame(frame).show(ctx, |ui| {
        render_header(ui);
        ui.separator();
        render_text_buffer(ui, &vim_state.buffer);
    });
}

fn render_status_line(ui: &mut Ui, vim_state: &VimState) {
    ui.add_space(4.0);

    let mode_str = vim_state.get_mode_display_string();

    ui.label(
        RichText::new(mode_str)
            .strong()
            .size(14.0)
            .color(Color32::WHITE),
    );

    ui.add_space(4.0);
}

fn render_header(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.heading(RichText::new("Vim Engine E2E Test").color(Color32::GREEN));

        if let Some(hash) = option_env!("COMMIT_HASH") {
            ui.label(RichText::new(hash).small().color(Color32::GRAY));
        }
    });
}

fn render_text_buffer(ui: &mut Ui, buffer: &str) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.label(
            RichText::new(buffer)
                .monospace()
                .color(Color32::LIGHT_GREEN),
        );
    });
}
