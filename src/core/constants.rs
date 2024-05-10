use std::fmt::Display;

/// 应用类型
#[derive(Default, Hash, Eq, PartialEq, Debug, Clone)]
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
pub const VERSION: &str = "0.0.1";

/// Domain
pub const FEISHU_BASE_URL: &str = "https://open.feishu.cn";
pub const LARK_BASE_URL: &str = "https://open.larksuite.com";

/// HEADER
pub const USER_AGENT: &str = "User-Agent";
pub const AUTHORIZATION: &str = "Authorization";
pub const X_TT_LOGID: &str = "X-Tt-Logid";
pub const HTTP_HEADER_KEY_REQUEST_ID: &str = "X-Request-Id";
pub const HTTP_HEADER_REQUEST_ID: &str = "Request-Id";
pub const CONTENT_TYPE: &str = "Content-Type";
pub const Content_Disposition: &str = "Content-Disposition";
pub const LARK_REQUEST_TIMESTAMP: &str = "X-Lark-Request-Timestamp";
pub const LARK_REQUEST_NONCE: &str = "X-Lark-Request-Nonce";
pub const LARK_REQUEST_SIGNATURE: &str = "X-Lark-Signature";

/// Content-Type
pub const APPLICATION_JSON: &str = "application/json";

/// Event
pub const URL_VERIFICATION: &str = "url_verification";
