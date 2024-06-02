use std::{collections::HashMap, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use kanal::AsyncSender;
use log::{debug, error};
use prost::Message as ProstMessage;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use lark_protobuf::pbbp2::{Frame, Header};

use crate::core::{api_resp::BaseResponse, constants::FEISHU_BASE_URL};

const END_POINT_URL: &str = "/callback/ws/endpoint";

#[derive(Debug, Clone)]
pub struct LarkWsClient {
    app_id: String,
    app_secret: String,

    domain: String,
    conn_url: String,
}

impl LarkWsClient {
    pub fn new(app_id: impl ToString, app_secret: impl ToString) -> Self {
        Self {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            domain: FEISHU_BASE_URL.to_string(),
            conn_url: "".to_string(),
        }
    }

    pub async fn start(self) -> WsResult<()> {
        Self::connect(self).await
    }

    async fn connect(mut self) -> WsResult<()> {
        let conn_url = self.get_conn_url().await?;
        let url = Url::parse(&conn_url)?;

        let query_pairs: HashMap<_, _> = url.query_pairs().into_iter().collect();
        let conn_id = query_pairs.get("device_id").unwrap().to_string();
        let service_id = query_pairs.get("service_id").unwrap().to_string();

        self.conn_url = url.to_string();

        let (ws_stream, _response) = connect_async(url).await?;
        let (mut write, read) = ws_stream.split();
        let (sender_tx, sender_rx) = kanal::unbounded_async::<Message>();

        let write_task = async move {
            while let Ok(msg) = sender_rx.recv().await {
                match write.send(msg).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error: {:?}", e);
                    }
                }
            }
        };

        let ws_client = Client::new(conn_id, service_id, sender_tx.clone());

        let client = Arc::new(Mutex::new(ws_client));
        let read_client = Arc::clone(&client);

        let read_task = async move {
            let mut read = read;

            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => {
                        let mut new_client = read_client.lock().await;
                        new_client.handle_message(msg).unwrap();
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
        };

        let ping_client = Arc::clone(&client);

        let ping_task = async move {
            loop {
                let mut ping_client = ping_client.lock().await;
                let service_id: i32 = ping_client.service_id.parse().unwrap();
                let frame = new_frame(service_id);
                let msg = Message::Binary(frame.encode_to_vec());
                debug!("Sending ping message: {:?}", msg);
                ping_client.sender_tx.send(msg).await.unwrap();
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    ping_client.ping_interval as u64,
                ))
                .await;
            }
        };

        tokio::select! {
            _ = write_task => {}
            _ = read_task => {}
            _ = ping_task => {}
        }

        Ok(())
    }

    /// 获取连接地址
    async fn get_conn_url(&mut self) -> WsResult<String> {
        let body = json!({
            "AppID": self.app_id,
            "AppSecret": self.app_secret
        });

        let req = reqwest::Client::new()
            .post(&format!("{}{END_POINT_URL}", self.domain))
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

        let end_point = resp.data.unwrap();
        if end_point.url.is_none() || end_point.url.as_ref().unwrap().is_empty() {
            return Err(WsClientError::ServerError {
                code: 500,
                message: "No available endpoint".to_string(),
            });
        }

        Ok(end_point.url.unwrap())
    }
}

#[derive(Debug, Clone)]
struct Client {
    auto_reconnect: bool,
    reconnect_count: i32,
    reconnect_interval: i32,
    reconnect_nonce: i32,
    ping_interval: i32,
    conn_id: String,
    service_id: String,
    sender_tx: AsyncSender<Message>,
}

impl Client {
    pub fn new(conn_id: String, service_id: String, sender_tx: AsyncSender<Message>) -> Self {
        Self {
            auto_reconnect: true,
            reconnect_count: 30,
            reconnect_interval: -1,
            reconnect_nonce: 2 * 60,
            ping_interval: 2 * 60,
            conn_id,
            service_id,
            sender_tx,
        }
    }

    fn configure(&mut self, config: ClientConfig) {
        self.reconnect_count = config.reconnect_count;
        self.reconnect_interval = config.reconnect_interval;
        self.reconnect_nonce = config.reconnect_nonce;
        self.ping_interval = config.ping_interval;
    }

    fn handle_message(&mut self, message: Message) -> WsResult<()> {
        match message {
            Message::Text(text) => {
                debug!("Received a text message: {:?}", text);
            }
            Message::Binary(bin) => {
                let frame = Frame::decode(&*bin)?;
                debug!("Received a binary message: {:?}", frame);
                match frame.method {
                    // FrameTypeControl
                    0 => self.handle_control_frame(frame),
                    // FrameTypeData
                    1 => self.handle_data_frame(frame),

                    _ => {}
                }
            }
            Message::Ping(ping) => {
                debug!("Received a ping message {:?}", ping);
            }
            Message::Pong(pong) => {
                debug!("Received a pong message {:?}", pong);
            }
            Message::Close(close) => {
                debug!("Received a close message: {:?}", close);
            }
            Message::Frame(frame) => {
                debug!("Received a frame message: {:?}", frame);
            }
        }

        Ok(())
    }

    fn handle_control_frame(&mut self, frame: Frame) {
        let headers = frame.headers;
        let t = headers.iter().find(|h| h.key == "type").unwrap();
        match t.value.as_ref() {
            "pong" => {
                debug!("Received a pong frame");
                let config =
                    serde_json::from_slice::<ClientConfig>(&frame.payload.unwrap()).unwrap();
                self.configure(config);
            }
            _ => {}
        }
    }

    fn handle_data_frame(&mut self, frame: Frame) {
        let headers = frame.headers;
        // 拆包数, 未拆包为1
        let sum: i32 = headers
            .iter()
            .find(|h| h.key == "sum")
            .unwrap()
            .value
            .parse()
            .unwrap();
        // 包序号, 未拆包为0
        let seq: i32 = headers
            .iter()
            .find(|h| h.key == "seq")
            .unwrap()
            .value
            .parse()
            .unwrap();
        let type_ = headers
            .iter()
            .find(|h| h.key == "type")
            .unwrap()
            .value
            .as_str();
        //  消息ID, 拆包后继承
        let message_id = headers
            .iter()
            .find(|h| h.key == "message_id")
            .unwrap()
            .value
            .as_str();
        // 链路ID
        let trace_id = headers
            .iter()
            .find(|h| h.key == "trace_id")
            .unwrap()
            .value
            .as_str();

        let payload = frame.payload.unwrap();
        if sum > 1 {
            debug!("Received a multi-frame message");
        }

        match type_ {
            "data" => {
                debug!("Received a data frame");
            }
            _ => {}
        }
    }
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
    reconnect_count: i32,
    #[serde(rename = "ReconnectInterval")]
    reconnect_interval: i32,
    #[serde(rename = "ReconnectNonce")]
    reconnect_nonce: i32,
    #[serde(rename = "PingInterval")]
    ping_interval: i32,
}

pub type WsResult<T> = Result<T, WsClientError>;

#[derive(Debug, thiserror::Error)]
pub enum WsClientError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Server error: {code}, {message}")]
    ServerError { code: i32, message: String },
    #[error("Client error: {code}, {message}")]
    ClientError { code: i32, message: String },
    #[error("WebSocket error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Prost error: {0}")]
    ProstError(#[from] prost::DecodeError),
}

fn new_frame(service_id: i32) -> Frame {
    let mut headers = vec![];
    headers.push(Header {
        key: "type".to_string(),
        value: "ping".to_string(),
    });
    Frame {
        seq_id: 0,
        log_id: 0,
        service: service_id,
        method: 0,
        headers,
        payload_encoding: None,
        payload_type: None,
        payload: None,
        log_id_new: None,
    }
}
