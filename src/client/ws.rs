use std::collections::HashMap;

use futures_util::{future, pin_mut, StreamExt};
use log::debug;
use serde::Deserialize;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use crate::core::{api_resp::BaseResponse, constants::FEISHU_BASE_URL};

const END_POINT_URL: &str = "/callback/ws/endpoint";

#[derive(Debug)]
pub struct LarkWsClient {
    app_id: String,
    app_secret: String,
    auto_reconnect: bool,
    domain: String,
    reconnect_count: i32,
    reconnect_interval: i32,
    reconnect_nonce: i32,
    ping_interval: i32,
    conn_url: String,
    conn_id: String,
    service_id: String,
}

impl LarkWsClient {
    pub fn new(app_id: impl ToString, app_secret: impl ToString) -> Self {
        Self {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            auto_reconnect: true,
            domain: FEISHU_BASE_URL.to_string(),
            reconnect_count: 30,
            reconnect_interval: -1,
            reconnect_nonce: 2 * 60,
            ping_interval: 2 * 60,
            conn_url: "".to_string(),
            conn_id: "".to_string(),
            service_id: "".to_string(),
        }
    }

    pub async fn start(&mut self) -> WsResult<()> {
        self.connect().await
    }

    async fn connect(&mut self) -> WsResult<()> {
        let conn_url = self.get_conn().await?;
        let url = Url::parse(&conn_url)?;

        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        tokio::spawn(read_stdin(stdin_tx));

        let query_pairs: HashMap<_, _> = url.query_pairs().into_iter().collect();
        let conn_id = query_pairs.get("device_id").unwrap().to_string();
        let service_id = query_pairs.get("device_id").unwrap().to_string();
        self.conn_id = conn_id;
        self.service_id = service_id;
        self.conn_url = url.to_string();

        let (ws_stream, _response) = connect_async(url).await?;
        let (write, read) = ws_stream.split();



        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;

        Ok(())
    }

    async fn get_conn(&mut self) -> WsResult<String> {
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

        if let Some(client_config) = end_point.client_config {
            self.configure(client_config)
        }

        Ok(end_point.url.unwrap())
    }

    fn configure(&mut self, config: ClientConfig) {
        self.reconnect_count = config.reconnect_count;
        self.reconnect_interval = config.reconnect_interval;
        self.reconnect_nonce = config.reconnect_nonce;
        self.ping_interval = config.ping_interval;
    }
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
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
}
