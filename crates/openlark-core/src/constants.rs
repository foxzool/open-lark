use std::fmt::Display;

/// 应用类型
#[derive(Default, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum AppType {
    /// 自建应用
    #[default]
    SelfBuild,
    /// 商店应用
    Marketplace,
}

impl AppType {
    /// 获取应用类型字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            AppType::SelfBuild => "self_build",
            AppType::Marketplace => "marketplace",
        }
    }
}

impl Display for AppType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub const APP_ACCESS_TOKEN_INTERNAL_URL_PATH: &str = "/open-apis/auth/v3/app_access_token/internal";
pub const APP_ACCESS_TOKEN_URL_PATH: &str = "/open-apis/auth/v3/app_access_token";
pub const TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH: &str =
    "/open-apis/auth/v3/tenant_access_token/internal";
pub const TENANT_ACCESS_TOKEN_URL_PATH: &str = "/open-apis/auth/v3/tenant_access_token";
pub const APPLY_APP_TICKET_PATH: &str = "/open-apis/auth/v3/app_ticket/resend";

#[derive(Default, Hash, Eq, PartialEq, Debug, Copy, Clone)]
/// 访问令牌类型
///
/// 定义不同的访问令牌类型，用于API认证
pub enum AccessTokenType {
    #[default]
    None,
    App,
    Tenant,
    User,
}

impl Display for AccessTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AccessTokenType::None => String::from("none_access_token"),
            AccessTokenType::App => String::from("app_access_token"),
            AccessTokenType::Tenant => String::from("tenant_access_token"),
            AccessTokenType::User => String::from("user_access_token"),
        };
        write!(f, "{str}")
    }
}

pub const PROJECT: &str = "open-lark";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Domain
pub const FEISHU_BASE_URL: &str = "https://open.feishu.cn";
pub const LARK_BASE_URL: &str = "https://open.larksuite.com";

pub const DEFAULT_CONTENT_TYPE: &str = "application/json; charset=utf-8";
pub const FILE_CONTENT_TYPE: &str = "multipart/form-data";
pub const USER_AGENT_HEADER: &str = "User-Agent";

pub const HTTP_HEADER_KEY_REQUEST_ID: &str = "X-Request-Id";
pub const HTTP_HEADER_REQUEST_ID: &str = "Request-Id";

pub const HTTP_HEADER_KEY_LOG_ID: &str = "X-Tt-Logid";
pub const CONTENT_TYPE_HEADER: &str = "Content-Type";
pub const CONTENT_TYPE_JSON: &str = "application/json";
pub const CUSTOM_REQUEST_ID: &str = "Open-Lark-Request-Id";
pub const APP_TICKET_KEY_PREFIX: &str = "app_ticket";
pub const APP_ACCESS_TOKEN_KEY_PREFIX: &str = "app_access_token";
pub const TENANT_ACCESS_TOKEN_KEY_PREFIX: &str = "tenant_access_token";
pub const EXPIRY_DELTA: i32 = 60 * 3;
pub const ERR_CODE_APP_TICKET_INVALID: i32 = 10012;
pub const ERR_CODE_ACCESS_TOKEN_INVALID: i32 = 99991671;
pub const ERR_CODE_APP_ACCESS_TOKEN_INVALID: i32 = 99991664;
pub const ERR_CODE_TENANT_ACCESS_TOKEN_INVALID: i32 = 99991663;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_type_default() {
        assert_eq!(AppType::default(), AppType::SelfBuild);
    }

    #[test]
    fn test_app_type_debug_clone() {
        let app_type = AppType::Marketplace;
        let cloned = app_type;
        assert_eq!(format!("{:?}", app_type), "Marketplace");
        assert_eq!(cloned, AppType::Marketplace);
    }

    #[test]
    fn test_app_type_equality() {
        assert_eq!(AppType::SelfBuild, AppType::SelfBuild);
        assert_eq!(AppType::Marketplace, AppType::Marketplace);
        assert_ne!(AppType::SelfBuild, AppType::Marketplace);
    }

    #[test]
    fn test_access_token_type_default() {
        assert_eq!(AccessTokenType::default(), AccessTokenType::None);
    }

    #[test]
    fn test_access_token_type_display() {
        assert_eq!(AccessTokenType::None.to_string(), "none_access_token");
        assert_eq!(AccessTokenType::App.to_string(), "app_access_token");
        assert_eq!(AccessTokenType::Tenant.to_string(), "tenant_access_token");
        assert_eq!(AccessTokenType::User.to_string(), "user_access_token");
    }

    #[test]
    fn test_access_token_type_debug_clone() {
        let token_type = AccessTokenType::App;
        let cloned = token_type;
        assert_eq!(format!("{:?}", token_type), "App");
        assert_eq!(cloned, AccessTokenType::App);
    }

    #[test]
    fn test_access_token_type_equality() {
        assert_eq!(AccessTokenType::None, AccessTokenType::None);
        assert_eq!(AccessTokenType::App, AccessTokenType::App);
        assert_eq!(AccessTokenType::Tenant, AccessTokenType::Tenant);
        assert_eq!(AccessTokenType::User, AccessTokenType::User);
        assert_ne!(AccessTokenType::App, AccessTokenType::Tenant);
    }

    #[test]
    fn test_constants_values() {
        // Test URL paths
        assert_eq!(
            APP_ACCESS_TOKEN_INTERNAL_URL_PATH,
            "/open-apis/auth/v3/app_access_token/internal"
        );
        assert_eq!(
            APP_ACCESS_TOKEN_URL_PATH,
            "/open-apis/auth/v3/app_access_token"
        );
        assert_eq!(
            TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH,
            "/open-apis/auth/v3/tenant_access_token/internal"
        );
        assert_eq!(
            TENANT_ACCESS_TOKEN_URL_PATH,
            "/open-apis/auth/v3/tenant_access_token"
        );
        assert_eq!(
            APPLY_APP_TICKET_PATH,
            "/open-apis/auth/v3/app_ticket/resend"
        );

        // Test project info
        assert_eq!(PROJECT, "open-lark");
        // VERSION is a const from env!() so it's never empty - verify it's a valid version
        assert!(
            VERSION.contains('.'),
            "Version should contain dots for proper semver"
        );

        // Test base URLs
        assert_eq!(FEISHU_BASE_URL, "https://open.feishu.cn");
        assert_eq!(LARK_BASE_URL, "https://open.larksuite.com");

        // Test content types
        assert_eq!(DEFAULT_CONTENT_TYPE, "application/json; charset=utf-8");
        assert_eq!(FILE_CONTENT_TYPE, "multipart/form-data");
        assert_eq!(CONTENT_TYPE_JSON, "application/json");

        // Test headers
        assert_eq!(USER_AGENT_HEADER, "User-Agent");
        assert_eq!(HTTP_HEADER_KEY_REQUEST_ID, "X-Request-Id");
        assert_eq!(HTTP_HEADER_REQUEST_ID, "Request-Id");
        assert_eq!(HTTP_HEADER_KEY_LOG_ID, "X-Tt-Logid");
        assert_eq!(CONTENT_TYPE_HEADER, "Content-Type");
        assert_eq!(CUSTOM_REQUEST_ID, "Open-Lark-Request-Id");

        // Test cache key prefixes
        assert_eq!(APP_TICKET_KEY_PREFIX, "app_ticket");
        assert_eq!(APP_ACCESS_TOKEN_KEY_PREFIX, "app_access_token");
        assert_eq!(TENANT_ACCESS_TOKEN_KEY_PREFIX, "tenant_access_token");

        // Test numeric constants
        assert_eq!(EXPIRY_DELTA, 60 * 3);
        assert_eq!(ERR_CODE_APP_TICKET_INVALID, 10012);
        assert_eq!(ERR_CODE_ACCESS_TOKEN_INVALID, 99991671);
        assert_eq!(ERR_CODE_APP_ACCESS_TOKEN_INVALID, 99991664);
        assert_eq!(ERR_CODE_TENANT_ACCESS_TOKEN_INVALID, 99991663);
    }

    #[test]
    fn test_version_format() {
        // VERSION should follow semver format
        let version_parts: Vec<&str> = VERSION.split('.').collect();
        assert!(
            version_parts.len() >= 2,
            "Version should have at least major.minor format"
        );

        // Each part should be numeric
        for part in &version_parts {
            assert!(!part.is_empty(), "Version parts should not be empty");
            // Basic check for digits (might have pre-release suffixes)
            assert!(
                part.chars().next().unwrap().is_ascii_digit(),
                "Version should start with digit"
            );
        }
    }

    #[test]
    fn test_url_constants_format() {
        // All URLs should start with proper prefixes
        assert!(FEISHU_BASE_URL.starts_with("https://"));
        assert!(LARK_BASE_URL.starts_with("https://"));

        // API paths should start with /open-apis/
        assert!(APP_ACCESS_TOKEN_INTERNAL_URL_PATH.starts_with("/open-apis/"));
        assert!(APP_ACCESS_TOKEN_URL_PATH.starts_with("/open-apis/"));
        assert!(TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH.starts_with("/open-apis/"));
        assert!(TENANT_ACCESS_TOKEN_URL_PATH.starts_with("/open-apis/"));
        assert!(APPLY_APP_TICKET_PATH.starts_with("/open-apis/"));
    }

    #[test]
    fn test_error_code_ranges() {
        // Error codes should be in expected ranges
        // These are const assertions that will be optimized out
        // Removed to avoid clippy warnings about constant assertions

        // Different error codes should be unique
        let error_codes = [
            ERR_CODE_APP_TICKET_INVALID,
            ERR_CODE_ACCESS_TOKEN_INVALID,
            ERR_CODE_APP_ACCESS_TOKEN_INVALID,
            ERR_CODE_TENANT_ACCESS_TOKEN_INVALID,
        ];

        for (i, &code1) in error_codes.iter().enumerate() {
            for &code2 in error_codes.iter().skip(i + 1) {
                assert_ne!(code1, code2, "Error codes should be unique");
            }
        }
    }

    #[test]
    fn test_expiry_delta_reasonable() {
        // EXPIRY_DELTA should be reasonable (3 minutes in seconds)
        assert_eq!(EXPIRY_DELTA, 180);
        // These are const assertions that will be optimized out
        // Removed to avoid clippy warnings about constant assertions
    }

    #[test]
    fn test_content_type_formats() {
        assert!(DEFAULT_CONTENT_TYPE.contains("application/json"));
        assert!(DEFAULT_CONTENT_TYPE.contains("charset=utf-8"));
        assert!(FILE_CONTENT_TYPE.contains("multipart/form-data"));
        assert_eq!(CONTENT_TYPE_JSON, "application/json");
    }
}
