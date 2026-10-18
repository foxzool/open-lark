//! WebSocket 公开入口与 endpoint 发现。
//!
//! 会话协议实现见 [`super::session::Session`]。

use std::collections::HashMap;
use std::sync::Arc;

use openlark_core::api::{ApiRequest, ApiResponseTrait};
use openlark_core::constants::AccessTokenType;
use openlark_core::http::Transport;
use serde::Deserialize;
use serde_json::json;
use tokio_tungstenite::connect_async_with_config;
use url::Url;

use super::dispatcher::EventDispatcherHandler;
use super::session::{Session, SessionOptions, WsClientError, WsClientResult};

const END_POINT_URL: &str = "/callback/ws/endpoint";

/// 飞书 WebSocket 客户端入口。
///
/// 连接建立后由内部单一 session loop 拥有：I/O、心跳、控制帧、分包、事件调度与写回。
pub struct LarkWsClient;

impl LarkWsClient {
    /// 建立 WebSocket 长连接并运行完整会话，直到关闭或错误。
    ///
    /// # 返回
    ///
    /// 生产路径在会话终止时几乎总是 `Err`：
    /// - `Err(WsClientError::ConnectionClosed { reason })`：对端 Close（含正常关闭
    ///   code）或入站空闲超时；**正常断开也是 `Err`，调用方请匹配此变体**
    /// - 其它 `Err`：端点查询、传输、malformed 控制帧、未知 frame method、
    ///   非法会话状态等
    ///
    /// 入站空闲超时**仅**在收到 WebSocket 层 `Ping` 时刷新（与历史行为一致）。
    pub async fn open(
        config: Arc<openlark_core::config::Config>,
        event_handler: EventDispatcherHandler,
    ) -> WsClientResult<()> {
        Self::open_with(config, event_handler, SessionOptions::default()).await
    }

    /// 与 [`open`](Self::open) 相同，可注入会话选项（测试用心跳超时等）。
    pub(crate) async fn open_with(
        config: Arc<openlark_core::config::Config>,
        event_handler: EventDispatcherHandler,
        options: SessionOptions,
    ) -> WsClientResult<()> {
        let end_point = Self::get_conn_url(&config).await?;
        let conn_url = end_point.url.ok_or(WsClientError::UnexpectedResponse)?;
        let client_config = end_point
            .client_config
            .ok_or(WsClientError::UnexpectedResponse)?;
        let url = Url::parse(&conn_url)?;
        let query_pairs: HashMap<_, _> = url.query_pairs().into_iter().collect();
        let service_id = query_pairs
            .get("service_id")
            .ok_or(WsClientError::UnexpectedResponse)?
            .parse()
            .map_err(|_| WsClientError::UnexpectedResponse)?;

        let ws_config = tokio_tungstenite::tungstenite::protocol::WebSocketConfig::default()
            .max_message_size(Some(config.max_response_size() as usize))
            .max_frame_size(Some(config.max_response_size() as usize));

        let (conn, _response) = connect_async_with_config(conn_url, Some(ws_config), false).await?;
        log_connected(&url);

        Session::new(service_id, client_config, conn, event_handler, options)
            .run()
            .await
    }

    /// 获取连接配置：经 core `Transport` 发起端点发现 POST（None-token bootstrap，ADR-0003）。
    ///
    /// 收口 ws_client 唯一的 HTTP 出口：自动获得 tracing span / feishu_code 映射 / request_id。
    /// 声明 `[None]` token 类型，避免默认路径拉取并附加多余 access token。缺字段（URL /
    /// client_config / service_id）由 `open_with` 统一报 `UnexpectedResponse`；code!=0 的飞书
    /// 业务错误经 `?` 落入 `WsClientError::RequestError(CoreError)`，透传 request_id。
    async fn get_conn_url(
        config: &Arc<openlark_core::config::Config>,
    ) -> WsClientResult<EndPointResponse> {
        let req = ApiRequest::<()>::post(END_POINT_URL)
            .with_supported_access_token_types(vec![AccessTokenType::None])
            .header("locale", "zh")
            .body(json!({
                "AppID": config.app_id(),
                "AppSecret": config.app_secret(),
            }));
        let end_point =
            Transport::<EndPointResponse>::request_typed(req, config, None, "ws endpoint").await?;
        Ok(end_point)
    }
}

const SENSITIVE_WS_QUERY: &[&str] = &["access_key", "ticket"];

/// 连接日志用的 URL。`access_key` 和 `ticket` 换成 `***`，其余查询参数保留。
pub(crate) fn redact_ws_url(url: &Url) -> String {
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .map(|(key, value)| {
            let key = key.into_owned();
            let value = if is_sensitive_ws_query(&key) {
                "***".to_string()
            } else {
                value.into_owned()
            };
            (key, value)
        })
        .collect();
    if pairs.is_empty() {
        return url.to_string();
    }
    let mut redacted = url.clone();
    redacted.set_query(None);
    {
        let mut query = redacted.query_pairs_mut();
        for (key, value) in &pairs {
            query.append_pair(key, value);
        }
    }
    redacted.to_string()
}

fn is_sensitive_ws_query(key: &str) -> bool {
    SENSITIVE_WS_QUERY
        .iter()
        .any(|name| key.eq_ignore_ascii_case(name))
}

fn log_connected(url: &Url) {
    log_connected_to(url, log::logger());
}

fn log_connected_to(url: &Url, logger: &dyn log::Log) {
    let line = format!("connected to {}", redact_ws_url(url));
    logger.log(
        &log::Record::builder()
            .level(log::Level::Info)
            .target(module_path!())
            .args(format_args!("{line}"))
            .build(),
    );
}

/// WebSocket 端点查询响应（crate 内部）。
#[derive(Debug, Deserialize)]
pub(crate) struct EndPointResponse {
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "ClientConfig")]
    pub client_config: Option<ClientConfig>,
}

// 标准 code/msg/data 信封的 data 字段即本类型；默认 Data 解码路径（ADR-0003）。
impl ApiResponseTrait for EndPointResponse {}

/// 服务端下发的 WebSocket 客户端配置（crate 内部）。
///
/// 会话仅消费 `PingInterval`。endpoint/pong JSON 中可能还带 `Reconnect*` 字段，
/// serde 默认忽略未知键；本 crate 不实现重连策略（#421）。
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ClientConfig {
    #[serde(rename = "PingInterval")]
    pub(crate) ping_interval: i32,
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    struct Capture(Mutex<Vec<String>>);

    impl log::Log for Capture {
        fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
            metadata.level() <= log::Level::Info
        }

        fn log(&self, record: &log::Record<'_>) {
            if self.enabled(record.metadata()) {
                self.0.lock().expect("log").push(record.args().to_string());
            }
        }

        fn flush(&self) {}
    }

    #[test]
    fn ws_connected_log_redacts_access_key_and_ticket() {
        let access_key = "ak_super_secret_value";
        let ticket = "tk_super_secret_value";
        let url = Url::parse(&format!(
            "wss://msg-frontier.feishu.cn/ws/v2?service_id=33554678&access_key={access_key}&ticket={ticket}&device_id=dev-keep"
        ))
        .expect("url");
        let capture = Capture(Mutex::new(Vec::new()));
        log_connected_to(&url, &capture);
        let lines = capture.0.lock().expect("log").clone();
        assert_eq!(lines.len(), 1);
        let line = &lines[0];
        assert!(line.starts_with("connected to "), "{line}");
        assert!(line.contains("service_id=33554678"), "{line}");
        assert!(line.contains("device_id=dev-keep"), "{line}");
        assert!(line.contains("access_key=***"), "{line}");
        assert!(line.contains("ticket=***"), "{line}");
        assert!(!line.contains(access_key), "{line}");
        assert!(!line.contains(ticket), "{line}");

        let mixed = Url::parse(
            "wss://msg-frontier.feishu.cn/ws/v2?Access_Key=AK_CASE&Ticket=TK_CASE&service_id=1",
        )
        .expect("url");
        log_connected_to(&mixed, &capture);
        let lines = capture.0.lock().expect("log").clone();
        let second = &lines[1];
        assert!(!second.contains("AK_CASE"), "{second}");
        assert!(!second.contains("TK_CASE"), "{second}");
        assert!(second.contains("service_id=1"), "{second}");
    }
}
