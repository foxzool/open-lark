//! IM（即时消息）验证模块
//!
//! 提供消息相关功能的验证服务，包括消息内容、接收者、文件等验证。

use crate::core::validation::{ValidateBuilder, ValidationResult};
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

    #[test]
    fn test_validate_message_content() {
        // 有效内容
        assert!(matches!(
            validate_message_content("Hello", "text"),
            ValidationResult::Valid
        ));

        // 空内容
        assert!(matches!(
            validate_message_content("", "text"),
            ValidationResult::Invalid(_)
        ));

        // 过长内容
        let long_content = "a".repeat(153_601);
        assert!(matches!(
            validate_message_content(&long_content, "text"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_receiver_id() {
        // 有效ID
        assert!(matches!(
            validate_receiver_id("ou_1234567890123456789012345", "open_id"),
            ValidationResult::Valid
        ));

        // 无效前缀
        assert!(matches!(
            validate_receiver_id("abc123", "open_id"),
            ValidationResult::Invalid(_)
        ));

        // 无效长度
        assert!(matches!(
            validate_receiver_id("ou_123", "open_id"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_uuid() {
        // 有效UUID
        assert!(matches!(
            validate_uuid("550e8400-e29b-41d4-a716-446655440000"),
            ValidationResult::Valid
        ));

        // 无效格式
        assert!(matches!(
            validate_uuid("invalid-uuid"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_file_upload() {
        // 有效文件
        assert!(matches!(
            validate_file_upload("test.jpg", 1024, "image"),
            ValidationResult::Valid
        ));

        // 无效文件名
        assert!(matches!(
            validate_file_upload("", 1024, "image"),
            ValidationResult::Invalid(_)
        ));

        // 文件过大
        assert!(matches!(
            validate_file_upload("large.jpg", 200 * 1024 * 1024, "image"),
            ValidationResult::Invalid(_)
        ));

        // 无效文件类型
        assert!(matches!(
            validate_file_upload("test.exe", 1024, "image"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_message_receivers() {
        // 有效接收者
        let receivers = vec![
            json!({"user_id": "u_1234567890"}),
            json!({"open_id": "ou_1234567890123456789012345"}),
        ];
        assert!(matches!(
            validate_message_receivers(&receivers, 10),
            ValidationResult::Valid
        ));

        // 空列表
        assert!(matches!(
            validate_message_receivers(&[], 10),
            ValidationResult::Invalid(_)
        ));

        // 超过限制
        let mut many_receivers = Vec::new();
        for i in 0..20 {
            many_receivers.push(json!({"user_id": format!("u_{}", i)}));
        }
        assert!(matches!(
            validate_message_receivers(&many_receivers, 10),
            ValidationResult::Invalid(_)
        ));
    }
}
