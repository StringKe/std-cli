use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text, ThemeMode, ThemeProfile};

pub(crate) fn install_visuals(ctx: &egui::Context, theme: &str) -> ThemeProfile {
    ThemeProfile::apply(ctx, ThemeMode::resolve(theme))
}

pub(crate) fn panel_alt(ctx: &egui::Context) -> egui::Color32 {
    Color::bg_surface_2(ctx)
}

pub(crate) fn selected_bg(ctx: &egui::Context) -> egui::Color32 {
    Color::accent_weak(ctx)
}

pub(crate) fn muted_text(ctx: &egui::Context) -> egui::Color32 {
    Color::fg_secondary(ctx)
}

pub(crate) fn strong_text(ctx: &egui::Context) -> egui::Color32 {
    Color::fg_primary(ctx)
}

pub(crate) fn warn_bg(ctx: &egui::Context) -> egui::Color32 {
    Color::status_warning(ctx)
}

pub(crate) fn ok_bg(ctx: &egui::Context) -> egui::Color32 {
    Color::status_success(ctx)
}

pub(crate) fn surface_frame(ctx: &egui::Context) -> egui::Frame {
    egui::Frame::new()
        .fill(Color::bg_surface_0(ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_divider(ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::same(Space::SM))
}

pub(crate) fn subtle_frame(ctx: &egui::Context) -> egui::Frame {
    egui::Frame::new()
        .fill(Color::bg_surface_1(ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_divider(ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::SM))
        .inner_margin(egui::Margin::same(Space::XS))
}

pub(crate) fn section_header(ui: &mut egui::Ui, title: &str, detail: &str) {
    let ctx = ui.ctx().clone();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(title)
                .strong()
                .font(Text::title())
                .color(strong_text(&ctx)),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(detail)
                    .font(Text::caption())
                    .color(muted_text(&ctx)),
            );
        });
    });
    ui.add_space(Space::XS as f32);
}

pub(crate) fn metric(ui: &mut egui::Ui, title: &str, value: impl ToString, detail: &str) {
    let ctx = ui.ctx().clone();
    ui.horizontal(|ui| {
        ui.set_min_height(28.0);
        ui.label(
            egui::RichText::new(value.to_string())
                .font(Text::title())
                .strong()
                .color(strong_text(&ctx)),
        );
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(title).color(strong_text(&ctx)));
            ui.label(
                egui::RichText::new(detail)
                    .font(Text::caption())
                    .color(muted_text(&ctx)),
            );
        });
    });
}

pub(crate) fn chip(ui: &mut egui::Ui, text: &str, fill: egui::Color32) -> egui::Response {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(Radius::SM))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .font(Text::caption())
                    .color(strong_text(&ctx)),
            );
        })
        .response
}

pub(crate) fn quiet_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let ctx = ui.ctx().clone();
    let response = ui.add(
        egui::Button::new(egui::RichText::new(label).color(strong_text(&ctx)))
            .fill(Color::bg_surface_0(&ctx))
            .stroke(egui::Stroke::new(1.0, Color::stroke_divider(&ctx)))
            .corner_radius(egui::CornerRadius::same(Radius::SM)),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    response
}

pub(crate) fn empty_state(ui: &mut egui::Ui, text: &str) {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::LG as f32);
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(text).color(muted_text(&ctx)));
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn quiet_button_registers_button_widget_info_for_all_callers() {
        let source = include_str!("ui.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("pub(crate) fn quiet_button"));
        assert!(implementation.contains("response.widget_info"));
        assert!(implementation.contains("WidgetType::Button"));
        assert!(implementation.contains("WidgetInfo::labeled"));
    }
}
