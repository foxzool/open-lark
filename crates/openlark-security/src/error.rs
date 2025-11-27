//! 错误处理模块
//!
//! 定义安全模块的专用错误类型。

use thiserror::Error;

/// 安全模块专用错误类型
#[derive(Debug, Error)]
pub enum SecurityError {
    /// 参数错误
    #[error("参数错误: {parameter} - {reason}")]
    InvalidParameter {
        /// 参数名
        parameter: String,
        /// 错误原因
        reason: String,
    },

    /// 设备相关错误
    #[error("设备错误: {0}")]
    DeviceError(String),

    /// 访问控制相关错误
    #[error("访问控制错误: {0}")]
    AccessControlError(String),

    /// 权限相关错误
    #[error("权限错误: {0}")]
    PermissionError(String),

    /// 人脸识别相关错误
    #[error("人脸识别错误: {0}")]
    FaceRecognitionError(String),

    /// 访客管理相关错误
    #[error("访客管理错误: {0}")]
    VisitorError(String),

    /// 合规相关错误
    #[error("合规检查错误: {0}")]
    ComplianceError(String),

    /// 审计日志相关错误
    #[error("审计日志错误: {0}")]
    AuditLogError(String),

    /// 网络相关错误
    #[error("网络错误: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// 序列化错误
    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// 配置相关错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 时间相关错误
    #[error("时间错误: {0}")]
    TimeError(String),

    /// 加密相关错误
    #[error("加密错误: {0}")]
    CryptoError(String),

    /// 限流错误
    #[error("请求频率过高，请稍后重试")]
    RateLimitError,

    /// API调用错误
    #[error("API错误 - 状态码: {code}, 消息: {message}")]
    APIError {
        /// 错误码
        code: i32,
        /// 错误消息
        message: String,
    },

    /// 未知错误
    #[error("未知错误: {0}")]
    UnknownError(String),
}

impl SecurityError {
    /// 检查错误是否可重试
    pub fn is_retryable(&self) -> bool {
        match self {
            SecurityError::NetworkError(_) => true,
            SecurityError::DeviceError(msg) if msg.contains("temporary") => true,
            SecurityError::APIError {
                code: 429 | 500..599,
                ..
            } => true,
            _ => false,
        }
    }

    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            SecurityError::DeviceError(msg) if msg.contains("not_found") => {
                "设备未找到，请检查设备ID是否正确".to_string()
            }
            SecurityError::PermissionError(_) => "权限不足，无法执行此操作".to_string(),
            SecurityError::FaceRecognitionError(msg) if msg.contains("invalid") => {
                "人脸识别失败，请重新上传清晰的人脸照片".to_string()
            }
            SecurityError::VisitorError(msg) if msg.contains("expired") => {
                "访客权限已过期，请重新申请".to_string()
            }
            SecurityError::ComplianceError(msg) if msg.contains("violated") => {
                format!("设备合规检查失败: {}", msg)
            }
            SecurityError::NetworkError(_) => "网络连接失败，请检查网络设置".to_string(),
            SecurityError::SerializationError(err) => {
                format!("数据序列化失败: {}", err)
            }
            SecurityError::ConfigError(msg) => {
                format!("配置错误: {}", msg)
            }
            SecurityError::RateLimitError => "请求过于频繁，请稍后重试".to_string(),
            SecurityError::APIError { code: 401, .. } => "身份验证失败，请检查访问权限".to_string(),
            SecurityError::APIError { code: 403, .. } => "权限不足，无法访问此资源".to_string(),
            _ => {
                format!("操作失败: {}", self)
            }
        }
    }
}

/// 安全模块结果类型
pub type SecurityResult<T> = Result<T, SecurityError>;
