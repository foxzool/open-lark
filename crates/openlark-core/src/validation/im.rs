//! IM（即时消息）验证模块
//!
//! 提供消息相关功能的验证服务，包括消息内容、接收者、文件等验证。

use crate::validation::{ValidateBuilder, ValidationResult};
use serde_json::Value;

/// 验证消息内容长度
///
/// # 参数
/// - `content`: 消息内容
/// - `message_type`: 消息类型（text, post, interactive）
///
/// # 返回
/// - ValidationResult: 验证结果
///
/// # 限制
/// - 文本消息: 最大 150KB (153,600 字符)
/// - 富文本消息: 最大 30KB (30,720 字符)
/// - 互动消息: 最大 30KB (30,720 字符)
pub fn validate_message_content(content: &str, message_type: &str) -> ValidationResult {
    if content.is_empty() {
        return ValidationResult::Invalid("Message content cannot be empty".to_string());
    }

    let max_length = match message_type {
        "text" => 153_600,                // 150KB
        "post" | "interactive" => 30_720, // 30KB
        _ => {
            return ValidationResult::Invalid(format!("Unsupported message type: {}", message_type))
        }
    };

    if content.len() > max_length {
        return ValidationResult::Invalid(format!(
            "Message content too long. Maximum {} characters allowed for {} messages, got {}",
            max_length,
            message_type,
            content.len()
        ));
    }

    ValidationResult::Valid
}

/// 验证接收者ID格式
///
/// # 参数
/// - `receiver_id`: 接收者ID
/// - `id_type`: ID类型（open_id, user_id, union_id, chat_id）
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_receiver_id(receiver_id: &str, id_type: &str) -> ValidationResult {
    if receiver_id.is_empty() {
        return ValidationResult::Invalid("Receiver ID cannot be empty".to_string());
    }

    // 验证不同类型的ID格式
    match id_type {
        "open_id" => {
            if !receiver_id.starts_with("ou_") {
                return ValidationResult::Invalid("Open ID must start with 'ou_'".to_string());
            }
            if receiver_id.len() != 28 {
                return ValidationResult::Invalid("Open ID must be 28 characters long".to_string());
            }
        }
        "user_id" => {
            if !receiver_id.starts_with("u_") {
                return ValidationResult::Invalid("User ID must start with 'u_'".to_string());
            }
        }
        "union_id" => {
            if !receiver_id.starts_with("on_") {
                return ValidationResult::Invalid("Union ID must start with 'on_'".to_string());
            }
            if receiver_id.len() != 28 {
                return ValidationResult::Invalid(
                    "Union ID must be 28 characters long".to_string(),
                );
            }
        }
        "chat_id" => {
            if !receiver_id.starts_with("oc_") {
                return ValidationResult::Invalid("Chat ID must start with 'oc_'".to_string());
            }
            if receiver_id.len() != 28 {
                return ValidationResult::Invalid("Chat ID must be 28 characters long".to_string());
            }
        }
        _ => return ValidationResult::Invalid(format!("Unsupported ID type: {}", id_type)),
    }

    // 验证ID字符（只允许字母、数字和下划线）
    if !receiver_id.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return ValidationResult::Invalid(
            "ID contains invalid characters. Only alphanumeric and underscore are allowed"
                .to_string(),
        );
    }

    ValidationResult::Valid
}

/// 验证消息类型
///
/// # 参数
/// - `message_type`: 消息类型
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_message_type(message_type: &str) -> ValidationResult {
    let valid_types = [
        "text",
        "post",
        "image",
        "file",
        "audio",
        "media",
        "sticker",
        "interactive",
        "share_chat",
    ];

    if !valid_types.contains(&message_type) {
        return ValidationResult::Invalid(format!(
            "Invalid message type '{}'. Valid types are: {}",
            message_type,
            valid_types.join(", ")
        ));
    }

    ValidationResult::Valid
}

/// 验证UUID格式
///
/// # 参数
/// - `uuid`: UUID字符串
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_uuid(uuid: &str) -> ValidationResult {
    if uuid.is_empty() {
        return ValidationResult::Invalid("UUID cannot be empty".to_string());
    }

    // 简单的UUID格式验证（实际可能需要更复杂的验证）
    if uuid.len() != 36 {
        return ValidationResult::Invalid("UUID must be 36 characters long".to_string());
    }

    let parts: Vec<&str> = uuid.split('-').collect();
    if parts.len() != 5 {
        return ValidationResult::Invalid(
            "UUID must have 5 parts separated by hyphens".to_string(),
        );
    }

    // 验证各部分长度
    let expected_lengths = [8, 4, 4, 4, 12];
    for (i, (part, expected_len)) in parts.iter().zip(expected_lengths.iter()).enumerate() {
        if part.len() != *expected_len {
            return ValidationResult::Invalid(format!(
                "UUID part {} must be {} characters long, got {}",
                i + 1,
                expected_len,
                part.len()
            ));
        }

        // 验证字符
        if !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return ValidationResult::Invalid(format!(
                "UUID part {} contains invalid characters. Only hexadecimal digits are allowed",
                i + 1
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证文件上传信息
///
/// # 参数
/// - `file_name`: 文件名
/// - `file_size`: 文件大小（字节）
/// - `file_type`: 文件类型
///
/// # 返回
/// - ValidationResult: 验证结果
///
/// # 限制
/// - 文件名: 最大 255 字符
/// - 文件大小: 最大 100MB (104,857,600 字节)
/// - 文件类型: 根据不同消息类型有不同限制
pub fn validate_file_upload(file_name: &str, file_size: u64, file_type: &str) -> ValidationResult {
    // 验证文件名
    if file_name.is_empty() {
        return ValidationResult::Invalid("File name cannot be empty".to_string());
    }

    if file_name.len() > 255 {
        return ValidationResult::Invalid(
            "File name too long. Maximum 255 characters allowed".to_string(),
        );
    }

    // 验证文件名字符（不允许特殊字符）
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if file_name.chars().any(|c| invalid_chars.contains(&c)) {
        return ValidationResult::Invalid("File name contains invalid characters".to_string());
    }

    // 验证文件大小
    if file_size > 100 * 1024 * 1024 {
        // 100MB
        return ValidationResult::Invalid("File too large. Maximum 100MB allowed".to_string());
    }

    // 验证文件类型
    let allowed_types: &[&str] = match file_type {
        "image" => &["jpg", "jpeg", "png", "gif", "bmp", "webp"],
        "audio" => &["mp3", "wav", "amr", "aac", "ogg"],
        "video" => &["mp4", "mov", "avi", "mkv", "flv"],
        "file" => &[
            "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "zip", "rar",
        ],
        _ => return ValidationResult::Invalid(format!("Unsupported file type: {}", file_type)),
    };

    let file_ext = file_name
        .split('.')
        .next_back()
        .unwrap_or("")
        .to_lowercase();
    if !allowed_types.contains(&file_ext.as_str()) {
        return ValidationResult::Invalid(format!(
            "File type '.{}' is not allowed for {} files",
            file_ext, file_type
        ));
    }

    ValidationResult::Valid
}

/// 验证消息撤回请求
///
/// # 参数
/// - `message_id`: 消息ID
/// - `chat_id`: 聊天ID
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_message_recall(message_id: &str, chat_id: &str) -> ValidationResult {
    if message_id.is_empty() {
        return ValidationResult::Invalid("Message ID cannot be empty".to_string());
    }

    if chat_id.is_empty() {
        return ValidationResult::Invalid("Chat ID cannot be empty".to_string());
    }

    // 验证消息ID格式（通常是UUID）
    if let ValidationResult::Invalid(msg) = validate_uuid(message_id) {
        return ValidationResult::Invalid(format!("Invalid message ID: {}", msg));
    }

    // 验证聊天ID
    if let ValidationResult::Invalid(msg) = validate_receiver_id(chat_id, "chat_id") {
        return ValidationResult::Invalid(format!("Invalid chat ID: {}", msg));
    }

    ValidationResult::Valid
}

/// 验证消息读取状态更新
///
/// # 参数
/// - `message_id`: 消息ID
/// - `user_id`: 用户ID
/// - `read_timestamp`: 读取时间戳
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_message_read_status(
    message_id: &str,
    user_id: &str,
    read_timestamp: i64,
) -> ValidationResult {
    if message_id.is_empty() {
        return ValidationResult::Invalid("Message ID cannot be empty".to_string());
    }

    if user_id.is_empty() {
        return ValidationResult::Invalid("User ID cannot be empty".to_string());
    }

    // 验证时间戳（不能是未来时间）
    let current_time = chrono::Utc::now().timestamp();
    if read_timestamp > current_time {
        return ValidationResult::Invalid("Read timestamp cannot be in the future".to_string());
    }

    // 验证时间戳不能太早（比如2020年之前）
    if read_timestamp < 1_577_836_800 {
        // 2020-01-01
        return ValidationResult::Invalid("Read timestamp is too early".to_string());
    }

    ValidationResult::Valid
}

/// 验证消息转发请求
///
/// # 参数
/// - `source_message_id`: 源消息ID
/// - `target_chat_id`: 目标聊天ID
/// - `forward_type`: 转发类型
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_message_forward(
    source_message_id: &str,
    target_chat_id: &str,
    forward_type: &str,
) -> ValidationResult {
    if source_message_id.is_empty() {
        return ValidationResult::Invalid("Source message ID cannot be empty".to_string());
    }

    if target_chat_id.is_empty() {
        return ValidationResult::Invalid("Target chat ID cannot be empty".to_string());
    }

    // 验证转发类型
    let valid_forward_types = ["normal", "forward_as_quote"];
    if !valid_forward_types.contains(&forward_type) {
        return ValidationResult::Invalid(format!(
            "Invalid forward type '{}'. Valid types are: {}",
            forward_type,
            valid_forward_types.join(", ")
        ));
    }

    // 验证ID格式
    if let ValidationResult::Invalid(msg) = validate_uuid(source_message_id) {
        return ValidationResult::Invalid(format!("Invalid source message ID: {}", msg));
    }

    if let ValidationResult::Invalid(msg) = validate_receiver_id(target_chat_id, "chat_id") {
        return ValidationResult::Invalid(format!("Invalid target chat ID: {}", msg));
    }

    ValidationResult::Valid
}

/// 验证消息接收者列表
///
/// # 参数
/// - `receivers`: 接收者列表
/// - `max_receivers`: 最大接收者数量
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_message_receivers(receivers: &[Value], max_receivers: usize) -> ValidationResult {
    if receivers.is_empty() {
        return ValidationResult::Invalid("At least one receiver is required".to_string());
    }

    if receivers.len() > max_receivers {
        return ValidationResult::Invalid(format!(
            "Too many receivers. Maximum {} allowed, got {}",
            max_receivers,
            receivers.len()
        ));
    }

    for (i, receiver) in receivers.iter().enumerate() {
        if let Some(obj) = receiver.as_object() {
            // 验证必需字段
            if !obj.contains_key("user_id")
                && !obj.contains_key("union_id")
                && !obj.contains_key("open_id")
            {
                return ValidationResult::Invalid(format!(
                    "Receiver at index {} must have either user_id, union_id, or open_id",
                    i
                ));
            }

            // 验证ID格式
            if let Some(user_id) = obj.get("user_id").and_then(|v| v.as_str()) {
                if let ValidationResult::Invalid(msg) = validate_receiver_id(user_id, "user_id") {
                    return ValidationResult::Invalid(format!(
                        "Invalid user_id at index {}: {}",
                        i, msg
                    ));
                }
            }

            if let Some(union_id) = obj.get("union_id").and_then(|v| v.as_str()) {
                if let ValidationResult::Invalid(msg) = validate_receiver_id(union_id, "union_id") {
                    return ValidationResult::Invalid(format!(
                        "Invalid union_id at index {}: {}",
                        i, msg
                    ));
                }
            }

            if let Some(open_id) = obj.get("open_id").and_then(|v| v.as_str()) {
                if let ValidationResult::Invalid(msg) = validate_receiver_id(open_id, "open_id") {
                    return ValidationResult::Invalid(format!(
                        "Invalid open_id at index {}: {}",
                        i, msg
                    ));
                }
            }
        } else {
            return ValidationResult::Invalid(format!("Receiver at index {} must be an object", i));
        }
    }

    ValidationResult::Valid
}

/// 验证消息模板内容
///
/// # 参数
/// - `template_content`: 模板内容
/// - `template_id`: 模板ID
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_message_template(template_content: &str, template_id: &str) -> ValidationResult {
    if template_content.is_empty() {
        return ValidationResult::Invalid("Template content cannot be empty".to_string());
    }

    if template_id.is_empty() {
        return ValidationResult::Invalid("Template ID cannot be empty".to_string());
    }

    // 验证模板内容长度
    if template_content.len() > 50_000 {
        // 50KB
        return ValidationResult::Invalid(
            "Template content too long. Maximum 50KB allowed".to_string(),
        );
    }

    // 验证模板ID格式（通常是UUID）
    if let ValidationResult::Invalid(msg) = validate_uuid(template_id) {
        return ValidationResult::Invalid(format!("Invalid template ID: {}", msg));
    }

    // 验证模板内容格式（JSON格式）
    if serde_json::from_str::<Value>(template_content).is_err() {
        return ValidationResult::Invalid("Template content must be valid JSON".to_string());
    }

    ValidationResult::Valid
}

/// 验证消息表情回复
///
/// # 参数
/// - `message_id`: 消息ID
/// - `emoji_type`: 表情类型
/// - `emoji_key`: 表情key
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_message_reaction(
    message_id: &str,
    emoji_type: &str,
    emoji_key: &str,
) -> ValidationResult {
    if message_id.is_empty() {
        return ValidationResult::Invalid("Message ID cannot be empty".to_string());
    }

    if emoji_type.is_empty() {
        return ValidationResult::Invalid("Emoji type cannot be empty".to_string());
    }

    if emoji_key.is_empty() {
        return ValidationResult::Invalid("Emoji key cannot be empty".to_string());
    }

    // 验证表情类型
    let valid_emoji_types = ["emoji", "custom"];
    if !valid_emoji_types.contains(&emoji_type) {
        return ValidationResult::Invalid(format!(
            "Invalid emoji type '{}'. Valid types are: {}",
            emoji_type,
            valid_emoji_types.join(", ")
        ));
    }

    // 验证表情key长度
    if emoji_key.len() > 100 {
        return ValidationResult::Invalid(
            "Emoji key too long. Maximum 100 characters allowed".to_string(),
        );
    }

    ValidationResult::Valid
}

/// Builder trait for IM validation
pub trait ValidateImBuilder {
    /// 验证消息内容
    fn validate_message_content(&self, content: &str, message_type: &str) -> ValidationResult {
        validate_message_content(content, message_type)
    }

    /// 验证接收者ID
    fn validate_receiver_id(&self, receiver_id: &str, id_type: &str) -> ValidationResult {
        validate_receiver_id(receiver_id, id_type)
    }

    /// 验证文件上传
    fn validate_file_upload(
        &self,
        file_name: &str,
        file_size: u64,
        file_type: &str,
    ) -> ValidationResult {
        validate_file_upload(file_name, file_size, file_type)
    }

    /// 验证消息接收者列表
    fn validate_message_receivers(
        &self,
        receivers: &[Value],
        max_receivers: usize,
    ) -> ValidationResult {
        validate_message_receivers(receivers, max_receivers)
    }
}

// 为所有实现了 ValidateBuilder 的类型自动实现 ValidateImBuilder
impl<T: ValidateBuilder> ValidateImBuilder for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ========== validate_message_content 测试 ==========

    #[test]
    fn test_validate_message_content_valid_cases() {
        // 文本消息 - 各种长度
        assert!(matches!(
            validate_message_content("Hello", "text"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_message_content("A", "text"),
            ValidationResult::Valid
        ));

        // 富文本消息 - 最大长度边界
        let post_content = "A".repeat(30_720);
        assert!(matches!(
            validate_message_content(&post_content, "post"),
            ValidationResult::Valid
        ));

        // 互动消息 - 最大长度边界
        let interactive_content = "A".repeat(30_720);
        assert!(matches!(
            validate_message_content(&interactive_content, "interactive"),
            ValidationResult::Valid
        ));

        // Unicode字符支持
        assert!(matches!(
            validate_message_content("🎉 Hello World! 你好世界！", "text"),
            ValidationResult::Valid
        ));

        // 边界情况 - 文本消息最大长度
        let max_text_content = "A".repeat(153_600);
        assert!(matches!(
            validate_message_content(&max_text_content, "text"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_message_content_invalid_cases() {
        // 空内容
        assert!(matches!(
            validate_message_content("", "text"),
            ValidationResult::Invalid(msg) if msg.contains("cannot be empty")
        ));

        // 不支持的消息类型
        assert!(matches!(
            validate_message_content("Hello", "video"),
            ValidationResult::Invalid(msg) if msg.contains("Unsupported message type")
        ));

        // 文本消息过长
        let too_long_text = "A".repeat(153_601);
        assert!(matches!(
            validate_message_content(&too_long_text, "text"),
            ValidationResult::Invalid(msg) if msg.contains("too long")
        ));

        // 富文本消息过长
        let too_long_post = "A".repeat(30_721);
        assert!(matches!(
            validate_message_content(&too_long_post, "post"),
            ValidationResult::Invalid(msg) if msg.contains("too long")
        ));

        // 互动消息过长
        let too_long_interactive = "A".repeat(30_721);
        assert!(matches!(
            validate_message_content(&too_long_interactive, "interactive"),
            ValidationResult::Invalid(msg) if msg.contains("too long")
        ));
    }

    // ========== validate_receiver_id 测试 ==========

    #[test]
    fn test_validate_receiver_id_valid_cases() {
        // open_id 有效格式
        assert!(matches!(
            validate_receiver_id("ou_1234567890123456789012345", "open_id"),
            ValidationResult::Valid
        ));

        // user_id 有效格式
        assert!(matches!(
            validate_receiver_id("u_1234567890", "user_id"),
            ValidationResult::Valid
        ));

        // union_id 有效格式
        assert!(matches!(
            validate_receiver_id("on_1234567890123456789012345", "union_id"),
            ValidationResult::Valid
        ));

        // chat_id 有效格式
        assert!(matches!(
            validate_receiver_id("oc_1234567890123456789012345", "chat_id"),
            ValidationResult::Valid
        ));

        // 数字和下划线组合
        assert!(matches!(
            validate_receiver_id("ou_ABC123def4567890123456789", "open_id"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_receiver_id_invalid_cases() {
        // 空ID
        assert!(matches!(
            validate_receiver_id("", "open_id"),
            ValidationResult::Invalid(msg) if msg.contains("cannot be empty")
        ));

        // open_id 错误前缀
        assert!(matches!(
            validate_receiver_id("u_1234567890123456789012345", "open_id"),
            ValidationResult::Invalid(msg) if msg.contains("must start with 'ou_'")
        ));

        // open_id 错误长度
        assert!(matches!(
            validate_receiver_id("ou_123", "open_id"),
            ValidationResult::Invalid(msg) if msg.contains("must be 28 characters long")
        ));

        // user_id 错误前缀
        assert!(matches!(
            validate_receiver_id("ou_1234567890", "user_id"),
            ValidationResult::Invalid(msg) if msg.contains("must start with 'u_'")
        ));

        // union_id 错误前缀
        assert!(matches!(
            validate_receiver_id("u_1234567890123456789012345", "union_id"),
            ValidationResult::Invalid(msg) if msg.contains("must start with 'on_'")
        ));

        // chat_id 错误前缀
        assert!(matches!(
            validate_receiver_id("u_1234567890123456789012345", "chat_id"),
            ValidationResult::Invalid(msg) if msg.contains("must start with 'oc_'")
        ));

        // 不支持的ID类型
        assert!(matches!(
            validate_receiver_id("test123", "invalid_type"),
            ValidationResult::Invalid(msg) if msg.contains("Unsupported ID type")
        ));

        // 包含无效字符 - 测试连字符（使用正确的长度）
        assert!(matches!(
            validate_receiver_id("ou_123456789-123456789012345", "open_id"),
            ValidationResult::Invalid(msg) if msg.contains("invalid characters")
        ));

        // 包含无效字符 - 测试@符号（使用正确的长度）
        assert!(matches!(
            validate_receiver_id("ou_123456789@123456789012345", "open_id"),
            ValidationResult::Invalid(msg) if msg.contains("invalid characters")
        ));
    }

    // ========== validate_message_type 测试 ==========

    #[test]
    fn test_validate_message_type_valid_cases() {
        let valid_types = [
            "text",
            "post",
            "image",
            "file",
            "audio",
            "media",
            "sticker",
            "interactive",
            "share_chat",
        ];

        for message_type in valid_types {
            assert!(
                matches!(validate_message_type(message_type), ValidationResult::Valid),
                "Should be valid: {}",
                message_type
            );
        }
    }

    #[test]
    fn test_validate_message_type_invalid_cases() {
        let invalid_types = ["video", "voice", "document", "unknown", ""];

        for message_type in invalid_types {
            assert!(
                matches!(
                    validate_message_type(message_type),
                    ValidationResult::Invalid(msg) if msg.contains("Invalid message type")
                ),
                "Should be invalid: {}",
                message_type
            );
        }
    }

    // ========== validate_uuid 测试 ==========

    #[test]
    fn test_validate_uuid_valid_cases() {
        // 标准UUID格式
        assert!(matches!(
            validate_uuid("550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Valid
        ));

        // 全零UUID
        assert!(matches!(
            validate_uuid("00000000-0000-0000-0000-000000000000"),
            ValidationResult::Valid
        ));

        // 全F UUID
        assert!(matches!(
            validate_uuid("ffffffff-ffff-ffff-ffff-ffffffffffff"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_uuid_invalid_cases() {
        // 空UUID
        assert!(matches!(
            validate_uuid(""),
            ValidationResult::Invalid(msg) if msg.contains("cannot be empty")
        ));

        // 错误长度
        assert!(matches!(
            validate_uuid("550e8400-e29b-41d4-a716"),
            ValidationResult::Invalid(msg) if msg.contains("must be 36 characters long")
        ));

        // 缺少连字符
        assert!(matches!(
            validate_uuid("550e8400e29b41d4a716446655440000"),
            ValidationResult::Invalid(msg) if msg.contains("must be 36 characters long")
        ));

        // 连字符过多
        assert!(matches!(
            validate_uuid("550e-8400-e29b-41d4-a716-446655-440000"),
            ValidationResult::Invalid(msg) if msg.contains("must be 36 characters long")
        ));

        // 部分长度错误
        assert!(matches!(
            validate_uuid("550e84-e29b-41d4-a716-446655440000"),
            ValidationResult::Invalid(msg) if msg.contains("must be 36 characters long")
        ));

        // 非十六进制字符
        assert!(matches!(
            validate_uuid("550e8400-e29b-41d4-a716-44665544zzzz"),
            ValidationResult::Invalid(msg) if msg.contains("hexadecimal digits")
        ));

        // 小写验证
        assert!(matches!(
            validate_uuid("550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Valid
        ));
    }

    // ========== validate_file_upload 测试 ==========

    #[test]
    fn test_validate_file_upload_valid_cases() {
        // 图片文件 - 各种格式
        let image_formats = ["jpg", "jpeg", "png", "gif", "bmp", "webp"];
        for format in image_formats {
            assert!(matches!(
                validate_file_upload(&format!("test.{}", format), 1024, "image"),
                ValidationResult::Valid
            ));
        }

        // 音频文件
        let audio_formats = ["mp3", "wav", "amr", "aac", "ogg"];
        for format in audio_formats {
            assert!(matches!(
                validate_file_upload(&format!("audio.{}", format), 2048, "audio"),
                ValidationResult::Valid
            ));
        }

        // 视频文件
        let video_formats = ["mp4", "mov", "avi", "mkv", "flv"];
        for format in video_formats {
            assert!(matches!(
                validate_file_upload(&format!("video.{}", format), 5_000_000, "video"),
                ValidationResult::Valid
            ));
        }

        // 文档文件
        let doc_formats = [
            "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "zip", "rar",
        ];
        for format in doc_formats {
            assert!(matches!(
                validate_file_upload(&format!("document.{}", format), 100_000, "file"),
                ValidationResult::Valid
            ));
        }

        // 边界情况 - 最大文件名长度
        let max_filename = "A".repeat(251); // 251 + ".txt" = 255 characters
        assert!(matches!(
            validate_file_upload(&format!("{}.txt", &max_filename), 1024, "file"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大文件大小
        assert!(matches!(
            validate_file_upload("large_file.pdf", 100 * 1024 * 1024, "file"),
            ValidationResult::Valid
        ));

        // 小写扩展名处理
        assert!(matches!(
            validate_file_upload("image.JPG", 1024, "image"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_file_upload_invalid_cases() {
        // 空文件名
        assert!(matches!(
            validate_file_upload("", 1024, "image"),
            ValidationResult::Invalid(msg) if msg.contains("File name cannot be empty")
        ));

        // 文件名过长
        let too_long_filename = "A".repeat(256);
        assert!(matches!(
            validate_file_upload(&too_long_filename, 1024, "image"),
            ValidationResult::Invalid(msg) if msg.contains("File name too long")
        ));

        // 文件名包含无效字符
        let invalid_chars_filenames = [
            "file/name.txt",
            "file\\name.txt",
            "file:name.txt",
            "file*name.txt",
            "file?name.txt",
            "file\"name.txt",
            "file<name.txt",
            "file>name.txt",
            "file|name.txt",
        ];

        for filename in invalid_chars_filenames {
            assert!(matches!(
                validate_file_upload(filename, 1024, "image"),
                ValidationResult::Invalid(msg) if msg.contains("invalid characters")
            ));
        }

        // 文件过大
        assert!(matches!(
            validate_file_upload("huge_file.pdf", 101 * 1024 * 1024, "file"),
            ValidationResult::Invalid(msg) if msg.contains("File too large")
        ));

        // 不支持的文件类型
        assert!(matches!(
            validate_file_upload("test.exe", 1024, "image"),
            ValidationResult::Invalid(msg) if msg.contains("not allowed")
        ));

        // 不支持的文件类型类别
        assert!(matches!(
            validate_file_upload("test.txt", 1024, "unknown"),
            ValidationResult::Invalid(msg) if msg.contains("Unsupported file type")
        ));

        // 缺少扩展名
        assert!(matches!(
            validate_file_upload("noextension", 1024, "image"),
            ValidationResult::Invalid(msg) if msg.contains("not allowed")
        ));
    }

    // ========== validate_message_recall 测试 ==========

    #[test]
    fn test_validate_message_recall_valid_cases() {
        // 有效参数
        assert!(matches!(
            validate_message_recall(
                "550e8400-e29b-41d4-a716-446655440000",
                "oc_1234567890123456789012345"
            ),
            ValidationResult::Valid
        ));

        // 使用标准UUID
        assert!(matches!(
            validate_message_recall(
                "00000000-0000-0000-0000-000000000000",
                "oc_0000000000000000000000000"
            ),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_message_recall_invalid_cases() {
        // 空消息ID
        assert!(matches!(
            validate_message_recall("", "oc_1234567890123456789012345"),
            ValidationResult::Invalid(msg) if msg.contains("Message ID cannot be empty")
        ));

        // 空聊天ID
        assert!(matches!(
            validate_message_recall("550e8400-e29b-41d4-a716-446655440000", ""),
            ValidationResult::Invalid(msg) if msg.contains("Chat ID cannot be empty")
        ));

        // 无效消息ID格式
        assert!(matches!(
            validate_message_recall("invalid-uuid", "oc_1234567890123456789012345"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid message ID")
        ));

        // 无效聊天ID格式
        assert!(matches!(
            validate_message_recall("550e8400-e29b-41d4-a716-446655440000", "invalid_chat_id"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid chat ID")
        ));

        // 聊天ID长度错误
        assert!(matches!(
            validate_message_recall("550e8400-e29b-41d4-a716-446655440000", "oc_123"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid chat ID")
        ));
    }

    // ========== validate_message_read_status 测试 ==========

    #[test]
    fn test_validate_message_read_status_valid_cases() {
        // 有效时间戳（当前时间）
        let current_time = chrono::Utc::now().timestamp();
        assert!(matches!(
            validate_message_read_status(
                "550e8400-e29b-41d4-a716-446655440000",
                "u_1234567890",
                current_time
            ),
            ValidationResult::Valid
        ));

        // 有效时间戳（过去时间）
        let past_time = current_time - 3600; // 1小时前
        assert!(matches!(
            validate_message_read_status(
                "550e8400-e29b-41d4-a716-446655440000",
                "u_1234567890",
                past_time
            ),
            ValidationResult::Valid
        ));

        // 边界时间戳（2020-01-01）
        assert!(matches!(
            validate_message_read_status(
                "550e8400-e29b-41d4-a716-446655440000",
                "u_1234567890",
                1_577_836_800
            ),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_message_read_status_invalid_cases() {
        // 空消息ID
        assert!(matches!(
            validate_message_read_status("", "u_1234567890", 1234567890),
            ValidationResult::Invalid(msg) if msg.contains("Message ID cannot be empty")
        ));

        // 空用户ID
        assert!(matches!(
            validate_message_read_status("550e8400-e29b-41d4-a716-446655440000", "", 1234567890),
            ValidationResult::Invalid(msg) if msg.contains("User ID cannot be empty")
        ));

        // 未来时间戳
        let future_time = chrono::Utc::now().timestamp() + 3600;
        assert!(matches!(
            validate_message_read_status(
                "550e8400-e29b-41d4-a716-446655440000",
                "u_1234567890",
                future_time
            ),
            ValidationResult::Invalid(msg) if msg.contains("cannot be in the future")
        ));

        // 时间戳太早
        assert!(matches!(
            validate_message_read_status(
                "550e8400-e29b-41d4-a716-446655440000",
                "u_1234567890",
                1_500_000_000
            ),
            ValidationResult::Invalid(msg) if msg.contains("too early")
        ));
    }

    // ========== validate_message_forward 测试 ==========

    #[test]
    fn test_validate_message_forward_valid_cases() {
        // 正常转发
        assert!(matches!(
            validate_message_forward(
                "550e8400-e29b-41d4-a716-446655440000",
                "oc_1234567890123456789012345",
                "normal"
            ),
            ValidationResult::Valid
        ));

        // 引用转发
        assert!(matches!(
            validate_message_forward(
                "550e8400-e29b-41d4-a716-446655440000",
                "oc_1234567890123456789012345",
                "forward_as_quote"
            ),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_message_forward_invalid_cases() {
        // 空源消息ID
        assert!(matches!(
            validate_message_forward("", "oc_1234567890123456789012345", "normal"),
            ValidationResult::Invalid(msg) if msg.contains("Source message ID cannot be empty")
        ));

        // 空目标聊天ID
        assert!(matches!(
            validate_message_forward("550e8400-e29b-41d4-a716-446655440000", "", "normal"),
            ValidationResult::Invalid(msg) if msg.contains("Target chat ID cannot be empty")
        ));

        // 无效转发类型
        assert!(matches!(
            validate_message_forward(
                "550e8400-e29b-41d4-a716-446655440000",
                "oc_1234567890123456789012345",
                "invalid_type"
            ),
            ValidationResult::Invalid(msg) if msg.contains("Invalid forward type")
        ));

        // 无效源消息ID格式
        assert!(matches!(
            validate_message_forward(
                "invalid-uuid",
                "oc_1234567890123456789012345",
                "normal"
            ),
            ValidationResult::Invalid(msg) if msg.contains("Invalid source message ID")
        ));

        // 无效目标聊天ID格式
        assert!(matches!(
            validate_message_forward(
                "550e8400-e29b-41d4-a716-446655440000",
                "invalid_chat_id",
                "normal"
            ),
            ValidationResult::Invalid(msg) if msg.contains("Invalid target chat ID")
        ));
    }

    // ========== validate_message_receivers 测试 ==========

    #[test]
    fn test_validate_message_receivers_valid_cases() {
        // 单个接收者
        let single_receiver = vec![json!({"user_id": "u_1234567890"})];
        assert!(matches!(
            validate_message_receivers(&single_receiver, 10),
            ValidationResult::Valid
        ));

        // 多个接收者，不同ID类型
        let multiple_receivers = vec![
            json!({"user_id": "u_1234567890"}),
            json!({"open_id": "ou_1234567890123456789012345"}),
            json!({"union_id": "on_1234567890123456789012345"}),
        ];
        assert!(matches!(
            validate_message_receivers(&multiple_receivers, 10),
            ValidationResult::Valid
        ));

        // 达到最大接收者数量
        let mut max_receivers = Vec::new();
        for i in 0..5 {
            max_receivers.push(json!({"user_id": format!("u_{}", i)}));
        }
        assert!(matches!(
            validate_message_receivers(&max_receivers, 5),
            ValidationResult::Valid
        ));

        // 复杂对象结构
        let complex_receiver = vec![json!({
            "user_id": "u_1234567890",
            "department_id": "od_123456",
            "name": "John Doe"
        })];
        assert!(matches!(
            validate_message_receivers(&complex_receiver, 10),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_message_receivers_invalid_cases() {
        // 空接收者列表
        assert!(matches!(
            validate_message_receivers(&[], 10),
            ValidationResult::Invalid(msg) if msg.contains("At least one receiver is required")
        ));

        // 超过最大接收者数量
        let mut too_many_receivers = Vec::new();
        for i in 0..11 {
            too_many_receivers.push(json!({"user_id": format!("u_{}", i)}));
        }
        assert!(matches!(
            validate_message_receivers(&too_many_receivers, 10),
            ValidationResult::Invalid(msg) if msg.contains("Too many receivers")
        ));

        // 接收者缺少必需的ID字段
        let receiver_without_id = vec![json!({"name": "John Doe"})];
        assert!(matches!(
            validate_message_receivers(&receiver_without_id, 10),
            ValidationResult::Invalid(msg) if msg.contains("must have either user_id, union_id, or open_id")
        ));

        // 接收者不是对象
        let non_object_receiver = vec![json!("string_value")];
        assert!(matches!(
            validate_message_receivers(&non_object_receiver, 10),
            ValidationResult::Invalid(msg) if msg.contains("must be an object")
        ));

        // 接收者包含无效ID
        let invalid_id_receiver = vec![json!({"user_id": "invalid_user_id"})];
        assert!(matches!(
            validate_message_receivers(&invalid_id_receiver, 10),
            ValidationResult::Invalid(msg) if msg.contains("Invalid user_id")
        ));

        // 多个无效接收者中的第一个被发现
        let multiple_invalid_receivers = vec![
            json!({"user_id": "invalid_user_id"}), // 不以u_开头
            json!({"open_id": "ou_invalid"}),
        ];
        assert!(matches!(
            validate_message_receivers(&multiple_invalid_receivers, 10),
            ValidationResult::Invalid(msg) if msg.contains("Invalid user_id")
        ));
    }

    // ========== validate_message_template 测试 ==========

    #[test]
    fn test_validate_message_template_valid_cases() {
        // 简单JSON模板
        let simple_template = r#"{"text": "Hello World"}"#;
        assert!(matches!(
            validate_message_template(simple_template, "550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Valid
        ));

        // 复杂JSON模板
        let complex_template = r#"{
            "content": {
                "title": "Welcome",
                "body": "Thank you for joining!",
                "actions": [
                    {"type": "button", "text": "OK", "url": "https://example.com"}
                ]
            }
        }"#;
        assert!(matches!(
            validate_message_template(complex_template, "550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Valid
        ));

        // 最大长度边界
        let max_template = "A".repeat(50_000);
        let max_template_json = format!(r#"{{"text": "{}"}}"#, &max_template[..49_980]);
        assert!(matches!(
            validate_message_template(&max_template_json, "550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Valid
        ));

        // Unicode内容
        let unicode_template = r#"{"message": "🎉 欢迎加入我们的团队！"}"#;
        assert!(matches!(
            validate_message_template(unicode_template, "550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_message_template_invalid_cases() {
        // 空模板内容
        assert!(matches!(
            validate_message_template("", "550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Invalid(msg) if msg.contains("Template content cannot be empty")
        ));

        // 空模板ID
        assert!(matches!(
            validate_message_template(r#"{"text": "Hello"}"#, ""),
            ValidationResult::Invalid(msg) if msg.contains("Template ID cannot be empty")
        ));

        // 模板内容过长
        let too_long_template = "A".repeat(50_001);
        let too_long_json = format!(r#"{{"text": "{}"}}"#, too_long_template);
        assert!(matches!(
            validate_message_template(&too_long_json, "550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Invalid(msg) if msg.contains("Template content too long")
        ));

        // 无效模板ID格式
        assert!(matches!(
            validate_message_template(r#"{"text": "Hello"}"#, "invalid-uuid"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid template ID")
        ));

        // 无效JSON格式
        assert!(matches!(
            validate_message_template(r#"{"text": "Hello", "invalid": }"#, "550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Invalid(msg) if msg.contains("must be valid JSON")
        ));

        // 非JSON内容
        assert!(matches!(
            validate_message_template("plain text content", "550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Invalid(msg) if msg.contains("must be valid JSON")
        ));
    }

    // ========== validate_message_reaction 测试 ==========

    #[test]
    fn test_validate_message_reaction_valid_cases() {
        // 标准表情
        assert!(matches!(
            validate_message_reaction("550e8400-e29b-41d4-a716-446655440000", "emoji", "👍"),
            ValidationResult::Valid
        ));

        // 自定义表情
        assert!(matches!(
            validate_message_reaction(
                "550e8400-e29b-41d4-a716-446655440000",
                "custom",
                "custom_emoji_123"
            ),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大key长度
        let max_key = "A".repeat(100);
        assert!(matches!(
            validate_message_reaction("550e8400-e29b-41d4-a716-446655440000", "emoji", &max_key),
            ValidationResult::Valid
        ));

        // Unicode表情key
        assert!(matches!(
            validate_message_reaction("550e8400-e29b-41d4-a716-446655440000", "emoji", "😀"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_message_reaction_invalid_cases() {
        // 空消息ID
        assert!(matches!(
            validate_message_reaction("", "emoji", "👍"),
            ValidationResult::Invalid(msg) if msg.contains("Message ID cannot be empty")
        ));

        // 空表情类型
        assert!(matches!(
            validate_message_reaction("550e8400-e29b-41d4-a716-446655440000", "", "👍"),
            ValidationResult::Invalid(msg) if msg.contains("Emoji type cannot be empty")
        ));

        // 空表情key
        assert!(matches!(
            validate_message_reaction("550e8400-e29b-41d4-a716-446655440000", "emoji", ""),
            ValidationResult::Invalid(msg) if msg.contains("Emoji key cannot be empty")
        ));

        // 无效表情类型
        assert!(matches!(
            validate_message_reaction(
                "550e8400-e29b-41d4-a716-446655440000",
                "invalid_type",
                "👍"
            ),
            ValidationResult::Invalid(msg) if msg.contains("Invalid emoji type")
        ));

        // 表情key过长
        let too_long_key = "A".repeat(101);
        assert!(matches!(
            validate_message_reaction(
                "550e8400-e29b-41d4-a716-446655440000",
                "emoji",
                &too_long_key
            ),
            ValidationResult::Invalid(msg) if msg.contains("Emoji key too long")
        ));
    }

    // ========== ValidateImBuilder trait 测试 ==========

    #[test]
    fn test_validate_im_builder_trait() {
        // 创建测试用的验证器
        struct TestValidator;
        impl ValidateBuilder for TestValidator {
            type Output = Result<String, Vec<String>>;

            fn required(self, _value: Option<String>, _field_name: &str) -> Self {
                self
            }

            fn length(
                self,
                _value: String,
                _min_len: usize,
                _max_len: usize,
                _field_name: &str,
            ) -> Self {
                self
            }

            fn custom<F>(self, _value: String, _validator: F, _error_msg: &str) -> Self
            where
                F: FnOnce(&str) -> bool,
            {
                self
            }

            fn validate(&self) -> ValidationResult {
                ValidationResult::Valid
            }

            fn build(self) -> Self::Output {
                Ok("test".to_string())
            }
        }

        let validator = TestValidator;

        // 测试 trait 方法
        assert!(matches!(
            validator.validate_message_content("Hello World", "text"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validator.validate_receiver_id("ou_1234567890123456789012345", "open_id"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validator.validate_file_upload("test.jpg", 1024, "image"),
            ValidationResult::Valid
        ));

        let receivers = vec![json!({"user_id": "u_1234567890"})];
        assert!(matches!(
            validator.validate_message_receivers(&receivers, 10),
            ValidationResult::Valid
        ));
    }

    // ========== 综合场景测试 ==========

    #[test]
    fn test_complete_message_workflow_validation() {
        // 测试完整的消息发送验证流程

        // 1. 验证消息内容
        let content_result = validate_message_content("Hello, this is a test message", "text");
        assert!(matches!(content_result, ValidationResult::Valid));

        // 2. 验证消息类型
        let type_result = validate_message_type("text");
        assert!(matches!(type_result, ValidationResult::Valid));

        // 3. 验证接收者ID
        let receiver_result = validate_receiver_id("ou_1234567890123456789012345", "open_id");
        assert!(matches!(receiver_result, ValidationResult::Valid));

        // 4. 验证接收者列表
        let receivers = vec![
            json!({"user_id": "u_1234567890"}),
            json!({"open_id": "ou_1234567890123456789012345"}),
        ];
        let receivers_result = validate_message_receivers(&receivers, 10);
        assert!(matches!(receivers_result, ValidationResult::Valid));

        // 5. 验证消息模板（如果使用模板）
        let template_content = r#"{"text": "Hello {name}"}"#;
        let template_id = "550e8400-e29b-41d4-a716-446655440000";
        let template_result = validate_message_template(template_content, template_id);
        assert!(matches!(template_result, ValidationResult::Valid));
    }

    #[test]
    fn test_error_message_content() {
        // 测试错误消息的内容是否包含有用信息
        let result = validate_message_content("", "text");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("empty"));
            assert!(msg.contains("content"));
        }

        let result = validate_receiver_id("invalid", "open_id");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("must start") || msg.contains("invalid characters"));
        }

        let result = validate_uuid("invalid");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("long") || msg.contains("parts") || msg.contains("hexadecimal"));
        }

        let result = validate_file_upload("bad/file.exe", 1024, "image");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("characters") || msg.contains("allowed"));
        }
    }

    #[test]
    fn test_unicode_and_special_characters() {
        // 测试Unicode字符支持
        assert!(matches!(
            validate_message_content("🎉 Hello World! 你好世界！", "text"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_message_template(
                r#"{"message": "🎉 欢迎加入我们的团队！"}"#,
                "550e8400-e29b-41d4-a716-446655440000"
            ),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_message_reaction("550e8400-e29b-41d4-a716-446655440000", "emoji", "😀"),
            ValidationResult::Valid
        ));

        // 测试ID中的字母数字组合
        assert!(matches!(
            validate_receiver_id("ou_ABC123def4567890123456789", "open_id"),
            ValidationResult::Valid
        ));

        // 测试文件名中的合法字符
        assert!(matches!(
            validate_file_upload("test_file_v2.1.txt", 1024, "file"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_boundary_conditions() {
        // 测试各种边界条件

        // 消息内容长度边界
        let max_text_content = "A".repeat(153_600);
        assert!(matches!(
            validate_message_content(&max_text_content, "text"),
            ValidationResult::Valid
        ));

        let too_long_text = "A".repeat(153_601);
        assert!(matches!(
            validate_message_content(&too_long_text, "text"),
            ValidationResult::Invalid(_)
        ));

        // 文件大小边界
        assert!(matches!(
            validate_file_upload("test.pdf", 100 * 1024 * 1024, "file"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_file_upload("test.pdf", 100 * 1024 * 1024 + 1, "file"),
            ValidationResult::Invalid(_)
        ));

        // 接收者数量边界
        let mut max_receivers = Vec::new();
        for i in 0..50 {
            max_receivers.push(json!({"user_id": format!("u_{}", i)}));
        }
        assert!(matches!(
            validate_message_receivers(&max_receivers, 50),
            ValidationResult::Valid
        ));

        let mut too_many_receivers = Vec::new();
        for i in 0..51 {
            too_many_receivers.push(json!({"user_id": format!("u_{}", i)}));
        }
        assert!(matches!(
            validate_message_receivers(&too_many_receivers, 50),
            ValidationResult::Invalid(_)
        ));

        // 时间戳边界
        assert!(matches!(
            validate_message_read_status(
                "550e8400-e29b-41d4-a716-446655440000",
                "u_1234567890",
                1_577_836_800 // 2020-01-01
            ),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_message_read_status(
                "550e8400-e29b-41d4-a716-446655440000",
                "u_1234567890",
                1_577_836_799 // 2019-12-31
            ),
            ValidationResult::Invalid(_)
        ));
    }
}
