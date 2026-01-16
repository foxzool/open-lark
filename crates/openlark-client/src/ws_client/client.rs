#![allow(missing_docs)]

use std::collections::HashMap;
use std::time::Duration;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use lark_websocket_protobuf::pbbp2::{Frame, Header};
use log::{debug, error, info, trace};
use prost::Message as ProstMessage;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio::{net::TcpStream, sync::mpsc, time::Instant, time::Interval};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::{frame::coding::CloseCode, Message},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use super::{state_machine::StateMachineEvent, FrameHandler, WebSocketStateMachine};
use openlark_core::{
    api::Response,
    constants::FEISHU_BASE_URL,
    // event::dispatcher::EventDispatcherHandler, // TODO: 需要实现 event 模块
};

/// 分包消息缓存（按 message_id 聚合）
#[derive(Debug, Default)]
struct FramePackageBuffer {
    sum: usize,
    parts: Vec<Option<Vec<u8>>>,
    received: usize,
}

impl FramePackageBuffer {
    fn new(sum: usize) -> Self {
        Self {
            sum,
            parts: vec![None; sum],
            received: 0,
        }
    }

    fn insert_part(&mut self, seq: usize, payload: Vec<u8>) {
        if seq >= self.sum {
            return;
        }

        if self.parts[seq].is_none() {
            self.received = self.received.saturating_add(1);
        }
        self.parts[seq] = Some(payload);
    }

    fn is_complete(&self) -> bool {
        self.sum > 0 && self.received == self.sum && self.parts.iter().all(|p| p.is_some())
    }

    fn combine(self) -> Vec<u8> {
        let total_len: usize = self
            .parts
            .iter()
            .filter_map(|p| p.as_ref().map(|v| v.len()))
            .sum();
        let mut out = Vec::with_capacity(total_len);
        for part in self.parts.into_iter().flatten() {
            out.extend_from_slice(&part);
        }
        out
    }
}

// 临时类型占位符，等待 event 模块实现
#[derive(Debug, Clone)]
pub struct EventDispatcherHandler;

impl EventDispatcherHandler {
    pub fn builder() -> Self {
        Self
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn do_without_validation(&self, _payload: &[u8]) -> Result<(), String> {
        // 临时实现，等待实际的 event 模块实现
        Ok(())
    }
}

const END_POINT_URL: &str = "/callback/ws/endpoint";
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(120);

pub struct LarkWsClient {
    frame_tx: mpsc::UnboundedSender<Frame>,
    event_rx: mpsc::UnboundedReceiver<WsEvent>,
    state_machine: WebSocketStateMachine,
    package_buffers: HashMap<String, FramePackageBuffer>,
}

impl LarkWsClient {
    #[cfg(test)]
    pub fn new_for_test() -> Self {
        let (frame_tx, _frame_rx) = mpsc::unbounded_channel();
        let (_event_tx, event_rx) = mpsc::unbounded_channel();

        Self {
            frame_tx,
            event_rx,
            state_machine: WebSocketStateMachine::new(),
            package_buffers: HashMap::new(),
        }
    }

    pub async fn open(
        config: std::sync::Arc<crate::config::Config>,
        event_handler: EventDispatcherHandler,
    ) -> WsClientResult<()> {
        let end_point = Self::get_conn_url(&config).await?;
        let conn_url = end_point.url.ok_or(WsClientError::UnexpectedResponse)?;
        let client_config = end_point
            .client_config
            .ok_or(WsClientError::UnexpectedResponse)?;
        let url = Url::parse(&conn_url)?;
        let query_pairs: HashMap<_, _> = url.query_pairs().into_iter().collect();
        // let conn_id = query_pairs.get("device_id").unwrap();
        let service_id = query_pairs
            .get("service_id")
            .ok_or(WsClientError::UnexpectedResponse)?
            .parse()
            .map_err(|_| WsClientError::UnexpectedResponse)?;

        // 开始连接状态转换
        let mut state_machine = WebSocketStateMachine::new();
        if let Err(e) = state_machine.handle_event(StateMachineEvent::StartConnection) {
            error!("Failed to transition to connecting state: {e}");
        }

        let (conn, _response) = connect_async(conn_url).await?;
        info!("connected to {url}");

        // 连接成功状态转换
        if let Err(e) = state_machine.handle_event(StateMachineEvent::ConnectionEstablished) {
            error!("Failed to transition to connected state: {e}");
        }
        let (frame_tx, frame_rx) = mpsc::unbounded_channel();
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        tokio::spawn(client_loop(
            service_id,
            client_config,
            conn,
            frame_rx,
            event_tx,
        ));
        let mut client = LarkWsClient {
            frame_tx,
            event_rx,
            state_machine,
            package_buffers: HashMap::new(),
        };

        client.handler_loop(event_handler).await;

        Ok(())
    }

    async fn handler_loop(&mut self, event_handler: EventDispatcherHandler) {
        while let Some(ws_event) = self.event_rx.recv().await {
            if let WsEvent::Data(frame) = ws_event {
                // 更新状态机：收到数据
                if let Err(e) = self
                    .state_machine
                    .handle_event(StateMachineEvent::DataReceived)
                {
                    error!("Failed to handle DataReceived event: {e}");
                }

                // 检查是否可以处理数据
                if !self.state_machine.can_send_data() {
                    debug!(
                        "Cannot process data in current state: {:?}",
                        self.state_machine.current_state()
                    );
                    continue;
                }

                // 处理分包逻辑
                let processed_frame = self.process_frame_packages_internal(frame).await;
                let Some(frame) = processed_frame else {
                    continue;
                };

                // 使用 FrameHandler 处理帧
                // 创建一个临时的事件发送器，因为FrameHandler需要WsEvent类型
                let (temp_tx, mut temp_rx) = mpsc::unbounded_channel::<WsEvent>();
                if let Some(response_frame) =
                    FrameHandler::handle_frame(frame, &event_handler, &temp_tx).await
                {
                    if let Err(e) = self.frame_tx.send(response_frame) {
                        error!("Failed to send response frame: {e:?}");
                    }
                }

                // 处理临时接收器中的任何事件 - 目前FrameHandler不会发送事件到这里，保留以备未来使用
                while let Ok(_ws_event) = temp_rx.try_recv() {
                    // 暂时忽略，因为FrameHandler主要处理控制帧
                }
            }
        }
    }

    /// 处理分包的 Frame，如果需要组合多个包则返回组合后的结果
    #[cfg(test)]
    pub async fn process_frame_packages(&mut self, frame: Frame) -> Option<Frame> {
        self.process_frame_packages_internal(frame).await
    }

    /// 内部处理分包的 Frame，如果需要组合多个包则返回组合后的结果
    async fn process_frame_packages_internal(&mut self, mut frame: Frame) -> Option<Frame> {
        let headers: &[Header] = frame.headers.as_ref();

        fn header_value<'a>(headers: &'a [Header], key: &str) -> Option<&'a str> {
            headers.iter().find(|h| h.key == key).map(|h| h.value.as_str())
        }

        fn header_usize(headers: &[Header], key: &str) -> Option<usize> {
            header_value(headers, key).and_then(|v| v.parse().ok())
        }

        // 拆包数，未拆包为1
        let sum: usize = headers
            .iter()
            .find(|h| h.key == "sum")
            .and_then(|h| h.value.parse().ok())
            .unwrap_or(1);

        // 包序号，未拆包为0
        let seq: usize = header_usize(headers, "seq").unwrap_or(0);

        // 消息ID，拆包后继承
        let msg_id: &str = header_value(headers, "message_id").unwrap_or("");

        let Some(payload) = frame.payload.take() else {
            error!("Frame payload is empty");
            return None;
        };

        let sum = if sum == 0 { 1 } else { sum };

        // 单包：直接透传
        if sum == 1 {
            frame.payload = Some(payload);
            return Some(frame);
        }

        // 分包：需要 message_id 作为聚合键；缺失则降级为单包透传，避免丢消息
        if msg_id.is_empty() {
            debug!("收到分包帧但 message_id 为空，无法聚合，降级为单包处理（sum={sum}, seq={seq}）");
            frame.payload = Some(payload);
            return Some(frame);
        }

        if seq >= sum {
            debug!("收到分包帧但 seq 越界，降级为单包处理（sum={sum}, seq={seq}, message_id={msg_id}）");
            frame.payload = Some(payload);
            return Some(frame);
        }

        // 获取/初始化缓存；sum 不一致时以最新为准重置（防止错配）
        let buffer = self.package_buffers.entry(msg_id.to_string()).or_insert_with(|| {
            debug!("开始聚合分包消息（sum={sum}, message_id={msg_id}）");
            FramePackageBuffer::new(sum)
        });

        if buffer.sum != sum {
            debug!(
                "分包聚合参数变化，重置缓存（old_sum={}, new_sum={}, message_id={msg_id}）",
                buffer.sum, sum
            );
            *buffer = FramePackageBuffer::new(sum);
        }

        buffer.insert_part(seq, payload);

        if !buffer.is_complete() {
            return None;
        }

        let buffer = self
            .package_buffers
            .remove(msg_id)
            .unwrap_or_else(|| FramePackageBuffer::new(sum));

        frame.payload = Some(buffer.combine());
        Some(frame)
    }

    /// 获取连接配置
    async fn get_conn_url(
        config: &std::sync::Arc<crate::config::Config>,
    ) -> WsClientResult<EndPointResponse> {
        let body = json!({
            "AppID": &config.app_id,
            "AppSecret": &config.app_secret
        });

        let http_client = Client::builder().timeout(config.timeout).build()?;

        let req = http_client
            .post(format!("{FEISHU_BASE_URL}/{END_POINT_URL}"))
            .header("locale", "zh")
            .json(&body)
            .send()
            .await?;

        let resp = req.json::<Response<EndPointResponse>>().await?;
        debug!("{:?}", resp.data);

        if !resp.is_success() {
            return match resp.raw_response.code {
                1 => Err(WsClientError::ServerError {
                    code: resp.raw_response.code,
                    message: resp.raw_response.msg,
                }),
                1000040343 => Err(WsClientError::ServerError {
                    code: resp.raw_response.code,
                    message: resp.raw_response.msg,
                }),
                _ => Err(WsClientError::ClientError {
                    code: resp.raw_response.code,
                    message: resp.raw_response.msg,
                }),
            };
        }

        let end_point = resp.data.ok_or(WsClientError::UnexpectedResponse)?;
        if end_point.url.as_ref().is_none_or(|url| url.is_empty()) {
            return Err(WsClientError::ServerError {
                code: 500,
                message: "No available endpoint".to_string(),
            });
        }

        Ok(end_point)
    }
}

#[derive(Debug, Deserialize)]
pub struct EndPointResponse {
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "ClientConfig")]
    pub client_config: Option<ClientConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientConfig {
    #[serde(rename = "ReconnectCount")]
    pub reconnect_count: i32,
    #[serde(rename = "ReconnectInterval")]
    pub reconnect_interval: i32,
    #[serde(rename = "ReconnectNonce")]
    pub reconnect_nonce: i32,
    #[serde(rename = "PingInterval")]
    pub ping_interval: i32,
}

pub type WsClientResult<T> = Result<T, WsClientError>;

#[derive(Debug, thiserror::Error)]
/// WebSocket客户端错误类型
///
/// 定义WebSocket连接和通信过程中可能出现的各种错误
pub enum WsClientError {
    #[error("unexpected response")]
    UnexpectedResponse,
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Server error: {code}, {message}")]
    ServerError { code: i32, message: String },
    #[error("Client error: {code}, {message}")]
    ClientError { code: i32, message: String },
    #[error("connection closed")]
    ConnectionClosed {
        /// The reason the connection was closed
        reason: Option<WsCloseReason>,
    },
    #[error("WebSocket error: {0}")]
    WsError(Box<tokio_tungstenite::tungstenite::Error>),
    #[error("Prost error: {0}")]
    ProstError(#[from] prost::DecodeError),
}

impl From<tokio_tungstenite::tungstenite::Error> for WsClientError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        WsClientError::WsError(Box::new(error))
    }
}

struct Context<'a> {
    service_id: i32,
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>,
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    command_rx: &'a mut mpsc::UnboundedReceiver<Frame>,
    event_sender: &'a mut mpsc::UnboundedSender<WsEvent>,
    client_config: ClientConfig,
    ping_frame_interval: Interval,
}

impl<'a> Context<'a> {
    fn new(
        service_id: i32,
        client_config: ClientConfig,
        conn: WebSocketStream<MaybeTlsStream<TcpStream>>,
        command_rx: &'a mut mpsc::UnboundedReceiver<Frame>,
        event_sender: &'a mut mpsc::UnboundedSender<WsEvent>,
    ) -> Self {
        let (sink, stream) = conn.split();
        Context {
            service_id,
            sink,
            stream,
            command_rx,
            event_sender,
            ping_frame_interval: tokio::time::interval(Duration::from_secs(
                client_config.ping_interval as u64,
            )),
            client_config,
        }
    }

    fn send_event(&mut self, event: WsEvent) {
        let _ = self.event_sender.send(event);
    }

    async fn process_loop(&mut self) -> WsClientResult<()> {
        let mut ping_time = Instant::now();
        let mut checkout_timeout = tokio::time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                item = self.stream.next() => {
                    match item.transpose()? {
                        Some(msg) => {
                            if msg.is_ping() {
                                ping_time = Instant::now();
                            }
                            self.handle_message(msg).await?;
                        },
                        None => return Err(WsClientError::ConnectionClosed { reason: None}),
                    }
                }
                item = self.command_rx.recv() => {
                    match item {
                        Some(command) => self.handle_send_frame(command).await?,
                        None => return Ok(()),
                    }
                }
                _ = self.ping_frame_interval.tick() => {
                        let service_id: i32 = self.service_id;
                        let frame = FrameHandler::build_ping_frame(service_id);
                        let msg = Message::Binary(frame.encode_to_vec());
                        trace!(
                            "Sending ping message:  {:?} {} {}",
                            msg,
                            msg.len(),
                            service_id
                        );
                        if let Err(e) = self.sink.send(msg).await {
                            error!("Failed to send ping message: {e:?}");
                            return Err(WsClientError::WsError(Box::new(e)));
                        }

                }

                _ = checkout_timeout.tick() => {
                    if (Instant::now() - ping_time) > HEARTBEAT_TIMEOUT {
                        return Err(WsClientError::ConnectionClosed {
                            reason: None
                        });
                    }
                }
            }
        }
    }

    async fn handle_message(&mut self, msg: WsMessage) -> WsClientResult<()> {
        match msg {
            Message::Ping(data) => {
                self.sink.send(Message::Pong(data)).await?;
            }
            Message::Binary(data) => {
                let frame = Frame::decode(&*data)?;
                trace!("Received frame: {frame:?}");

                match frame.method {
                    // FrameTypeControl
                    0 => {
                        // 直接在本地处理控制帧，避免跨线程Send问题
                        self.handle_control_frame_direct(frame)?;
                    }
                    // FrameTypeData
                    1 => {
                        if let Err(e) = self.event_sender.send(WsEvent::Data(frame)) {
                            error!("Failed to send data event: {e:?}");
                        }
                    }
                    _ => {}
                }
            }
            Message::Close(Some(close_frame)) => {
                return Err(WsClientError::ConnectionClosed {
                    reason: Some(WsCloseReason {
                        code: close_frame.code,
                        message: close_frame.reason.into_owned(),
                    }),
                });
            }
            _ => return Err(WsClientError::UnexpectedResponse),
        }

        Ok(())
    }

    /// 直接处理控制帧
    fn handle_control_frame_direct(&mut self, frame: Frame) -> WsClientResult<()> {
        let headers = &frame.headers;
        let frame_type = headers
            .iter()
            .find(|h| h.key == "type")
            .map(|h| h.value.as_str())
            .unwrap_or("");

        trace!("Received control frame: {frame_type}");

        if frame_type == "pong" {
            self.handle_pong_frame_direct(frame)?;
        }

        Ok(())
    }

    /// 处理Pong帧
    fn handle_pong_frame_direct(&mut self, frame: Frame) -> WsClientResult<()> {
        let Some(payload) = frame.payload else {
            error!("Pong frame missing payload");
            return Ok(());
        };

        let config = match serde_json::from_slice::<ClientConfig>(&payload) {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to parse ClientConfig: {e:?}");
                return Ok(());
            }
        };

        // 更新ping间隔
        let ping_interval = config.ping_interval;
        self.ping_frame_interval = tokio::time::interval(Duration::from_secs(ping_interval as u64));
        self.ping_frame_interval
            .reset_after(Duration::from_secs(ping_interval as u64));
        self.client_config = config;

        debug!("Updated ping interval from pong response: {ping_interval}s");

        Ok(())
    }

    async fn handle_send_frame(&mut self, frame: Frame) -> WsClientResult<()> {
        trace!("send frame: {frame:?}");
        let msg = Message::Binary(frame.encode_to_vec());

        self.sink.send(msg).await?;
        Ok(())
    }
}

async fn client_loop(
    service_id: i32,
    client_config: ClientConfig,
    conn: WebSocketStream<MaybeTlsStream<TcpStream>>,
    mut frame_tx: mpsc::UnboundedReceiver<Frame>,
    mut event_sender: mpsc::UnboundedSender<WsEvent>,
) {
    let mut ctx = Context::new(
        service_id,
        client_config,
        conn,
        &mut frame_tx,
        &mut event_sender,
    );

    let res = ctx.process_loop().await;
    match res {
        Ok(()) => (),
        Err(err) => {
            ctx.send_event(WsEvent::Error(err));
        }
    };
}

#[derive(Debug)]
/// WebSocket事件类型
///
/// 定义WebSocket连接中可能接收到的各种事件
pub enum WsEvent {
    Error(WsClientError),
    Data(Frame),
}

/// Connection close reason
#[derive(Debug)]
pub struct WsCloseReason {
    /// Close code
    pub code: CloseCode,

    /// Reason string
    pub message: String,
}
