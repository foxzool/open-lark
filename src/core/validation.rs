/// 验证工具模块
///
/// 提供通用的验证功能，用于构建器和其他需要数据验证的场景
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
fn is_chinese_char(c: char) -> bool {
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
            "{} exceeds maximum length of {} characters: {}",
            field_name,
            max_len,
            input.len()
        );
        input[..max_len].to_string()
    } else {
        input
    }
}

/// 验证必填字段不为空
///
/// # 参数
/// - `value`: 字段值
/// - `field_name`: 字段名称（用于日志）
///
/// # 返回
/// true 如果字段有效，false 如果字段为空
pub fn validate_required(value: &str, field_name: &str) -> bool {
    if value.is_empty() {
        error!("{} is required but empty", field_name);
        false
    } else {
        true
    }
}

/// 验证内容大小
///
/// # 参数
/// - `content`: 内容字符串
/// - `max_size`: 最大大小（字节）
/// - `content_type`: 内容类型（用于日志）
///
/// # 返回
/// true 如果内容大小有效，false 如果超过限制
pub fn validate_content_size(content: &str, max_size: usize, content_type: &str) -> bool {
    if content.len() > max_size {
        error!(
            "{} content exceeds maximum size of {} bytes: {}",
            content_type,
            max_size,
            content.len()
        );
        false
    } else {
        true
    }
}

/// 验证结果枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    /// 验证通过
    Valid,
    /// 验证失败，但有默认值或修复值
    Warning(String),
    /// 验证失败，无法继续
    Invalid(String),
}

impl ValidationResult {
    /// 检查验证是否通过（包括警告）
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid | ValidationResult::Warning(_))
    }

    /// 检查验证是否严格通过（无警告）
    pub fn is_strictly_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// 获取错误信息（如果有）
    pub fn error(&self) -> Option<&str> {
        match self {
            ValidationResult::Invalid(msg) | ValidationResult::Warning(msg) => Some(msg),
            ValidationResult::Valid => None,
        }
    }
}

/// 验证构建器 trait
///
/// 可以为构建器实现此 trait 以提供统一的验证接口
pub trait ValidateBuilder {
    /// 验证构建器状态
    ///
    /// # 返回
    /// 验证结果
    fn validate(&self) -> ValidationResult;

    /// 验证并报告错误
    ///
    /// 执行验证并记录任何错误到日志
    ///
    /// # 返回
    /// true 如果验证通过或只有警告，false 如果有严重错误
    fn validate_and_log(&self) -> bool {
        match self.validate() {
            ValidationResult::Valid => true,
            ValidationResult::Warning(msg) => {
                error!("Builder validation warning: {}", msg);
                true
            }
            ValidationResult::Invalid(msg) => {
                error!("Builder validation failed: {}", msg);
                false
            }
        }
    }
}

/// 消息内容大小限制（字节）
pub mod message_limits {
    /// 文本消息最大大小
    pub const TEXT_MESSAGE_MAX_SIZE: usize = 150 * 1024; // 150KB
    /// 富文本消息最大大小
    pub const RICH_MESSAGE_MAX_SIZE: usize = 30 * 1024; // 30KB
}

/// UUID 验证常量
pub mod uuid_limits {
    /// UUID 最大长度
    pub const MAX_LENGTH: usize = 50;
}

/// 密码验证常量
pub mod password_limits {
    /// 密码最小长度
    pub const MIN_LENGTH: usize = 8;
    /// 密码最大长度
    pub const MAX_LENGTH: usize = 128;
    /// 密码必须包含的字符类型
    pub const REQUIRE_UPPERCASE: bool = true;
    pub const REQUIRE_LOWERCASE: bool = true;
    pub const REQUIRE_DIGIT: bool = true;
    pub const REQUIRE_SPECIAL: bool = true;
}

/// 文件上传验证常量
pub mod file_limits {
    /// 文件上传最大大小 (100MB)
    pub const MAX_FILE_SIZE: usize = 100 * 1024 * 1024;
    /// IM 文件上传最大大小 (50MB)
    pub const IM_MAX_FILE_SIZE: usize = 50 * 1024 * 1024;
    /// 图片上传最大大小 (20MB)
    pub const MAX_IMAGE_SIZE: usize = 20 * 1024 * 1024;
    /// 文件名最大长度
    pub const MAX_FILENAME_LENGTH: usize = 255;
    /// 文件扩展名最大长度
    pub const MAX_EXTENSION_LENGTH: usize = 10;
    /// 允许的文件类型
    pub const ALLOWED_FILE_TYPES: &[&str] = &[
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "csv", "zip", "rar", "7z",
        "tar", "gz", "jpg", "jpeg", "png", "gif", "bmp", "svg", "mp4", "avi", "mov", "wmv", "flv",
        "mkv", "mp3", "wav", "flac", "aac", "ogg", "json", "xml", "html", "css", "js", "py", "rs",
        "go",
    ];
    /// 允许的图片类型
    pub const ALLOWED_IMAGE_TYPES: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "svg", "webp"];
}

/// 验证密码强度
///
/// # 参数
/// - `password`: 密码字符串
///
/// # 返回
/// 验证结果
pub fn validate_password_strength(password: &str) -> ValidationResult {
    // 检查长度
    if password.len() < password_limits::MIN_LENGTH {
        return ValidationResult::Invalid(format!(
            "Password must be at least {} characters long",
            password_limits::MIN_LENGTH
        ));
    }

    if password.len() > password_limits::MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "Password must not exceed {} characters",
            password_limits::MAX_LENGTH
        ));
    }

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;
    let mut has_special = false;

    for ch in password.chars() {
        if ch.is_uppercase() {
            has_uppercase = true;
        } else if ch.is_lowercase() {
            has_lowercase = true;
        } else if ch.is_ascii_digit() {
            has_digit = true;
        } else if ch.is_ascii_punctuation() || ch.is_ascii_whitespace() {
            has_special = true;
        }
    }

    let mut missing_requirements = Vec::new();

    if password_limits::REQUIRE_UPPERCASE && !has_uppercase {
        missing_requirements.push("uppercase letter");
    }

    if password_limits::REQUIRE_LOWERCASE && !has_lowercase {
        missing_requirements.push("lowercase letter");
    }

    if password_limits::REQUIRE_DIGIT && !has_digit {
        missing_requirements.push("digit");
    }

    if password_limits::REQUIRE_SPECIAL && !has_special {
        missing_requirements.push("special character");
    }

    if !missing_requirements.is_empty() {
        return ValidationResult::Invalid(format!(
            "Password is missing required character types: {}",
            missing_requirements.join(", ")
        ));
    }

    ValidationResult::Valid
}

/// 验证并规范化密码
///
/// # 参数
/// - `password`: 密码字符串
/// - `field_name`: 字段名称（用于日志）
///
/// # 返回
/// 验证后的密码字符串和验证结果
pub fn validate_and_sanitize_password(
    password: String,
    field_name: &str,
) -> (String, ValidationResult) {
    // 去除首尾空白
    let password = password.trim().to_string();

    // 验证强度
    let result = validate_password_strength(&password);

    if let ValidationResult::Invalid(msg) = &result {
        error!("{} validation failed: {}", field_name, msg);
    }

    (password, result)
}

/// 验证文件大小
///
/// # 参数
/// - `file_size`: 文件大小（字节）
/// - `max_size`: 最大允许大小（字节）
/// - `file_name`: 文件名（用于日志）
///
/// # 返回
/// 验证结果
pub fn validate_file_size(file_size: usize, max_size: usize, file_name: &str) -> ValidationResult {
    if file_size == 0 {
        return ValidationResult::Invalid("File size cannot be zero".to_string());
    }

    if file_size > max_size {
        return ValidationResult::Invalid(format!(
            "File '{}' exceeds maximum size of {} bytes (actual: {} bytes)",
            file_name, max_size, file_size
        ));
    }

    ValidationResult::Valid
}

/// 验证文件名
///
/// # 参数
/// - `file_name`: 文件名
///
/// # 返回
/// 验证结果和清理后的文件名
pub fn validate_file_name(file_name: &str) -> (String, ValidationResult) {
    let cleaned_name = file_name.trim();

    // 检查是否为空
    if cleaned_name.is_empty() {
        return (
            String::new(),
            ValidationResult::Invalid("File name cannot be empty".to_string()),
        );
    }

    // 检查长度
    if cleaned_name.len() > file_limits::MAX_FILENAME_LENGTH {
        return (
            String::new(),
            ValidationResult::Invalid(format!(
                "File name exceeds maximum length of {} characters",
                file_limits::MAX_FILENAME_LENGTH
            )),
        );
    }

    // 检查非法字符
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    for ch in cleaned_name.chars() {
        if invalid_chars.contains(&ch) {
            return (
                String::new(),
                ValidationResult::Invalid(format!(
                    "File name contains invalid character: '{}'",
                    ch
                )),
            );
        }
    }

    // 检查保留文件名
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    let upper_name = cleaned_name.to_uppercase();
    let name_without_ext = if let Some(dot_pos) = upper_name.find('.') {
        &upper_name[..dot_pos]
    } else {
        &upper_name
    };

    if reserved_names.contains(&name_without_ext) {
        return (
            String::new(),
            ValidationResult::Invalid(format!("'{}' is a reserved file name", cleaned_name)),
        );
    }

    (cleaned_name.to_string(), ValidationResult::Valid)
}

/// 验证文件扩展名
///
/// # 参数
/// - `file_name`: 文件名
/// - `allowed_types`: 允许的扩展名列表
///
/// # 返回
/// 验证结果和扩展名
pub fn validate_file_extension(
    file_name: &str,
    allowed_types: &[&str],
) -> (Option<String>, ValidationResult) {
    let (_, validation_result) = validate_file_name(file_name);
    if !validation_result.is_valid() {
        return (None, validation_result);
    }

    // 提取扩展名
    let extension = if let Some(dot_pos) = file_name.rfind('.') {
        let ext = &file_name[dot_pos + 1..];
        if ext.is_empty() {
            return (
                None,
                ValidationResult::Invalid("File extension cannot be empty".to_string()),
            );
        }

        // 检查扩展名长度
        if ext.len() > file_limits::MAX_EXTENSION_LENGTH {
            return (
                None,
                ValidationResult::Invalid(format!(
                    "File extension exceeds maximum length of {} characters",
                    file_limits::MAX_EXTENSION_LENGTH
                )),
            );
        }

        Some(ext.to_lowercase())
    } else {
        return (
            None,
            ValidationResult::Invalid("File must have an extension".to_string()),
        );
    };

    // 检查扩展名是否在允许列表中
    if let Some(ref ext) = extension {
        if !allowed_types.contains(&ext.as_str()) {
            return (
                None,
                ValidationResult::Invalid(format!(
                    "File extension '.{}' is not allowed. Allowed types: {}",
                    ext,
                    allowed_types.join(", ")
                )),
            );
        }
    }

    (extension, ValidationResult::Valid)
}

/// 验证图片文件
///
/// # 参数
/// - `file_data`: 图片文件数据
/// - `file_name`: 文件名
///
/// # 返回
/// 验证结果
pub fn validate_image_file(file_data: &[u8], file_name: &str) -> ValidationResult {
    // 验证文件大小
    let size_result = validate_file_size(file_data.len(), file_limits::MAX_IMAGE_SIZE, file_name);
    if !size_result.is_valid() {
        return size_result;
    }

    // 验证扩展名
    let (_, ext_result) = validate_file_extension(file_name, file_limits::ALLOWED_IMAGE_TYPES);
    if !ext_result.is_valid() {
        return ext_result;
    }

    // ENHANCEMENT: 可增强图片文件头验证（Magic Number 检查）
    // 通过检查文件签名防止文件扩展名伪造攻击，增强安全性
    // 可考虑使用 `infer` crate 或手动实现常见格式的文件头检查

    ValidationResult::Valid
}

/// 验证上传文件
///
/// # 参数
/// - `file_data`: 文件数据
/// - `file_name`: 文件名
/// - `is_im_upload`: 是否为IM上传（有更小的文件大小限制）
///
/// # 返回
/// 验证结果
pub fn validate_upload_file(
    file_data: &[u8],
    file_name: &str,
    is_im_upload: bool,
) -> ValidationResult {
    // 验证文件大小
    let max_size = if is_im_upload {
        file_limits::IM_MAX_FILE_SIZE
    } else {
        file_limits::MAX_FILE_SIZE
    };

    let size_result = validate_file_size(file_data.len(), max_size, file_name);
    if !size_result.is_valid() {
        return size_result;
    }

    // 验证文件名
    let (_, name_result) = validate_file_name(file_name);
    if !name_result.is_valid() {
        return name_result;
    }

    // 验证扩展名
    let (_, ext_result) = validate_file_extension(file_name, file_limits::ALLOWED_FILE_TYPES);
    if !ext_result.is_valid() {
        return ext_result;
    }

    ValidationResult::Valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_string_length() {
        // 测试正常情况
        let input = "hello".to_string();
        let result = validate_string_length(input, 10, "test_field");
        assert_eq!(result, "hello");

        // 测试截断
        let input = "hello world".to_string();
        let result = validate_string_length(input, 5, "test_field");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_validate_required() {
        // 测试有效值
        assert!(validate_required("hello", "test_field"));

        // 测试空值
        assert!(!validate_required("", "test_field"));
    }

    #[test]
    fn test_validate_content_size() {
        // 测试有效内容
        assert!(validate_content_size("hello", 10, "test_content"));

        // 测试过大内容
        assert!(!validate_content_size("hello world", 5, "test_content"));
    }

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::Valid;
        assert!(valid.is_valid());
        assert!(valid.is_strictly_valid());
        assert!(valid.error().is_none());

        let warning = ValidationResult::Warning("test warning".to_string());
        assert!(warning.is_valid());
        assert!(!warning.is_strictly_valid());
        assert_eq!(warning.error(), Some("test warning"));

        let invalid = ValidationResult::Invalid("test error".to_string());
        assert!(!invalid.is_valid());
        assert!(!invalid.is_strictly_valid());
        assert_eq!(invalid.error(), Some("test error"));
    }

    #[test]
    fn test_validate_password_strength() {
        // 测试有效密码
        let valid_password = "SecurePass123!";
        assert!(matches!(
            validate_password_strength(valid_password),
            ValidationResult::Valid
        ));

        // 测试太短的密码
        let short_password = "Short1!";
        assert!(matches!(
            validate_password_strength(short_password),
            ValidationResult::Invalid(_)
        ));

        // 测试太长的密码
        let long_password = "a".repeat(password_limits::MAX_LENGTH + 1);
        assert!(matches!(
            validate_password_strength(&long_password),
            ValidationResult::Invalid(_)
        ));

        // 测试缺少大写字母
        let no_upper = "lowercase123!";
        assert!(matches!(
            validate_password_strength(no_upper),
            ValidationResult::Invalid(_)
        ));

        // 测试缺少小写字母
        let no_lower = "UPPERCASE123!";
        assert!(matches!(
            validate_password_strength(no_lower),
            ValidationResult::Invalid(_)
        ));

        // 测试缺少数字
        let no_digit = "NoDigitsHere!";
        assert!(matches!(
            validate_password_strength(no_digit),
            ValidationResult::Invalid(_)
        ));

        // 测试缺少特殊字符
        let no_special = "NoSpecialChars123";
        assert!(matches!(
            validate_password_strength(no_special),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_and_sanitize_password() {
        // 测试正常密码
        let (password, result) =
            validate_and_sanitize_password("  GoodPass123!  ".to_string(), "test_password");
        assert_eq!(password, "GoodPass123!");
        assert!(matches!(result, ValidationResult::Valid));

        // 测试无效密码
        let (password, result) =
            validate_and_sanitize_password("  weak  ".to_string(), "test_password");
        assert_eq!(password, "weak");
        assert!(matches!(result, ValidationResult::Invalid(_)));
    }

    #[test]
    fn test_validate_file_size() {
        // 测试有效文件大小
        assert!(matches!(
            validate_file_size(1024, 2048, "test.txt"),
            ValidationResult::Valid
        ));

        // 测试零大小文件
        assert!(matches!(
            validate_file_size(0, 2048, "test.txt"),
            ValidationResult::Invalid(_)
        ));

        // 测试过大文件
        assert!(matches!(
            validate_file_size(3000, 2048, "test.txt"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_file_name() {
        // 测试有效文件名
        let (name, result) = validate_file_name("document.pdf");
        assert_eq!(name, "document.pdf");
        assert!(matches!(result, ValidationResult::Valid));

        // 测试前后空格
        let (name, result) = validate_file_name("  document.pdf  ");
        assert_eq!(name, "document.pdf");
        assert!(matches!(result, ValidationResult::Valid));

        // 测试空文件名
        let (name, result) = validate_file_name("");
        assert_eq!(name, "");
        assert!(matches!(result, ValidationResult::Invalid(_)));

        // 测试过长文件名
        let long_name = "a".repeat(file_limits::MAX_FILENAME_LENGTH + 1);
        let (name, result) = validate_file_name(&long_name);
        assert_eq!(name, "");
        assert!(matches!(result, ValidationResult::Invalid(_)));

        // 测试非法字符
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        for ch in invalid_chars {
            let (name, result) = validate_file_name(&format!("test{}.txt", ch));
            assert_eq!(name, "");
            assert!(matches!(result, ValidationResult::Invalid(_)));
        }

        // 测试保留文件名
        let reserved_names = ["CON", "PRN", "AUX", "NUL", "COM1", "LPT1"];
        for name in reserved_names {
            let (cleaned, result) = validate_file_name(name);
            assert_eq!(cleaned, "");
            assert!(matches!(result, ValidationResult::Invalid(_)));
        }
    }

    #[test]
    fn test_validate_file_extension() {
        // 测试有效扩展名
        let (ext, result) =
            validate_file_extension("document.pdf", file_limits::ALLOWED_FILE_TYPES);
        assert_eq!(ext, Some("pdf".to_string()));
        assert!(matches!(result, ValidationResult::Valid));

        // 测试无扩展名
        let (ext, result) = validate_file_extension("document", file_limits::ALLOWED_FILE_TYPES);
        assert_eq!(ext, None);
        assert!(matches!(result, ValidationResult::Invalid(_)));

        // 测试空扩展名
        let (ext, result) = validate_file_extension("document.", file_limits::ALLOWED_FILE_TYPES);
        assert_eq!(ext, None);
        assert!(matches!(result, ValidationResult::Invalid(_)));

        // 测试不允许的扩展名
        let (ext, result) =
            validate_file_extension("document.exe", file_limits::ALLOWED_FILE_TYPES);
        assert_eq!(ext, None);
        assert!(matches!(result, ValidationResult::Invalid(_)));

        // 测试大小写不敏感
        let (ext, result) =
            validate_file_extension("document.PDF", file_limits::ALLOWED_FILE_TYPES);
        assert_eq!(ext, Some("pdf".to_string()));
        assert!(matches!(result, ValidationResult::Valid));
    }

    #[test]
    fn test_validate_image_file() {
        // 测试有效图片
        let image_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG header
        assert!(matches!(
            validate_image_file(&image_data, "image.png"),
            ValidationResult::Valid
        ));

        // 测试过大图片
        let large_image_data = vec![0u8; file_limits::MAX_IMAGE_SIZE + 1];
        assert!(matches!(
            validate_image_file(&large_image_data, "large.png"),
            ValidationResult::Invalid(_)
        ));

        // 测试不允许的图片类型
        let image_data = vec![0x89, 0x50, 0x4E, 0x47];
        assert!(matches!(
            validate_image_file(&image_data, "image.tiff"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_upload_file() {
        // 测试有效文件上传
        let file_data = vec![0u8; 1024];
        assert!(matches!(
            validate_upload_file(&file_data, "document.pdf", false),
            ValidationResult::Valid
        ));

        // 测试IM文件上传（较小的限制）
        let im_file_data = vec![0u8; file_limits::IM_MAX_FILE_SIZE + 1];
        assert!(matches!(
            validate_upload_file(&im_file_data, "large.pdf", true),
            ValidationResult::Invalid(_)
        ));

        // 测试普通文件上传（较大的限制）
        let normal_file_data = vec![0u8; file_limits::MAX_FILE_SIZE + 1];
        assert!(matches!(
            validate_upload_file(&normal_file_data, "large.pdf", false),
            ValidationResult::Invalid(_)
        ));
    }
}

// ============================================================================
// 员工/人才验证常量
// ============================================================================

/// 员工/人才验证常量
pub mod employee_limits {
    /// 姓名最小长度
    pub const NAME_MIN_LENGTH: usize = 2;
    /// 姓名最大长度
    pub const NAME_MAX_LENGTH: usize = 100;
    /// 邮箱最大长度
    pub const EMAIL_MAX_LENGTH: usize = 254;
    /// 电话号码最大长度
    pub const PHONE_MAX_LENGTH: usize = 20;
    /// 电话号码最小长度
    pub const PHONE_MIN_LENGTH: usize = 7;
    /// 工作年限最小值
    pub const WORK_EXPERIENCE_MIN: u32 = 0;
    /// 工作年限最大值
    pub const WORK_EXPERIENCE_MAX: u32 = 50;
    /// 简历附件最大数量
    pub const MAX_RESUME_ATTACHMENTS: usize = 10;
    /// 人才标签最大数量
    pub const MAX_TALENT_TAGS: usize = 20;
    /// 标签最大长度
    pub const TAG_MAX_LENGTH: usize = 50;
    /// 自定义字段键最大长度
    pub const CUSTOM_FIELD_KEY_MAX_LENGTH: usize = 100;
    /// 自定义字段值最大长度
    pub const CUSTOM_FIELD_VALUE_MAX_LENGTH: usize = 1000;
    /// 期望薪资最大长度
    pub const EXPECTED_SALARY_MAX_LENGTH: usize = 100;
}

// ============================================================================
// 员工/人才验证函数
// ============================================================================

/// 验证姓名
pub fn validate_name(name: &str, field_name: &str) -> ValidationResult {
    if name.is_empty() {
        return ValidationResult::Invalid(format!("{} cannot be empty", field_name));
    }

    // 计算字符数（不是字节数）
    let char_count = name.chars().count();

    if char_count < employee_limits::NAME_MIN_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} must be at least {} characters long",
            field_name,
            employee_limits::NAME_MIN_LENGTH
        ));
    }

    if char_count > employee_limits::NAME_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} must not exceed {} characters",
            field_name,
            employee_limits::NAME_MAX_LENGTH
        ));
    }

    // 检查是否包含特殊字符（只允许字母、数字、中文、空格和常见标点）
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c.is_whitespace() || is_chinese_char(c) || "-.".contains(c))
    {
        return ValidationResult::Invalid(format!(
            "{} contains invalid characters. Only letters, numbers, Chinese characters, spaces, hyphens and periods are allowed",
            field_name
        ));
    }

    ValidationResult::Valid
}

/// 验证邮箱地址
pub fn validate_email(email: &str, field_name: &str) -> ValidationResult {
    if email.is_empty() {
        return ValidationResult::Invalid(format!("{} cannot be empty", field_name));
    }

    if email.len() > employee_limits::EMAIL_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} must not exceed {} characters",
            field_name,
            employee_limits::EMAIL_MAX_LENGTH
        ));
    }

    // 简单的邮箱格式验证
    let email_regex =
        regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(email) {
        return ValidationResult::Invalid(format!("{} must be a valid email address", field_name));
    }

    ValidationResult::Valid
}

/// 验证电话号码
pub fn validate_phone(phone: &str, field_name: &str) -> ValidationResult {
    if phone.is_empty() {
        return ValidationResult::Valid; // 电话是可选的
    }

    if phone.len() < employee_limits::PHONE_MIN_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} must be at least {} characters long (got {})",
            field_name,
            employee_limits::PHONE_MIN_LENGTH,
            phone.len()
        ));
    }

    if phone.len() > employee_limits::PHONE_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} must not exceed {} characters",
            field_name,
            employee_limits::PHONE_MAX_LENGTH
        ));
    }

    // 检查是否只包含数字、加号、空格和连字符
    if !phone
        .chars()
        .all(|c| c.is_ascii_digit() || c == '+' || c == ' ' || c == '-')
    {
        return ValidationResult::Invalid(format!(
            "{} contains invalid characters. Only numbers, +, spaces and hyphens are allowed",
            field_name
        ));
    }

    ValidationResult::Valid
}

/// 验证工作年限
pub fn validate_work_experience(years: u32, field_name: &str) -> ValidationResult {
    if years > employee_limits::WORK_EXPERIENCE_MAX {
        return ValidationResult::Invalid(format!(
            "{} must not exceed {} years",
            field_name,
            employee_limits::WORK_EXPERIENCE_MAX
        ));
    }

    ValidationResult::Valid
}

/// 验证生日（可选字段）
pub fn validate_birthday(birthday: &Option<String>, field_name: &str) -> ValidationResult {
    if let Some(bday) = birthday {
        if bday.is_empty() {
            return ValidationResult::Valid;
        }

        // 简单的日期格式验证（YYYY-MM-DD）
        let date_regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        if !date_regex.is_match(bday) {
            return ValidationResult::Invalid(format!(
                "{} must be in YYYY-MM-DD format",
                field_name
            ));
        }

        // 验证日期有效性
        if chrono::NaiveDate::parse_from_str(bday, "%Y-%m-%d").is_err() {
            return ValidationResult::Invalid(format!(
                "{} must be a valid date in YYYY-MM-DD format",
                field_name
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证期望薪资
pub fn validate_expected_salary(salary: &Option<String>, field_name: &str) -> ValidationResult {
    if let Some(sal) = salary {
        if sal.is_empty() {
            return ValidationResult::Valid;
        }

        if sal.len() > employee_limits::EXPECTED_SALARY_MAX_LENGTH {
            return ValidationResult::Invalid(format!(
                "{} must not exceed {} characters",
                field_name,
                employee_limits::EXPECTED_SALARY_MAX_LENGTH
            ));
        }

        // 检查薪资格式（例如：10-20K, 50-80万, 100万+）
        let salary_regex = regex::Regex::new(r"^(\d+(-\d+)?[Kk万]?([+-])?|面议)$").unwrap();
        if !salary_regex.is_match(sal) {
            return ValidationResult::Invalid(format!(
                "{} must be a valid salary format (e.g., 10-20K, 50-80万, 100万+)",
                field_name
            ));
        }

        // 检查薪资数值是否合理
        if let Some(captures) = salary_regex.captures(sal) {
            if let Some(num_str) = captures.get(1) {
                let num_part = num_str.as_str();
                // 提取数字部分
                let num_regex = regex::Regex::new(r"^(\d+)").unwrap();
                if let Some(num_captures) = num_regex.captures(num_part) {
                    if let Some(num_match) = num_captures.get(1) {
                        if let Ok(num) = num_match.as_str().parse::<u32>() {
                            // 检查是否是K（千）为单位
                            let is_k = sal.contains('K') || sal.contains('k');
                            // 检查是否是万为单位
                            let is_wan = sal.contains('万');

                            let actual_num = if is_k {
                                num
                            } else if is_wan {
                                num * 10 // 转换为千单位
                            } else {
                                num / 1000 // 假设没有单位的是元，转换为千元
                            };

                            // 检查薪资是否过高（月薪超过500K或年薪超过600万）
                            if actual_num > 500 {
                                return ValidationResult::Invalid(format!(
                                    "{}: salary range is unreasonably high",
                                    field_name
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    ValidationResult::Valid
}

/// 验证标签列表
pub fn validate_tags(tags: &[String], field_name: &str) -> ValidationResult {
    if tags.len() > employee_limits::MAX_TALENT_TAGS {
        return ValidationResult::Invalid(format!(
            "{} cannot have more than {} tags",
            field_name,
            employee_limits::MAX_TALENT_TAGS
        ));
    }

    for (i, tag) in tags.iter().enumerate() {
        if tag.is_empty() {
            return ValidationResult::Invalid(format!(
                "{} tag at index {} cannot be empty",
                field_name, i
            ));
        }

        if tag.len() > employee_limits::TAG_MAX_LENGTH {
            return ValidationResult::Invalid(format!(
                "{} tag '{}' exceeds maximum length of {} characters",
                field_name,
                tag,
                employee_limits::TAG_MAX_LENGTH
            ));
        }

        // 检查标签字符
        if !tag
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || is_chinese_char(c))
        {
            return ValidationResult::Invalid(format!(
                "{} tag '{}' contains invalid characters. Only letters, numbers, Chinese characters, underscores and hyphens are allowed",
                field_name,
                tag
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证自定义字段
pub fn validate_custom_fields(
    fields: &Option<std::collections::HashMap<String, serde_json::Value>>,
    field_name: &str,
) -> ValidationResult {
    if let Some(custom_fields) = fields {
        if custom_fields.len() > 50 {
            // 限制自定义字段数量
            return ValidationResult::Invalid(format!(
                "{} cannot have more than 50 custom fields",
                field_name
            ));
        }

        for (key, value) in custom_fields {
            // 验证键
            if key.is_empty() {
                return ValidationResult::Invalid(format!(
                    "{} custom field key cannot be empty",
                    field_name
                ));
            }

            if key.len() > employee_limits::CUSTOM_FIELD_KEY_MAX_LENGTH {
                return ValidationResult::Invalid(format!(
                    "{} custom field key '{}' exceeds maximum length of {} characters",
                    field_name,
                    key,
                    employee_limits::CUSTOM_FIELD_KEY_MAX_LENGTH
                ));
            }

            // 验证值
            match value {
                serde_json::Value::String(s) => {
                    if s.len() > employee_limits::CUSTOM_FIELD_VALUE_MAX_LENGTH {
                        return ValidationResult::Invalid(format!(
                            "{} custom field value for key '{}' exceeds maximum length of {} characters",
                            field_name,
                            key,
                            employee_limits::CUSTOM_FIELD_VALUE_MAX_LENGTH
                        ));
                    }
                }
                serde_json::Value::Number(n) => {
                    if !n.is_i64() && !n.is_u64() && !n.is_f64() {
                        return ValidationResult::Invalid(format!(
                            "{} custom field value for key '{}' is not a valid number",
                            field_name, key
                        ));
                    }
                }
                serde_json::Value::Bool(_) => {
                    // 布尔值总是有效的
                }
                serde_json::Value::Array(arr) => {
                    if arr.len() > 100 {
                        return ValidationResult::Invalid(format!(
                            "{} custom field array for key '{}' cannot have more than 100 items",
                            field_name, key
                        ));
                    }
                }
                serde_json::Value::Object(_) => {
                    return ValidationResult::Invalid(format!(
                        "{} custom field value for key '{}' cannot be an object",
                        field_name, key
                    ));
                }
                serde_json::Value::Null => {
                    // null 值是允许的
                }
            }
        }
    }

    ValidationResult::Valid
}

/// 验证简历附件ID列表
pub fn validate_resume_attachment_ids(
    attachment_ids: &[String],
    field_name: &str,
) -> ValidationResult {
    if attachment_ids.len() > employee_limits::MAX_RESUME_ATTACHMENTS {
        return ValidationResult::Invalid(format!(
            "{} cannot have more than {} resume attachments",
            field_name,
            employee_limits::MAX_RESUME_ATTACHMENTS
        ));
    }

    for (i, id) in attachment_ids.iter().enumerate() {
        if id.is_empty() {
            return ValidationResult::Invalid(format!(
                "{} attachment ID at index {} cannot be empty",
                field_name, i
            ));
        }

        // 检查ID格式（假设是UUID或类似格式）
        if id.len() < 10 || id.len() > 100 {
            return ValidationResult::Invalid(format!(
                "{} attachment ID at index {} has invalid length",
                field_name, i
            ));
        }
    }

    ValidationResult::Valid
}

/// 清理和验证姓名
pub fn sanitize_name(name: &str) -> String {
    // 去除首尾空格
    let trimmed = name.trim();

    // 替换多个连续空格为单个空格
    let normalized = trimmed.chars().collect::<Vec<_>>();
    let mut result = Vec::new();
    let mut prev_was_space = false;

    for c in normalized {
        if c.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(c);
            prev_was_space = false;
        }
    }

    result.into_iter().collect()
}

/// 清理和验证标签
pub fn sanitize_tags(tags: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    for tag in tags {
        let sanitized = tag
            .trim()
            .replace(['_', '-'], "_") // 统一替换为下划线
            .to_lowercase();

        if !sanitized.is_empty() && !result.contains(&sanitized) {
            result.push(sanitized);
        }
    }

    result
}

/// 验证单个标签
///
/// # 参数
/// - `tag`: 标签字符串
/// - `field_name`: 字段名称（用于错误消息）
///
/// # 返回
/// 验证结果
pub fn validate_talent_tag(tag: &str, field_name: &str) -> ValidationResult {
    if tag.is_empty() {
        return ValidationResult::Invalid(format!("{}: tag cannot be empty", field_name));
    }

    if tag.len() > 50 {
        return ValidationResult::Invalid(format!(
            "{}: tag must not exceed 50 characters (got {})",
            field_name,
            tag.len()
        ));
    }

    // 检查标签格式（允许字母、数字、下划线、连字符、空格和中文）
    for c in tag.chars() {
        if !(c.is_alphanumeric() || c == '_' || c == '-' || c == ' ' || is_chinese_char(c)) {
            return ValidationResult::Invalid(format!(
                "{}: tag contains invalid character '{}'. Only letters, numbers, spaces, hyphens, underscores and Chinese characters are allowed",
                field_name, c
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证标签列表
///
/// # 参数
/// - `tags`: 标签列表
///
/// # 返回
/// 验证结果
pub fn validate_talent_tags(tags: &[String]) -> ValidationResult {
    if tags.len() > 20 {
        return ValidationResult::Invalid(format!(
            "Invalid tags: maximum number of tags is 20 (got {})",
            tags.len()
        ));
    }

    for (index, tag) in tags.iter().enumerate() {
        match validate_talent_tag(tag, &format!("tags[{}]", index)) {
            ValidationResult::Valid => {}
            ValidationResult::Warning(msg) => {
                return ValidationResult::Warning(msg);
            }
            ValidationResult::Invalid(msg) => {
                return ValidationResult::Invalid(msg);
            }
        }
    }

    ValidationResult::Valid
}

/// 验证单个简历附件ID
///
/// # 参数
/// - `attachment_id`: 附件ID
/// - `field_name`: 字段名称（用于错误消息）
///
/// # 返回
/// 验证结果
pub fn validate_resume_attachment(attachment_id: &str, field_name: &str) -> ValidationResult {
    if attachment_id.is_empty() {
        return ValidationResult::Invalid(format!("{}: attachment ID cannot be empty", field_name));
    }

    if attachment_id.len() > 100 {
        return ValidationResult::Invalid(format!(
            "{}: attachment ID must not exceed 100 characters (got {})",
            field_name,
            attachment_id.len()
        ));
    }

    // 检查附件ID格式（通常只包含字母、数字和连字符）
    if !attachment_id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return ValidationResult::Invalid(format!(
            "{}: attachment ID can only contain letters, numbers, hyphens and underscores",
            field_name
        ));
    }

    ValidationResult::Valid
}

/// 分页验证常量
pub mod pagination_limits {
    /// 默认分页大小
    pub const DEFAULT_PAGE_SIZE: u32 = 20;
    /// 最小分页大小
    pub const MIN_PAGE_SIZE: u32 = 1;
    /// 最大分页大小
    pub const MAX_PAGE_SIZE: u32 = 500;
    /// 推荐的分页大小（性能最优）
    pub const RECOMMENDED_PAGE_SIZE: u32 = 50;
    /// 分页标记最大长度
    pub const MAX_PAGE_TOKEN_LENGTH: usize = 1024;
}

/// 验证分页大小
///
/// # 参数
/// - `page_size`: 分页大小
/// - `field_name`: 字段名称（用于错误消息）
///
/// # 返回
/// 验证结果
pub fn validate_page_size(page_size: u32, field_name: &str) -> ValidationResult {
    if page_size < pagination_limits::MIN_PAGE_SIZE {
        return ValidationResult::Invalid(format!(
            "{}: page size must be at least {}",
            field_name,
            pagination_limits::MIN_PAGE_SIZE
        ));
    }

    if page_size > pagination_limits::MAX_PAGE_SIZE {
        return ValidationResult::Invalid(format!(
            "{}: page size must not exceed {} (recommended: {})",
            field_name,
            pagination_limits::MAX_PAGE_SIZE,
            pagination_limits::RECOMMENDED_PAGE_SIZE
        ));
    }

    // 检查是否为推荐的分页大小（性能考虑）
    if page_size > pagination_limits::RECOMMENDED_PAGE_SIZE {
        log::warn!(
            "{}: page size {} is larger than recommended value {}. This may impact performance.",
            field_name,
            page_size,
            pagination_limits::RECOMMENDED_PAGE_SIZE
        );
    }

    ValidationResult::Valid
}

/// 验证分页标记
///
/// # 参数
/// - `page_token`: 分页标记
/// - `field_name`: 字段名称（用于错误消息）
///
/// # 返回
/// 验证结果
pub fn validate_page_token(page_token: &str, field_name: &str) -> ValidationResult {
    if page_token.is_empty() {
        // 空的 page_token 是有效的，表示第一页
        return ValidationResult::Valid;
    }

    if page_token.len() > pagination_limits::MAX_PAGE_TOKEN_LENGTH {
        return ValidationResult::Invalid(format!(
            "{}: page token must not exceed {} characters",
            field_name,
            pagination_limits::MAX_PAGE_TOKEN_LENGTH
        ));
    }

    // 检查 page_token 格式（通常为 base64 编码的字符串）
    if !page_token.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '/' || c == '+' || c == '=' || c == '-' || c == '_'
    }) {
        return ValidationResult::Invalid(format!(
            "{}: page token contains invalid characters. Expected base64 format",
            field_name
        ));
    }

    ValidationResult::Valid
}

/// 验证分页参数组合
///
/// # 参数
/// - `page_size`: 分页大小
/// - `page_token`: 分页标记
/// - `field_prefix`: 字段前缀（用于错误消息）
///
/// # 返回
/// 验证结果
pub fn validate_pagination_params(
    page_size: Option<u32>,
    page_token: Option<&str>,
    field_prefix: &str,
) -> ValidationResult {
    // 验证分页大小
    if let Some(size) = page_size {
        match validate_page_size(size, &format!("{}_page_size", field_prefix)) {
            ValidationResult::Valid => {}
            ValidationResult::Warning(msg) => {
                log::warn!("Pagination warning: {}", msg);
            }
            ValidationResult::Invalid(msg) => {
                return ValidationResult::Invalid(msg);
            }
        }
    }

    // 验证分页标记
    if let Some(token) = page_token {
        match validate_page_token(token, &format!("{}_page_token", field_prefix)) {
            ValidationResult::Valid => {}
            ValidationResult::Warning(msg) => {
                log::warn!("Pagination warning: {}", msg);
            }
            ValidationResult::Invalid(msg) => {
                return ValidationResult::Invalid(msg);
            }
        }
    }

    // 检查参数组合的逻辑性
    if page_token.is_some() && page_size.is_none() {
        log::warn!(
            "{}: page_token provided without page_size. Using default page size {}",
            field_prefix,
            pagination_limits::DEFAULT_PAGE_SIZE
        );
    }

    ValidationResult::Valid
}

/// 清理单个标签
///
/// # 参数
/// - `tag`: 标签字符串
///
/// # 返回
/// 清理后的标签
pub fn sanitize_tag(tag: &str) -> String {
    tag.trim()
        .replace(['_', '-'], "_") // 统一替换为下划线
        .to_lowercase()
}

/// Sheets 验证模块
pub mod sheets;

/// IM（即时消息）验证模块
pub mod im;

/// 招聘服务验证模块
pub mod hire;

/// 日历服务验证模块
pub mod calendar;
/// Drive（云文档/文件）服务验证模块
pub mod drive;

/// 分页验证模块
pub mod pagination;

/// 重新导出分页相关的公共接口
pub use pagination::{PaginatedResponse, PaginationIterator, PaginationRequestBuilder};

/// 重新导出 Sheets 验证的公共接口
pub use sheets::{
    validate_cell_range, validate_data_matrix_consistency, validate_date_time_render_option,
    validate_find_options, validate_merge_range, validate_value_render_option,
};

/// 重新导出 IM 验证的公共接口
pub use im::{
    validate_file_upload, validate_message_content, validate_message_forward,
    validate_message_reaction, validate_message_read_status, validate_message_recall,
    validate_message_receivers, validate_message_template, validate_message_type,
    validate_receiver_id, validate_uuid, ValidateImBuilder,
};

/// 重新导出招聘验证的公共接口
pub use hire::{
    validate_birthday as validate_hire_birthday, validate_candidate_basic_info,
    validate_candidate_tags, validate_education_background, validate_hiring_requirement,
    validate_hiring_status_transition, validate_interview_arrangement, validate_interview_feedback,
    validate_job_position, validate_offer_info, validate_salary_range,
    validate_work_experience as validate_hire_work_experience, ValidateHireBuilder,
};
