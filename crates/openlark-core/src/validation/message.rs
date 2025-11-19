//! 消息验证模块

use super::core::ValidationResult;

/// 消息限制常量
pub mod message_limits {
    /// 文本消息最大长度
    pub const MAX_TEXT_LENGTH: usize = 2000;
    /// 富文本消息最大长度
    pub const MAX_RICH_TEXT_LENGTH: usize = 5000;
    /// 消息卡片JSON最大长度
    pub const MAX_CARD_JSON_LENGTH: usize = 10000;
    /// 图片最大大小（10MB）
    pub const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;
    /// 文件最大大小（50MB）
    pub const MAX_FILE_SIZE: usize = 50 * 1024 * 1024;
    /// 视频最大大小（100MB）
    pub const MAX_VIDEO_SIZE: usize = 100 * 1024 * 1024;
    /// 音频最大大小（20MB）
    pub const MAX_AUDIO_SIZE: usize = 20 * 1024 * 1024;
    /// 支持的图片格式
    pub const SUPPORTED_IMAGE_FORMATS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp", "bmp"];
    /// 支持的文件格式
    pub const SUPPORTED_FILE_FORMATS: &[&str] = &[
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "zip", "rar", "7z",
    ];
    /// 支持的视频格式
    pub const SUPPORTED_VIDEO_FORMATS: &[&str] = &["mp4", "mov", "avi", "mkv", "webm"];
    /// 支持的音频格式
    pub const SUPPORTED_AUDIO_FORMATS: &[&str] = &["mp3", "wav", "aac", "ogg", "m4a"];
}

/// 验证文本消息
///
/// # 参数
/// - `text`: 文本内容
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_text_message(text: &str, field_name: &str) -> ValidationResult {
    let trimmed_text = text.trim();

    if trimmed_text.is_empty() {
        return ValidationResult::Invalid(format!("{} 不能为空", field_name));
    }

    if trimmed_text.len() > message_limits::MAX_TEXT_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} 长度不能超过 {} 个字符，当前长度: {}",
            field_name,
            message_limits::MAX_TEXT_LENGTH,
            trimmed_text.len()
        ));
    }

    ValidationResult::Valid
}

/// 验证富文本消息
///
/// # 参数
/// - `rich_text`: 富文本内容
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_rich_text_message(rich_text: &str, field_name: &str) -> ValidationResult {
    if rich_text.is_empty() {
        return ValidationResult::Invalid(format!("{} 不能为空", field_name));
    }

    if rich_text.len() > message_limits::MAX_RICH_TEXT_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} 长度不能超过 {} 个字符，当前长度: {}",
            field_name,
            message_limits::MAX_RICH_TEXT_LENGTH,
            rich_text.len()
        ));
    }

    // 简单的HTML/Markdown格式验证
    if !is_valid_rich_text_format(rich_text) {
        return ValidationResult::Invalid(format!("{} 包含无效的富文本格式", field_name));
    }

    ValidationResult::Valid
}

/// 验证消息卡片JSON
///
/// # 参数
/// - `card_json`: 卡片JSON字符串
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_message_card(card_json: &str, field_name: &str) -> ValidationResult {
    if card_json.is_empty() {
        return ValidationResult::Invalid(format!("{} 不能为空", field_name));
    }

    if card_json.len() > message_limits::MAX_CARD_JSON_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} 长度不能超过 {} 个字符，当前长度: {}",
            field_name,
            message_limits::MAX_CARD_JSON_LENGTH,
            card_json.len()
        ));
    }

    // 验证JSON格式
    match serde_json::from_str::<serde_json::Value>(card_json) {
        Ok(_) => ValidationResult::Valid,
        Err(e) => ValidationResult::Invalid(format!("{} JSON格式不正确: {}", field_name, e)),
    }
}

/// 验证图片文件
///
/// # 参数
/// - `file_data`: 文件数据
/// - `file_name`: 文件名
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_image_file(
    file_data: &[u8],
    file_name: &str,
    field_name: &str,
) -> ValidationResult {
    // 验证文件大小
    if file_data.len() > message_limits::MAX_IMAGE_SIZE {
        let size_mb = file_data.len() as f64 / (1024.0 * 1024.0);
        return ValidationResult::Invalid(format!(
            "{} 大小 {:.2}MB 超过限制 {:.2}MB",
            field_name,
            size_mb,
            message_limits::MAX_IMAGE_SIZE as f64 / (1024.0 * 1024.0)
        ));
    }

    // 验证文件扩展名
    let extension = std::path::Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    if let Some(ext) = extension {
        if !message_limits::SUPPORTED_IMAGE_FORMATS.contains(&ext.as_str()) {
            return ValidationResult::Invalid(format!(
                "{} 不支持的图片格式: {}，支持的格式: {}",
                field_name,
                ext,
                message_limits::SUPPORTED_IMAGE_FORMATS.join(", ")
            ));
        }
    } else {
        return ValidationResult::Invalid(format!("{} 文件必须包含扩展名", field_name));
    }

    ValidationResult::Valid
}

/// 验证富文本格式是否有效
fn is_valid_rich_text_format(rich_text: &str) -> bool {
    // 简单检查是否包含基本的HTML/Markdown标记
    let has_valid_markup = rich_text.contains("<") && rich_text.contains(">")
        || rich_text.contains("**") && rich_text.contains("**")
        || rich_text.contains("#")
        || rich_text.contains("```");

    // 检查是否包含潜在的危险标签（简单安全检查）
    let dangerous_patterns = ["<script", "javascript:", "data:", "vbscript:"];
    let has_dangerous_content = dangerous_patterns
        .iter()
        .any(|pattern| rich_text.to_lowercase().contains(pattern));

    has_valid_markup && !has_dangerous_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_text_message() {
        // 有效文本
        assert!(validate_text_message("Hello World", "消息内容").is_valid());
        assert!(validate_text_message("你好，世界！", "消息内容").is_valid());

        // 无效文本
        assert!(!validate_text_message("", "消息内容").is_valid());
        assert!(!validate_text_message("   ", "消息内容").is_valid());

        // 文本过长
        let long_text = "a".repeat(message_limits::MAX_TEXT_LENGTH + 1);
        assert!(!validate_text_message(&long_text, "消息内容").is_valid());
    }

    #[test]
    fn test_validate_message_limits() {
        assert!(message_limits::MAX_TEXT_LENGTH > 0);
        assert!(message_limits::MAX_RICH_TEXT_LENGTH > 0);
        assert!(message_limits::MAX_CARD_JSON_LENGTH > 0);
        assert!(message_limits::MAX_IMAGE_SIZE > 0);
        assert!(message_limits::MAX_FILE_SIZE > 0);
        assert!(message_limits::MAX_VIDEO_SIZE > 0);
        assert!(message_limits::MAX_AUDIO_SIZE > 0);
        assert!(!message_limits::SUPPORTED_IMAGE_FORMATS.is_empty());
        assert!(!message_limits::SUPPORTED_FILE_FORMATS.is_empty());
        assert!(!message_limits::SUPPORTED_VIDEO_FORMATS.is_empty());
        assert!(!message_limits::SUPPORTED_AUDIO_FORMATS.is_empty());
    }

    #[test]
    fn test_is_valid_rich_text_format() {
        // 有效富文本格式
        assert!(is_valid_rich_text_format("<b>Hello</b>"));
        assert!(is_valid_rich_text_format("**Hello**"));

        // 无效富文本格式
        assert!(!is_valid_rich_text_format("plain"));
    }
}
