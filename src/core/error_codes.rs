use std::fmt::{Display, Formatter};

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
    /// 服务不可用
    ServiceUnavailable = 503,
    /// 网关超时
    GatewayTimeout = 504,
}

impl LarkErrorCode {
    /// 从错误码数值创建枚举
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::Success),
            10012 => Some(Self::AppTicketInvalid),
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
            503 => Some(Self::ServiceUnavailable),
            504 => Some(Self::GatewayTimeout),
            _ => None,
        }
    }

    /// 获取错误码的中文描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Success => "操作成功",
            Self::AppTicketInvalid => "应用票据无效",
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
            Self::ServiceUnavailable => "服务不可用",
            Self::GatewayTimeout => "网关超时",
        }
    }

    /// 获取详细的错误说明
    pub fn detailed_description(&self) -> &'static str {
        match self {
            Self::Success => "请求已成功处理",
            Self::AppTicketInvalid => "应用票据已失效，SDK会自动重新申请",
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
            Self::ServiceUnavailable => "服务暂时不可用，请稍后重试",
            Self::GatewayTimeout => "网关超时，请稍后重试",
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
        matches!(self, Self::Forbidden)
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
                | Self::ServiceUnavailable
                | Self::GatewayTimeout
        )
    }

    /// 获取建议的重试延迟时间（秒）
    pub fn suggested_retry_delay(&self) -> Option<u64> {
        match self {
            Self::TooManyRequests => Some(60),    // 限流错误建议等待60秒
            Self::InternalServerError => Some(5), // 服务器错误建议等待5秒
            Self::ServiceUnavailable => Some(10), // 服务不可用建议等待10秒
            Self::GatewayTimeout => Some(5),      // 网关超时建议等待5秒
            _ => None,
        }
    }

    /// 获取错误的严重程度
    pub fn severity(&self) -> super::error::ErrorSeverity {
        match self {
            Self::Success => super::error::ErrorSeverity::Info,
            Self::TooManyRequests => super::error::ErrorSeverity::Warning,
            Self::BadRequest | Self::Unauthorized | Self::Forbidden | Self::NotFound => {
                super::error::ErrorSeverity::Error
            }
            Self::InternalServerError | Self::ServiceUnavailable | Self::GatewayTimeout => {
                super::error::ErrorSeverity::Critical
            }
            _ => super::error::ErrorSeverity::Warning,
        }
    }

    /// 获取相关的帮助文档链接
    pub fn help_url(&self) -> Option<&'static str> {
        match self {
            Self::AppTicketInvalid
            | Self::AccessTokenInvalid
            | Self::AppAccessTokenInvalid
            | Self::TenantAccessTokenInvalid => {
                Some("https://open.feishu.cn/document/server-docs/authentication/access-token")
            }
            Self::Forbidden => {
                Some("https://open.feishu.cn/document/home/introduction-to-scope-and-authorization")
            }
            Self::TooManyRequests => {
                Some("https://open.feishu.cn/document/server-docs/api-call-guide/rate-limiting")
            }
            _ => Some("https://open.feishu.cn/document/"),
        }
    }
}

impl Display for LarkErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description(), *self as i32)
    }
}

/// 错误码分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl LarkErrorCode {
    /// 获取错误分类
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::AppTicketInvalid
            | Self::AccessTokenInvalid
            | Self::AppAccessTokenInvalid
            | Self::TenantAccessTokenInvalid
            | Self::Unauthorized => ErrorCategory::Authentication,
            Self::Forbidden => ErrorCategory::Permission,
            Self::BadRequest => ErrorCategory::Parameter,
            Self::NotFound | Self::Conflict => ErrorCategory::Resource,
            Self::TooManyRequests => ErrorCategory::RateLimit,
            Self::InternalServerError | Self::ServiceUnavailable => ErrorCategory::Server,
            Self::GatewayTimeout => ErrorCategory::Network,
            _ => ErrorCategory::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_from_code() {
        assert_eq!(LarkErrorCode::from_code(0), Some(LarkErrorCode::Success));
        assert_eq!(
            LarkErrorCode::from_code(10012),
            Some(LarkErrorCode::AppTicketInvalid)
        );
        assert_eq!(LarkErrorCode::from_code(404), Some(LarkErrorCode::NotFound));
        assert_eq!(LarkErrorCode::from_code(999999), None);
    }

    #[test]
    fn test_error_classification() {
        assert!(LarkErrorCode::AccessTokenInvalid.is_auth_error());
        assert!(LarkErrorCode::Forbidden.is_permission_error());
        assert!(LarkErrorCode::BadRequest.is_client_error());
        assert!(LarkErrorCode::InternalServerError.is_server_error());
        assert!(LarkErrorCode::TooManyRequests.is_retryable());
    }

    #[test]
    fn test_retry_delay() {
        assert_eq!(
            LarkErrorCode::TooManyRequests.suggested_retry_delay(),
            Some(60)
        );
        assert_eq!(LarkErrorCode::Success.suggested_retry_delay(), None);
    }

    #[test]
    fn test_error_category() {
        assert_eq!(
            LarkErrorCode::AccessTokenInvalid.category(),
            ErrorCategory::Authentication
        );
        assert_eq!(
            LarkErrorCode::Forbidden.category(),
            ErrorCategory::Permission
        );
        assert_eq!(
            LarkErrorCode::BadRequest.category(),
            ErrorCategory::Parameter
        );
    }
}
