//! 员工信息验证模块

use super::core::{is_chinese_char, ValidationResult};

/// 员工限制常量
pub mod employee_limits {
    /// 姓名最大长度
    pub const MAX_NAME_LENGTH: usize = 50;
    /// 邮箱最大长度
    pub const MAX_EMAIL_LENGTH: usize = 100;
    /// 电话号码最大长度
    pub const MAX_PHONE_LENGTH: usize = 20;
    /// 最小工作年限
    pub const MIN_WORK_EXPERIENCE: u32 = 0;
    /// 最大工作年限
    pub const MAX_WORK_EXPERIENCE: u32 = 50;
    /// 标签最大数量
    pub const MAX_TAGS_COUNT: usize = 10;
    /// 单个标签最大长度
    pub const MAX_TAG_LENGTH: usize = 20;
    /// 自定义字段最大数量
    pub const MAX_CUSTOM_FIELDS_COUNT: usize = 20;
}

/// 验证姓名
///
/// # 参数
/// - `name`: 姓名
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_name(name: &str, field_name: &str) -> ValidationResult {
    let trimmed_name = name.trim();

    if trimmed_name.is_empty() {
        return ValidationResult::Invalid(format!("{} 不能为空", field_name));
    }

    if trimmed_name.len() > employee_limits::MAX_NAME_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} 长度不能超过 {} 个字符",
            field_name,
            employee_limits::MAX_NAME_LENGTH
        ));
    }

    // 检查是否包含有效字符（中文、英文字母、空格、点号）
    let mut has_valid_char = false;
    for ch in trimmed_name.chars() {
        if is_chinese_char(ch) || ch.is_ascii_alphabetic() || ch == ' ' || ch == '.' {
            has_valid_char = true;
        } else {
            return ValidationResult::Invalid(format!("{} 包含无效字符: '{}'", field_name, ch));
        }
    }

    if !has_valid_char {
        return ValidationResult::Invalid(format!(
            "{} 必须包含有效字符（中文、英文或空格）",
            field_name
        ));
    }

    ValidationResult::Valid
}

/// 验证邮箱地址
///
/// # 参数
/// - `email`: 邮箱地址
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_email(email: &str, field_name: &str) -> ValidationResult {
    let trimmed_email = email.trim().to_lowercase();

    if trimmed_email.is_empty() {
        return ValidationResult::Invalid(format!("{} 不能为空", field_name));
    }

    if trimmed_email.len() > employee_limits::MAX_EMAIL_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} 长度不能超过 {} 个字符",
            field_name,
            employee_limits::MAX_EMAIL_LENGTH
        ));
    }

    // 简单的邮箱格式验证
    let email_regex =
        regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    if !email_regex.is_match(&trimmed_email) {
        return ValidationResult::Invalid(format!(
            "{} 格式不正确，请输入有效的邮箱地址",
            field_name
        ));
    }

    ValidationResult::Valid
}

/// 验证电话号码
///
/// # 参数
/// - `phone`: 电话号码
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_phone(phone: &str, field_name: &str) -> ValidationResult {
    let trimmed_phone = phone.trim();

    if trimmed_phone.is_empty() {
        return ValidationResult::Invalid(format!("{} 不能为空", field_name));
    }

    if trimmed_phone.len() > employee_limits::MAX_PHONE_LENGTH {
        return ValidationResult::Invalid(format!(
            "{} 长度不能超过 {} 个字符",
            field_name,
            employee_limits::MAX_PHONE_LENGTH
        ));
    }

    // 检查是否只包含数字、加号、连字符、括号和空格
    for ch in trimmed_phone.chars() {
        if !ch.is_ascii_digit() && ch != '+' && ch != '-' && ch != '(' && ch != ')' && ch != ' ' {
            return ValidationResult::Invalid(format!("{} 包含无效字符: '{}'", field_name, ch));
        }
    }

    // 至少包含一个数字
    if !trimmed_phone.chars().any(|c| c.is_ascii_digit()) {
        return ValidationResult::Invalid(format!("{} 必须包含至少一个数字", field_name));
    }

    ValidationResult::Valid
}

/// 验证工作年限
///
/// # 参数
/// - `years`: 工作年限
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_work_experience(years: u32, field_name: &str) -> ValidationResult {
    if years > employee_limits::MAX_WORK_EXPERIENCE {
        ValidationResult::Invalid(format!(
            "{} 不能超过 {}",
            field_name,
            employee_limits::MAX_WORK_EXPERIENCE
        ))
    } else {
        ValidationResult::Valid
    }
}

/// 验证生日
///
/// # 参数
/// - `birthday`: 生日字符串（可选）
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_birthday(birthday: &Option<String>, field_name: &str) -> ValidationResult {
    match birthday {
        None => ValidationResult::Valid, // 生日是可选的
        Some(date_str) => {
            let trimmed_date = date_str.trim();
            if trimmed_date.is_empty() {
                return ValidationResult::Valid;
            }

            // 验证日期格式 YYYY-MM-DD
            let date_regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
            if !date_regex.is_match(trimmed_date) {
                return ValidationResult::Invalid(format!(
                    "{} 格式不正确，请使用 YYYY-MM-DD 格式",
                    field_name
                ));
            }

            // 验证日期是否有效
            if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(trimmed_date, "%Y-%m-%d") {
                let now = chrono::Utc::now().date_naive();
                if parsed_date > now {
                    return ValidationResult::Invalid(format!("{} 不能是未来日期", field_name));
                }

                // 检查年龄是否合理（100岁以上需要特殊处理）
                let days_diff = now.signed_duration_since(parsed_date).num_days();
                let age = (days_diff / 365) as i32;
                if age > 100 {
                    return ValidationResult::Invalid(format!(
                        "{} 年龄过大，请检查日期是否正确",
                        field_name
                    ));
                }

                ValidationResult::Valid
            } else {
                ValidationResult::Invalid(format!("{} 包含无效的日期", field_name))
            }
        }
    }
}

/// 验证期望薪资
///
/// # 参数
/// - `salary`: 期望薪资字符串（可选）
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_expected_salary(salary: &Option<String>, field_name: &str) -> ValidationResult {
    match salary {
        None => ValidationResult::Valid, // 期望薪资是可选的
        Some(salary_str) => {
            let trimmed_salary = salary_str.trim();
            if trimmed_salary.is_empty() {
                return ValidationResult::Valid;
            }

            // 验证薪资格式（数字，可能包含千位分隔符）
            let salary_regex = regex::Regex::new(r"^\d{1,3}(,\d{3})*(\.\d+)?$").unwrap();
            if !salary_regex.is_match(trimmed_salary) {
                return ValidationResult::Invalid(format!(
                    "{} 格式不正确，请输入有效的数字",
                    field_name
                ));
            }

            // 移除千位分隔符并转换为数字
            let clean_salary = trimmed_salary.replace(',', "");
            if let Ok(salary_num) = clean_salary.parse::<f64>() {
                if salary_num < 0.0 {
                    return ValidationResult::Invalid(format!("{} 不能为负数", field_name));
                }

                // 检查薪资是否在合理范围内（最大1000万）
                if salary_num > 10_000_000.0 {
                    return ValidationResult::Invalid(format!(
                        "{} 超过合理范围，请检查输入",
                        field_name
                    ));
                }

                ValidationResult::Valid
            } else {
                ValidationResult::Invalid(format!("{} 包含无效的数字格式", field_name))
            }
        }
    }
}

/// 验证标签
///
/// # 参数
/// - `tags`: 标签列表
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_tags(tags: &[String], field_name: &str) -> ValidationResult {
    if tags.len() > employee_limits::MAX_TAGS_COUNT {
        return ValidationResult::Invalid(format!(
            "{} 数量不能超过 {}",
            field_name,
            employee_limits::MAX_TAGS_COUNT
        ));
    }

    for (index, tag) in tags.iter().enumerate() {
        let trimmed_tag = tag.trim();
        if trimmed_tag.is_empty() {
            return ValidationResult::Invalid(format!(
                "{} 第 {} 个标签不能为空",
                field_name,
                index + 1
            ));
        }

        if trimmed_tag.len() > employee_limits::MAX_TAG_LENGTH {
            return ValidationResult::Invalid(format!(
                "{} 第 {} 个标签长度不能超过 {} 个字符",
                field_name,
                index + 1,
                employee_limits::MAX_TAG_LENGTH
            ));
        }

        // 检查标签是否包含有效字符
        for ch in trimmed_tag.chars() {
            if !is_chinese_char(ch) && !ch.is_alphanumeric() && ch != '_' && ch != '-' {
                return ValidationResult::Invalid(format!(
                    "{} 第 {} 个标签包含无效字符: '{}'",
                    field_name,
                    index + 1,
                    ch
                ));
            }
        }
    }

    ValidationResult::Valid
}

/// 验证自定义字段
///
/// # 参数
/// - `custom_fields`: 自定义字段映射
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_custom_fields(
    custom_fields: &std::collections::HashMap<String, String>,
    field_name: &str,
) -> ValidationResult {
    if custom_fields.len() > employee_limits::MAX_CUSTOM_FIELDS_COUNT {
        return ValidationResult::Invalid(format!(
            "{} 数量不能超过 {}",
            field_name,
            employee_limits::MAX_CUSTOM_FIELDS_COUNT
        ));
    }

    for (key, value) in custom_fields {
        if key.trim().is_empty() {
            return ValidationResult::Invalid(format!("{} 包含空的字段名", field_name));
        }

        if key.len() > 50 {
            return ValidationResult::Invalid(format!(
                "{} 字段名 '{}' 长度不能超过 50 个字符",
                field_name, key
            ));
        }

        if value.len() > 200 {
            return ValidationResult::Invalid(format!(
                "{} 字段 '{}' 值的长度不能超过 200 个字符",
                field_name, key
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证简历附件ID
///
/// # 参数
/// - `attachment_ids`: 附件ID列表
/// - `field_name`: 字段名称
///
/// # 返回
/// 验证结果
pub fn validate_resume_attachment_ids(
    attachment_ids: &[String],
    field_name: &str,
) -> ValidationResult {
    if attachment_ids.is_empty() {
        return ValidationResult::Valid; // 附件是可选的
    }

    if attachment_ids.len() > 5 {
        return ValidationResult::Invalid(format!("{} 数量不能超过 5 个", field_name));
    }

    // 验证ID格式（字母数字和连字符）
    let id_regex = regex::Regex::new(r"^[a-zA-Z0-9-_]+$").unwrap();

    for (index, id) in attachment_ids.iter().enumerate() {
        let trimmed_id = id.trim();
        if trimmed_id.is_empty() {
            return ValidationResult::Invalid(format!(
                "{} 第 {} 个附件ID不能为空",
                field_name,
                index + 1
            ));
        }

        if !id_regex.is_match(trimmed_id) {
            return ValidationResult::Invalid(format!(
                "{} 第 {} 个附件ID格式不正确",
                field_name,
                index + 1
            ));
        }
    }

    ValidationResult::Valid
}

/// 清理姓名
///
/// # 参数
/// - `name`: 原始姓名
///
/// # 返回
/// 清理后的姓名
pub fn sanitize_name(name: &str) -> String {
    name.trim()
        .chars()
        .filter(|c| is_chinese_char(*c) || c.is_ascii_alphabetic() || *c == ' ' || *c == '.')
        .collect::<String>()
}

/// 清理标签
///
/// # 参数
/// - `tags`: 原始标签列表
///
/// # 返回
/// 清理后的标签列表
pub fn sanitize_tags(tags: &[String]) -> Vec<String> {
    tags.iter()
        .map(|tag| {
            tag.trim()
                .chars()
                .filter(|c| is_chinese_char(*c) || c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect::<String>()
        })
        .filter(|tag| !tag.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name() {
        // 有效姓名
        assert!(validate_name("张三", "姓名").is_valid());
        assert!(validate_name("John Smith", "姓名").is_valid());
        assert!(validate_name("张 三", "姓名").is_valid());
        assert!(validate_name("John Jr.", "姓名").is_valid());

        // 无效姓名
        assert!(!validate_name("", "姓名").is_valid());
        assert!(!validate_name("  ", "姓名").is_valid());
        assert!(!validate_name("张三123", "姓名").is_valid());
        assert!(!validate_name("张三@", "姓名").is_valid());
    }

    #[test]
    fn test_validate_email() {
        // 有效邮箱
        assert!(validate_email("user@example.com", "邮箱").is_valid());
        assert!(validate_email("user.name+tag@example.co.uk", "邮箱").is_valid());

        // 无效邮箱
        assert!(!validate_email("", "邮箱").is_valid());
        assert!(!validate_email("invalid-email", "邮箱").is_valid());
        assert!(!validate_email("@example.com", "邮箱").is_valid());
        assert!(!validate_email("user@", "邮箱").is_valid());
    }

    #[test]
    fn test_validate_phone() {
        // 有效电话
        assert!(validate_phone("13812345678", "电话").is_valid());
        assert!(validate_phone("+86 138 1234 5678", "电话").is_valid());
        assert!(validate_phone("(021) 1234-5678", "电话").is_valid());

        // 无效电话
        assert!(!validate_phone("", "电话").is_valid());
        assert!(!validate_phone("abc", "电话").is_valid());
        assert!(!validate_phone("13812345678@", "电话").is_valid());
    }

    #[test]
    fn test_validate_work_experience() {
        // 有效工作年限
        assert!(validate_work_experience(0, "工作年限").is_valid());
        assert!(validate_work_experience(5, "工作年限").is_valid());
        assert!(validate_work_experience(30, "工作年限").is_valid());

        // 无效工作年限
        assert!(!validate_work_experience(51, "工作年限").is_valid());
    }

    #[test]
    fn test_validate_tags() {
        // 有效标签
        let tags = vec!["技术".to_string(), "管理".to_string(), "Python".to_string()];
        assert!(validate_tags(&tags, "标签").is_valid());

        // 无效标签
        let invalid_tags = vec!["技术".to_string(), "标签@".to_string()];
        assert!(!validate_tags(&invalid_tags, "标签").is_valid());

        // 标签数量过多
        let too_many_tags: Vec<String> = (0..15).map(|i| format!("标签{}", i)).collect();
        assert!(!validate_tags(&too_many_tags, "标签").is_valid());
    }

    #[test]
    fn test_sanitize_name() {
        assert_eq!(sanitize_name("  张三  "), "张三");
        assert_eq!(sanitize_name("John Smith Jr."), "John Smith Jr.");
        assert_eq!(sanitize_name("张三@123"), "张三");
    }

    #[test]
    fn test_sanitize_tags() {
        let tags = vec![
            "  技术  ".to_string(),
            "管理@".to_string(),
            "Python-123".to_string(),
            "".to_string(),
        ];
        let sanitized = sanitize_tags(&tags);
        // @ 符号被过滤，所以 "管理@" 变成 "管理"
        assert_eq!(sanitized, vec!["技术", "管理", "Python-123"]);
    }

    #[test]
    fn test_employee_limits() {
        assert!(employee_limits::MAX_NAME_LENGTH > 0);
        assert!(employee_limits::MAX_EMAIL_LENGTH > 0);
        assert!(employee_limits::MAX_PHONE_LENGTH > 0);
        assert!(employee_limits::MAX_TAGS_COUNT > 0);
        assert!(employee_limits::MAX_CUSTOM_FIELDS_COUNT > 0);
    }
}
