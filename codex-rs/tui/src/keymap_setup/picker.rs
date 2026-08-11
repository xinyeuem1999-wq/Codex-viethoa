//! Shortcut picker construction for `/keymap`.

use codex_config::types::TuiKeymap;
use ratatui::style::Styled;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use unicode_width::UnicodeWidthStr;

use crate::app_event::AppEvent;
use crate::bottom_pane::ColumnWidthMode;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionRowDisplay;
use crate::bottom_pane::SelectionTab;
use crate::bottom_pane::SelectionViewParams;
use crate::keymap::RuntimeKeymap;
use crate::render::renderable::ColumnRenderable;
use crate::render::renderable::Renderable;
use crate::style::accent_style;

use super::actions::KEYMAP_ACTIONS;
use super::actions::KeymapActionFilter;
use super::actions::action_label;
use super::actions::format_action_binding_summary;
use super::has_custom_binding;

pub(crate) const KEYMAP_PICKER_VIEW_ID: &str = "keymap-picker";
pub(super) const KEYMAP_ALL_TAB_ID: &str = "all-shortcuts";
pub(super) const KEYMAP_COMMON_TAB_ID: &str = "common-shortcuts";
pub(super) const KEYMAP_CUSTOM_TAB_ID: &str = "custom-shortcuts";
pub(super) const KEYMAP_UNBOUND_TAB_ID: &str = "unbound-shortcuts";
pub(super) const KEYMAP_DEBUG_TAB_ID: &str = "debug-shortcuts";
const KEYMAP_CONTEXT_LABEL_WIDTH: usize = 12;
const KEYMAP_ROW_PREFIX_WIDTH: usize = KEYMAP_CONTEXT_LABEL_WIDTH + 3;

#[derive(Clone, Debug)]
struct KeymapActionRow {
    context: &'static str,
    context_label: &'static str,
    action: &'static str,
    label: String,
    description: &'static str,
    binding_summary: String,
    custom_binding: bool,
}

impl KeymapActionRow {
    fn is_unbound(&self) -> bool {
        self.binding_summary == "unbound"
    }
}

struct KeymapContextTab {
    id: &'static str,
    label: &'static str,
    description: &'static str,
    contexts: &'static [&'static str],
}

const KEYMAP_COMMON_ACTIONS: &[(&str, &str)] = &[
    ("composer", "submit"),
    ("chat", "interrupt_turn"),
    ("editor", "insert_newline"),
    ("composer", "queue"),
    ("global", "toggle_fast_mode"),
    ("global", "open_external_editor"),
    ("global", "copy"),
    ("global", "toggle_vim_mode"),
    ("editor", "delete_backward_word"),
    ("editor", "delete_forward_word"),
    ("editor", "move_word_left"),
    ("editor", "move_word_right"),
    ("global", "open_transcript"),
    ("pager", "close"),
    ("pager", "page_up"),
    ("pager", "page_down"),
    ("approval", "open_fullscreen"),
    ("approval", "approve"),
    ("approval", "approve_for_session"),
    ("approval", "decline"),
    ("approval", "cancel"),
];

const KEYMAP_CONTEXT_TABS: &[KeymapContextTab] = &[
    KeymapContextTab {
        id: "app-shortcuts",
        label: "Ứng dụng",
        description: "Các phím tắt cấp toàn cục và cấp trò chuyện.",
        contexts: &["global", "chat"],
    },
    KeymapContextTab {
        id: "composer-shortcuts",
        label: "Soạn tin",
        description: "Phím tắt gửi và xếp hàng soạn tin.",
        contexts: &["composer"],
    },
    KeymapContextTab {
        id: "editor-shortcuts",
        label: "Trình soạn thảo",
        description: "Phím tắt di chuyển và chỉnh sửa trong trình soạn thảo nội tuyến.",
        contexts: &["editor"],
    },
    KeymapContextTab {
        id: "vim-shortcuts",
        label: "Vim",
        description: "Phím tắt chế độ thường và toán tử của Vim.",
        contexts: &["vim_normal", "vim_operator", "vim_text_object"],
    },
    KeymapContextTab {
        id: "navigation-shortcuts",
        label: "Điều hướng",
        description: "Phím tắt điều hướng pager và danh sách lựa chọn.",
        contexts: &["pager", "list"],
    },
    KeymapContextTab {
        id: "approval-shortcuts",
        label: "Duyệt",
        description: "Phím tắt cho lời nhắc duyệt.",
        contexts: &["approval"],
    },
];

#[cfg(test)]
pub(crate) fn build_keymap_picker_params(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
) -> SelectionViewParams {
    build_keymap_picker_params_with_filter(
        runtime_keymap,
        keymap_config,
        KeymapActionFilter::default(),
    )
}

pub(crate) fn build_keymap_picker_params_with_filter(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    action_filter: KeymapActionFilter,
) -> SelectionViewParams {
    build_keymap_picker_params_for_action(
        runtime_keymap,
        keymap_config,
        action_filter,
        /*selected_action*/ None,
    )
}

#[cfg(test)]
pub(crate) fn build_keymap_picker_params_for_selected_action(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    context: &str,
    action: &str,
) -> SelectionViewParams {
    build_keymap_picker_params_for_selected_action_with_filter(
        runtime_keymap,
        keymap_config,
        KeymapActionFilter::default(),
        context,
        action,
    )
}

pub(crate) fn build_keymap_picker_params_for_selected_action_with_filter(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    action_filter: KeymapActionFilter,
    context: &str,
    action: &str,
) -> SelectionViewParams {
    build_keymap_picker_params_for_action(
        runtime_keymap,
        keymap_config,
        action_filter,
        Some((context, action)),
    )
}

fn build_keymap_picker_params_for_action(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    action_filter: KeymapActionFilter,
    selected_action: Option<(&str, &str)>,
) -> SelectionViewParams {
    let rows = build_keymap_rows(runtime_keymap, keymap_config, action_filter);
    let total = rows.len();
    let custom_count = rows.iter().filter(|row| row.custom_binding).count();
    let unbound_count = rows.iter().filter(|row| row.is_unbound()).count();
    let initial_selected_idx = selected_action.and_then(|(context, action)| {
        rows.iter()
            .position(|row| row.context == context && row.action == action)
    });
    let name_column_width = rows
        .iter()
        .map(|row| KEYMAP_ROW_PREFIX_WIDTH + UnicodeWidthStr::width(row.label.as_str()))
        .max();

    let mut tabs = Vec::new();
    tabs.push(SelectionTab {
        id: KEYMAP_ALL_TAB_ID.to_string(),
        label: "Tất cả".to_string(),
        header: keymap_header(
            "Tất cả phím tắt có thể cấu hình.".to_string(),
            format!("{total} hành động, {custom_count} đã tùy chỉnh, {unbound_count} chưa gán."),
        ),
        items: keymap_selection_items(
            rows.iter(),
            "Không có phím tắt nào",
            "Không có phím tắt nào có thể cấu hình.",
        ),
    });

    let common_rows = keymap_common_rows(&rows);
    let common_count = common_rows.len();
    tabs.push(SelectionTab {
        id: KEYMAP_COMMON_TAB_ID.to_string(),
        label: "Phổ biến".to_string(),
        header: keymap_header(
            "Các phím tắt thường được tùy chỉnh.".to_string(),
            action_count_line(common_count),
        ),
        items: keymap_selection_items(
            common_rows,
            "Không có phím tắt phổ biến",
            "Không có hành động phím tắt phổ biến nào.",
        ),
    });

    let custom_rows = rows
        .iter()
        .filter(|row| row.custom_binding)
        .collect::<Vec<_>>();
    tabs.push(SelectionTab {
        id: KEYMAP_CUSTOM_TAB_ID.to_string(),
        label: format!("Đã tùy chỉnh ({custom_count})"),
        header: keymap_header(
            "Ghi đè phím tắt cấp gốc.".to_string(),
            action_count_line(custom_count),
        ),
        items: keymap_selection_items(
            custom_rows,
            "Không có phím tắt tùy chỉnh",
            "Chưa có ghi đè keymap cấp gốc nào được cấu hình.",
        ),
    });

    let unbound_rows = rows
        .iter()
        .filter(|row| row.is_unbound())
        .collect::<Vec<_>>();
    tabs.push(SelectionTab {
        id: KEYMAP_UNBOUND_TAB_ID.to_string(),
        label: format!("Chưa gán ({unbound_count})"),
        header: keymap_header(
            "Các hành động chưa có phím tắt đang hoạt động.".to_string(),
            action_count_line(unbound_count),
        ),
        items: keymap_selection_items(
            unbound_rows,
            "Không có phím tắt chưa gán",
            "Mọi hành động có thể cấu hình hiện đều có phím tắt.",
        ),
    });

    for tab in KEYMAP_CONTEXT_TABS {
        let tab_rows = rows
            .iter()
            .filter(|row| tab.contexts.contains(&row.context))
            .collect::<Vec<_>>();
        let count = tab_rows.len();
        tabs.push(SelectionTab {
            id: tab.id.to_string(),
            label: tab.label.to_string(),
            header: keymap_header(tab.description.to_string(), action_count_line(count)),
            items: keymap_selection_items(
                tab_rows,
                "Không có phím tắt trong nhóm này",
                "Không có hành động có thể cấu hình nào trong nhóm này.",
            ),
        });
    }
    tabs.push(keymap_debug_tab());

    SelectionViewParams {
        view_id: Some(KEYMAP_PICKER_VIEW_ID),
        header: Box::new(()),
        footer_hint: Some(keymap_picker_hint_line()),
        tab_footer_hints: vec![(KEYMAP_DEBUG_TAB_ID.to_string(), keymap_debug_hint_line())],
        tabs,
        initial_tab_id: Some(KEYMAP_ALL_TAB_ID.to_string()),
        is_searchable: true,
        search_placeholder: Some("Gõ để tìm kiếm phím tắt".to_string()),
        col_width_mode: ColumnWidthMode::AutoAllRows,
        row_display: SelectionRowDisplay::SingleLine,
        name_column_width,
        initial_selected_idx,
        ..Default::default()
    }
}

fn keymap_debug_tab() -> SelectionTab {
    SelectionTab {
        id: KEYMAP_DEBUG_TAB_ID.to_string(),
        label: "Gỡ lỗi".to_string(),
        header: keymap_header(
            "Kiểm tra các phím nhấn từ terminal của bạn.".to_string(),
            "Xem phím Codex phát hiện và mọi phím tắt được gán cho phím đó.".to_string(),
        ),
        items: vec![SelectionItem {
            name: "Kiểm tra phím nhấn".to_string(),
            description: Some(
                "Nhấn Enter để bắt đầu. Sau đó nhấn bất kỳ phím nào để kiểm tra; Ctrl+C để thoát."
                    .to_string(),
            ),
            selected_description: Some(
                "Mở trình kiểm tra trực tiếp hiển thị phím phát hiện được, phím cấu hình và các hành động khớp."
                    .to_string(),
            ),
            actions: vec![Box::new(|tx| {
                tx.send(AppEvent::OpenKeymapDebug);
            })],
            search_value: Some("debug inspect keypress key terminal detected actions".to_string()),
            ..Default::default()
        }],
    }
}

fn build_keymap_rows(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    action_filter: KeymapActionFilter,
) -> Vec<KeymapActionRow> {
    KEYMAP_ACTIONS
        .iter()
        .copied()
        .filter(|descriptor| descriptor.is_visible(action_filter))
        .map(|descriptor| KeymapActionRow {
            context: descriptor.context,
            context_label: descriptor.context_label,
            action: descriptor.action,
            label: action_label(descriptor.action),
            description: descriptor.description,
            binding_summary: format_action_binding_summary(
                runtime_keymap,
                descriptor.context,
                descriptor.action,
            ),
            custom_binding: has_custom_binding(
                keymap_config,
                descriptor.context,
                descriptor.action,
            )
            .unwrap_or(/*default*/ false),
        })
        .collect()
}

fn keymap_common_rows(rows: &[KeymapActionRow]) -> Vec<&KeymapActionRow> {
    KEYMAP_COMMON_ACTIONS
        .iter()
        .filter_map(|(context, action)| {
            rows.iter()
                .find(|row| row.context == *context && row.action == *action)
        })
        .collect()
}

fn keymap_selection_items<'a>(
    rows: impl IntoIterator<Item = &'a KeymapActionRow>,
    empty_name: &str,
    empty_description: &str,
) -> Vec<SelectionItem> {
    let items = rows
        .into_iter()
        .map(keymap_selection_item)
        .collect::<Vec<_>>();
    if items.is_empty() {
        return vec![SelectionItem {
            name: empty_name.to_string(),
            description: Some(empty_description.to_string()),
            is_disabled: true,
            ..Default::default()
        }];
    }

    items
}

fn keymap_selection_item(row: &KeymapActionRow) -> SelectionItem {
    let context = row.context.to_string();
    let action = row.action.to_string();
    let source = if row.custom_binding {
        "Tùy chỉnh"
    } else {
        "Mặc định"
    };
    let search_value = format!(
        "{} {} {} {} {} {}",
        row.context_label, row.action, row.label, row.description, row.binding_summary, source
    );

    SelectionItem {
        name: row.label.clone(),
        name_prefix_spans: keymap_row_prefix(row),
        description: Some(row.binding_summary.clone()),
        actions: vec![Box::new(move |tx| {
            tx.send(AppEvent::OpenKeymapActionMenu {
                context: context.clone(),
                action: action.clone(),
            });
        })],
        search_value: Some(search_value),
        ..Default::default()
    }
}

fn keymap_row_prefix(row: &KeymapActionRow) -> Vec<Span<'static>> {
    let indicator = if row.custom_binding {
        "*".set_style(accent_style())
    } else if row.is_unbound() {
        "-".dim()
    } else {
        " ".into()
    };

    vec![
        format!(
            "{:<width$} ",
            row.context_label,
            width = KEYMAP_CONTEXT_LABEL_WIDTH
        )
        .dim(),
        indicator,
        " ".dim(),
    ]
}

fn keymap_header(description: String, summary: String) -> Box<dyn Renderable> {
    let mut header = ColumnRenderable::new();
    header.push(Line::from("Keymap".bold()));
    header.push(Line::from(description.dim()));
    header.push(Line::from(summary.dim()));
    Box::new(header)
}

fn action_count_line(count: usize) -> String {
    match count {
        1 => "1 hành động.".to_string(),
        _ => format!("{count} hành động."),
    }
}

fn keymap_picker_hint_line() -> Line<'static> {
    let style = accent_style();
    Line::from(vec![
        "left/right".set_style(style),
        " group · ".dim(),
        "enter".set_style(style),
        " edit shortcut · ".dim(),
        "*".set_style(style),
        " custom · ".dim(),
        "-".set_style(style),
        " unbound · ".dim(),
        "esc".set_style(style),
        " để đóng".dim(),
    ])
}

fn keymap_debug_hint_line() -> Line<'static> {
    let style = accent_style();
    Line::from(vec![
        "enter".set_style(style),
        " start inspector · ".dim(),
        "esc".set_style(style),
        " để đóng".dim(),
    ])
}
