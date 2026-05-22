use crate::{ui_metrics, ui_result_model::LauncherResultListItem};

const OVERSCAN_ROWS: usize = 5;

pub(crate) fn total_height(items: &[LauncherResultListItem]) -> f32 {
    items.iter().map(item_height).sum()
}

pub(crate) fn item_height(item: &LauncherResultListItem) -> f32 {
    match item {
        LauncherResultListItem::Group { .. } => ui_metrics::group_header_slot_height(),
        LauncherResultListItem::Row(_) => ui_metrics::result_list_slot_height(),
    }
}

pub(crate) fn visible_range(
    items: &[LauncherResultListItem],
    clip_min_y: f32,
    clip_max_y: f32,
) -> (usize, usize, f32) {
    let mut y = 0.0;
    let mut start = 0;
    for (index, item) in items.iter().enumerate() {
        let next_y = y + item_height(item);
        if next_y > clip_min_y {
            start = index;
            break;
        }
        y = next_y;
        start = index + 1;
    }

    let mut end = start;
    let mut row_y = y;
    for item in &items[start..] {
        if row_y > clip_max_y {
            break;
        }
        row_y += item_height(item);
        end += 1;
    }
    let visible_start = start;
    let visible_end = end.max(start);
    let start = visible_start.saturating_sub(OVERSCAN_ROWS);
    let end = (visible_end + OVERSCAN_ROWS).min(items.len());
    let y = height_before(items, start);
    (start, end, y)
}

fn height_before(items: &[LauncherResultListItem], index: usize) -> f32 {
    items.iter().take(index).map(item_height).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui_result_model::LauncherResultRowModel;

    #[test]
    fn virtual_results_measure_group_headers_at_spec_height() {
        let items = vec![
            LauncherResultListItem::Group {
                label: "Action / Workflow".to_string(),
            },
            LauncherResultListItem::Row(Box::new(row("Index", 0))),
            LauncherResultListItem::Row(Box::new(row("Terminal", 1))),
        ];

        assert_eq!(item_height(&items[0]), 24.0);
        assert_eq!(item_height(&items[1]), 36.0);
        assert_eq!(total_height(&items), 96.0);
    }

    #[test]
    fn virtual_results_find_visible_slice_with_variable_heights() {
        let items = vec![
            LauncherResultListItem::Group {
                label: "Action / Workflow".to_string(),
            },
            LauncherResultListItem::Row(Box::new(row("Index", 0))),
            LauncherResultListItem::Group {
                label: "App / File".to_string(),
            },
            LauncherResultListItem::Row(Box::new(row("Studio", 1))),
        ];

        assert_eq!(visible_range(&items, 30.0, 84.0), (0, 4, 0.0));
    }

    #[test]
    fn virtual_results_include_five_rows_of_overscan() {
        let items = (0..20)
            .map(|index| LauncherResultListItem::Row(Box::new(row("Item", index))))
            .collect::<Vec<_>>();

        assert_eq!(visible_range(&items, 360.0, 396.0), (5, 17, 180.0));
    }

    fn row(title: &str, result_index: usize) -> LauncherResultRowModel {
        LauncherResultRowModel {
            title: title.to_string(),
            title_segments: vec![crate::ui_result_model::TitleSegment {
                text: title.to_string(),
                matched: false,
            }],
            subtitle: "Test row".to_string(),
            match_badge: None,
            kind: "Command".to_string(),
            icon_label: "CMD".to_string(),
            group: "Action / Workflow".to_string(),
            position: format!("{} of 2", result_index + 1),
            direct_shortcut: None,
            primary_shortcut: None,
            action_hint: None,
            action_label: "Run".to_string(),
            result_index,
        }
    }
}
