//! 文档服务错误处理模块

use openlark_core::prelude::*;
use thiserror::Error;

/// 文档服务专用错误类型
#[derive(Debug, Error)]
pub enum DocsError {
    /// 核心错误 - 来自openlark-core
    #[error("Core error: {0}")]
    Core(#[from] LarkAPIError),

    /// 文档处理错误
    #[error("Document processing error: {message}")]
    DocumentError { message: String, code: Option<i32> },

    /// 表格处理错误
    #[error("Spreadsheet processing error: {message}")]
    SpreadsheetError { message: String, code: Option<i32> },

    /// 文件上传错误
    #[error("File upload error: {message}")]
    FileUploadError {
        message: String,
        file_id: Option<String>,
    },

    /// 权限错误
    #[error("Permission denied: {resource}")]
    PermissionDenied { resource: String, action: String },

    /// 资源未找到
    #[error("Resource not found: {resource_type} '{id}'")]
    ResourceNotFound { resource_type: String, id: String },

    /// 配额超限
    #[error("Quota exceeded: {quota_type}")]
    QuotaExceeded {
        quota_type: String,
        limit: Option<u64>,
    },

    /// 版本不支持
    #[error("Version not supported: {version}")]
    VersionNotSupported { version: String },

    /// 请求参数错误
    #[error("Invalid parameter: {parameter} - {reason}")]
    InvalidParameter { parameter: String, reason: String },

    /// 网络错误
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON序列化错误
    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    /// 其他错误
    #[error("Other error: {message}")]
    Other { message: String },
}

impl DocsError {
    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            DocsError::Core(core_err) => {
                format!("核心错误: {}", core_err)
            }

            DocsError::DocumentError { message, .. } => {
                format!("文档处理失败: {}", message)
            }

            DocsError::SpreadsheetError { message, .. } => {
                format!("表格处理失败: {}", message)
            }

            DocsError::FileUploadError { message, .. } => {
                format!("文件上传失败: {}", message)
            }

            DocsError::PermissionDenied { resource, action } => {
                format!("权限不足，无法对{resource}执行{action}操作")
            }

            DocsError::ResourceNotFound { resource_type, id } => {
                format!("{} '{}' 不存在或已删除", resource_type, id)
            }

            DocsError::QuotaExceeded { quota_type, .. } => {
                format!("{}配额已用完，请稍后重试", quota_type)
            }

            DocsError::VersionNotSupported { version } => {
                format!("不支持的API版本: {}", version)
            }

            DocsError::InvalidParameter { parameter, reason } => {
                format!("参数错误: {} - {}", parameter, reason)
            }

            DocsError::Network(err) => {
                format!("网络连接失败: {}", err)
            }

            DocsError::JsonSerialization(err) => {
                format!("数据格式错误: {}", err)
            }

            DocsError::Other { message } => {
                format!("未知错误: {}", message)
            }
        }
    }

    /// 检查是否为可重试的错误
    pub fn is_retryable(&self) -> bool {
        match self {
            DocsError::Core(_) => true, // 简化处理
            DocsError::Network(_) => true,
            DocsError::QuotaExceeded { .. } => true,
            _ => false,
        }
    }

    /// 检查是否为认证错误
    pub fn is_auth_error(&self) -> bool {
        match self {
            DocsError::Core(_) => false, // 简化处理
            DocsError::PermissionDenied { .. } => true,
            _ => false,
        }
    }
}

/// 文档服务Result类型
pub type DocsResult<T> = Result<T, DocsError>;

// 已经通过 #[from] 属性自动实现，无需手动实现

/// 文档服务错误上下文
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// 请求ID
    pub request_id: Option<String>,
    /// 操作类型
    pub operation: String,
    /// 资源ID
    pub resource_id: Option<String>,
    /// 额外的上下文信息
    pub extra: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// 创建新的错误上下文
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            request_id: None,
            operation: operation.into(),
            resource_id: None,
            extra: std::collections::HashMap::new(),
        }
    }

    /// 设置请求ID
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// 设置资源ID
    pub fn with_resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    /// 添加额外信息
    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}
