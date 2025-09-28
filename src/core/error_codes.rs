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
            Self::TooManyRequests => Some(60),        // 限流错误建议等待60秒
            Self::InternalServerError => Some(5),     // 服务器错误建议等待5秒
            Self::BadGateway => Some(3),              // 网关错误建议等待3秒
            Self::ServiceUnavailable => Some(10),     // 服务不可用建议等待10秒
            Self::GatewayTimeout => Some(5),          // 网关超时建议等待5秒
            Self::NetworkTimeout => Some(3),          // 网络超时建议等待3秒
            Self::NetworkConnectionFailed => Some(5), // 网络连接失败建议等待5秒
            Self::DnsResolutionFailed => Some(10),    // DNS解析失败建议等待10秒
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

impl LarkErrorCode {
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
        // 认证错误
        assert_eq!(
            LarkErrorCode::AccessTokenInvalid.category(),
            ErrorCategory::Authentication
        );
        assert_eq!(
            LarkErrorCode::AppTicketInvalid.category(),
            ErrorCategory::Authentication
        );
        assert_eq!(
            LarkErrorCode::Unauthorized.category(),
            ErrorCategory::Authentication
        );

        // 权限错误
        assert_eq!(
            LarkErrorCode::Forbidden.category(),
            ErrorCategory::Permission
        );
        assert_eq!(
            LarkErrorCode::AppPermissionDenied.category(),
            ErrorCategory::Permission
        );
        assert_eq!(
            LarkErrorCode::DocumentPermissionDenied.category(),
            ErrorCategory::Permission
        );

        // 参数错误
        assert_eq!(
            LarkErrorCode::BadRequest.category(),
            ErrorCategory::Parameter
        );
        assert_eq!(
            LarkErrorCode::FileSizeExceeded.category(),
            ErrorCategory::Parameter
        );
        assert_eq!(
            LarkErrorCode::EventConflict.category(),
            ErrorCategory::Parameter
        );

        // 资源错误
        assert_eq!(LarkErrorCode::NotFound.category(), ErrorCategory::Resource);
        assert_eq!(
            LarkErrorCode::UserNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::DocumentNotFound.category(),
            ErrorCategory::Resource
        );

        // 限流错误
        assert_eq!(
            LarkErrorCode::TooManyRequests.category(),
            ErrorCategory::RateLimit
        );

        // 服务器错误
        assert_eq!(
            LarkErrorCode::InternalServerError.category(),
            ErrorCategory::Server
        );
        assert_eq!(
            LarkErrorCode::DocumentLocked.category(),
            ErrorCategory::Server
        );

        // 网络错误
        assert_eq!(
            LarkErrorCode::NetworkTimeout.category(),
            ErrorCategory::Network
        );
        assert_eq!(
            LarkErrorCode::DnsResolutionFailed.category(),
            ErrorCategory::Network
        );
    }

    #[test]
    fn test_business_error_codes() {
        // 测试业务特定错误码
        assert_eq!(
            LarkErrorCode::from_code(60001),
            Some(LarkErrorCode::UserNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(70001),
            Some(LarkErrorCode::ChatNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(120001),
            Some(LarkErrorCode::DocumentNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(110001),
            Some(LarkErrorCode::CalendarNotFound)
        );

        // 测试网络错误码
        assert_eq!(
            LarkErrorCode::from_code(999001),
            Some(LarkErrorCode::NetworkTimeout)
        );
        assert_eq!(
            LarkErrorCode::from_code(999004),
            Some(LarkErrorCode::SslCertificateError)
        );
    }

    #[test]
    fn test_error_descriptions() {
        // 测试中文描述
        assert_eq!(LarkErrorCode::Success.description(), "操作成功");
        assert_eq!(LarkErrorCode::UserNotFound.description(), "用户不存在");
        assert_eq!(LarkErrorCode::ChatNotFound.description(), "群组不存在");
        assert_eq!(LarkErrorCode::DocumentLocked.description(), "文档已锁定");
        assert_eq!(LarkErrorCode::NetworkTimeout.description(), "网络连接超时");

        // 测试详细描述
        assert!(LarkErrorCode::UserNotFound
            .detailed_description()
            .contains("用户ID"));
        assert!(LarkErrorCode::TooManyRequests
            .detailed_description()
            .contains("请求频率"));
        assert!(LarkErrorCode::DocumentPermissionDenied
            .detailed_description()
            .contains("权限不足"));
    }

    #[test]
    fn test_help_urls() {
        // 测试帮助链接
        assert!(LarkErrorCode::AccessTokenInvalid.help_url().is_some());
        assert!(LarkErrorCode::TooManyRequests.help_url().is_some());
        assert!(LarkErrorCode::Forbidden.help_url().is_some());

        let auth_url = LarkErrorCode::AppAccessTokenInvalid.help_url().unwrap();
        assert!(auth_url.contains("authentication"));

        let rate_limit_url = LarkErrorCode::TooManyRequests.help_url().unwrap();
        assert!(rate_limit_url.contains("rate-limiting"));
    }

    #[test]
    fn test_severity_levels() {
        use crate::core::error::ErrorSeverity;

        // 测试不同严重级别
        assert_eq!(LarkErrorCode::Success.severity(), ErrorSeverity::Info);
        assert_eq!(
            LarkErrorCode::TooManyRequests.severity(),
            ErrorSeverity::Warning
        );
        assert_eq!(LarkErrorCode::BadRequest.severity(), ErrorSeverity::Error);
        assert_eq!(
            LarkErrorCode::InternalServerError.severity(),
            ErrorSeverity::Critical
        );
    }

    #[test]
    fn test_comprehensive_error_properties() {
        // 测试标准HTTP客户端错误
        let client_error = LarkErrorCode::BadRequest;
        assert!(!client_error.is_auth_error());
        assert!(!client_error.is_permission_error());
        assert!(client_error.is_client_error());
        assert!(!client_error.is_server_error());
        assert!(!client_error.is_retryable());
        assert_eq!(client_error.suggested_retry_delay(), None);
        assert_eq!(client_error.category(), ErrorCategory::Parameter);

        // 测试业务错误（不在HTTP范围内）
        let business_error = LarkErrorCode::DocumentNotFound;
        assert!(!business_error.is_auth_error());
        assert!(!business_error.is_permission_error());
        assert!(!business_error.is_client_error()); // 120001 不在400-499范围
        assert!(!business_error.is_server_error()); // 120001 不在500-599范围
        assert!(!business_error.is_retryable());
        assert_eq!(business_error.suggested_retry_delay(), None);
        assert_eq!(business_error.category(), ErrorCategory::Resource);

        // 测试网络错误的重试特性
        let network_error = LarkErrorCode::NetworkTimeout;
        assert!(network_error.is_retryable());
        assert_eq!(network_error.suggested_retry_delay(), Some(3));
        assert_eq!(network_error.category(), ErrorCategory::Network);

        // 测试权限错误
        let permission_error = LarkErrorCode::DocumentPermissionDenied;
        assert!(!permission_error.is_auth_error());
        assert!(permission_error.is_permission_error());
        assert_eq!(permission_error.category(), ErrorCategory::Permission);

        // 测试认证错误
        let auth_error = LarkErrorCode::AccessTokenInvalid;
        assert!(auth_error.is_auth_error());
        assert!(!auth_error.is_permission_error());
        assert_eq!(auth_error.category(), ErrorCategory::Authentication);

        // 测试服务器错误
        let server_error = LarkErrorCode::InternalServerError;
        assert!(!server_error.is_auth_error());
        assert!(!server_error.is_permission_error());
        assert!(!server_error.is_client_error());
        assert!(server_error.is_server_error());
        assert!(server_error.is_retryable());
        assert_eq!(server_error.suggested_retry_delay(), Some(5));
        assert_eq!(server_error.category(), ErrorCategory::Server);
    }

    #[test]
    fn test_all_error_codes_from_code_coverage() {
        // 测试所有错误码的from_code方法，确保完整覆盖
        assert_eq!(LarkErrorCode::from_code(0), Some(LarkErrorCode::Success));
        assert_eq!(
            LarkErrorCode::from_code(10003),
            Some(LarkErrorCode::AppNotInstalled)
        );
        assert_eq!(
            LarkErrorCode::from_code(10012),
            Some(LarkErrorCode::AppTicketInvalid)
        );
        assert_eq!(
            LarkErrorCode::from_code(10013),
            Some(LarkErrorCode::AppStatusException)
        );
        assert_eq!(
            LarkErrorCode::from_code(19001),
            Some(LarkErrorCode::AppPermissionDenied)
        );
        assert_eq!(
            LarkErrorCode::from_code(99991671),
            Some(LarkErrorCode::AccessTokenInvalid)
        );
        assert_eq!(
            LarkErrorCode::from_code(99991664),
            Some(LarkErrorCode::AppAccessTokenInvalid)
        );
        assert_eq!(
            LarkErrorCode::from_code(99991663),
            Some(LarkErrorCode::TenantAccessTokenInvalid)
        );
        assert_eq!(
            LarkErrorCode::from_code(400),
            Some(LarkErrorCode::BadRequest)
        );
        assert_eq!(
            LarkErrorCode::from_code(401),
            Some(LarkErrorCode::Unauthorized)
        );
        assert_eq!(
            LarkErrorCode::from_code(403),
            Some(LarkErrorCode::Forbidden)
        );
        assert_eq!(LarkErrorCode::from_code(404), Some(LarkErrorCode::NotFound));
        assert_eq!(
            LarkErrorCode::from_code(405),
            Some(LarkErrorCode::MethodNotAllowed)
        );
        assert_eq!(LarkErrorCode::from_code(409), Some(LarkErrorCode::Conflict));
        assert_eq!(
            LarkErrorCode::from_code(429),
            Some(LarkErrorCode::TooManyRequests)
        );
        assert_eq!(
            LarkErrorCode::from_code(500),
            Some(LarkErrorCode::InternalServerError)
        );
        assert_eq!(
            LarkErrorCode::from_code(502),
            Some(LarkErrorCode::BadGateway)
        );
        assert_eq!(
            LarkErrorCode::from_code(503),
            Some(LarkErrorCode::ServiceUnavailable)
        );
        assert_eq!(
            LarkErrorCode::from_code(504),
            Some(LarkErrorCode::GatewayTimeout)
        );
        assert_eq!(
            LarkErrorCode::from_code(60001),
            Some(LarkErrorCode::UserNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(60002),
            Some(LarkErrorCode::UserStatusException)
        );
        assert_eq!(
            LarkErrorCode::from_code(60003),
            Some(LarkErrorCode::DepartmentNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(70001),
            Some(LarkErrorCode::ChatNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(70002),
            Some(LarkErrorCode::ChatTypeNotSupported)
        );
        assert_eq!(
            LarkErrorCode::from_code(80001),
            Some(LarkErrorCode::MessageNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(80002),
            Some(LarkErrorCode::MessageTypeNotSupported)
        );
        assert_eq!(
            LarkErrorCode::from_code(90001),
            Some(LarkErrorCode::FileNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(90002),
            Some(LarkErrorCode::FileSizeExceeded)
        );
        assert_eq!(
            LarkErrorCode::from_code(90003),
            Some(LarkErrorCode::FileTypeNotSupported)
        );
        assert_eq!(
            LarkErrorCode::from_code(110001),
            Some(LarkErrorCode::CalendarNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(110002),
            Some(LarkErrorCode::EventNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(110003),
            Some(LarkErrorCode::EventConflict)
        );
        assert_eq!(
            LarkErrorCode::from_code(120001),
            Some(LarkErrorCode::DocumentNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(120002),
            Some(LarkErrorCode::DocumentPermissionDenied)
        );
        assert_eq!(
            LarkErrorCode::from_code(120003),
            Some(LarkErrorCode::DocumentLocked)
        );
        assert_eq!(
            LarkErrorCode::from_code(120011),
            Some(LarkErrorCode::SheetNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(120021),
            Some(LarkErrorCode::TableNotFound)
        );
        assert_eq!(
            LarkErrorCode::from_code(130001),
            Some(LarkErrorCode::AppNotPublished)
        );
        assert_eq!(
            LarkErrorCode::from_code(130002),
            Some(LarkErrorCode::AppVersionIncompatible)
        );
        assert_eq!(
            LarkErrorCode::from_code(999001),
            Some(LarkErrorCode::NetworkTimeout)
        );
        assert_eq!(
            LarkErrorCode::from_code(999002),
            Some(LarkErrorCode::NetworkConnectionFailed)
        );
        assert_eq!(
            LarkErrorCode::from_code(999003),
            Some(LarkErrorCode::DnsResolutionFailed)
        );
        assert_eq!(
            LarkErrorCode::from_code(999004),
            Some(LarkErrorCode::SslCertificateError)
        );

        // 测试未知错误码
        assert_eq!(LarkErrorCode::from_code(-1), None);
        assert_eq!(LarkErrorCode::from_code(12345), None);
        assert_eq!(LarkErrorCode::from_code(999999), None);
    }

    #[test]
    fn test_all_error_descriptions_coverage() {
        // 测试所有错误的中文描述
        assert_eq!(LarkErrorCode::Success.description(), "操作成功");
        assert_eq!(LarkErrorCode::AppNotInstalled.description(), "应用未安装");
        assert_eq!(
            LarkErrorCode::AppTicketInvalid.description(),
            "应用票据无效"
        );
        assert_eq!(
            LarkErrorCode::AppStatusException.description(),
            "应用状态异常"
        );
        assert_eq!(
            LarkErrorCode::AppPermissionDenied.description(),
            "应用权限不足"
        );
        assert_eq!(
            LarkErrorCode::AccessTokenInvalid.description(),
            "访问令牌无效"
        );
        assert_eq!(
            LarkErrorCode::AppAccessTokenInvalid.description(),
            "应用访问令牌无效"
        );
        assert_eq!(
            LarkErrorCode::TenantAccessTokenInvalid.description(),
            "租户访问令牌无效"
        );
        assert_eq!(LarkErrorCode::BadRequest.description(), "请求参数错误");
        assert_eq!(LarkErrorCode::Unauthorized.description(), "认证失败");
        assert_eq!(LarkErrorCode::Forbidden.description(), "权限不足");
        assert_eq!(LarkErrorCode::NotFound.description(), "资源不存在");
        assert_eq!(
            LarkErrorCode::MethodNotAllowed.description(),
            "请求方法不允许"
        );
        assert_eq!(LarkErrorCode::Conflict.description(), "请求冲突");
        assert_eq!(LarkErrorCode::TooManyRequests.description(), "请求频率过高");
        assert_eq!(
            LarkErrorCode::InternalServerError.description(),
            "内部服务器错误"
        );
        assert_eq!(LarkErrorCode::BadGateway.description(), "网关错误");
        assert_eq!(
            LarkErrorCode::ServiceUnavailable.description(),
            "服务不可用"
        );
        assert_eq!(LarkErrorCode::GatewayTimeout.description(), "网关超时");
        assert_eq!(LarkErrorCode::UserNotFound.description(), "用户不存在");
        assert_eq!(
            LarkErrorCode::UserStatusException.description(),
            "用户状态异常"
        );
        assert_eq!(
            LarkErrorCode::DepartmentNotFound.description(),
            "部门不存在"
        );
        assert_eq!(LarkErrorCode::ChatNotFound.description(), "群组不存在");
        assert_eq!(
            LarkErrorCode::ChatTypeNotSupported.description(),
            "群组类型不支持"
        );
        assert_eq!(LarkErrorCode::MessageNotFound.description(), "消息不存在");
        assert_eq!(
            LarkErrorCode::MessageTypeNotSupported.description(),
            "消息类型不支持"
        );
        assert_eq!(LarkErrorCode::FileNotFound.description(), "文件不存在");
        assert_eq!(
            LarkErrorCode::FileSizeExceeded.description(),
            "文件大小超限"
        );
        assert_eq!(
            LarkErrorCode::FileTypeNotSupported.description(),
            "文件类型不支持"
        );
        assert_eq!(LarkErrorCode::CalendarNotFound.description(), "日历不存在");
        assert_eq!(LarkErrorCode::EventNotFound.description(), "日程不存在");
        assert_eq!(LarkErrorCode::EventConflict.description(), "日程冲突");
        assert_eq!(LarkErrorCode::DocumentNotFound.description(), "文档不存在");
        assert_eq!(
            LarkErrorCode::DocumentPermissionDenied.description(),
            "文档权限不足"
        );
        assert_eq!(LarkErrorCode::DocumentLocked.description(), "文档已锁定");
        assert_eq!(LarkErrorCode::SheetNotFound.description(), "工作表不存在");
        assert_eq!(LarkErrorCode::TableNotFound.description(), "表格不存在");
        assert_eq!(LarkErrorCode::AppNotPublished.description(), "应用未发布");
        assert_eq!(
            LarkErrorCode::AppVersionIncompatible.description(),
            "应用版本不兼容"
        );
        assert_eq!(LarkErrorCode::NetworkTimeout.description(), "网络连接超时");
        assert_eq!(
            LarkErrorCode::NetworkConnectionFailed.description(),
            "网络连接失败"
        );
        assert_eq!(
            LarkErrorCode::DnsResolutionFailed.description(),
            "DNS解析失败"
        );
        assert_eq!(
            LarkErrorCode::SslCertificateError.description(),
            "SSL证书错误"
        );
    }

    #[test]
    fn test_all_detailed_descriptions_coverage() {
        // 测试所有错误的详细描述（测试关键内容）
        assert!(LarkErrorCode::Success
            .detailed_description()
            .contains("成功处理"));
        assert!(LarkErrorCode::AppNotInstalled
            .detailed_description()
            .contains("应用未安装"));
        assert!(LarkErrorCode::AppTicketInvalid
            .detailed_description()
            .contains("自动重新申请"));
        assert!(LarkErrorCode::AppPermissionDenied
            .detailed_description()
            .contains("开发者后台"));
        assert!(LarkErrorCode::AccessTokenInvalid
            .detailed_description()
            .contains("重新获取"));
        assert!(LarkErrorCode::BadRequest
            .detailed_description()
            .contains("参数格式"));
        assert!(LarkErrorCode::TooManyRequests
            .detailed_description()
            .contains("降低请求频率"));
        assert!(LarkErrorCode::UserNotFound
            .detailed_description()
            .contains("用户ID"));
        assert!(LarkErrorCode::DocumentPermissionDenied
            .detailed_description()
            .contains("联系文档所有者"));
        assert!(LarkErrorCode::NetworkTimeout
            .detailed_description()
            .contains("网络设置"));
    }

    #[test]
    fn test_all_error_categories_coverage() {
        // 测试所有错误分类，确保每个枚举值都被测试

        // Authentication 类别
        assert_eq!(
            LarkErrorCode::AppTicketInvalid.category(),
            ErrorCategory::Authentication
        );
        assert_eq!(
            LarkErrorCode::AccessTokenInvalid.category(),
            ErrorCategory::Authentication
        );
        assert_eq!(
            LarkErrorCode::AppAccessTokenInvalid.category(),
            ErrorCategory::Authentication
        );
        assert_eq!(
            LarkErrorCode::TenantAccessTokenInvalid.category(),
            ErrorCategory::Authentication
        );
        assert_eq!(
            LarkErrorCode::Unauthorized.category(),
            ErrorCategory::Authentication
        );

        // Permission 类别
        assert_eq!(
            LarkErrorCode::Forbidden.category(),
            ErrorCategory::Permission
        );
        assert_eq!(
            LarkErrorCode::AppPermissionDenied.category(),
            ErrorCategory::Permission
        );
        assert_eq!(
            LarkErrorCode::DocumentPermissionDenied.category(),
            ErrorCategory::Permission
        );

        // Parameter 类别
        assert_eq!(
            LarkErrorCode::BadRequest.category(),
            ErrorCategory::Parameter
        );
        assert_eq!(
            LarkErrorCode::MethodNotAllowed.category(),
            ErrorCategory::Parameter
        );
        assert_eq!(
            LarkErrorCode::FileSizeExceeded.category(),
            ErrorCategory::Parameter
        );
        assert_eq!(
            LarkErrorCode::FileTypeNotSupported.category(),
            ErrorCategory::Parameter
        );
        assert_eq!(
            LarkErrorCode::MessageTypeNotSupported.category(),
            ErrorCategory::Parameter
        );
        assert_eq!(
            LarkErrorCode::ChatTypeNotSupported.category(),
            ErrorCategory::Parameter
        );
        assert_eq!(
            LarkErrorCode::EventConflict.category(),
            ErrorCategory::Parameter
        );
        assert_eq!(
            LarkErrorCode::AppVersionIncompatible.category(),
            ErrorCategory::Parameter
        );

        // Resource 类别
        assert_eq!(LarkErrorCode::NotFound.category(), ErrorCategory::Resource);
        assert_eq!(
            LarkErrorCode::UserNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::DepartmentNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::ChatNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::MessageNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::FileNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::CalendarNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::EventNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::DocumentNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::SheetNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::TableNotFound.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::AppNotInstalled.category(),
            ErrorCategory::Resource
        );
        assert_eq!(
            LarkErrorCode::AppNotPublished.category(),
            ErrorCategory::Resource
        );
        assert_eq!(LarkErrorCode::Conflict.category(), ErrorCategory::Resource);

        // RateLimit 类别
        assert_eq!(
            LarkErrorCode::TooManyRequests.category(),
            ErrorCategory::RateLimit
        );

        // Server 类别
        assert_eq!(
            LarkErrorCode::InternalServerError.category(),
            ErrorCategory::Server
        );
        assert_eq!(LarkErrorCode::BadGateway.category(), ErrorCategory::Server);
        assert_eq!(
            LarkErrorCode::ServiceUnavailable.category(),
            ErrorCategory::Server
        );
        assert_eq!(
            LarkErrorCode::AppStatusException.category(),
            ErrorCategory::Server
        );
        assert_eq!(
            LarkErrorCode::UserStatusException.category(),
            ErrorCategory::Server
        );
        assert_eq!(
            LarkErrorCode::DocumentLocked.category(),
            ErrorCategory::Server
        );

        // Network 类别
        assert_eq!(
            LarkErrorCode::GatewayTimeout.category(),
            ErrorCategory::Network
        );
        assert_eq!(
            LarkErrorCode::NetworkTimeout.category(),
            ErrorCategory::Network
        );
        assert_eq!(
            LarkErrorCode::NetworkConnectionFailed.category(),
            ErrorCategory::Network
        );
        assert_eq!(
            LarkErrorCode::DnsResolutionFailed.category(),
            ErrorCategory::Network
        );
        assert_eq!(
            LarkErrorCode::SslCertificateError.category(),
            ErrorCategory::Network
        );

        // Other 类别
        assert_eq!(LarkErrorCode::Success.category(), ErrorCategory::Other);
    }

    #[test]
    fn test_all_auth_errors() {
        // 测试所有认证相关错误
        assert!(LarkErrorCode::AppTicketInvalid.is_auth_error());
        assert!(LarkErrorCode::AccessTokenInvalid.is_auth_error());
        assert!(LarkErrorCode::AppAccessTokenInvalid.is_auth_error());
        assert!(LarkErrorCode::TenantAccessTokenInvalid.is_auth_error());
        assert!(LarkErrorCode::Unauthorized.is_auth_error());

        // 确保非认证错误不被误判
        assert!(!LarkErrorCode::Success.is_auth_error());
        assert!(!LarkErrorCode::Forbidden.is_auth_error());
        assert!(!LarkErrorCode::BadRequest.is_auth_error());
        assert!(!LarkErrorCode::NotFound.is_auth_error());
        assert!(!LarkErrorCode::InternalServerError.is_auth_error());
    }

    #[test]
    fn test_all_permission_errors() {
        // 测试所有权限相关错误
        assert!(LarkErrorCode::Forbidden.is_permission_error());
        assert!(LarkErrorCode::AppPermissionDenied.is_permission_error());
        assert!(LarkErrorCode::DocumentPermissionDenied.is_permission_error());

        // 确保非权限错误不被误判
        assert!(!LarkErrorCode::Success.is_permission_error());
        assert!(!LarkErrorCode::Unauthorized.is_permission_error());
        assert!(!LarkErrorCode::BadRequest.is_permission_error());
        assert!(!LarkErrorCode::NotFound.is_permission_error());
        assert!(!LarkErrorCode::InternalServerError.is_permission_error());
    }

    #[test]
    fn test_client_error_range() {
        // 测试所有HTTP客户端错误（400-499，除了429）
        assert!(LarkErrorCode::BadRequest.is_client_error());
        assert!(LarkErrorCode::Unauthorized.is_client_error());
        assert!(LarkErrorCode::Forbidden.is_client_error());
        assert!(LarkErrorCode::NotFound.is_client_error());
        assert!(LarkErrorCode::MethodNotAllowed.is_client_error());
        assert!(LarkErrorCode::Conflict.is_client_error());

        // 429 被排除在客户端错误之外
        assert!(!LarkErrorCode::TooManyRequests.is_client_error());

        // 确保非客户端错误不被误判
        assert!(!LarkErrorCode::Success.is_client_error());
        assert!(!LarkErrorCode::InternalServerError.is_client_error());
        assert!(!LarkErrorCode::UserNotFound.is_client_error()); // 60001 不在400-499范围
    }

    #[test]
    fn test_server_error_range() {
        // 测试所有HTTP服务器错误（500-599）
        assert!(LarkErrorCode::InternalServerError.is_server_error());
        assert!(LarkErrorCode::BadGateway.is_server_error());
        assert!(LarkErrorCode::ServiceUnavailable.is_server_error());
        assert!(LarkErrorCode::GatewayTimeout.is_server_error());

        // 确保非服务器错误不被误判
        assert!(!LarkErrorCode::Success.is_server_error());
        assert!(!LarkErrorCode::BadRequest.is_server_error());
        assert!(!LarkErrorCode::UserNotFound.is_server_error()); // 60001 不在500-599范围
        assert!(!LarkErrorCode::NetworkTimeout.is_server_error()); // 999001 不在500-599范围
    }

    #[test]
    fn test_all_retryable_errors() {
        // 测试所有可重试错误
        assert!(LarkErrorCode::TooManyRequests.is_retryable());
        assert!(LarkErrorCode::InternalServerError.is_retryable());
        assert!(LarkErrorCode::BadGateway.is_retryable());
        assert!(LarkErrorCode::ServiceUnavailable.is_retryable());
        assert!(LarkErrorCode::GatewayTimeout.is_retryable());
        assert!(LarkErrorCode::NetworkTimeout.is_retryable());
        assert!(LarkErrorCode::NetworkConnectionFailed.is_retryable());
        assert!(LarkErrorCode::DnsResolutionFailed.is_retryable());

        // 确保不可重试错误不被误判
        assert!(!LarkErrorCode::Success.is_retryable());
        assert!(!LarkErrorCode::BadRequest.is_retryable());
        assert!(!LarkErrorCode::Unauthorized.is_retryable());
        assert!(!LarkErrorCode::Forbidden.is_retryable());
        assert!(!LarkErrorCode::NotFound.is_retryable());
        assert!(!LarkErrorCode::UserNotFound.is_retryable());
        assert!(!LarkErrorCode::SslCertificateError.is_retryable());
    }

    #[test]
    fn test_all_retry_delays() {
        // 测试所有重试延迟
        assert_eq!(
            LarkErrorCode::TooManyRequests.suggested_retry_delay(),
            Some(60)
        );
        assert_eq!(
            LarkErrorCode::InternalServerError.suggested_retry_delay(),
            Some(5)
        );
        assert_eq!(LarkErrorCode::BadGateway.suggested_retry_delay(), Some(3));
        assert_eq!(
            LarkErrorCode::ServiceUnavailable.suggested_retry_delay(),
            Some(10)
        );
        assert_eq!(
            LarkErrorCode::GatewayTimeout.suggested_retry_delay(),
            Some(5)
        );
        assert_eq!(
            LarkErrorCode::NetworkTimeout.suggested_retry_delay(),
            Some(3)
        );
        assert_eq!(
            LarkErrorCode::NetworkConnectionFailed.suggested_retry_delay(),
            Some(5)
        );
        assert_eq!(
            LarkErrorCode::DnsResolutionFailed.suggested_retry_delay(),
            Some(10)
        );

        // 测试无重试延迟的错误
        assert_eq!(LarkErrorCode::Success.suggested_retry_delay(), None);
        assert_eq!(LarkErrorCode::BadRequest.suggested_retry_delay(), None);
        assert_eq!(LarkErrorCode::Unauthorized.suggested_retry_delay(), None);
        assert_eq!(
            LarkErrorCode::SslCertificateError.suggested_retry_delay(),
            None
        );
    }

    #[test]
    fn test_all_help_urls() {
        // 测试有帮助链接的错误
        let auth_errors = [
            LarkErrorCode::AppTicketInvalid,
            LarkErrorCode::AccessTokenInvalid,
            LarkErrorCode::AppAccessTokenInvalid,
            LarkErrorCode::TenantAccessTokenInvalid,
        ];
        for error in &auth_errors {
            let url = error.help_url().unwrap();
            assert!(url.contains("authentication"));
        }

        let forbidden_url = LarkErrorCode::Forbidden.help_url().unwrap();
        assert!(forbidden_url.contains("scope-and-authorization"));

        let rate_limit_url = LarkErrorCode::TooManyRequests.help_url().unwrap();
        assert!(rate_limit_url.contains("rate-limiting"));

        // 测试默认帮助链接
        let default_url = LarkErrorCode::UserNotFound.help_url().unwrap();
        assert!(default_url.contains("open.feishu.cn/document"));
    }

    #[test]
    fn test_all_severity_levels() {
        use crate::core::error::ErrorSeverity;

        // Info级别
        assert_eq!(LarkErrorCode::Success.severity(), ErrorSeverity::Info);

        // Warning级别
        assert_eq!(
            LarkErrorCode::TooManyRequests.severity(),
            ErrorSeverity::Warning
        );

        // Error级别
        assert_eq!(LarkErrorCode::BadRequest.severity(), ErrorSeverity::Error);
        assert_eq!(LarkErrorCode::Unauthorized.severity(), ErrorSeverity::Error);
        assert_eq!(LarkErrorCode::Forbidden.severity(), ErrorSeverity::Error);
        assert_eq!(LarkErrorCode::NotFound.severity(), ErrorSeverity::Error);

        // Critical级别
        assert_eq!(
            LarkErrorCode::InternalServerError.severity(),
            ErrorSeverity::Critical
        );
        assert_eq!(
            LarkErrorCode::ServiceUnavailable.severity(),
            ErrorSeverity::Critical
        );
        assert_eq!(
            LarkErrorCode::GatewayTimeout.severity(),
            ErrorSeverity::Critical
        );

        // 其他错误默认为Warning
        assert_eq!(
            LarkErrorCode::UserNotFound.severity(),
            ErrorSeverity::Warning
        );
        assert_eq!(
            LarkErrorCode::AppNotInstalled.severity(),
            ErrorSeverity::Warning
        );
        assert_eq!(
            LarkErrorCode::NetworkTimeout.severity(),
            ErrorSeverity::Warning
        );
    }

    #[test]
    fn test_display_trait() {
        // 测试Display trait的实现
        let error = LarkErrorCode::BadRequest;
        let display_str = format!("{}", error);
        assert!(display_str.contains("请求参数错误"));
        assert!(display_str.contains("400"));

        let success = LarkErrorCode::Success;
        let success_str = format!("{}", success);
        assert!(success_str.contains("操作成功"));
        assert!(success_str.contains("0"));
    }

    #[test]
    fn test_error_category_traits() {
        // 测试ErrorCategory的基本trait
        let category = ErrorCategory::Authentication;

        // 测试Debug
        let debug_str = format!("{:?}", category);
        assert!(debug_str.contains("Authentication"));

        // 测试Clone和Copy
        let cloned = category;
        assert_eq!(category, cloned);

        // 测试PartialEq和Eq
        assert_eq!(ErrorCategory::Authentication, ErrorCategory::Authentication);
        assert_ne!(ErrorCategory::Authentication, ErrorCategory::Permission);

        // 测试Hash（通过将其放入HashMap验证）
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(category, "test");
        assert_eq!(map.get(&ErrorCategory::Authentication), Some(&"test"));
    }

    #[test]
    fn test_lark_error_code_traits() {
        // 测试LarkErrorCode的基本trait
        let error = LarkErrorCode::BadRequest;

        // 测试Debug
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("BadRequest"));

        // 测试Clone和Copy
        let cloned = error;
        assert_eq!(error, cloned);

        // 测试PartialEq和Eq
        assert_eq!(LarkErrorCode::BadRequest, LarkErrorCode::BadRequest);
        assert_ne!(LarkErrorCode::BadRequest, LarkErrorCode::NotFound);

        // 测试Hash（通过将其放入HashMap验证）
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(error, "test");
        assert_eq!(map.get(&LarkErrorCode::BadRequest), Some(&"test"));
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界情况

        // 测试最大和最小的错误码
        assert_eq!(LarkErrorCode::from_code(0), Some(LarkErrorCode::Success));
        assert_eq!(
            LarkErrorCode::from_code(99991671),
            Some(LarkErrorCode::AccessTokenInvalid)
        );

        // 测试负数错误码
        assert_eq!(LarkErrorCode::from_code(-1), None);
        assert_eq!(LarkErrorCode::from_code(i32::MIN), None);
        assert_eq!(LarkErrorCode::from_code(i32::MAX), None);

        // 测试HTTP状态码边界
        let boundary_codes = [399, 400, 429, 430, 499, 500, 599, 600];
        for &code in &boundary_codes {
            match code {
                400 | 401 | 403 | 404 | 405 | 409 => {
                    assert!(LarkErrorCode::from_code(code).is_some());
                }
                429 => {
                    assert_eq!(
                        LarkErrorCode::from_code(code),
                        Some(LarkErrorCode::TooManyRequests)
                    );
                }
                500 | 502 | 503 | 504 => {
                    assert!(LarkErrorCode::from_code(code).is_some());
                }
                _ => {
                    assert_eq!(LarkErrorCode::from_code(code), None);
                }
            }
        }
    }
}
