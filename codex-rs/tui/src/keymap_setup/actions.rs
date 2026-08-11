//! Catalog and accessors for keymap actions shown by `/keymap`.
//!
//! The descriptor table is the single UI-facing inventory of configurable
//! actions. Each descriptor ties together the config path segment, user-facing
//! context label, stable action name, and short description used by the picker
//! and action menu.
//!
//! Root-config accessors mirror the descriptor table, while runtime lookups
//! reuse the inventory owned by [`crate::keymap`]. A catalog action must remain
//! both writable in `TuiKeymap` and readable from the shared runtime inventory.

use std::collections::BTreeSet;

use codex_config::types::KeybindingsSpec;
use codex_config::types::TuiKeymap;
use crossterm::event::KeyEvent;

use crate::keymap::RuntimeKeymap;
use crate::keymap::bindings_for_action;

#[derive(Clone, Copy, Debug)]
pub(super) struct KeymapActionDescriptor {
    /// Config context segment, such as `composer` in `tui.keymap.composer.submit`.
    pub(super) context: &'static str,
    /// Human-readable group label shown in the picker.
    pub(super) context_label: &'static str,
    /// Config action segment, such as `submit` in `tui.keymap.composer.submit`.
    pub(super) action: &'static str,
    /// Short user-facing explanation of what the action does.
    pub(super) description: &'static str,
    /// Feature required before the action appears in `/keymap`.
    required_feature: Option<KeymapActionFeature>,
}

const fn action(
    context: &'static str,
    context_label: &'static str,
    action: &'static str,
    description: &'static str,
) -> KeymapActionDescriptor {
    KeymapActionDescriptor {
        context,
        context_label,
        action,
        description,
        required_feature: None,
    }
}

const fn gated_action(
    context: &'static str,
    context_label: &'static str,
    action: &'static str,
    description: &'static str,
    required_feature: KeymapActionFeature,
) -> KeymapActionDescriptor {
    KeymapActionDescriptor {
        context,
        context_label,
        action,
        description,
        required_feature: Some(required_feature),
    }
}

#[derive(Clone, Copy, Debug)]
enum KeymapActionFeature {
    FastMode,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct KeymapActionFilter {
    pub(crate) fast_mode_enabled: bool,
}

impl KeymapActionDescriptor {
    pub(super) fn is_visible(self, filter: KeymapActionFilter) -> bool {
        match self.required_feature {
            None => true,
            Some(KeymapActionFeature::FastMode) => filter.fast_mode_enabled,
        }
    }
}

#[rustfmt::skip]
pub(super) const KEYMAP_ACTIONS: &[KeymapActionDescriptor] = &[
    action("global", "Toàn cục", "open_transcript", "Mở lớp phủ transcript."),
    action("global", "Toàn cục", "open_external_editor", "Mở bản nháp hiện tại trong trình soạn thảo ngoài."),
    action("global", "Toàn cục", "copy", "Sao chép phản hồi agent cuối vào clipboard."),
    action("global", "Toàn cục", "clear_terminal", "Xóa giao diện terminal."),
    action("global", "Toàn cục", "toggle_vim_mode", "Bật/tắt chế độ Vim cho ô soạn thảo."),
    gated_action("global", "Toàn cục", "toggle_fast_mode", "Bật/tắt chế độ Nhanh.", KeymapActionFeature::FastMode),
    action("global", "Toàn cục", "toggle_raw_output", "Bật/tắt chế độ cuộn thô."),
    action("global", "Toàn cục", "toggle_side_conversation", "Chuyển giữa hội thoại bên và hội thoại cha của nó."),
    action("chat", "Trò chuyện", "interrupt_turn", "Ngắt lượt đang hoạt động."),
    action("chat", "Trò chuyện", "decrease_reasoning_effort", "Giảm mức độ suy luận."),
    action("chat", "Trò chuyện", "increase_reasoning_effort", "Tăng mức độ suy luận."),
    action("chat", "Trò chuyện", "edit_queued_message", "Sửa tin nhắn được xếp hàng gần nhất."),
    action("composer", "Soạn thảo", "submit", "Gửi bản nháp hiện tại."),
    action("composer", "Soạn thảo", "queue", "Xếp bản nháp vào hàng đợi trong khi tác vụ đang chạy."),
    action("composer", "Soạn thảo", "toggle_shortcuts", "Hiện/ẩn lớp phủ phím tắt của ô soạn thảo."),
    action("composer", "Soạn thảo", "history_search_previous", "Mở tìm kiếm lịch sử hoặc nhảy đến kết quả trước."),
    action("composer", "Soạn thảo", "history_search_next", "Nhảy đến kết quả tìm kiếm lịch sử tiếp theo."),
    action("editor", "Trình soạn thảo", "insert_newline", "Chèn dòng mới trong trình soạn thảo."),
    action("editor", "Trình soạn thảo", "move_left", "Di chuyển con trỏ sang trái."),
    action("editor", "Trình soạn thảo", "move_right", "Di chuyển con trỏ sang phải."),
    action("editor", "Trình soạn thảo", "move_up", "Di chuyển con trỏ lên."),
    action("editor", "Trình soạn thảo", "move_down", "Di chuyển con trỏ xuống."),
    action("editor", "Trình soạn thảo", "move_word_left", "Nhảy về đầu từ trước."),
    action("editor", "Trình soạn thảo", "move_word_right", "Nhảy đến cuối từ tiếp theo."),
    action("editor", "Trình soạn thảo", "move_line_start", "Nhảy về đầu dòng."),
    action("editor", "Trình soạn thảo", "move_line_end", "Nhảy đến cuối dòng."),
    action("editor", "Trình soạn thảo", "delete_backward", "Xóa một ký tự bên trái."),
    action("editor", "Trình soạn thảo", "delete_forward", "Xóa một ký tự bên phải."),
    action("editor", "Trình soạn thảo", "delete_backward_word", "Xóa từ trước."),
    action("editor", "Trình soạn thảo", "delete_forward_word", "Xóa từ tiếp theo."),
    action("editor", "Trình soạn thảo", "kill_line_start", "Xóa từ con trỏ đến đầu dòng."),
    action("editor", "Trình soạn thảo", "kill_whole_line", "Xóa dòng hiện tại."),
    action("editor", "Trình soạn thảo", "kill_line_end", "Xóa từ con trỏ đến cuối dòng."),
    action("editor", "Trình soạn thảo", "yank", "Dán nội dung bộ nhớ tạm (kill buffer)."),
    action("vim_normal", "Vim thường", "enter_insert", "Vào chế độ chèn tại con trỏ."),
    action("vim_normal", "Vim thường", "append_after_cursor", "Vào chế độ chèn sau con trỏ."),
    action("vim_normal", "Vim thường", "append_line_end", "Vào chế độ chèn ở cuối dòng."),
    action("vim_normal", "Vim thường", "insert_line_start", "Vào chế độ chèn tại ký tự khác khoảng trắng đầu tiên."),
    action("vim_normal", "Vim thường", "open_line_below", "Mở dòng mới bên dưới và vào chế độ chèn."),
    action("vim_normal", "Vim thường", "open_line_above", "Mở dòng mới bên trên và vào chế độ chèn."),
    action("vim_normal", "Vim thường", "move_left", "Di chuyển trái trong chế độ Vim thường."),
    action("vim_normal", "Vim thường", "move_right", "Di chuyển phải trong chế độ Vim thường."),
    action("vim_normal", "Vim thường", "move_up", "Di chuyển lên hoặc xem lịch sử cũ hơn trong chế độ Vim thường."),
    action("vim_normal", "Vim thường", "move_down", "Di chuyển xuống hoặc xem lịch sử mới hơn trong chế độ Vim thường."),
    action("vim_normal", "Vim thường", "move_word_forward", "Nhảy đến đầu từ tiếp theo."),
    action("vim_normal", "Vim thường", "move_word_backward", "Nhảy đến đầu từ trước."),
    action("vim_normal", "Vim thường", "move_word_end", "Nhảy đến cuối từ hiện tại hoặc từ tiếp theo."),
    action("vim_normal", "Vim thường", "move_line_start", "Nhảy về đầu dòng."),
    action("vim_normal", "Vim thường", "move_line_end", "Nhảy đến cuối dòng."),
    action("vim_normal", "Vim thường", "delete_char", "Xóa ký tự dưới con trỏ."),
    action("vim_normal", "Vim thường", "substitute_char", "Xóa ký tự dưới con trỏ và vào chế độ chèn."),
    action("vim_normal", "Vim thường", "delete_to_line_end", "Xóa từ con trỏ đến cuối dòng."),
    action("vim_normal", "Vim thường", "change_to_line_end", "Thay đổi từ con trỏ đến cuối dòng và vào chế độ chèn."),
    action("vim_normal", "Vim thường", "yank_line", "Sao chép (yank) toàn bộ dòng."),
    action("vim_normal", "Vim thường", "paste_after", "Dán sau con trỏ."),
    action("vim_normal", "Vim thường", "start_delete_operator", "Bắt đầu toán tử xóa và chờ chuyển động."),
    action("vim_normal", "Vim thường", "start_yank_operator", "Bắt đầu toán tử sao chép và chờ chuyển động."),
    action("vim_normal", "Vim thường", "start_change_operator", "Bắt đầu toán tử thay đổi và chờ đối tượng văn bản."),
    action("vim_normal", "Vim thường", "cancel_operator", "Hủy toán tử Vim đang chờ."),
    action("vim_operator", "Toán tử Vim", "delete_line", "Lặp toán tử xóa để xóa cả dòng."),
    action("vim_operator", "Toán tử Vim", "yank_line", "Lặp toán tử sao chép để sao chép cả dòng."),
    action("vim_operator", "Toán tử Vim", "motion_left", "Chuyển động toán tử sang trái."),
    action("vim_operator", "Toán tử Vim", "motion_right", "Chuyển động toán tử sang phải."),
    action("vim_operator", "Toán tử Vim", "motion_up", "Chuyển động toán tử lên."),
    action("vim_operator", "Toán tử Vim", "motion_down", "Chuyển động toán tử xuống."),
    action("vim_operator", "Toán tử Vim", "motion_word_forward", "Chuyển động toán tử đến đầu từ tiếp theo."),
    action("vim_operator", "Toán tử Vim", "motion_word_backward", "Chuyển động toán tử đến đầu từ trước."),
    action("vim_operator", "Toán tử Vim", "motion_word_end", "Chuyển động toán tử đến cuối từ."),
    action("vim_operator", "Toán tử Vim", "motion_line_start", "Chuyển động toán tử đến đầu dòng."),
    action("vim_operator", "Toán tử Vim", "motion_line_end", "Chuyển động toán tử đến cuối dòng."),
    action("vim_operator", "Toán tử Vim", "select_inner_text_object", "Chọn đối tượng văn bản bên trong."),
    action("vim_operator", "Toán tử Vim", "select_around_text_object", "Chọn đối tượng văn bản bao quanh."),
    action("vim_operator", "Toán tử Vim", "cancel", "Hủy toán tử đang chờ."),
    action("vim_text_object", "Đối tượng văn bản Vim", "word", "Nhắm vào từ hiện tại."),
    action("vim_text_object", "Đối tượng văn bản Vim", "big_word", "Nhắm vào WORD hiện tại."),
    action("vim_text_object", "Đối tượng văn bản Vim", "parentheses", "Nhắm vào cặp ngoặc tròn bao quanh."),
    action("vim_text_object", "Đối tượng văn bản Vim", "brackets", "Nhắm vào cặp ngoặc vuông bao quanh."),
    action("vim_text_object", "Đối tượng văn bản Vim", "braces", "Nhắm vào cặp ngoặc nhọn bao quanh."),
    action("vim_text_object", "Đối tượng văn bản Vim", "double_quote", "Nhắm vào cặp nháy kép bao quanh."),
    action("vim_text_object", "Đối tượng văn bản Vim", "single_quote", "Nhắm vào cặp nháy đơn bao quanh."),
    action("vim_text_object", "Đối tượng văn bản Vim", "backtick", "Nhắm vào cặp backtick bao quanh."),
    action("vim_text_object", "Đối tượng văn bản Vim", "cancel", "Hủy đối tượng văn bản đang chờ."),
    action("pager", "Phân trang", "scroll_up", "Cuộn lên một dòng."),
    action("pager", "Phân trang", "scroll_down", "Cuộn xuống một dòng."),
    action("pager", "Phân trang", "page_up", "Cuộn lên một trang."),
    action("pager", "Phân trang", "page_down", "Cuộn xuống một trang."),
    action("pager", "Phân trang", "half_page_up", "Cuộn lên nửa trang."),
    action("pager", "Phân trang", "half_page_down", "Cuộn xuống nửa trang."),
    action("pager", "Phân trang", "jump_top", "Nhảy về đầu."),
    action("pager", "Phân trang", "jump_bottom", "Nhảy về cuối."),
    action("pager", "Phân trang", "close", "Đóng lớp phủ phân trang."),
    action("pager", "Phân trang", "close_transcript", "Đóng lớp phủ transcript."),
    action("list", "Danh sách", "move_up", "Di chuyển lựa chọn danh sách lên."),
    action("list", "Danh sách", "move_down", "Di chuyển lựa chọn danh sách xuống."),
    action("list", "Danh sách", "move_left", "Di chuyển ngang trái trong bộ chọn danh sách."),
    action("list", "Danh sách", "move_right", "Di chuyển ngang phải trong bộ chọn danh sách."),
    action("list", "Danh sách", "page_up", "Di chuyển lựa chọn danh sách lên một trang."),
    action("list", "Danh sách", "page_down", "Di chuyển lựa chọn danh sách xuống một trang."),
    action("list", "Danh sách", "jump_top", "Nhảy đến mục đầu tiên."),
    action("list", "Danh sách", "jump_bottom", "Nhảy đến mục cuối cùng."),
    action("list", "Danh sách", "accept", "Chấp nhận lựa chọn hiện tại."),
    action("list", "Danh sách", "cancel", "Hủy và đóng các màn hình lựa chọn."),
    action("approval", "Duyệt quyền", "open_fullscreen", "Mở chi tiết duyệt quyền toàn màn hình."),
    action("approval", "Duyệt quyền", "open_thread", "Mở luồng nguồn duyệt quyền nếu có."),
    action("approval", "Duyệt quyền", "approve", "Duyệt phương án chính."),
    action("approval", "Duyệt quyền", "approve_for_session", "Duyệt cho phiên nếu có."),
    action("approval", "Duyệt quyền", "approve_for_prefix", "Duyệt với tiền tố exec-policy nếu có."),
    action("approval", "Duyệt quyền", "deny", "Chọn phương án từ chối rõ ràng nếu có."),
    action("approval", "Duyệt quyền", "decline", "Từ chối và đưa ra hướng dẫn khắc phục."),
    action("approval", "Duyệt quyền", "cancel", "Hủy yêu cầu hỏi thông tin."),
];

/// Convert a stable action identifier into a display label.
///
/// This is intentionally presentation-only: the returned string must never be
/// parsed back into an action name, because underscores and casing are part of
/// the stable config contract.
pub(super) fn action_label(action: &str) -> String {
    action
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[rustfmt::skip]
/// Return the mutable root-config binding slot for one catalog action.
///
/// The returned `Option<KeybindingsSpec>` distinguishes three states that the
/// editor must preserve: absent means use fallback/default resolution, `Some`
/// with one or more keys is a custom binding, and `Some(Many([]))` is an
/// explicit unbind.
pub(super) fn binding_slot<'a>(
    keymap: &'a mut TuiKeymap,
    context: &str,
    action: &str,
) -> Option<&'a mut Option<KeybindingsSpec>> {
    match (context, action) {
        ("global", "open_transcript") => Some(&mut keymap.global.open_transcript),
        ("global", "open_external_editor") => Some(&mut keymap.global.open_external_editor),
        ("global", "copy") => Some(&mut keymap.global.copy),
        ("global", "clear_terminal") => Some(&mut keymap.global.clear_terminal),
        ("global", "toggle_vim_mode") => Some(&mut keymap.global.toggle_vim_mode),
        ("global", "toggle_fast_mode") => Some(&mut keymap.global.toggle_fast_mode),
        ("global", "toggle_raw_output") => Some(&mut keymap.global.toggle_raw_output),
        ("global", "toggle_side_conversation") => Some(&mut keymap.global.toggle_side_conversation),
        ("chat", "interrupt_turn") => Some(&mut keymap.chat.interrupt_turn),
        ("chat", "decrease_reasoning_effort") => Some(&mut keymap.chat.decrease_reasoning_effort),
        ("chat", "increase_reasoning_effort") => Some(&mut keymap.chat.increase_reasoning_effort),
        ("chat", "edit_queued_message") => Some(&mut keymap.chat.edit_queued_message),
        ("composer", "submit") => Some(&mut keymap.composer.submit),
        ("composer", "queue") => Some(&mut keymap.composer.queue),
        ("composer", "toggle_shortcuts") => Some(&mut keymap.composer.toggle_shortcuts),
        ("composer", "history_search_previous") => Some(&mut keymap.composer.history_search_previous),
        ("composer", "history_search_next") => Some(&mut keymap.composer.history_search_next),
        ("editor", "insert_newline") => Some(&mut keymap.editor.insert_newline),
        ("editor", "move_left") => Some(&mut keymap.editor.move_left),
        ("editor", "move_right") => Some(&mut keymap.editor.move_right),
        ("editor", "move_up") => Some(&mut keymap.editor.move_up),
        ("editor", "move_down") => Some(&mut keymap.editor.move_down),
        ("editor", "move_word_left") => Some(&mut keymap.editor.move_word_left),
        ("editor", "move_word_right") => Some(&mut keymap.editor.move_word_right),
        ("editor", "move_line_start") => Some(&mut keymap.editor.move_line_start),
        ("editor", "move_line_end") => Some(&mut keymap.editor.move_line_end),
        ("editor", "delete_backward") => Some(&mut keymap.editor.delete_backward),
        ("editor", "delete_forward") => Some(&mut keymap.editor.delete_forward),
        ("editor", "delete_backward_word") => Some(&mut keymap.editor.delete_backward_word),
        ("editor", "delete_forward_word") => Some(&mut keymap.editor.delete_forward_word),
        ("editor", "kill_line_start") => Some(&mut keymap.editor.kill_line_start),
        ("editor", "kill_whole_line") => Some(&mut keymap.editor.kill_whole_line),
        ("editor", "kill_line_end") => Some(&mut keymap.editor.kill_line_end),
        ("editor", "yank") => Some(&mut keymap.editor.yank),
        ("vim_normal", "enter_insert") => Some(&mut keymap.vim_normal.enter_insert),
        ("vim_normal", "append_after_cursor") => Some(&mut keymap.vim_normal.append_after_cursor),
        ("vim_normal", "append_line_end") => Some(&mut keymap.vim_normal.append_line_end),
        ("vim_normal", "insert_line_start") => Some(&mut keymap.vim_normal.insert_line_start),
        ("vim_normal", "open_line_below") => Some(&mut keymap.vim_normal.open_line_below),
        ("vim_normal", "open_line_above") => Some(&mut keymap.vim_normal.open_line_above),
        ("vim_normal", "move_left") => Some(&mut keymap.vim_normal.move_left),
        ("vim_normal", "move_right") => Some(&mut keymap.vim_normal.move_right),
        ("vim_normal", "move_up") => Some(&mut keymap.vim_normal.move_up),
        ("vim_normal", "move_down") => Some(&mut keymap.vim_normal.move_down),
        ("vim_normal", "move_word_forward") => Some(&mut keymap.vim_normal.move_word_forward),
        ("vim_normal", "move_word_backward") => Some(&mut keymap.vim_normal.move_word_backward),
        ("vim_normal", "move_word_end") => Some(&mut keymap.vim_normal.move_word_end),
        ("vim_normal", "move_line_start") => Some(&mut keymap.vim_normal.move_line_start),
        ("vim_normal", "move_line_end") => Some(&mut keymap.vim_normal.move_line_end),
        ("vim_normal", "delete_char") => Some(&mut keymap.vim_normal.delete_char),
        ("vim_normal", "substitute_char") => Some(&mut keymap.vim_normal.substitute_char),
        ("vim_normal", "delete_to_line_end") => Some(&mut keymap.vim_normal.delete_to_line_end),
        ("vim_normal", "change_to_line_end") => Some(&mut keymap.vim_normal.change_to_line_end),
        ("vim_normal", "yank_line") => Some(&mut keymap.vim_normal.yank_line),
        ("vim_normal", "paste_after") => Some(&mut keymap.vim_normal.paste_after),
        ("vim_normal", "start_delete_operator") => Some(&mut keymap.vim_normal.start_delete_operator),
        ("vim_normal", "start_yank_operator") => Some(&mut keymap.vim_normal.start_yank_operator),
        ("vim_normal", "start_change_operator") => Some(&mut keymap.vim_normal.start_change_operator),
        ("vim_normal", "cancel_operator") => Some(&mut keymap.vim_normal.cancel_operator),
        ("vim_operator", "delete_line") => Some(&mut keymap.vim_operator.delete_line),
        ("vim_operator", "yank_line") => Some(&mut keymap.vim_operator.yank_line),
        ("vim_operator", "motion_left") => Some(&mut keymap.vim_operator.motion_left),
        ("vim_operator", "motion_right") => Some(&mut keymap.vim_operator.motion_right),
        ("vim_operator", "motion_up") => Some(&mut keymap.vim_operator.motion_up),
        ("vim_operator", "motion_down") => Some(&mut keymap.vim_operator.motion_down),
        ("vim_operator", "motion_word_forward") => Some(&mut keymap.vim_operator.motion_word_forward),
        ("vim_operator", "motion_word_backward") => Some(&mut keymap.vim_operator.motion_word_backward),
        ("vim_operator", "motion_word_end") => Some(&mut keymap.vim_operator.motion_word_end),
        ("vim_operator", "motion_line_start") => Some(&mut keymap.vim_operator.motion_line_start),
        ("vim_operator", "motion_line_end") => Some(&mut keymap.vim_operator.motion_line_end),
        ("vim_operator", "select_inner_text_object") => Some(&mut keymap.vim_operator.select_inner_text_object),
        ("vim_operator", "select_around_text_object") => Some(&mut keymap.vim_operator.select_around_text_object),
        ("vim_operator", "cancel") => Some(&mut keymap.vim_operator.cancel),
        ("vim_text_object", "word") => Some(&mut keymap.vim_text_object.word),
        ("vim_text_object", "big_word") => Some(&mut keymap.vim_text_object.big_word),
        ("vim_text_object", "parentheses") => Some(&mut keymap.vim_text_object.parentheses),
        ("vim_text_object", "brackets") => Some(&mut keymap.vim_text_object.brackets),
        ("vim_text_object", "braces") => Some(&mut keymap.vim_text_object.braces),
        ("vim_text_object", "double_quote") => Some(&mut keymap.vim_text_object.double_quote),
        ("vim_text_object", "single_quote") => Some(&mut keymap.vim_text_object.single_quote),
        ("vim_text_object", "backtick") => Some(&mut keymap.vim_text_object.backtick),
        ("vim_text_object", "cancel") => Some(&mut keymap.vim_text_object.cancel),
        ("pager", "scroll_up") => Some(&mut keymap.pager.scroll_up),
        ("pager", "scroll_down") => Some(&mut keymap.pager.scroll_down),
        ("pager", "page_up") => Some(&mut keymap.pager.page_up),
        ("pager", "page_down") => Some(&mut keymap.pager.page_down),
        ("pager", "half_page_up") => Some(&mut keymap.pager.half_page_up),
        ("pager", "half_page_down") => Some(&mut keymap.pager.half_page_down),
        ("pager", "jump_top") => Some(&mut keymap.pager.jump_top),
        ("pager", "jump_bottom") => Some(&mut keymap.pager.jump_bottom),
        ("pager", "close") => Some(&mut keymap.pager.close),
        ("pager", "close_transcript") => Some(&mut keymap.pager.close_transcript),
        ("list", "move_up") => Some(&mut keymap.list.move_up),
        ("list", "move_down") => Some(&mut keymap.list.move_down),
        ("list", "move_left") => Some(&mut keymap.list.move_left),
        ("list", "move_right") => Some(&mut keymap.list.move_right),
        ("list", "page_up") => Some(&mut keymap.list.page_up),
        ("list", "page_down") => Some(&mut keymap.list.page_down),
        ("list", "jump_top") => Some(&mut keymap.list.jump_top),
        ("list", "jump_bottom") => Some(&mut keymap.list.jump_bottom),
        ("list", "accept") => Some(&mut keymap.list.accept),
        ("list", "cancel") => Some(&mut keymap.list.cancel),
        ("approval", "open_fullscreen") => Some(&mut keymap.approval.open_fullscreen),
        ("approval", "open_thread") => Some(&mut keymap.approval.open_thread),
        ("approval", "approve") => Some(&mut keymap.approval.approve),
        ("approval", "approve_for_session") => Some(&mut keymap.approval.approve_for_session),
        ("approval", "approve_for_prefix") => Some(&mut keymap.approval.approve_for_prefix),
        ("approval", "deny") => Some(&mut keymap.approval.deny),
        ("approval", "decline") => Some(&mut keymap.approval.decline),
        ("approval", "cancel") => Some(&mut keymap.approval.cancel),
        _ => None,
    }
}

/// Format an action's active single-key and chord alternatives in config order.
///
/// Duplicate runtime variants that normalize to the same config spec are shown
/// once so compatibility defaults do not appear as separate user choices.
pub(super) fn format_action_binding_summary(
    runtime_keymap: &RuntimeKeymap,
    context: &str,
    action: &str,
) -> String {
    let specs = super::active_binding_specs(runtime_keymap, context, action).unwrap_or_else(|_| {
        bindings_for_action(runtime_keymap, context, action)
            .unwrap_or_default()
            .iter()
            .filter_map(|binding| super::binding_to_config_key_spec(*binding).ok())
            .collect()
    });
    let mut seen = BTreeSet::new();
    let specs = specs
        .into_iter()
        .filter(|spec| seen.insert(spec.clone()))
        .collect::<Vec<_>>();
    if specs.is_empty() {
        "unbound".to_string()
    } else {
        specs.join(", ")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum KeymapDebugBindingSource {
    Custom,
    CustomGlobal,
    Default,
}

impl KeymapDebugBindingSource {
    pub(super) const fn label(&self) -> &'static str {
        match self {
            Self::Custom => "Tùy chỉnh",
            Self::CustomGlobal => "Tùy chỉnh toàn cục",
            Self::Default => "Mặc định",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct KeymapDebugActionMatch {
    pub(super) context: &'static str,
    pub(super) action: &'static str,
    pub(super) label: String,
    pub(super) description: &'static str,
    pub(super) source: KeymapDebugBindingSource,
}

pub(super) fn matching_actions_for_key_event(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    event: KeyEvent,
) -> Vec<KeymapDebugActionMatch> {
    KEYMAP_ACTIONS
        .iter()
        .filter_map(|descriptor| {
            let bindings =
                bindings_for_action(runtime_keymap, descriptor.context, descriptor.action)?;
            bindings
                .iter()
                .any(|binding| binding.is_press(event))
                .then(|| KeymapDebugActionMatch {
                    context: descriptor.context,
                    action: descriptor.action,
                    label: action_label(descriptor.action),
                    description: descriptor.description,
                    source: debug_binding_source(keymap_config, descriptor),
                })
        })
        .collect()
}

fn debug_binding_source(
    keymap_config: &TuiKeymap,
    descriptor: &KeymapActionDescriptor,
) -> KeymapDebugBindingSource {
    let mut keymap_config = keymap_config.clone();
    let Some(slot) = binding_slot(&mut keymap_config, descriptor.context, descriptor.action) else {
        return KeymapDebugBindingSource::Default;
    };
    if slot.is_some() {
        return KeymapDebugBindingSource::Custom;
    }

    let Some(global_slot) = global_fallback_slot(&mut keymap_config, descriptor) else {
        return KeymapDebugBindingSource::Default;
    };
    if global_slot.is_some() {
        KeymapDebugBindingSource::CustomGlobal
    } else {
        KeymapDebugBindingSource::Default
    }
}

fn global_fallback_slot<'a>(
    keymap: &'a mut TuiKeymap,
    descriptor: &KeymapActionDescriptor,
) -> Option<&'a mut Option<KeybindingsSpec>> {
    if descriptor.context != "composer" {
        return None;
    }

    match descriptor.action {
        "submit" => Some(&mut keymap.global.submit),
        "queue" => Some(&mut keymap.global.queue),
        "toggle_shortcuts" => Some(&mut keymap.global.toggle_shortcuts),
        _ => None,
    }
}
