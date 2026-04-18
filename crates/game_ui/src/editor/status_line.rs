use bevy_egui::egui::{Align, Color32, Label, Layout, RichText, Ui};
use game_core::state::vim::{ModeKind, VimState};

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

pub fn render_status_line(ui: &mut Ui, vim_state: &VimState) {
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
