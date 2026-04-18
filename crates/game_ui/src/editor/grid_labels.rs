use crate::resources::grid::GridRenderConfig;
use bevy_egui::egui::{Area, Color32, Context, FontId, Id, Pos2, RichText};

pub fn render_grid_labels(ctx: &Context, config: &GridRenderConfig, origin_x: f32, origin_y: f32) {
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
