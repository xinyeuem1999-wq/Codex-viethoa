use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Ide,
    Permissions,
    Keymap,
    Vim,
    #[strum(serialize = "setup-default-sandbox")]
    ElevateSandbox,
    #[strum(serialize = "sandbox-add-read-dir")]
    SandboxReadRoot,
    Experimental,
    #[strum(to_string = "approve")]
    AutoReview,
    Memories,
    Skills,
    Import,
    Hooks,
    Review,
    Rename,
    New,
    Archive,
    Delete,
    Resume,
    Fork,
    App,
    Init,
    Compact,
    Plan,
    Goal,
    Agent,
    Side,
    Btw,
    Copy,
    Raw,
    Diff,
    Mention,
    Status,
    Usage,
    DebugConfig,
    Title,
    Statusline,
    Theme,
    #[strum(to_string = "pets", serialize = "pet")]
    Pets,
    Mcp,
    Apps,
    Plugins,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    #[strum(to_string = "stop", serialize = "clean")]
    Stop,
    Clear,
    Personality,
    TestApproval,
    #[strum(serialize = "subagents")]
    MultiAgents,
    // Debugging commands.
    #[strum(serialize = "debug-m-drop")]
    MemoryDrop,
    #[strum(serialize = "debug-m-update")]
    MemoryUpdate,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Feedback => "gửi log cho đội phát triển",
            SlashCommand::New => "bắt đầu cuộc trò chuyện mới trong hội thoại",
            SlashCommand::Init => "tạo file AGENTS.md với hướng dẫn cho Codex",
            SlashCommand::Compact => "tóm tắt hội thoại để tránh chạm giới hạn ngữ cảnh",
            SlashCommand::Review => "xem xét các thay đổi hiện tại và tìm vấn đề",
            SlashCommand::Rename => "đổi tên luồng hiện tại",
            SlashCommand::Resume => "tiếp tục cuộc trò chuyện đã lưu",
            SlashCommand::Archive => "lưu trữ phiên này và thoát",
            SlashCommand::Delete => "xóa vĩnh viễn phiên này và thoát",
            SlashCommand::Clear => "xóa terminal và bắt đầu cuộc trò chuyện mới",
            SlashCommand::Fork => "fork cuộc trò chuyện hiện tại",
            SlashCommand::App => "tiếp tục phiên này trong ứng dụng Desktop",
            SlashCommand::Quit | SlashCommand::Exit => "thoát Codex",
            SlashCommand::Copy => "sao chép phản hồi cuối dạng markdown",
            SlashCommand::Raw => {
                "bật/tắt chế độ cuộn thô để chọn bản sao thân thiện với terminal"
            }
            SlashCommand::Diff => "hiển thị git diff (bao gồm file chưa theo dõi)",
            SlashCommand::Mention => "nhắc đến một file",
            SlashCommand::Skills => "dùng skill để cải thiện cách Codex thực hiện tác vụ cụ thể",
            SlashCommand::Import => {
                "nhập cấu hình, dự án này và các cuộc trò chuyện gần đây từ Claude Code"
            }
            SlashCommand::Hooks => "xem và quản lý lifecycle hooks",
            SlashCommand::Status => {
                "hiển thị cấu hình phiên hiện tại và lượng token đã dùng"
            }
            SlashCommand::Usage => "xem mức sử dụng tài khoản hoặc đặt lại giới hạn sử dụng",
            SlashCommand::DebugConfig => {
                "hiển thị các lớp cấu hình và nguồn yêu cầu để gỡ lỗi"
            }
            SlashCommand::Title => "cấu hình các mục xuất hiện trong tiêu đề terminal",
            SlashCommand::Statusline => "cấu hình các mục xuất hiện trong dòng trạng thái",
            SlashCommand::Theme => "chọn theme tô màu cú pháp",
            SlashCommand::Pets => "chọn hoặc ẩn thú cưng terminal",
            SlashCommand::Ps => "liệt kê terminal nền",
            SlashCommand::Stop => "dừng mọi terminal nền",
            SlashCommand::MemoryDrop => "DO NOT USE",
            SlashCommand::MemoryUpdate => "DO NOT USE",
            SlashCommand::Model => "chọn model và mức độ suy luận cần dùng",
            SlashCommand::Ide => {
                "đưa lựa chọn hiện tại, file đang mở và ngữ cảnh khác từ IDE của bạn"
            }
            SlashCommand::Personality => "chọn phong cách giao tiếp cho Codex",
            SlashCommand::Plan => "chuyển sang chế độ Kế hoạch",
            SlashCommand::Goal => "đặt hoặc xem mục tiêu cho tác vụ chạy lâu",
            SlashCommand::Agent | SlashCommand::MultiAgents => "chuyển luồng agent đang hoạt động",
            SlashCommand::Side | SlashCommand::Btw => {
                "bắt đầu hội thoại bên trong một fork tạm thời"
            }
            SlashCommand::Permissions => "chọn những gì Codex được phép làm",
            SlashCommand::Keymap => "đổi phím tắt TUI",
            SlashCommand::Vim => "bật/tắt chế độ Vim cho ô soạn thảo",
            SlashCommand::ElevateSandbox => "thiết lập sandbox agent nâng quyền",
            SlashCommand::SandboxReadRoot => {
                "cho phép sandbox đọc một thư mục: /sandbox-add-read-dir <absolute_path>"
            }
            SlashCommand::Experimental => "bật/tắt tính năng thử nghiệm",
            SlashCommand::AutoReview => {
                "duyệt một lần thử lại cho lần từ chối tự động gần đây"
            }
            SlashCommand::Memories => "cấu hình việc dùng và tạo memory",
            SlashCommand::Mcp => "liệt kê công cụ MCP đã cấu hình; dùng /mcp verbose để xem chi tiết",
            SlashCommand::Apps => "quản lý ứng dụng",
            SlashCommand::Plugins => "duyệt plugin",
            SlashCommand::Logout => "đăng xuất Codex",
            SlashCommand::Rollout => "in đường dẫn file rollout",
            SlashCommand::TestApproval => "test yêu cầu duyệt",
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Rename
                | SlashCommand::New
                | SlashCommand::Clear
                | SlashCommand::Fork
                | SlashCommand::Plan
                | SlashCommand::Goal
                | SlashCommand::Ide
                | SlashCommand::Keymap
                | SlashCommand::Mcp
                | SlashCommand::Raw
                | SlashCommand::Usage
                | SlashCommand::Pets
                | SlashCommand::Side
                | SlashCommand::Btw
                | SlashCommand::Resume
                | SlashCommand::SandboxReadRoot
        )
    }

    /// Whether this command remains available inside an active side conversation.
    pub fn available_in_side_conversation(self) -> bool {
        matches!(
            self,
            SlashCommand::Copy
                | SlashCommand::Raw
                | SlashCommand::Diff
                | SlashCommand::Mention
                | SlashCommand::Status
                | SlashCommand::Usage
                | SlashCommand::Ide
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Archive
            | SlashCommand::Delete
            | SlashCommand::Fork
            | SlashCommand::Init
            | SlashCommand::Compact
            | SlashCommand::Keymap
            | SlashCommand::Vim
            | SlashCommand::ElevateSandbox
            | SlashCommand::SandboxReadRoot
            | SlashCommand::Experimental
            | SlashCommand::Memories
            | SlashCommand::Import
            | SlashCommand::Review
            | SlashCommand::Plan
            | SlashCommand::Clear
            | SlashCommand::Logout
            | SlashCommand::MemoryDrop
            | SlashCommand::MemoryUpdate => false,
            SlashCommand::Diff
            | SlashCommand::Resume
            | SlashCommand::Model
            | SlashCommand::Personality
            | SlashCommand::Permissions
            | SlashCommand::Copy
            | SlashCommand::Raw
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Hooks
            | SlashCommand::Status
            | SlashCommand::Usage
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Stop
            | SlashCommand::App
            | SlashCommand::Goal
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Plugins
            | SlashCommand::Title
            | SlashCommand::Statusline
            | SlashCommand::AutoReview
            | SlashCommand::Feedback
            | SlashCommand::Ide
            | SlashCommand::Quit
            | SlashCommand::Exit
            | SlashCommand::Side
            | SlashCommand::Btw => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Agent | SlashCommand::MultiAgents => true,
            SlashCommand::Theme | SlashCommand::Pets => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::SandboxReadRoot => cfg!(target_os = "windows"),
            SlashCommand::Copy => !cfg!(target_os = "android"),
            SlashCommand::App => cfg!(any(target_os = "macos", target_os = "windows")),
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    use super::SlashCommand;

    #[test]
    fn stop_command_is_canonical_name() {
        assert_eq!(SlashCommand::Stop.command(), "stop");
    }

    #[test]
    fn clean_alias_parses_to_stop_command() {
        assert_eq!(SlashCommand::from_str("clean"), Ok(SlashCommand::Stop));
    }

    #[test]
    fn pet_alias_parses_to_pets_command() {
        assert_eq!(SlashCommand::Pets.command(), "pets");
        assert_eq!(SlashCommand::from_str("pet"), Ok(SlashCommand::Pets));
    }

    #[test]
    fn certain_commands_are_available_during_task() {
        assert!(SlashCommand::Goal.available_during_task());
        assert!(SlashCommand::Ide.available_during_task());
        assert!(SlashCommand::Title.available_during_task());
        assert!(SlashCommand::Statusline.available_during_task());
        assert!(SlashCommand::Raw.available_during_task());
        assert!(SlashCommand::Raw.available_in_side_conversation());
        assert!(SlashCommand::Raw.supports_inline_args());
        assert!(SlashCommand::App.available_during_task());
    }

    #[test]
    fn auto_review_command_is_approve() {
        assert_eq!(SlashCommand::AutoReview.command(), "approve");
        assert_eq!(
            SlashCommand::from_str("approve"),
            Ok(SlashCommand::AutoReview)
        );
    }
}
