//! 核心验证功能

use log::error;

/// 检查字符是否为中文字符
///
/// 使用 Unicode 范围检查来识别中文字符
///
/// # 参数
/// - `c`: 要检查的字符
///
/// # 返回
/// true 如果是中文字符，false 否则
pub fn is_chinese_char(c: char) -> bool {
    // CJK Unified Ideographs (扩展A-F区)
    let ranges = [
        (0x4E00, 0x9FFF),   // CJK Unified Ideographs
        (0x3400, 0x4DBF),   // CJK Unified Ideographs Extension A
        (0x20000, 0x2A6DF), // CJK Unified Ideographs Extension B
        (0x2A700, 0x2B73F), // CJK Unified Ideographs Extension C
        (0x2B740, 0x2B81F), // CJK Unified Ideographs Extension D
        (0x2B820, 0x2CEAF), // CJK Unified Ideographs Extension E
        (0x2CEB0, 0x2EBEF), // CJK Unified Ideographs Extension F
        (0x3000, 0x303F),   // CJK Symbols and Punctuation
        (0x31C0, 0x31EF),   // CJK Strokes
        (0x2F00, 0x2FD5),   // Kangxi Radicals
        (0x2E80, 0x2EFF),   // CJK Radicals Supplement
        (0xF900, 0xFAFF),   // CJK Compatibility Ideographs
        (0x2F800, 0x2FA1F), // CJK Compatibility Ideographs Supplement
    ];

    let code = c as u32;
    ranges
        .iter()
        .any(|&(start, end)| code >= start && code <= end)
}

/// 验证字符串长度，如果超过最大长度则截断
///
/// # 参数
/// - `input`: 输入字符串
/// - `max_len`: 最大长度
/// - `field_name`: 字段名称（用于日志）
///
/// # 返回
/// 验证后的字符串（可能被截断）
pub fn validate_string_length(input: String, max_len: usize, field_name: &str) -> String {
    if input.len() > max_len {
        error!(
            "字段 {} 超过最大长度 {}，当前长度 {}，将被截断",
            field_name,
            max_len,
            input.len()
        );
        // 安全截断，确保不会截断UTF-8字符
        input.chars().take(max_len).collect()
    } else {
        input
    }
}

/// 验证必填字段
///
/// # 参数
/// - `value`: 字段值
/// - `field_name`: 字段名称（用于日志）
///
/// # 返回
/// true 如果字段有效，false 否则
pub fn validate_required(value: &str, field_name: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        error!("必填字段 {} 为空", field_name);
        false
    } else {
        true
    }
}

/// 验证内容大小
///
/// # 参数
/// - `content`: 内容
/// - `max_size`: 最大大小（字节）
/// - `content_type`: 内容类型（用于日志）
///
/// # 返回
/// true 如果大小有效，false 否则
pub fn validate_content_size(content: &str, max_size: usize, content_type: &str) -> bool {
    let content_size = content.len();
    if content_size > max_size {
        error!(
            "{} 内容大小 {} 超过最大限制 {}",
            content_type, content_size, max_size
        );
        false
    } else {
        true
    }
}

/// 验证结果
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    /// 验证成功
    Valid,
    /// 验证失败，附带错误信息
    Invalid(String),
    /// 需要清理，附带清理后的值
    Sanitized(String),
}

impl ValidationResult {
    /// 检查验证结果是否有效
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// 获取错误信息（如果有）
    pub fn error_message(&self) -> Option<&String> {
        match self {
            ValidationResult::Invalid(msg) => Some(msg),
            _ => None,
        }
    }

    /// 获取清理后的值（如果有）
    pub fn sanitized_value(&self) -> Option<&String> {
        match self {
            ValidationResult::Sanitized(value) => Some(value),
            _ => None,
        }
    }

    /// 将验证结果转换为结果类型
    pub fn into_result(self) -> Result<String, String> {
        match self {
            ValidationResult::Valid => Ok(String::new()),
            ValidationResult::Invalid(msg) => Err(msg),
            ValidationResult::Sanitized(value) => Ok(value),
        }
    }
}

/// 验证构建器特征
pub trait ValidateBuilder {
    /// 最终的输出类型
    type Output;

    /// 添加必填验证
    fn required(self, value: Option<String>, field_name: &str) -> Self;

    /// 添加长度验证
    fn length(self, value: String, min_len: usize, max_len: usize, field_name: &str) -> Self;

    /// 添加自定义验证
    fn custom<F>(self, value: String, validator: F, error_msg: &str) -> Self
    where
        F: FnOnce(&str) -> bool;

    /// 执行验证
    fn validate(&self) -> ValidationResult;

    /// 构建最终结果
    fn build(self) -> Self::Output;
}

/// 默认验证构建器实现
#[derive(Debug)]
pub struct DefaultValidateBuilder {
    value: Option<String>,
    errors: Vec<String>,
}

impl DefaultValidateBuilder {
    /// 创建新的验证构建器
    pub fn new() -> Self {
        Self {
            value: None,
            errors: Vec::new(),
        }
    }

    /// 设置初始值
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
}

impl ValidateBuilder for DefaultValidateBuilder {
    type Output = Result<String, Vec<String>>;

    fn required(mut self, value: Option<String>, field_name: &str) -> Self {
        match value {
            Some(v) => {
                if !validate_required(&v, field_name) {
                    self.errors.push(format!("字段 {} 不能为空", field_name));
                } else {
                    self.value = Some(v);
                }
            }
            None => {
                self.errors.push(format!("字段 {} 不能为空", field_name));
            }
        }
        self
    }

    fn length(mut self, value: String, min_len: usize, max_len: usize, field_name: &str) -> Self {
        if value.len() < min_len {
            self.errors.push(format!(
                "字段 {} 长度 {} 小于最小长度 {}",
                field_name,
                value.len(),
                min_len
            ));
        } else if value.len() > max_len {
            self.errors.push(format!(
                "字段 {} 长度 {} 超过最大长度 {}",
                field_name,
                value.len(),
                max_len
            ));
        } else {
            self.value = Some(validate_string_length(value, max_len, field_name));
        }
        self
    }

    fn custom<F>(mut self, value: String, validator: F, error_msg: &str) -> Self
    where
        F: FnOnce(&str) -> bool,
    {
        if validator(&value) {
            self.value = Some(value);
        } else {
            self.errors.push(error_msg.to_string());
        }
        self
    }

    fn validate(&self) -> ValidationResult {
        if self.errors.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid(self.errors.join("; "))
        }
    }

    fn build(self) -> Self::Output {
        if self.errors.is_empty() {
            Ok(self.value.unwrap_or_default())
        } else {
            Err(self.errors)
        }
    }
}

impl Default for DefaultValidateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_chinese_char() {
        assert!(is_chinese_char('你'));
        assert!(is_chinese_char('好'));
        assert!(is_chinese_char('世'));
        assert!(is_chinese_char('界'));
        assert!(!is_chinese_char('a'));
        assert!(!is_chinese_char('1'));
        assert!(!is_chinese_char('!'));
    }

    #[test]
    fn test_validate_string_length() {
        let long_string = "这是一个很长的字符串用于测试截断功能".to_string();
        let result = validate_string_length(long_string.clone(), 10, "测试字段");
        assert_eq!(result.chars().count(), 10); // 确保字符数正好是10
        assert!(result.len() <= 30); // 字节数可能更多（UTF-8）

        let short_string = "短".to_string();
        let result = validate_string_length(short_string, 10, "测试字段");
        assert_eq!(result, "短");
    }

    #[test]
    fn test_validate_required() {
        assert!(validate_required("有效值", "测试字段"));
        assert!(!validate_required("", "测试字段"));
        assert!(!validate_required("   ", "测试字段"));
        assert!(validate_required("  有效值  ", "测试字段")); // trim后有效
    }

    #[test]
    fn test_validate_content_size() {
        let content = "这是一个测试内容";
        assert!(validate_content_size(content, 30, "测试内容")); // 30 > 25字节
        assert!(!validate_content_size(content, 20, "测试内容")); // 20 < 25字节
    }

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::Valid;
        assert!(valid.is_valid());
        assert_eq!(valid.error_message(), None);
        assert_eq!(valid.sanitized_value(), None);

        let invalid = ValidationResult::Invalid("错误".to_string());
        assert!(!invalid.is_valid());
        assert_eq!(invalid.error_message(), Some(&"错误".to_string()));
        assert_eq!(invalid.sanitized_value(), None);

        let sanitized = ValidationResult::Sanitized("清理后".to_string());
        assert!(!sanitized.is_valid());
        assert_eq!(sanitized.error_message(), None);
        assert_eq!(sanitized.sanitized_value(), Some(&"清理后".to_string()));
    }

    #[test]
    fn test_default_validate_builder() {
        let builder = DefaultValidateBuilder::new();

        // 成功案例
        let result = builder
            .value("测试值".to_string())
            .length("测试值".to_string(), 1, 10, "测试字段")
            .build();
        assert!(result.is_ok());

        // 失败案例
        let result = DefaultValidateBuilder::new()
            .value("这是一个很长的测试值".to_string())
            .length("这是一个很长的测试值".to_string(), 1, 5, "测试字段")
            .build();
        assert!(result.is_err());
    }
}
