/// API 错误类型定义
///
/// 定义多维表格 API 的特定错误类型，提供更好的类型安全和错误信息。
use thiserror::Error;

/// 多维表格错误类型
#[derive(Debug, thiserror::Error)]
pub enum BitableError {
    /// 应用不存在
    #[error("应用不存在: {0}")]
    AppNotFound(String),

    /// 数据表不存在
    #[error("数据表不存在: {0}")]
    TableNotFound(String),

    /// 记录不存在
    #[error("记录不存在: {0}")]
    RecordNotFound(String),

    /// 字段不存在
    #[error("字段不存在: {0}")]
    FieldNotFound(String),

    /// 字段类型不匹配
    #[error("字段类型不匹配: {0}")]
    FieldTypeMismatch {
        field_name: String,
        expected_type: String,
        actual_type: String,
    },

    /// 字段值验证失败
    #[error("字段值验证失败: {0}: {1}")]
    FieldValidationError { field_name: String, message: String },

    /// 权限不足
    #[error("权限不足: 无权执行此操作")]
    PermissionDenied,

    /// 频率限制
    #[error("请求过于频繁，每秒最多 {0} 次请求")]
    RateLimitExceeded(i32),

    /// 数据一致性问题
    #[error("数据一致性问题: {0}")]
    DataConsistencyError(String),
}

impl BitableError {
    /// 获取错误代码
    pub fn code(&self) -> u32 {
        match self {
            BitableError::AppNotFound(_) => 10100,
            BitableError::TableNotFound(_) => 10101,
            BitableError::RecordNotFound(_) => 10102,
            BitableError::FieldNotFound(_) => 10103,
            BitableError::FieldTypeMismatch { .. } => 10104,
            BitableError::FieldValidationError { .. } => 10105,
            BitableError::PermissionDenied => 10106,
            BitableError::RateLimitExceeded(_) => 10107,
            BitableError::DataConsistencyError(_) => 10108,
        }
    }

    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            BitableError::AppNotFound(app_token) => format!(
                "应用不存在 (app_token: {}), \
                请检查应用 token 是否正确，或者是否有访问该应用的权限。",
                app_token
            ),
            BitableError::TableNotFound(table_id) => format!(
                "数据表不存在 (table_id: {}), \
                请检查数据表 ID 是否正确，或者该数据表是否已被删除。",
                table_id
            ),
            BitableError::RecordNotFound(record_id) => format!(
                "记录不存在 (record_id: {}), \
                请检查记录 ID 是否正确，或者该记录是否已被删除。",
                record_id
            ),
            BitableError::FieldNotFound(field_name) => format!(
                "字段不存在 (field_name: {}), \
                请检查字段名是否正确。",
                field_name
            ),
            BitableError::FieldTypeMismatch {
                field_name,
                expected_type,
                actual_type,
            } => format!(
                "字段类型不匹配 (field_name: {}), \
                预期类型: {expected_type}, 实际类型: {actual_type}",
                field_name
            ),
            Bitable::FieldValidationError {
                field_name,
                message,
            } => format!("字段值验证失败 (field_name: {}): {message}", field_name),
            BitableError::PermissionDenied => "权限不足，无权执行此操作".to_string(),
            Bitable::Error::RateLimitExceeded(limit) => {
                format!("请求过于频繁，每秒最多 {} 次请求，请稍后重试。", limit)
            }
            Bitable::Error::DataConsistencyError(msg) => {
                format!("数据一致性问题: {msg}", msg)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(BitableError::AppNotFound("test".to_string()).code(), 10100);
        assert_eq!(
            BitableError::TableNotFound("test".to_string()).code(),
            10101
        );
        assert_eq!(Bitable::RateLimitExceeded(50).code(), 10107);

        let msg = BitableError::FieldValidationError {
            field_name: "field".to_string(),
            message: "test error".to_string(),
        };
        assert!(msg.user_friendly_message().contains("field"));
        assert!(msg.code() == 10105);
    }

    #[test]
    fn test_user_friendly_message() {
        let err = BitableError::PermissionDenied;
        assert_eq!(err.code(), 10106);
        assert!(err.user_friendly_message() == "权限不足，无权执行此操作");

        let err = BitableError::RecordNotFound("rec_123".to_string());
        assert!(err.user_friendly_message().contains("rec_123"));
        assert!(err.user_friendly_message().contains("请检查"));
    }
}
