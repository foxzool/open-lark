//! RFC 8628 设备码注册应用（最小核心，#695）。
//!
//! 服务「还没有应用凭证」的调用方：`begin` 发起设备授权拿到扫码 URL，
//! 用户扫码确认后 `poll` 轮询拿回 `client_id` / `client_secret`。
//! 已持有应用凭证的配置与请求路径不受影响。
//!
//! wire 权威：oapi-sdk-go v3.12.2 `scene/registration`
//! （`POST {accounts}/oauth/v1/app/registration`，form 编码，账号域默认
//! `https://accounts.feishu.cn`，Lark 域名切换不在本版范围）。
//!
//! API 形态为拆开的 `begin` / `poll`：调用方自己按 `interval_secs` 睡眠重试；
//! 轮询结果用 [`PollOutcome`] 表达「还没好」的续态（pending / slow_down
//! 不是错误），终态拒绝与过期为独立的 [`RegistrationError`] 变体。

use std::fmt;

use openlark_core::{config::Config, error::CoreError};
use serde::Deserialize;

use crate::token_provider::resolve_accounts_origin;

/// 设备注册端点路径（账号域下）
const REGISTRATION_PATH: &str = "/oauth/v1/app/registration";
/// begin 请求固定的应用原型与授权方式（对齐官方 SDK）
const BEGIN_ARCHETYPE: &str = "PersonalAgent";
const BEGIN_AUTH_METHOD: &str = "client_secret";
const BEGIN_REQUEST_USER_INFO: &str = "open_id";
/// 扫码 URL 追加的 SDK 标识（对齐官方 from/tp/source 装饰）
const QR_FROM: &str = "sdk";
const QR_TP: &str = "sdk";
const QR_SOURCE: &str = "openlark";
/// 服务端未给 interval / expire_in 时的兜底值（对齐官方 SDK）
const DEFAULT_INTERVAL_SECS: u32 = 5;
const DEFAULT_EXPIRE_IN_SECS: u32 = 600;
/// slow_down 的建议间隔增量（RFC 8628 / 官方实现均为 +5s）
const SLOW_DOWN_STEP_SECS: u32 = 5;

/// 设备码注册的终态错误（可区分变体，#695 决策 3）。
#[derive(Debug)]
pub enum RegistrationError {
    /// 用户或服务端拒绝授权（`access_denied`）。
    AccessDenied {
        /// 服务端描述
        description: String,
    },
    /// 设备码过期（`expired_token`）。
    ExpiredToken {
        /// 服务端描述
        description: String,
    },
    /// 响应不可用：空 body、缺字段、非法 JSON、非 2xx 或未知错误码。
    InvalidResponse {
        /// 问题描述
        description: String,
    },
    /// 网络层错误（发送/读取失败，含超时）。
    Network {
        /// core 网络错误
        source: CoreError,
    },
    /// 端点解析失败：自定义 base_url 未配 `oauth_base_url` 显式原点，
    /// 或显式原点本身非法。
    Configuration {
        /// 具体的配置错误
        source: CoreError,
    },
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccessDenied { description } => {
                write!(
                    f,
                    "device_registration: 授权被拒绝（access_denied）: {description}"
                )
            }
            Self::ExpiredToken { description } => {
                write!(
                    f,
                    "device_registration: 设备码已过期（expired_token）: {description}"
                )
            }
            Self::InvalidResponse { description } => {
                write!(f, "device_registration: 响应不可用: {description}")
            }
            Self::Network { source } => {
                write!(f, "device_registration: 网络错误: {source}")
            }
            Self::Configuration { source } => {
                write!(f, "device_registration: 端点配置错误: {source}")
            }
        }
    }
}

impl std::error::Error for RegistrationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Network { source } => Some(source),
            Self::Configuration { source } => Some(source),
            _ => None,
        }
    }
}

/// `begin` 成功后交给调用方的授权句柄：渲染二维码用 `qr_url`，
/// 轮询时原样传回 [`DeviceRegistration::poll`]。
#[derive(Debug, Clone)]
pub struct DeviceAuthorization {
    /// 设备码（poll 凭证）
    pub device_code: String,
    /// 渲染二维码用的完整验证 URL（已追加 from/tp/source 标识）
    pub qr_url: String,
    /// 授权有效期（秒），服务端缺省时为 600
    pub expire_in_secs: u32,
    /// 建议轮询间隔（秒），服务端缺省时为 5
    pub interval_secs: u32,
}

/// `poll` 的轮询结果：续态不是错误（#695 决策 3）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PollOutcome {
    /// 授权完成，应用凭证就绪
    Ready(AppCredentials),
    /// 用户尚未完成授权，按建议间隔继续轮询
    Pending,
    /// 轮询过快：按返回的（已 +5s 的）间隔降速后继续
    SlowDown {
        /// 降速后的建议轮询间隔（秒）
        interval_secs: u32,
    },
}

/// 注册成功返回的应用凭证。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppCredentials {
    /// 应用 app id
    pub client_id: String,
    /// 应用 app secret
    pub client_secret: String,
    /// 扫码用户的 open_id（服务端可能不给）
    pub user_open_id: Option<String>,
    /// 扫码用户所属租户品牌（服务端可能不给）
    pub tenant_brand: Option<String>,
}

/// RFC 8628 设备码注册客户端（最小核心）。
#[derive(Debug, Clone)]
pub struct DeviceRegistration {
    config: Config,
    registration_url: String,
}

impl DeviceRegistration {
    /// 从 `Config` 构造：账号域解析复用 client assertion 的
    /// `oauth_base_url`（显式原点）→ 已知域（feishu/lark）→ 自定义 base_url
    /// 需显式配置的规则。
    pub fn new(config: &Config) -> Result<Self, RegistrationError> {
        let origin = resolve_accounts_origin(config.base_url(), config.oauth_base_url())
            .map_err(|source| RegistrationError::Configuration { source })?;
        Ok(Self {
            config: config.clone(),
            registration_url: format!("{origin}{REGISTRATION_PATH}"),
        })
    }

    /// 发起设备授权：返回扫码 URL 与轮询句柄。
    pub async fn begin(&self) -> Result<DeviceAuthorization, RegistrationError> {
        let resp = self
            .post_form(&[
                ("action", "begin"),
                ("archetype", BEGIN_ARCHETYPE),
                ("auth_method", BEGIN_AUTH_METHOD),
                ("request_user_info", BEGIN_REQUEST_USER_INFO),
            ])
            .await?;
        if resp.device_code.is_empty() {
            return Err(RegistrationError::InvalidResponse {
                description: "device_code is empty".to_string(),
            });
        }
        if resp.verification_uri_complete.is_empty() {
            return Err(RegistrationError::InvalidResponse {
                description: "verification_uri_complete is empty".to_string(),
            });
        }
        let interval_secs = normalize(resp.interval, DEFAULT_INTERVAL_SECS);
        Ok(DeviceAuthorization {
            qr_url: decorate_qr_url(&resp.verification_uri_complete),
            expire_in_secs: normalize(resp.expire_in, DEFAULT_EXPIRE_IN_SECS),
            interval_secs,
            device_code: resp.device_code,
        })
    }

    /// 轮询授权状态：`Ready` 拿凭证；`Pending`/`SlowDown` 按建议间隔重试；
    /// 拒绝与过期为终态错误。服务端无 error 也无凭证时按官方行为返回
    /// `Pending`（Node 对齐）。
    ///
    /// `SlowDown` 会把句柄的 `interval_secs` 累计 +5s（对齐官方实现的
    /// 递增语义），后续轮询直接用句柄值即可。
    pub async fn poll(
        &self,
        authorization: &mut DeviceAuthorization,
    ) -> Result<PollOutcome, RegistrationError> {
        let resp = self
            .post_form(&[
                ("action", "poll"),
                ("device_code", authorization.device_code.as_str()),
            ])
            .await?;

        if !resp.client_id.is_empty() && !resp.client_secret.is_empty() {
            let user_info = resp.user_info.unwrap_or_default();
            return Ok(PollOutcome::Ready(AppCredentials {
                client_id: resp.client_id,
                client_secret: resp.client_secret,
                user_open_id: user_info.open_id,
                tenant_brand: user_info.tenant_brand,
            }));
        }

        match resp.error.as_str() {
            "" | "authorization_pending" => Ok(PollOutcome::Pending),
            "slow_down" => {
                authorization.interval_secs = authorization
                    .interval_secs
                    .saturating_add(SLOW_DOWN_STEP_SECS);
                Ok(PollOutcome::SlowDown {
                    interval_secs: authorization.interval_secs,
                })
            }
            "access_denied" => Err(RegistrationError::AccessDenied {
                description: resp.error_description.unwrap_or_default(),
            }),
            "expired_token" => Err(RegistrationError::ExpiredToken {
                description: resp.error_description.unwrap_or_default(),
            }),
            other => Err(RegistrationError::InvalidResponse {
                description: format!(
                    "未知错误码 {other}: {}",
                    resp.error_description.unwrap_or_default()
                ),
            }),
        }
    }

    async fn post_form(
        &self,
        form: &[(&str, &str)],
    ) -> Result<RegistrationResponse, RegistrationError> {
        let http_resp = self
            .config
            .http_client()
            .post(&self.registration_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serialize_form(form))
            .send()
            .await
            .map_err(|source| RegistrationError::Network {
                source: openlark_core::error::network_error(source.to_string()),
            })?;
        let status = http_resp.status();
        let bytes =
            openlark_core::http::read_body_with_limit(http_resp, self.config.max_response_size())
                .await
                .map_err(|source| RegistrationError::Network { source })?;
        let body = String::from_utf8_lossy(&bytes).to_string();
        if !status.is_success() {
            return Err(RegistrationError::InvalidResponse {
                description: format!("http status {status}"),
            });
        }
        if body.trim().is_empty() {
            return Err(RegistrationError::InvalidResponse {
                description: "empty response body".to_string(),
            });
        }
        serde_json::from_str::<RegistrationResponse>(&body).map_err(|source| {
            RegistrationError::InvalidResponse {
                description: format!("decode response failed: {source}"),
            }
        })
    }
}

/// begin 与 poll 共用的宽松响应体（字段见 oapi-sdk-go types.go）
#[derive(Debug, Deserialize)]
struct RegistrationResponse {
    #[serde(default)]
    device_code: String,
    #[serde(default)]
    verification_uri_complete: String,
    #[serde(default)]
    interval: i64,
    #[serde(default)]
    expire_in: i64,
    #[serde(default)]
    client_id: String,
    #[serde(default)]
    client_secret: String,
    #[serde(default)]
    user_info: Option<RegistrationUserInfo>,
    #[serde(default)]
    error: String,
    #[serde(default)]
    error_description: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct RegistrationUserInfo {
    #[serde(default)]
    open_id: Option<String>,
    #[serde(default)]
    tenant_brand: Option<String>,
}

/// form 编码（core 的 reqwest 裁掉了 .form()，用 url crate 序列化）
fn serialize_form(form: &[(&str, &str)]) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    for (key, value) in form {
        serializer.append_pair(key, value);
    }
    serializer.finish()
}

fn normalize(value: i64, default: u32) -> u32 {
    if value <= 0 {
        default
    } else {
        u32::try_from(value).unwrap_or(default)
    }
}

/// 给验证 URL 设置 from/tp/source 标识（覆盖同名键，保留其余 query；
/// 解析失败时原样返回——标识仅用于 SDK 归因，非功能性，二维码必须可用）。
fn decorate_qr_url(raw: &str) -> String {
    match url::Url::parse(raw) {
        Ok(mut parsed) => {
            let mut kept: Vec<(String, String)> = parsed
                .query_pairs()
                .filter(|(key, _)| !matches!(key.as_ref(), "from" | "tp" | "source"))
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect();
            kept.push(("from".into(), QR_FROM.into()));
            kept.push(("tp".into(), QR_TP.into()));
            kept.push(("source".into(), QR_SOURCE.into()));
            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            for (key, value) in kept {
                serializer.append_pair(&key, &value);
            }
            parsed.set_query(Some(&serializer.finish()));
            parsed.to_string()
        }
        Err(_) => raw.to_string(),
    }
}

#[cfg(test)]
mod tests;
