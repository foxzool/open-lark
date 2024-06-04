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

pub const APP_ACCESS_TOKEN_INTERNAL_URL_PATH: &str = "/open-apis/auth/v3/app_access_token/internal";
pub const APP_ACCESS_TOKEN_URL_PATH: &str = "/open-apis/auth/v3/app_access_token";
pub const TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH: &str =
    "/open-apis/auth/v3/tenant_access_token/internal";
pub const TENANT_ACCESS_TOKEN_URL_PATH: &str = "/open-apis/auth/v3/tenant_access_token";
pub const APPLY_APP_TICKET_PATH: &str = "/open-apis/auth/v3/app_ticket/resend";

#[derive(Default, Hash, Eq, PartialEq, Debug, Copy, Clone)]
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
        write!(f, "{}", str)
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
