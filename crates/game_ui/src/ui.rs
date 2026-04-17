use crate::resources::grid::GridRenderConfig;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::{Area, FontId, Id, Pos2};
use bevy_egui::{EguiContexts, egui};
use egui::{CentralPanel, Color32, Frame, RichText, ScrollArea, TopBottomPanel, Ui};
use game_core::state::vim::VimState;

// To avoid initialization lag between macOS and Metal, the first 10 frames will skip the rendering process.
const WARMUP_FRAMES: u32 = 10;

pub fn render_editor_ui(
    mut contexts: EguiContexts,
    vim_state: Res<VimState>,
    config: Res<GridRenderConfig>,
    windows: Query<&Window, With<PrimaryWindow>>,
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

    // Convert grid world-space origin to screen pixels.
    // offset_x / offset_z are the world-space positions of cell (0,0) center.
    let Ok(window) = windows.single() else { return };
    let win_w = window.width();
    let win_h = window.height();
    let cx = win_w * 0.5;
    let cy = win_h * 0.5;

    let grid_origin_x = cx + config.offset_x - config.tile_w * 0.5;
    let grid_origin_y = cy + config.offset_z - config.tile_h * 0.5;

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

    render_grid_labels(ctx, &config, grid_origin_x, grid_origin_y);
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

fn render_grid_labels(
    ctx: &egui::Context,
    config: &GridRenderConfig,
    origin_x: f32,
    origin_y: f32,
) {
    let label_color = Color32::from_rgb(40, 100, 160);
    let font = FontId::monospace(16.0);

    // egui uses logical pixels; Bevy window size is in physical pixels.
    // Divide by pixels_per_point to convert to logical coordinates.
    let ppp = ctx.pixels_per_point();
    let ox = origin_x / ppp;
    let oy = origin_y / ppp;
    let tw = config.tile_w / ppp;
    let th = config.tile_h / ppp;

    // Column numbers along the top edge of the grid.
    for col in 0..=config.max_col {
        let x = ox + col as f32 * tw + tw * 0.5 - 8.0;
        let y = oy - 24.0;

        Area::new(Id::new(("col_label", col)))
            .fixed_pos(Pos2::new(x, y))
            .interactable(false)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new((col + 1).to_string())
                        .font(font.clone())
                        .color(label_color),
                );
            });
    }

    // Row numbers along the left edge of the grid.
    for row in 0..=config.max_row {
        let x = ox - 16.0;
        let y = oy + row as f32 * th + th * 0.5 - 6.0;

        Area::new(Id::new(("row_label", row)))
            .fixed_pos(Pos2::new(x, y))
            .interactable(false)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new((row + 1).to_string())
                        .font(font.clone())
                        .color(label_color),
                );
            });
    }
}
