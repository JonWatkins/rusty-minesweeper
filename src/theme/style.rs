use eframe::egui;

pub fn apply_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals = egui::Visuals::dark();

    let rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.inactive.rounding = rounding;
    style.visuals.widgets.hovered.rounding = rounding;
    style.visuals.widgets.active.rounding = rounding;
    style.visuals.window_rounding = egui::Rounding::same(12.0);

    style.visuals.selection.bg_fill = egui::Color32::from_rgb(59, 130, 246);
    style.visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(147, 197, 253));
    style.visuals.hyperlink_color = egui::Color32::from_rgb(96, 165, 250);

    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.spacing.window_margin = egui::style::Margin::same(8.0);

    ctx.set_style(style);
}
