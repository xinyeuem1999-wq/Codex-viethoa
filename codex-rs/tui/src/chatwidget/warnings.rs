use std::collections::HashSet;

const FALLBACK_MODEL_METADATA_WARNING_PREFIX: &str = "Model metadata for `";
const FALLBACK_MODEL_METADATA_WARNING_SUFFIX: &str =
    "` không tìm thấy. Đang dùng siêu dữ liệu dự phòng; điều này có thể làm giảm hiệu suất và gây sự cố.";

#[derive(Default)]
pub(super) struct WarningDisplayState {
    fallback_model_metadata_slugs: HashSet<String>,
}

impl WarningDisplayState {
    pub(super) fn should_display(&mut self, message: &str) -> bool {
        fallback_model_metadata_warning_slug(message)
            .is_none_or(|slug| self.fallback_model_metadata_slugs.insert(slug.to_string()))
    }
}

fn fallback_model_metadata_warning_slug(message: &str) -> Option<&str> {
    message
        .strip_prefix(FALLBACK_MODEL_METADATA_WARNING_PREFIX)?
        .strip_suffix(FALLBACK_MODEL_METADATA_WARNING_SUFFIX)
}
