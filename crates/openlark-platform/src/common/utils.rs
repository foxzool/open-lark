//! 平台服务工具函数

/// 验证应用 ID 格式
///
/// # 参数
///
/// * `app_id` - 应用 ID
///
/// # 返回
///
/// 返回验证结果
pub fn validate_app_id(app_id: &str) -> Result<(), String> {
    if app_id.is_empty() {
        return Err("应用 ID 不能为空".to_string());
    }

    if app_id.len() > 64 {
        return Err("应用 ID 长度不能超过 64 个字符".to_string());
    }

    Ok(())
}

/// 验证应用名称格式
///
/// # 参数
///
/// * `name` - 应用名称
///
/// # 返回
///
/// 返回验证结果
pub fn validate_app_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("应用名称不能为空".to_string());
    }

    if name.len() > 100 {
        return Err("应用名称长度不能超过 100 个字符".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_app_id() {
        // 有效应用 ID
        assert!(validate_app_id("app_123").is_ok());
        assert!(validate_app_id("cli_a1b2c3d4").is_ok());

        // 无效应用 ID
        assert!(validate_app_id("").is_err());
        assert!(validate_app_id("a".repeat(65).as_str()).is_err());
    }

    #[test]
    fn test_validate_app_name() {
        // 有效应用名称
        assert!(validate_app_name("我的应用").is_ok());
        assert!(validate_app_name("Test App").is_ok());

        // 无效应用名称
        assert!(validate_app_name("").is_err());
        assert!(validate_app_name("a".repeat(101).as_str()).is_err());
    }
}
