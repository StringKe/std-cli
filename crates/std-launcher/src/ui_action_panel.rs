use crate::ui_parts::{keycap, quiet_label};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};
use std_launcher::{ActionPanelItem, LauncherKey, LauncherState};

pub(crate) fn render(ctx: &egui::Context, anchor_rect: egui::Rect, state: &mut LauncherState) {
    if !state.action_panel.open {
        return;
    }
    let width = 320.0_f32.min(anchor_rect.width());
    let pos = egui::pos2(
        anchor_rect.right() - width,
        anchor_rect.top() - action_panel_height(state),
    );
    egui::Area::new("launcher_action_panel".into())
        .order(egui::Order::Foreground)
        .fixed_pos(pos)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(Color::bg_surface_1(ctx))
                .stroke(egui::Stroke::new(1.0, Color::stroke_border(ctx)))
                .corner_radius(egui::CornerRadius::same(Radius::LG))
                .inner_margin(egui::Margin::same(Space::SM))
                .show(ui, |ui| {
                    ui.set_width(width);
                    header(ui, state);
                    ui.add_space(Space::XS as f32);
                    search(ui, state);
                    ui.add_space(Space::XS as f32);
                    actions(ui, state);
                });
        });
}

fn action_panel_height(state: &LauncherState) -> f32 {
    let row_height = 34.0 * state.action_panel.items.len() as f32;
    44.0 + row_height
}

fn header(ui: &mut egui::Ui, state: &LauncherState) {
    let ctx = ui.ctx().clone();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(i18n::t("launcher.action.actions"))
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            quiet_label(ui, &state.action_panel.action_name);
        });
    });
}

fn search(ui: &mut egui::Ui, state: &mut LauncherState) {
    let ctx = ui.ctx().clone();
    let response = ui.add_sized(
        [ui.available_width(), 28.0],
        egui::TextEdit::singleline(&mut state.action_panel.query)
            .hint_text("Filter actions")
            .font(Text::body()),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            "Action Panel filter",
        )
    });
    if response.changed() {
        state.update_action_panel_query(state.action_panel.query.clone());
    }
    ui.painter().rect_stroke(
        response.rect.expand(2.0),
        egui::CornerRadius::same(Radius::MD),
        egui::Stroke::new(1.0, Color::stroke_divider(&ctx)),
        egui::StrokeKind::Outside,
    );
}

fn actions(ui: &mut egui::Ui, state: &mut LauncherState) {
    let items = state
        .action_panel
        .visible_items()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    if items.is_empty() {
        quiet_label(ui, "No matching actions");
        return;
    }
    for (index, item) in items.iter().enumerate() {
        if action_row(ui, item, index == state.action_panel.selected).clicked() {
            state.action_panel.selected = index;
        }
        ui.add_space(Space::TWO_XS as f32);
    }
    if ui.input(|input| input.key_pressed(egui::Key::ArrowDown)) {
        state.handle_keyboard_input_by_user(LauncherKey::ArrowDown, false);
    }
    if ui.input(|input| input.key_pressed(egui::Key::ArrowUp)) {
        state.handle_keyboard_input_by_user(LauncherKey::ArrowUp, false);
    }
}

fn action_row(ui: &mut egui::Ui, item: &ActionPanelItem, selected: bool) -> egui::Response {
    let ctx = ui.ctx().clone();
    let fill = if selected {
        Color::accent_weak(&ctx)
    } else {
        egui::Color32::TRANSPARENT
    };
    egui::Frame::new()
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            let response =
                ui.allocate_response(egui::vec2(ui.available_width(), 32.0), egui::Sense::click());
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    format!("{} action", item.title()),
                )
            });
            ui.scope_builder(egui::UiBuilder::new().max_rect(response.rect), |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(item.title())
                            .font(Text::body())
                            .color(Color::fg_primary(&ctx)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        keycap(ui, item.shortcut());
                    });
                });
            });
            response
        })
        .inner
}
