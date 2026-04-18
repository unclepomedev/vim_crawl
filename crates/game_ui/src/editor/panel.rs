use bevy_egui::egui::{Color32, RichText, ScrollArea, Ui};

pub fn render_header(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.heading(RichText::new("Vim Engine E2E Test").color(Color32::GREEN));

        if let Some(hash) = option_env!("COMMIT_HASH") {
            ui.label(RichText::new(hash).small().color(Color32::GRAY));
        }
    });
}

pub fn render_text_buffer(ui: &mut Ui, buffer: &str) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.label(
            RichText::new(buffer)
                .monospace()
                .color(Color32::LIGHT_GREEN),
        );
    });
}
