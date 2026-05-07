mod grid_labels;
mod panel;
mod status_line;

use self::grid_labels::render_grid_labels;
use self::panel::{render_header, render_text_buffer};
use self::status_line::render_status_line;
use crate::resources::grid::GridRenderConfig;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use bevy_egui::egui::{CentralPanel, Color32, Frame, Margin, TopBottomPanel};
use game_core::state::vim::VimState;

pub fn render_editor_ui(
    mut contexts: EguiContexts,
    vim_state: Res<VimState>,
    config: Res<GridRenderConfig>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let bottom_frame = Frame {
        fill: Color32::from_rgb(8, 18, 12), // almost-black dark green
        inner_margin: Margin::symmetric(8, 4),
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

    // Grid labels require the window size; skip silently if unavailable.
    let Ok(window) = windows.single() else { return };
    let cx = window.width() * 0.5;
    let cy = window.height() * 0.5;
    let grid_origin_x = cx + config.offset_x - config.tile_w * 0.5;
    let grid_origin_y = cy + config.offset_z - config.tile_h * 0.5;
    render_grid_labels(ctx, &config, grid_origin_x, grid_origin_y);
}
