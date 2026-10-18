//! client assertion 换票。
//!
//! 只由 `AuthTokenProvider::get_token` 的断言分支调用。不经过 `Transport`，
//! 因此没有 SDK `User-Agent`，也不走统一重试和 flatten 解码。
//! 请求超时仍取 `Config::req_timeout`，响应体仍按 `Config::max_response_size` 截断。

use openlark_core::{
    SDKResult,
    auth::{
        CLIENT_ASSERTION_CUSTOM_BASE_URL, CLIENT_ASSERTION_OAUTH_BASE_INVALID,
        CLIENT_ASSERTION_OAUTH_FAILED_PREFIX,
    },
    config::Config,
    constants::{FEISHU_BASE_URL, LARK_BASE_URL},
    error::configuration_error,
};
use serde::Serialize;

const GRANT_TYPE_JWT_BEARER: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";
const CLIENT_ASSERTION_TYPE_JWT_BEARER: &str =
    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";
const OAUTH_TOKEN_PATH: &str = "/oauth/v3/token";

/// 解析后的 OAuth 换票端点。不公开。
#[derive(Debug)]
pub(crate) struct OauthEndpoint {
    pub(crate) token_url: String,
    pub(crate) aud: String,
}

/// 断言通路的缓存键。不含 `app_id`、`tenant_key`、`app_ticket`。
pub(crate) fn assertion_cache_key(aud: &str) -> String {
    format!("client_assertion:{aud}")
}

/// 构建期检查：供给已设置时，OAuth 端点必须能解析。
///
/// 策略只写在这一处解析里，包装本身不另做判断。
pub fn ensure_client_assertion_endpoint(
    base_url: &str,
    oauth_base_url: Option<&str>,
) -> SDKResult<()> {
    resolve_oauth_endpoint(base_url, oauth_base_url).map(|_| ())
}

pub(crate) fn resolve_oauth_endpoint(
    base_url: &str,
    oauth_base_url: Option<&str>,
) -> SDKResult<OauthEndpoint> {
    if let Some(explicit) = oauth_base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return endpoint_from_explicit(explicit);
    }

    let normalized = base_url.trim().trim_end_matches('/');
    if normalized == FEISHU_BASE_URL {
        return Ok(known_endpoint(
            "https://accounts.feishu.cn",
            "accounts.feishu.cn",
        ));
    }
    if normalized == LARK_BASE_URL {
        return Ok(known_endpoint(
            "https://accounts.larksuite.com",
            "accounts.larksuite.com",
        ));
    }
    Err(configuration_error(CLIENT_ASSERTION_CUSTOM_BASE_URL))
}

fn known_endpoint(origin: &str, aud: &str) -> OauthEndpoint {
    OauthEndpoint {
        token_url: format!("{origin}{OAUTH_TOKEN_PATH}"),
        aud: aud.to_string(),
    }
}

fn endpoint_from_explicit(raw: &str) -> SDKResult<OauthEndpoint> {
    let url = url::Url::parse(raw).map_err(|_| invalid_oauth_base())?;
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(invalid_oauth_base());
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(invalid_oauth_base());
    }
    if url.query().is_some() || url.fragment().is_some() {
        return Err(invalid_oauth_base());
    }
    let path = url.path();
    if path != "/" && !path.is_empty() {
        return Err(invalid_oauth_base());
    }
    let host = url.host_str().ok_or_else(invalid_oauth_base)?;
    let aud = match url.port() {
        Some(port) => format!("{host}:{port}"),
        None => host.to_string(),
    };
    let origin = url.origin().ascii_serialization();
    if origin == "null" {
        return Err(invalid_oauth_base());
    }
    Ok(OauthEndpoint {
        token_url: format!("{origin}{OAUTH_TOKEN_PATH}"),
        aud,
    })
}

fn invalid_oauth_base() -> openlark_core::CoreError {
    configuration_error(CLIENT_ASSERTION_OAUTH_BASE_INVALID)
}

#[derive(Serialize)]
struct JwtBearerTokenRequest<'a> {
    grant_type: &'static str,
    client_assertion_type: &'static str,
    client_assertion: &'a str,
    client_id: &'a str,
}

/// 用绝对 URL 换 tenant access token。成功体是 OAuth JSON，不是飞书 envelope。
pub(crate) async fn exchange_tenant_token(
    config: &Config,
    token_url: &str,
    assertion: &str,
) -> SDKResult<(String, i64)> {
    let body = JwtBearerTokenRequest {
        grant_type: GRANT_TYPE_JWT_BEARER,
        client_assertion_type: CLIENT_ASSERTION_TYPE_JWT_BEARER,
        client_assertion: assertion,
        client_id: config.app_id(),
    };
    let response = post_json(config, token_url, &body).await?;
    parse_oauth_token(response.status, &response.body)
}

struct RawHttpResponse {
    status: u16,
    body: String,
}

async fn post_json(
    config: &Config,
    token_url: &str,
    body: &JwtBearerTokenRequest<'_>,
) -> SDKResult<RawHttpResponse> {
    let mut request = config.http_client().post(token_url).json(body);
    if let Some(timeout) = config.req_timeout() {
        request = request.timeout(timeout);
    }
    let response = request.send().await?;
    let status = response.status().as_u16();
    let bytes =
        openlark_core::http::read_body_with_limit(response, config.max_response_size()).await?;
    let body = String::from_utf8(bytes).map_err(|err| {
        openlark_core::error::network_error(format!("oauth token 响应不是 UTF-8: {err}"))
    })?;
    Ok(RawHttpResponse { status, body })
}

fn parse_oauth_token(status: u16, body: &str) -> SDKResult<(String, i64)> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(body) else {
        return Err(oauth_failure(status.to_string()));
    };
    let token = value
        .get("access_token")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    let expires_in = value
        .get("expires_in")
        .and_then(serde_json::Value::as_i64)
        .filter(|seconds| *seconds >= 0);
    if !token.is_empty()
        && let Some(expires_in) = expires_in
    {
        return Ok((token.to_string(), expires_in));
    }

    let detail = string_field(&value, "error_description")
        .or_else(|| string_field(&value, "error"))
        .unwrap_or_else(|| status.to_string());
    Err(oauth_failure(detail))
}

fn string_field(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

fn oauth_failure(detail: String) -> openlark_core::CoreError {
    configuration_error(format!("{CLIENT_ASSERTION_OAUTH_FAILED_PREFIX}{detail}"))
}
