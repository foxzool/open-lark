use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use lark_websocket_protobuf::pbbp2::{Frame, Header};
use log::{debug, error, info, trace};
use prost::Message as ProstMessage;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{net::TcpStream, sync::mpsc, time::Interval};
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::{frame::coding::CloseCode, Message},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::{
    core::{api_resp::BaseResponse, cache::QuickCache, constants::FEISHU_BASE_URL},
    event::dispatcher::EventDispatcherHandler,
};

const END_POINT_URL: &str = "/callback/ws/endpoint";
/// 业务处理时长，单位ms
const HEADER_BIZ_RT: &str = "biz_rt";
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(120);

pub struct LarkWsClient {
    frame_tx: mpsc::UnboundedSender<Frame>,
    event_rx: mpsc::UnboundedReceiver<WsEvent>,
    cache: QuickCache<Vec<Vec<u8>>>,
}

impl LarkWsClient {
    pub async fn open(
        app_id: &str,
        app_secret: &str,
        event_handler: EventDispatcherHandler,
    ) -> WsClientResult<()> {
        // 创建临时的 Config 以复用 http_client
        let config = crate::core::config::Config {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            ..Default::default()
        };
        let end_point = get_conn_url(&config).await?;
        let conn_url = end_point
            .url
            .ok_or(WsClientError::UnexpectedResponse)?;
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

        let (conn, _response) = connect_async(conn_url).await?;
        info!("connected to {url}");
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
            cache: QuickCache::new(),
        };

        client.handler_loop(event_handler).await;

        Ok(())
    }

    async fn handler_loop(&mut self, event_handler: EventDispatcherHandler) {
        while let Some(ws_event) = self.event_rx.recv().await {
            if let WsEvent::Data(mut frame) = ws_event {
                let headers: &[Header] = frame.headers.as_ref();
                // 拆包数, 未拆包为1
                let sum: usize = headers
                    .iter()
                    .find(|h| h.key == "sum")
                    .and_then(|h| h.value.parse().ok())
                    .unwrap_or(1);
                // 包序号, 未拆包为0
                let seq: usize = headers
                    .iter()
                    .find(|h| h.key == "seq")
                    .and_then(|h| h.value.parse().ok())
                    .unwrap_or(0);
                let msg_type = headers
                    .iter()
                    .find(|h| h.key == "type")
                    .map(|h| h.value.as_str())
                    .unwrap_or("");
                //  消息ID, 拆包后继承
                let msg_id = headers
                    .iter()
                    .find(|h| h.key == "message_id")
                    .map(|h| h.value.as_str())
                    .unwrap_or("");
                // 链路ID
                let trace_id = headers
                    .iter()
                    .find(|h| h.key == "trace_id")
                    .map(|h| h.value.as_str())
                    .unwrap_or("");

                let Some(mut pl) = frame.payload else {
                    error!("Frame payload is empty");
                    continue;
                };
                if sum > 1 {
                    match self.combine(msg_id, sum, seq, &pl) {
                        Some(payload) => {
                            pl = payload;
                        }
                        None => {
                            return;
                        }
                    }
                }
                debug!(
                    "Received a data frame, message type: {}, message_id: {}, payload: {}",
                    msg_type,
                    trace_id,
                    String::from_utf8(pl.clone()).unwrap_or_else(|_| "<binary data>".to_string())
                );
                match msg_type {
                    "event" => {
                        let mut resp = NewWsResponse::new(StatusCode::OK);

                        let start = Instant::now();
                        match event_handler.do_without_validation(pl) {
                            Ok(_) => {
                                let end = start.elapsed().as_millis();
                                frame.headers.push(Header {
                                    key: HEADER_BIZ_RT.to_string(),
                                    value: end.to_string(),
                                });
                            }
                            Err(err) => {
                                error!("handle message failed, message_type: {msg_type}, message_id: {msg_id}, trace_id: {trace_id}, err:  {:?}",  err);
                                resp = NewWsResponse::new(StatusCode::INTERNAL_SERVER_ERROR);
                            }
                        };
                        frame.payload = Some(serde_json::to_vec(&resp).unwrap_or_else(|e| {
                            error!("Failed to serialize response: {:?}", e);
                            vec![]
                        }));

                        if let Err(e) = self.frame_tx.send(frame) {
                            error!("Failed to send frame: {:?}", e);
                        }
                    }
                    "card" => {
                        return;
                    }
                    _ => return,
                }
            }
        }
    }

    fn combine(&mut self, msg_id: &str, sum: usize, seq: usize, bs: &[u8]) -> Option<Vec<u8>> {
        let val = self.cache.get(msg_id);
        if val.is_none() {
            let mut buf = vec![Vec::new(); sum];
            buf[seq] = bs.to_vec();
            self.cache.set(msg_id, buf, 5);
            return None;
        }

        let Some(mut val) = val else {
            return None;
        };
        val[seq] = bs.to_vec();
        let mut pl = Vec::new();
        for v in val.iter() {
            if v.is_empty() {
                self.cache.set(msg_id, val, 5);
                return None;
            }
            pl.extend_from_slice(v);
        }

        Some(pl)
    }
}

/// 获取连接配置
async fn get_conn_url(config: &crate::core::config::Config) -> WsClientResult<EndPointResponse> {
    let body = json!({
        "AppID": &config.app_id,
        "AppSecret": &config.app_secret
    });

    let req = config.http_client
        .post(format!("{FEISHU_BASE_URL}/{END_POINT_URL}"))
        .header("locale", "zh")
        .json(&body)
        .send()
        .await?;

    let resp = req.json::<BaseResponse<EndPointResponse>>().await?;
    debug!("{:?}", resp.data);

    if !resp.success() {
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
    if end_point.url.as_ref().map_or(true, |url| url.is_empty()) {
        return Err(WsClientError::ServerError {
            code: 500,
            message: "No available endpoint".to_string(),
        });
    }

    Ok(end_point)
}

#[derive(Debug, Deserialize)]
pub struct EndPointResponse {
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "ClientConfig")]
    pub client_config: Option<ClientConfig>,
}

#[derive(Debug, Deserialize)]
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
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Prost error: {0}")]
    ProstError(#[from] prost::DecodeError),
}

fn new_ping_frame(service_id: i32) -> Frame {
    let headers = vec![Header {
        key: "type".to_string(),
        value: "ping".to_string(),
    }];
    Frame {
        seq_id: 0,
        log_id: 0,
        service: service_id,
        method: 0,
        headers,
        payload_encoding: Some("".to_string()),
        payload_type: Some("".to_string()),
        payload: None,
        log_id_new: Some("".to_string()),
    }
}

struct Context<'a> {
    service_id: i32,
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
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
                        let frame = new_ping_frame(service_id);
                        let msg = Message::Binary(frame.encode_to_vec());
                        trace!(
                            "Sending ping message:  {:?} {} {}",
                            msg,
                            msg.len(),
                            service_id
                        );
                        if let Err(e) = self.sink.send(msg).await {
                            error!("Failed to send ping message: {:?}", e);
                            return Err(WsClientError::WsError(e));
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

    async fn handle_message(&mut self, msg: Message) -> WsClientResult<()> {
        match msg {
            Message::Ping(data) => {
                self.sink.send(Message::Pong(data)).await?;
            }
            Message::Binary(data) => {
                let frame = Frame::decode(&*data)?;
                trace!("Received frame: {:?}", frame);
                match frame.method {
                    // FrameTypeControl
                    0 => self.handle_control_frame(frame),
                    // FrameTypeData
                    1 => {
                        if let Err(e) = self.event_sender.send(WsEvent::Data(frame)) {
                            error!("Failed to send data event: {:?}", e);
                        }
                    }

                    _ => {}
                }
                // self.send_event(WsEvent::Push(frame));
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

    fn handle_control_frame(&mut self, frame: Frame) {
        let headers = frame.headers;
        let Some(t) = headers.iter().find(|h| h.key == "type") else {
            error!("Control frame missing type header");
            return;
        };
        if t.value == "pong" {
            let Some(payload) = frame.payload else {
                error!("Pong frame missing payload");
                return;
            };
            let config = match serde_json::from_slice::<ClientConfig>(&payload) {
                Ok(cfg) => cfg,
                Err(e) => {
                    error!("Failed to parse ClientConfig: {:?}", e);
                    return;
                }
            };
            self.ping_frame_interval =
                tokio::time::interval(Duration::from_secs(config.ping_interval as u64));
            self.ping_frame_interval
                .reset_after(Duration::from_secs(config.ping_interval as u64));
            self.client_config = config;
        }
    }

    async fn handle_send_frame(&mut self, frame: Frame) -> WsClientResult<()> {
        trace!("send frame: {:?}", frame);
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

#[derive(Serialize)]
struct NewWsResponse {
    code: u16,
    headers: HashMap<String, String>,
    data: Vec<u8>,
}

impl NewWsResponse {
    fn new(code: StatusCode) -> Self {
        Self {
            code: code.as_u16(),
            headers: Default::default(),
            data: Default::default(),
        }
    }
}
