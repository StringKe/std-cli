use eframe::egui;
use std_egui::tokens::{LauncherSize, UiScale};

pub(crate) fn result_right_affordance_layout(
    scale: UiScale,
    rect: egui::Rect,
    has_action: bool,
) -> LauncherResultRightAffordanceLayout {
    let gap = LauncherSize::result_right_gap(scale);
    let direct_width = LauncherSize::result_direct_keycap_width(scale);
    let primary_width = LauncherSize::result_primary_keycap_width(scale);
    let action_width = if has_action {
        LauncherSize::result_action_label_width(scale)
    } else {
        0.0
    };
    let mut right = rect.right();
    let direct_keycap = take_right_rect(rect, &mut right, direct_width);
    let primary_keycap = has_action.then(|| {
        right -= gap;
        take_right_rect(rect, &mut right, primary_width)
    });
    let action_label = has_action.then(|| {
        right -= gap;
        take_right_rect(rect, &mut right, action_width)
    });
    LauncherResultRightAffordanceLayout {
        direct_keycap,
        action_label,
        primary_keycap,
    }
}

fn take_right_rect(container: egui::Rect, right: &mut f32, width: f32) -> egui::Rect {
    let rect = egui::Rect::from_min_max(
        egui::pos2((*right - width).max(container.left()), container.top()),
        egui::pos2(*right, container.bottom()),
    );
    *right = rect.left();
    rect
}

pub(crate) fn loading_progress_size(scale: UiScale, available_width: f32) -> egui::Vec2 {
    LauncherSize::loading_progress_size(scale, available_width)
}

pub(crate) fn loading_progress_rect(
    scale: UiScale,
    available_width: f32,
    top_left: egui::Pos2,
) -> egui::Rect {
    LauncherSize::loading_progress_rect(scale, available_width, top_left)
}

pub(crate) fn group_divider_rect(
    scale: UiScale,
    available_width: f32,
    top_left: egui::Pos2,
) -> egui::Rect {
    LauncherSize::group_divider_rect(scale, available_width, top_left)
}

pub(crate) fn group_header_label_offset_y(scale: UiScale) -> f32 {
    LauncherSize::group_header_label_offset_y(scale)
}

pub(crate) fn result_row_size(scale: UiScale, available_width: f32) -> egui::Vec2 {
    LauncherSize::result_row_size(scale, available_width)
}

pub(crate) fn result_row_shrink(scale: UiScale) -> egui::Vec2 {
    LauncherSize::result_row_shrink(scale)
}

pub(crate) fn result_row_layout(scale: UiScale, rect: egui::Rect) -> LauncherResultRowLayout {
    let icon_size = LauncherSize::result_icon_size(scale);
    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + icon_size * 0.5, rect.center().y),
        egui::vec2(icon_size, icon_size),
    );
    let right_width = LauncherSize::result_right_area_width(scale, rect.width());
    let right_rect = egui::Rect::from_min_max(
        egui::pos2(rect.right() - right_width, rect.top()),
        rect.right_bottom(),
    );
    let text_left = icon_rect.right() + LauncherSize::result_icon_text_gap(scale);
    let text_right = right_rect.left() - LauncherSize::result_text_right_gap(scale);
    LauncherResultRowLayout {
        icon_rect,
        title_pos: egui::pos2(text_left, rect.top() + LauncherSize::result_title_y(scale)),
        title_rect: egui::Rect::from_min_size(
            egui::pos2(text_left, rect.top()),
            egui::vec2(
                (text_right - text_left).max(0.0),
                LauncherSize::result_title_height(scale),
            ),
        ),
        subtitle_pos: egui::pos2(
            text_left,
            rect.top() + LauncherSize::result_subtitle_y(scale),
        ),
        text_clip: egui::Rect::from_min_max(
            egui::pos2(text_left, rect.top()),
            egui::pos2(text_right.max(text_left), rect.bottom()),
        ),
        right_rect,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LauncherResultRowLayout {
    pub(crate) icon_rect: egui::Rect,
    pub(crate) title_pos: egui::Pos2,
    pub(crate) title_rect: egui::Rect,
    pub(crate) subtitle_pos: egui::Pos2,
    pub(crate) text_clip: egui::Rect,
    pub(crate) right_rect: egui::Rect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LauncherResultRightAffordanceLayout {
    pub(crate) direct_keycap: egui::Rect,
    pub(crate) action_label: Option<egui::Rect>,
    pub(crate) primary_keycap: Option<egui::Rect>,
}

#[cfg(test)]
pub(crate) fn loading_progress_metrics_for_scale(
    scale: UiScale,
    available_width: f32,
) -> (f32, f32) {
    let rect = loading_progress_rect(scale, available_width, egui::Pos2::ZERO);
    (rect.width(), rect.height())
}

#[cfg(test)]
pub(crate) fn group_header_metrics_for_scale(scale: UiScale, available_width: f32) -> (f32, f32) {
    let rect = group_divider_rect(scale, available_width, egui::Pos2::ZERO);
    (rect.width(), rect.height())
}

#[cfg(test)]
pub(crate) fn group_header_slot_metrics_for_scale(
    scale: UiScale,
    available_width: f32,
) -> (f32, f32, f32) {
    let slot = egui::vec2(
        available_width,
        LauncherSize::group_header_slot_height(scale),
    );
    (slot.x, slot.y, group_header_label_offset_y(scale))
}

#[cfg(test)]
pub(crate) fn result_row_layout_metrics_for_scale(scale: UiScale, width: f32) -> (f32, f32, f32) {
    let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, result_row_size(scale, width));
    let layout = result_row_layout(scale, rect);
    (
        layout.icon_rect.width(),
        layout.text_clip.width(),
        layout.right_rect.width(),
    )
}

#[cfg(test)]
pub(crate) fn result_right_affordance_metrics_for_scale(
    scale: UiScale,
    width: f32,
) -> (f32, f32, f32, f32) {
    let row = egui::Rect::from_min_size(egui::Pos2::ZERO, result_row_size(scale, width));
    let layout = result_row_layout(scale, row);
    let affordance = result_right_affordance_layout(scale, layout.right_rect, true);
    (
        affordance.direct_keycap.width(),
        affordance.action_label.unwrap().width(),
        affordance.primary_keycap.unwrap().width(),
        affordance.primary_keycap.unwrap().left() - affordance.action_label.unwrap().right(),
    )
}
