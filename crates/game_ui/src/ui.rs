use crate::resources::grid::GridRenderConfig;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use bevy_egui::egui::{
    Align, Area, CentralPanel, Color32, Context, FontId, Frame, Id, Label, Layout, Margin, Pos2,
    RichText, ScrollArea, TopBottomPanel, Ui,
};
use game_core::state::vim::{ModeKind, VimState};

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
        fill: Color32::from_rgb(8, 18, 12), // almost-black dark green
        inner_margin: Margin::symmetric(8, 4),
        ..default()
    };

    // These panels must always render after warmup, regardless of window availability.
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

fn render_status_line(ui: &mut Ui, vim_state: &VimState) {
    let badge = ModeBadge::from_mode(vim_state);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;

        // Mode block
        ui.add(Label::new(badge.rich_text()));

        // Separator arrow — mimics powerline without requiring a special font.
        ui.label(
            RichText::new("  ")
                .size(13.0)
                .color(Color32::from_rgb(60, 80, 60)),
        );

        // Filename / context.
        ui.label(
            RichText::new("buffer_breach.sh") // TODO: change this according to the stage name
                .monospace()
                .size(12.0)
                .color(Color32::from_rgb(100, 160, 100)),
        );

        // Push the cursor position to the right edge.
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                RichText::new("1,1") // TODO: replace with actual cursor position from VimState
                    .monospace()
                    .size(12.0)
                    .color(Color32::from_rgb(60, 100, 80)),
            );
        });
    });
}

/// Visual style for a vim mode badge in the status line.
struct ModeBadge {
    label: &'static str,
    bg: Color32,
    fg: Color32,
}

impl ModeBadge {
    fn from_mode(vim_state: &VimState) -> Self {
        match vim_state.mode_kind() {
            ModeKind::Normal => Self {
                label: "NORMAL",
                bg: Color32::from_rgb(30, 120, 60), // dark green bg
                fg: Color32::from_rgb(180, 255, 180), // bright green text
            },
            ModeKind::Insert => Self {
                label: "INSERT",
                bg: Color32::from_rgb(20, 80, 140),   // blue bg
                fg: Color32::from_rgb(160, 210, 255), // light blue text
            },
            ModeKind::Visual => Self {
                label: "VISUAL",
                bg: Color32::from_rgb(100, 40, 120),  // purple bg
                fg: Color32::from_rgb(220, 180, 255), // light purple text
            },
            ModeKind::OperatorPending => Self {
                label: "OPERATOR",
                bg: Color32::from_rgb(140, 80, 20),   // amber bg
                fg: Color32::from_rgb(255, 210, 140), // light amber text
            },
        }
    }

    fn rich_text(&self) -> RichText {
        RichText::new(format!(" {} ", self.label))
            .strong()
            .size(13.0)
            .color(self.fg)
            .background_color(self.bg)
    }
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

fn render_grid_labels(ctx: &Context, config: &GridRenderConfig, origin_x: f32, origin_y: f32) {
    let label_color = Color32::from_rgb(40, 100, 160);
    let font = FontId::monospace(16.0);

    let place_label = |ctx: &Context, id: Id, pos: Pos2, text: String| {
        Area::new(id)
            .fixed_pos(pos)
            .interactable(false)
            .show(ctx, |ui| {
                ui.label(RichText::new(text).font(font.clone()).color(label_color));
            });
    };

    let tw = config.tile_w;
    for col in 0..=config.max_col {
        place_label(
            ctx,
            Id::new(("col_label", col)),
            Pos2::new(origin_x + col as f32 * tw + tw * 0.5 - 8.0, origin_y - 24.0),
            (col + 1).to_string(),
        );
    }

    let th = config.tile_h;
    for row in 0..=config.max_row {
        place_label(
            ctx,
            Id::new(("row_label", row)),
            Pos2::new(origin_x - 16.0, origin_y + row as f32 * th + th * 0.5 - 6.0),
            (row + 1).to_string(),
        );
    }
}
