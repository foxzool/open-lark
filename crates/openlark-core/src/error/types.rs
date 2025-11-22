//! Error Types and Codes Module
//!
//! 统一的错误类型定义和错误码系统，整合了LarkAPIError和LarkErrorCode。
//! 提供完整的错误分类、处理建议和用户友好的错误消息。
//!
//! # 错误类型分类
//!
//! - **网络错误**: RequestError, IOErr, UrlParseError
//! - **数据错误**: DeserializeError, DataError
//! - **参数错误**: IllegalParamError, BadRequest
//! - **API错误**: ApiError, APIError
//! - **认证错误**: MissingAccessToken, AuthenticationError
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_core::error::prelude::*;
//!
//! fn handle_error(error: &LarkAPIError) {
//!     match error {
//!         LarkAPIError::MissingAccessToken => {
//!             println!("请检查应用凭据配置");
//!         }
//!         LarkAPIError::ApiError { code, message, .. } if *code == 403 => {
//!             println!("权限不足: {}", message);
//!         }
//!         err if err.is_retryable() => {
//!             println!("网络错误，可以重试: {}", err.user_friendly_message());
//!         }
//!         _ => {
//!             println!("操作失败: {}", error.user_friendly_message());
//!         }
//!     }
//! }
//! ```

use std::fmt::{Display, Formatter};
use thiserror::Error;

/// 主要的错误类型枚举
///
/// 包含所有可能的API调用错误，提供详细的错误信息和处理建议。
/// 支持错误分类、重试判断和用户友好的错误消息。
#[derive(Error, Debug, Clone)]
pub enum LarkAPIError {
    /// 输入输出错误
    ///
    /// 通常由文件操作、网络IO等底层操作失败引起。
    #[error("IO error: {0}")]
    IOErr(String),

    /// 非法参数错误
    ///
    /// 当传入的参数不符合API要求时抛出，如无效的ID格式、超出范围的值等。
    #[error("Invalid parameter: {0}")]
    IllegalParamError(String),

    /// JSON反序列化错误
    ///
    /// 当API响应的JSON格式无法解析为预期的数据结构时发生。
    #[error("JSON deserialization error: {0}")]
    DeserializeError(String),

    /// HTTP请求失败
    ///
    /// 网络请求层面的错误，如连接超时、DNS解析失败等。通常可以重试。
    #[error("HTTP request failed: {0}")]
    RequestError(String),

    /// URL解析错误
    ///
    /// 当构建的API请求URL格式不正确时发生。
    #[error("URL parse error: {0}")]
    UrlParseError(String),

    /// 增强的API错误
    ///
    /// 包含错误码、消息和请求ID的完整错误信息，便于调试和问题追踪。
    #[error("API error: {message} (code: {code}, request_id: {request_id:?})")]
    ApiError {
        /// API错误码
        code: i32,
        /// 错误消息
        message: String,
        /// 请求ID，用于问题追踪
        request_id: Option<String>,
    },

    /// 缺少访问令牌
    ///
    /// 当API调用需要认证但未提供有效的访问令牌时发生。
    #[error("Missing access token")]
    MissingAccessToken,

    /// 认证错误
    ///
    /// 包含详细认证信息的错误类型。
    #[error("Authentication error: {message}")]
    AuthenticationError {
        /// 错误消息
        message: String,
        /// 详细信息
        details: Option<String>,
    },

    /// 错误的请求
    ///
    /// 请求格式或内容不符合API规范。
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// 数据处理错误
    ///
    /// 数据验证、转换或处理过程中发生的错误。
    #[error("Data error: {0}")]
    DataError(String),

    /// 标准API响应错误
    ///
    /// 飞书开放平台返回的标准错误响应，包含完整的错误信息。
    #[error("API error: {msg} (code: {code})")]
    APIError {
        /// API错误码
        code: i32,
        /// 错误消息
        msg: String,
        /// 详细错误信息
        error: Option<String>,
    },

    /// 验证错误
    ///
    /// 数据验证失败时产生的错误。
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// 网络错误
    ///
    /// 专门用于网络相关错误的统一类型。
    #[error("Network error: {message}")]
    NetworkError {
        /// 错误消息
        message: String,
        /// 网络错误类型
        kind: NetworkErrorKind,
    },

    /// 权限错误
    ///
    /// 专门用于权限相关错误的统一类型。
    #[error("Permission denied: {message}")]
    PermissionError {
        /// 错误消息
        message: String,
        /// 权限类型
        permission_type: PermissionType,
    },
}

/// 网络错误类型分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkErrorKind {
    /// 连接超时
    Timeout,
    /// 连接被拒绝
    ConnectionRefused,
    /// DNS解析失败
    DnsResolutionFailed,
    /// SSL/TLS错误
    SslError,
    /// 其他网络错误
    Other,
}

/// 权限类型分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionType {
    /// 应用权限不足
    Application,
    /// 用户权限不足
    User,
    /// 租户权限不足
    Tenant,
    /// 文档权限不足
    Document,
    /// 其他权限错误
    Other,
}

/// 错误处理分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorHandlingCategory {
    /// 可重试的错误
    Retryable,
    /// 权限相关错误
    Permission,
    /// 认证相关错误
    Authentication,
    /// 参数错误
    Parameter,
    /// 系统错误
    System,
    /// 业务逻辑错误
    Business,
    /// 服务器错误
    Server,
    /// 网络错误
    Network,
    /// 限流错误
    RateLimit,
    /// 未知错误
    Unknown,
}

/// 飞书API错误码的语义化枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LarkErrorCode {
    /// 成功
    Success = 0,

    // 认证相关错误
    /// 应用票据无效
    AppTicketInvalid = 10012,
    /// 访问令牌无效
    AccessTokenInvalid = 99991671,
    /// 应用访问令牌无效
    AppAccessTokenInvalid = 99991664,
    /// 租户访问令牌无效
    TenantAccessTokenInvalid = 99991663,

    // HTTP标准错误码
    /// 请求参数错误
    BadRequest = 400,
    /// 认证失败
    Unauthorized = 401,
    /// 权限不足
    Forbidden = 403,
    /// 资源不存在
    NotFound = 404,
    /// 方法不允许
    MethodNotAllowed = 405,
    /// 请求冲突
    Conflict = 409,
    /// 请求频率过高
    TooManyRequests = 429,

    // 服务器错误
    /// 内部服务器错误
    InternalServerError = 500,
    /// 网关错误
    BadGateway = 502,
    /// 服务不可用
    ServiceUnavailable = 503,
    /// 网关超时
    GatewayTimeout = 504,

    // 飞书业务错误码
    /// 应用未安装
    AppNotInstalled = 10003,
    /// 应用状态异常
    AppStatusException = 10013,
    /// 应用权限不足
    AppPermissionDenied = 19001,
    /// 用户不存在
    UserNotFound = 60001,
    /// 用户状态异常
    UserStatusException = 60002,
    /// 部门不存在
    DepartmentNotFound = 60003,
    /// 群组不存在
    ChatNotFound = 70001,
    /// 群组类型不支持
    ChatTypeNotSupported = 70002,
    /// 消息不存在
    MessageNotFound = 80001,
    /// 消息类型不支持
    MessageTypeNotSupported = 80002,
    /// 文件不存在
    FileNotFound = 90001,
    /// 文件大小超限
    FileSizeExceeded = 90002,
    /// 文件类型不支持
    FileTypeNotSupported = 90003,

    // 日历相关错误
    /// 日历不存在
    CalendarNotFound = 110001,
    /// 日程不存在
    EventNotFound = 110002,
    /// 日程冲突
    EventConflict = 110003,

    // 云文档相关错误
    /// 文档不存在
    DocumentNotFound = 120001,
    /// 文档权限不足
    DocumentPermissionDenied = 120002,
    /// 文档已锁定
    DocumentLocked = 120003,
    /// 工作表不存在
    SheetNotFound = 120011,
    /// 表格不存在
    TableNotFound = 120021,

    // 应用商店相关错误
    /// 应用未发布
    AppNotPublished = 130001,
    /// 应用版本不兼容
    AppVersionIncompatible = 130002,

    // 网络和传输错误
    /// 网络连接超时
    NetworkTimeout = 999001,
    /// 网络连接失败
    NetworkConnectionFailed = 999002,
    /// DNS解析失败
    DnsResolutionFailed = 999003,
    /// SSL证书错误
    SslCertificateError = 999004,
}

/// 错误码分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// 认证错误
    Authentication,
    /// 权限错误
    Permission,
    /// 参数错误
    Parameter,
    /// 资源错误
    Resource,
    /// 服务器错误
    Server,
    /// 网络错误
    Network,
    /// 限流错误
    RateLimit,
    /// 其他错误
    Other,
}

/// 错误严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorSeverity {
    /// 信息级别 - 一般性提示信息
    Info,
    /// 警告级别 - 可能的问题但不影响核心功能
    Warning,
    /// 错误级别 - 操作失败但系统可恢复
    Error,
    /// 严重错误级别 - 可能影响系统稳定性
    Critical,
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "Info"),
            ErrorSeverity::Warning => write!(f, "Warning"),
            ErrorSeverity::Error => write!(f, "Error"),
            ErrorSeverity::Critical => write!(f, "Critical"),
        }
    }
}

// ============================================================================
// 类型转换特征实现
// ============================================================================

impl From<std::io::Error> for LarkAPIError {
    fn from(err: std::io::Error) -> Self {
        Self::IOErr(err.to_string())
    }
}

impl From<serde_json::Error> for LarkAPIError {
    fn from(err: serde_json::Error) -> Self {
        Self::DeserializeError(err.to_string())
    }
}

impl From<reqwest::Error> for LarkAPIError {
    fn from(err: reqwest::Error) -> Self {
        let message = err.to_string();
        let kind = if message.contains("timeout") {
            NetworkErrorKind::Timeout
        } else if message.contains("connect") {
            NetworkErrorKind::ConnectionRefused
        } else if message.contains("dns") {
            NetworkErrorKind::DnsResolutionFailed
        } else if message.contains("ssl") || message.contains("tls") {
            NetworkErrorKind::SslError
        } else {
            NetworkErrorKind::Other
        };

        Self::NetworkError { message, kind }
    }
}

impl From<url::ParseError> for LarkAPIError {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParseError(err.to_string())
    }
}

// ============================================================================
// LarkErrorCode 实现
// ============================================================================

impl LarkErrorCode {
    /// 从错误码数值创建枚举
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::Success),
            10003 => Some(Self::AppNotInstalled),
            10012 => Some(Self::AppTicketInvalid),
            10013 => Some(Self::AppStatusException),
            19001 => Some(Self::AppPermissionDenied),
            99991671 => Some(Self::AccessTokenInvalid),
            99991664 => Some(Self::AppAccessTokenInvalid),
            99991663 => Some(Self::TenantAccessTokenInvalid),
            400 => Some(Self::BadRequest),
            401 => Some(Self::Unauthorized),
            403 => Some(Self::Forbidden),
            404 => Some(Self::NotFound),
            405 => Some(Self::MethodNotAllowed),
            409 => Some(Self::Conflict),
            429 => Some(Self::TooManyRequests),
            500 => Some(Self::InternalServerError),
            502 => Some(Self::BadGateway),
            503 => Some(Self::ServiceUnavailable),
            504 => Some(Self::GatewayTimeout),
            60001 => Some(Self::UserNotFound),
            60002 => Some(Self::UserStatusException),
            60003 => Some(Self::DepartmentNotFound),
            70001 => Some(Self::ChatNotFound),
            70002 => Some(Self::ChatTypeNotSupported),
            80001 => Some(Self::MessageNotFound),
            80002 => Some(Self::MessageTypeNotSupported),
            90001 => Some(Self::FileNotFound),
            90002 => Some(Self::FileSizeExceeded),
            90003 => Some(Self::FileTypeNotSupported),
            110001 => Some(Self::CalendarNotFound),
            110002 => Some(Self::EventNotFound),
            110003 => Some(Self::EventConflict),
            120001 => Some(Self::DocumentNotFound),
            120002 => Some(Self::DocumentPermissionDenied),
            120003 => Some(Self::DocumentLocked),
            120011 => Some(Self::SheetNotFound),
            120021 => Some(Self::TableNotFound),
            130001 => Some(Self::AppNotPublished),
            130002 => Some(Self::AppVersionIncompatible),
            999001 => Some(Self::NetworkTimeout),
            999002 => Some(Self::NetworkConnectionFailed),
            999003 => Some(Self::DnsResolutionFailed),
            999004 => Some(Self::SslCertificateError),
            _ => None,
        }
    }

    /// 获取错误码的中文描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Success => "操作成功",
            Self::AppNotInstalled => "应用未安装",
            Self::AppTicketInvalid => "应用票据无效",
            Self::AppStatusException => "应用状态异常",
            Self::AppPermissionDenied => "应用权限不足",
            Self::AccessTokenInvalid => "访问令牌无效",
            Self::AppAccessTokenInvalid => "应用访问令牌无效",
            Self::TenantAccessTokenInvalid => "租户访问令牌无效",
            Self::BadRequest => "请求参数错误",
            Self::Unauthorized => "认证失败",
            Self::Forbidden => "权限不足",
            Self::NotFound => "资源不存在",
            Self::MethodNotAllowed => "请求方法不允许",
            Self::Conflict => "请求冲突",
            Self::TooManyRequests => "请求频率过高",
            Self::InternalServerError => "内部服务器错误",
            Self::BadGateway => "网关错误",
            Self::ServiceUnavailable => "服务不可用",
            Self::GatewayTimeout => "网关超时",
            Self::UserNotFound => "用户不存在",
            Self::UserStatusException => "用户状态异常",
            Self::DepartmentNotFound => "部门不存在",
            Self::ChatNotFound => "群组不存在",
            Self::ChatTypeNotSupported => "群组类型不支持",
            Self::MessageNotFound => "消息不存在",
            Self::MessageTypeNotSupported => "消息类型不支持",
            Self::FileNotFound => "文件不存在",
            Self::FileSizeExceeded => "文件大小超限",
            Self::FileTypeNotSupported => "文件类型不支持",
            Self::CalendarNotFound => "日历不存在",
            Self::EventNotFound => "日程不存在",
            Self::EventConflict => "日程冲突",
            Self::DocumentNotFound => "文档不存在",
            Self::DocumentPermissionDenied => "文档权限不足",
            Self::DocumentLocked => "文档已锁定",
            Self::SheetNotFound => "工作表不存在",
            Self::TableNotFound => "表格不存在",
            Self::AppNotPublished => "应用未发布",
            Self::AppVersionIncompatible => "应用版本不兼容",
            Self::NetworkTimeout => "网络连接超时",
            Self::NetworkConnectionFailed => "网络连接失败",
            Self::DnsResolutionFailed => "DNS解析失败",
            Self::SslCertificateError => "SSL证书错误",
        }
    }

    /// 获取详细的错误说明
    pub fn detailed_description(&self) -> &'static str {
        match self {
            Self::Success => "请求已成功处理",
            Self::AppNotInstalled => "应用未安装到当前企业，请先在飞书管理后台安装应用",
            Self::AppTicketInvalid => "应用票据已失效，SDK会自动重新申请",
            Self::AppStatusException => "应用状态异常，请检查应用是否正常启用",
            Self::AppPermissionDenied => "应用缺少执行此操作的权限，请在开发者后台配置相应权限",
            Self::AccessTokenInvalid => "用户访问令牌已过期，需要重新获取用户授权",
            Self::AppAccessTokenInvalid => "应用访问令牌无效，请检查应用ID和密钥配置",
            Self::TenantAccessTokenInvalid => "租户访问令牌无效，请检查应用是否正确安装到企业",
            Self::BadRequest => "请求参数格式错误或缺少必需参数",
            Self::Unauthorized => "身份认证失败，请检查访问令牌",
            Self::Forbidden => "当前用户或应用缺少执行此操作的权限",
            Self::NotFound => "请求的资源不存在或已被删除",
            Self::MethodNotAllowed => "当前API不支持此HTTP方法",
            Self::Conflict => "请求与当前资源状态冲突",
            Self::TooManyRequests => "请求频率超过限制，请降低请求频率",
            Self::InternalServerError => "服务器内部错误，请稍后重试",
            Self::BadGateway => "网关错误，请检查网络连接或稍后重试",
            Self::ServiceUnavailable => "服务暂时不可用，请稍后重试",
            Self::GatewayTimeout => "网关超时，请稍后重试",
            Self::UserNotFound => "指定的用户不存在，请检查用户ID是否正确",
            Self::UserStatusException => "用户状态异常，可能已被禁用或删除",
            Self::DepartmentNotFound => "指定的部门不存在，请检查部门ID是否正确",
            Self::ChatNotFound => "指定的群组不存在或机器人未加入该群组",
            Self::ChatTypeNotSupported => "当前群组类型不支持此操作",
            Self::MessageNotFound => "指定的消息不存在或已被删除",
            Self::MessageTypeNotSupported => "不支持的消息类型",
            Self::FileNotFound => "指定的文件不存在或已被删除",
            Self::FileSizeExceeded => "文件大小超出限制，请压缩后重试",
            Self::FileTypeNotSupported => "不支持的文件类型",
            Self::CalendarNotFound => "指定的日历不存在",
            Self::EventNotFound => "指定的日程不存在或已被删除",
            Self::EventConflict => "日程时间冲突，请选择其他时间",
            Self::DocumentNotFound => "指定的文档不存在或已被删除",
            Self::DocumentPermissionDenied => "文档权限不足，请联系文档所有者授权",
            Self::DocumentLocked => "文档已被其他用户锁定，请稍后再试",
            Self::SheetNotFound => "指定的工作表不存在",
            Self::TableNotFound => "指定的表格不存在",
            Self::AppNotPublished => "应用尚未发布到应用商店",
            Self::AppVersionIncompatible => "应用版本不兼容，请更新到最新版本",
            Self::NetworkTimeout => "网络连接超时，请检查网络设置",
            Self::NetworkConnectionFailed => "网络连接失败，请检查网络设置",
            Self::DnsResolutionFailed => "DNS解析失败，请检查域名设置",
            Self::SslCertificateError => "SSL证书验证失败，请检查证书配置",
        }
    }

    /// 检查是否为认证相关错误
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            Self::AppTicketInvalid
                | Self::AccessTokenInvalid
                | Self::AppAccessTokenInvalid
                | Self::TenantAccessTokenInvalid
                | Self::Unauthorized
        )
    }

    /// 检查是否为权限相关错误
    pub fn is_permission_error(&self) -> bool {
        matches!(
            self,
            Self::Forbidden | Self::AppPermissionDenied | Self::DocumentPermissionDenied
        )
    }

    /// 检查是否为客户端错误
    pub fn is_client_error(&self) -> bool {
        let code = *self as i32;
        (400..=499).contains(&code) && code != 429
    }

    /// 检查是否为服务器错误
    pub fn is_server_error(&self) -> bool {
        let code = *self as i32;
        (500..=599).contains(&code)
    }

    /// 检查是否可以重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::TooManyRequests
                | Self::InternalServerError
                | Self::BadGateway
                | Self::ServiceUnavailable
                | Self::GatewayTimeout
                | Self::NetworkTimeout
                | Self::NetworkConnectionFailed
                | Self::DnsResolutionFailed
        )
    }

    /// 获取建议的重试延迟时间（秒）
    pub fn suggested_retry_delay(&self) -> Option<u64> {
        match self {
            Self::TooManyRequests => Some(60),
            Self::InternalServerError => Some(5),
            Self::BadGateway => Some(3),
            Self::ServiceUnavailable => Some(10),
            Self::GatewayTimeout => Some(5),
            Self::NetworkTimeout => Some(3),
            Self::NetworkConnectionFailed => Some(5),
            Self::DnsResolutionFailed => Some(10),
            _ => None,
        }
    }

    /// 获取错误分类
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::AppTicketInvalid
            | Self::AccessTokenInvalid
            | Self::AppAccessTokenInvalid
            | Self::TenantAccessTokenInvalid
            | Self::Unauthorized => ErrorCategory::Authentication,

            Self::Forbidden | Self::AppPermissionDenied | Self::DocumentPermissionDenied => {
                ErrorCategory::Permission
            }

            Self::BadRequest
            | Self::MethodNotAllowed
            | Self::FileSizeExceeded
            | Self::FileTypeNotSupported
            | Self::MessageTypeNotSupported
            | Self::ChatTypeNotSupported
            | Self::EventConflict
            | Self::AppVersionIncompatible => ErrorCategory::Parameter,

            Self::NotFound
            | Self::UserNotFound
            | Self::DepartmentNotFound
            | Self::ChatNotFound
            | Self::MessageNotFound
            | Self::FileNotFound
            | Self::CalendarNotFound
            | Self::EventNotFound
            | Self::DocumentNotFound
            | Self::SheetNotFound
            | Self::TableNotFound
            | Self::AppNotInstalled
            | Self::AppNotPublished
            | Self::Conflict => ErrorCategory::Resource,

            Self::TooManyRequests => ErrorCategory::RateLimit,

            Self::InternalServerError
            | Self::BadGateway
            | Self::ServiceUnavailable
            | Self::AppStatusException
            | Self::UserStatusException
            | Self::DocumentLocked => ErrorCategory::Server,

            Self::GatewayTimeout
            | Self::NetworkTimeout
            | Self::NetworkConnectionFailed
            | Self::DnsResolutionFailed
            | Self::SslCertificateError => ErrorCategory::Network,

            Self::Success => ErrorCategory::Other,
        }
    }

    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Success => ErrorSeverity::Info,
            Self::TooManyRequests => ErrorSeverity::Warning,
            Self::BadRequest | Self::Unauthorized | Self::Forbidden | Self::NotFound => {
                ErrorSeverity::Error
            }
            Self::InternalServerError | Self::ServiceUnavailable | Self::GatewayTimeout => {
                ErrorSeverity::Critical
            }
            _ => ErrorSeverity::Warning,
        }
    }

    /// 获取帮助文档URL
    pub fn help_url(&self) -> Option<&'static str> {
        match self {
            Self::Success => None,
            _ => Some("https://open.feishu.cn/document/"),
        }
    }
}

impl Display for LarkErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description(), *self as i32)
    }
}

// ============================================================================
// LarkAPIError 核心方法实现
// ============================================================================

impl LarkAPIError {
    /// 创建包含上下文信息的API错误
    pub fn api_error<M: Into<String>>(code: i32, message: M, request_id: Option<String>) -> Self {
        Self::ApiError {
            code,
            message: message.into(),
            request_id,
        }
    }

    /// 创建非法参数错误
    pub fn illegal_param<T: Into<String>>(message: T) -> Self {
        Self::IllegalParamError(message.into())
    }

    /// 创建验证错误
    pub fn validation_error<T: Into<String>>(message: T) -> Self {
        Self::ValidationError(message.into())
    }

    /// 创建认证错误
    pub fn auth_error<M: Into<String>>(message: M) -> Self {
        Self::AuthenticationError {
            message: message.into(),
            details: None,
        }
    }

    /// 创建认证错误（带详细信息）
    pub fn auth_error_with_details<M: Into<String>, D: Into<String>>(
        message: M,
        details: D,
    ) -> Self {
        Self::AuthenticationError {
            message: message.into(),
            details: Some(details.into()),
        }
    }

    /// 创建权限错误
    pub fn permission_error<M: Into<String>>(message: M, permission_type: PermissionType) -> Self {
        Self::PermissionError {
            message: message.into(),
            permission_type,
        }
    }

    /// 创建网络错误
    pub fn network_error<M: Into<String>>(message: M, kind: NetworkErrorKind) -> Self {
        Self::NetworkError {
            message: message.into(),
            kind,
        }
    }

    /// 检查是否为权限相关错误
    pub fn is_permission_error(&self) -> bool {
        match self {
            Self::ApiError { code, .. } => {
                *code == 403
                    || LarkErrorCode::from_code(*code)
                        .map(|ec| ec.is_permission_error())
                        .unwrap_or(false)
            }
            Self::PermissionError { .. } => true,
            _ => false,
        }
    }

    /// 检查错误是否可以重试
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::ApiError { code, .. } => LarkErrorCode::from_code(*code)
                .map(|ec| ec.is_retryable())
                .unwrap_or(false),
            Self::NetworkError { kind, .. } => {
                matches!(
                    kind,
                    NetworkErrorKind::Timeout
                        | NetworkErrorKind::ConnectionRefused
                        | NetworkErrorKind::DnsResolutionFailed
                )
            }
            Self::RequestError(req_err) => {
                req_err.contains("timeout")
                    || req_err.contains("timed out")
                    || req_err.contains("connect")
                    || req_err.contains("connection")
            }
            _ => false,
        }
    }

    /// 检查是否为认证相关错误
    pub fn is_authentication_error(&self) -> bool {
        match self {
            Self::ApiError { code, .. } => LarkErrorCode::from_code(*code)
                .map(|ec| ec.is_auth_error())
                .unwrap_or(false),
            Self::AuthenticationError { .. } | Self::MissingAccessToken => true,
            _ => false,
        }
    }

    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            Self::ApiError { code, message, .. } => {
                if let Some(error_code) = LarkErrorCode::from_code(*code) {
                    error_code.detailed_description().to_string()
                } else {
                    format!("API调用失败: {message} (错误码: {code})")
                }
            }
            Self::MissingAccessToken => "缺少访问令牌，请检查认证配置".to_string(),
            Self::IllegalParamError(msg) => format!("参数错误: {msg}"),
            Self::ValidationError(msg) => format!("验证失败: {msg}"),
            Self::AuthenticationError { message, .. } => {
                format!("认证失败: {message}")
            }
            Self::PermissionError { message, .. } => {
                format!("权限不足: {message}")
            }
            Self::RequestError(req_err) => {
                if req_err.contains("timeout") || req_err.contains("timed out") {
                    "请求超时，请检查网络连接".to_string()
                } else if req_err.contains("connect") || req_err.contains("connection") {
                    "连接失败，请检查网络设置".to_string()
                } else {
                    format!("网络请求失败: {req_err}")
                }
            }
            Self::NetworkError { message, kind } => match kind {
                NetworkErrorKind::Timeout => "网络超时，请检查网络设置".to_string(),
                NetworkErrorKind::ConnectionRefused => "连接被拒绝，请检查服务状态".to_string(),
                NetworkErrorKind::DnsResolutionFailed => "DNS解析失败，请检查网络设置".to_string(),
                NetworkErrorKind::SslError => "SSL证书错误，请检查安全设置".to_string(),
                NetworkErrorKind::Other => format!("网络错误: {message}"),
            },
            _ => self.to_string(),
        }
    }

    /// 获取错误处理分类
    pub fn handling_category(&self) -> ErrorHandlingCategory {
        match self {
            Self::ApiError { code, .. } => {
                if let Some(error_code) = LarkErrorCode::from_code(*code) {
                    // 优先检查服务器错误
                    match error_code {
                        LarkErrorCode::InternalServerError
                        | LarkErrorCode::BadGateway
                        | LarkErrorCode::ServiceUnavailable
                        | LarkErrorCode::GatewayTimeout => ErrorHandlingCategory::Server,
                        _ => {
                            if error_code.is_retryable() {
                                ErrorHandlingCategory::Retryable
                            } else if error_code.is_auth_error() {
                                ErrorHandlingCategory::Authentication
                            } else if error_code.is_permission_error() {
                                ErrorHandlingCategory::Permission
                            } else if error_code.is_client_error() {
                                ErrorHandlingCategory::Parameter
                            } else {
                                ErrorHandlingCategory::Business
                            }
                        }
                    }
                } else {
                    ErrorHandlingCategory::Unknown
                }
            }
            Self::NetworkError { .. } | Self::RequestError(_) => ErrorHandlingCategory::Retryable,
            Self::AuthenticationError { .. } | Self::MissingAccessToken => {
                ErrorHandlingCategory::Authentication
            }
            Self::PermissionError { .. } => ErrorHandlingCategory::Permission,
            Self::IllegalParamError(_) | Self::ValidationError(_) | Self::BadRequest(_) => {
                ErrorHandlingCategory::Parameter
            }
            Self::IOErr(_) | Self::DeserializeError(_) | Self::DataError(_) => {
                ErrorHandlingCategory::System
            }
            Self::APIError { .. } => ErrorHandlingCategory::Business,
            Self::UrlParseError(_) => ErrorHandlingCategory::Parameter,
        }
    }

    /// 获取建议的重试延迟时间（秒）
    pub fn suggested_retry_delay(&self) -> Option<u64> {
        match self {
            Self::ApiError { code, .. } => LarkErrorCode::from_code(*code)?.suggested_retry_delay(),
            Self::NetworkError { kind, .. } => match kind {
                NetworkErrorKind::Timeout => Some(3),
                NetworkErrorKind::ConnectionRefused => Some(5),
                NetworkErrorKind::DnsResolutionFailed => Some(10),
                _ => None,
            },
            Self::RequestError(req_err) => {
                if req_err.contains("timeout") || req_err.contains("timed out") {
                    Some(3)
                } else if req_err.contains("connect") || req_err.contains("connection") {
                    Some(5)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::ApiError { code, .. } => LarkErrorCode::from_code(*code)
                .map(|ec| ec.severity())
                .unwrap_or(ErrorSeverity::Error),
            Self::NetworkError { .. } | Self::RequestError(_) | Self::BadRequest(_) => {
                ErrorSeverity::Error
            }
            Self::AuthenticationError { .. } | Self::MissingAccessToken => ErrorSeverity::Error,
            Self::PermissionError { .. } => ErrorSeverity::Error,
            Self::IOErr(_) | Self::DeserializeError(_) => ErrorSeverity::Critical,
            _ => ErrorSeverity::Warning,
        }
    }
}

// ============================================================================
// 错误特征定义
// ============================================================================

/// 错误显示特征
pub trait ErrorDisplay {
    /// 获取用户友好的错误消息
    fn user_friendly_message(&self) -> String;

    /// 获取详细的错误信息
    fn detailed_message(&self) -> String;
}

/// 错误恢复特征
pub trait ErrorRecovery {
    /// 检查是否可以重试
    fn is_retryable(&self) -> bool;

    /// 获取建议的重试延迟时间（秒）
    fn suggested_retry_delay(&self) -> Option<u64>;
}

/// 错误分类特征
pub trait ErrorClassification {
    /// 获取错误处理分类
    fn handling_category(&self) -> ErrorHandlingCategory;

    /// 获取错误严重程度
    fn severity(&self) -> ErrorSeverity;
}

impl ErrorDisplay for LarkAPIError {
    fn user_friendly_message(&self) -> String {
        self.user_friendly_message()
    }

    fn detailed_message(&self) -> String {
        self.to_string()
    }
}

impl ErrorRecovery for LarkAPIError {
    fn is_retryable(&self) -> bool {
        self.is_retryable()
    }

    fn suggested_retry_delay(&self) -> Option<u64> {
        self.suggested_retry_delay()
    }
}

impl ErrorClassification for LarkAPIError {
    fn handling_category(&self) -> ErrorHandlingCategory {
        self.handling_category()
    }

    fn severity(&self) -> ErrorSeverity {
        self.severity()
    }
}

// ============================================================================
// 便利类型别名
// ============================================================================

/// API响应错误类型
pub type ApiResponseError = LarkAPIError;

/// 网络错误类型
pub type NetworkError = LarkAPIError;

/// 认证错误类型
pub type AuthenticationError = LarkAPIError;

/// 验证错误类型
pub type ValidationError = LarkAPIError;

/// 权限错误类型
pub type PermissionError = LarkAPIError;

/// 结果类型别名
pub type SDKResult<T> = Result<T, LarkAPIError>;

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lark_api_error_creation() {
        let error = LarkAPIError::IOErr("test error".to_string());
        assert!(matches!(error, LarkAPIError::IOErr(_)));
    }

    #[test]
    fn test_error_code_conversion() {
        assert_eq!(LarkErrorCode::from_code(0), Some(LarkErrorCode::Success));
        assert_eq!(LarkErrorCode::from_code(404), Some(LarkErrorCode::NotFound));
        assert_eq!(LarkErrorCode::from_code(999999), None);
    }

    #[test]
    fn test_error_features() {
        let error = LarkAPIError::RequestError("connection timeout".to_string());

        // 测试特征实现
        assert!(error.is_retryable());
        assert!(error.user_friendly_message().contains("超时"));

        // 测试特征调用
        assert!(<dyn ErrorRecovery>::is_retryable(&error));
        assert!(<dyn ErrorDisplay>::user_friendly_message(&error).contains("超时"));
        assert!(<dyn ErrorClassification>::severity(&error) == ErrorSeverity::Error);
    }

    #[test]
    fn test_new_error_types() {
        let auth_error = LarkAPIError::auth_error("Invalid credentials");
        assert!(matches!(
            auth_error,
            LarkAPIError::AuthenticationError { .. }
        ));

        let validation_error = LarkAPIError::validation_error("Invalid format");
        assert!(matches!(validation_error, LarkAPIError::ValidationError(_)));

        let permission_error =
            LarkAPIError::permission_error("Access denied", PermissionType::Application);
        assert!(matches!(
            permission_error,
            LarkAPIError::PermissionError { .. }
        ));

        let network_error = LarkAPIError::network_error("Timeout", NetworkErrorKind::Timeout);
        assert!(matches!(network_error, LarkAPIError::NetworkError { .. }));
    }

    #[test]
    fn test_error_classification() {
        let auth_error = LarkAPIError::MissingAccessToken;
        assert_eq!(
            auth_error.handling_category(),
            ErrorHandlingCategory::Authentication
        );

        let permission_error = LarkAPIError::permission_error("Denied", PermissionType::User);
        assert_eq!(
            permission_error.handling_category(),
            ErrorHandlingCategory::Permission
        );

        let network_error = LarkAPIError::network_error("Failed", NetworkErrorKind::Timeout);
        assert_eq!(
            network_error.handling_category(),
            ErrorHandlingCategory::Retryable
        );

        let validation_error = LarkAPIError::validation_error("Invalid data");
        assert_eq!(
            validation_error.handling_category(),
            ErrorHandlingCategory::Parameter
        );
    }

    #[test]
    fn test_error_severity() {
        let critical_error = LarkAPIError::IOErr("System error".to_string());
        assert_eq!(critical_error.severity(), ErrorSeverity::Critical);

        let error = LarkAPIError::BadRequest("Bad request".to_string());
        assert_eq!(error.severity(), ErrorSeverity::Error);

        let warning = LarkAPIError::DataError("Warning".to_string());
        assert_eq!(warning.severity(), ErrorSeverity::Warning);
    }
}
