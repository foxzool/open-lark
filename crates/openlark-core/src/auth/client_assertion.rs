//! 外部签名的 client assertion 供给。
//!
//! SDK 不签 JWT、不解析 JWT。调用方实现 [`ClientAssertionProvider`]，换票逻辑在 `openlark-auth`。

use std::{future::Future, pin::Pin};

use crate::SDKResult;

/// 供给存在时请求 app access token。
pub const CLIENT_ASSERTION_APP_TOKEN_UNAVAILABLE: &str =
    "token_provider: client assertion 模式下不能获取 app_access_token";

/// 供给存在且应用类型是 marketplace。
pub const CLIENT_ASSERTION_MARKETPLACE_UNSUPPORTED: &str =
    "token_provider: client assertion 模式不支持 marketplace 应用";

/// 供给返回空白，或 trim 之后为空。
pub const CLIENT_ASSERTION_EMPTY: &str = "token_provider: client assertion 为空";

/// 供给返回 `Err` 时的消息前缀，后面接内部错误的 `Display`。
pub const CLIENT_ASSERTION_PROVIDER_FAILED_PREFIX: &str =
    "token_provider: client assertion 供给失败: ";

/// 自定义或未映射的 `base_url`，且没有 `oauth_base_url`。
pub const CLIENT_ASSERTION_CUSTOM_BASE_URL: &str =
    "token_provider: 自定义 base_url 必须设置 oauth_base_url";

/// 未启用 `auth` feature 但设置了供给。
pub const CLIENT_ASSERTION_NEEDS_AUTH_FEATURE: &str =
    "token_provider: client assertion 需要启用 auth feature";

/// OAuth 换票失败时的消息前缀。
pub const CLIENT_ASSERTION_OAUTH_FAILED_PREFIX: &str = "token_provider: oauth token 换票失败: ";

/// `oauth_base_url` 不是可用的 http(s) 原点。
pub const CLIENT_ASSERTION_OAUTH_BASE_INVALID: &str = "token_provider: oauth_base_url 必须是 http 或 https 原点，路径只能为空或 /，且不能包含用户信息、查询或片段";

/// 外部签发的 RFC 7523 `private_key_jwt` client assertion。
///
/// 对应 Java `retrieveToken(aud)` 取回的 assertion 字符串。SDK 只把 trim 后的
/// 字符串放进 OAuth body，不检查它是不是 JWT。
pub trait ClientAssertionProvider: Send + Sync + std::fmt::Debug {
    /// 按 OAuth audience 取回 assertion。
    ///
    /// `aud` 是 OAuth host，不是完整 URL，也不是 access token。非默认端口时为 `host:port`。
    /// `Ok` 的字符串会先 trim，再放进 body 的 `client_assertion`。
    fn retrieve_assertion<'a>(
        &'a self,
        aud: &'a str,
    ) -> Pin<Box<dyn Future<Output = SDKResult<String>> + Send + 'a>>;
}
