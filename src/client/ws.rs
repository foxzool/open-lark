use crate::core::{api_resp::BaseResponse, constants::FEISHU_BASE_URL, SDKResult};
use serde::Deserialize;
use serde_json::json;

const END_POINT_URL: &str = "/callback/ws/endpoint";

#[derive(Debug)]
pub struct LarkWsClient {
    app_id: String,
    app_secret: String,
    auto_reconnect: bool,
    domain: String,
}

impl LarkWsClient {
    pub fn new(app_id: impl ToString, app_secret: impl ToString) -> Self {
        Self {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            auto_reconnect: true,
            domain: FEISHU_BASE_URL.to_string(),
        }
    }

    pub async fn start(&self) -> SDKResult<()> {
        self.connect().await
    }

    async fn connect(&self) -> SDKResult<()> {
        self.get_conn().await
    }

    async fn get_conn(&self) -> SDKResult<()> {
        let body = json!({
            "AppID": self.app_id,
            "AppSecret": self.app_secret
        });

        let req = reqwest::Client::new()
            .post(&format!("{}{END_POINT_URL}", self.domain))
            .json(&body)
            .send()
            .await?;

        // println!("{:?}", req.text().await.unwrap());
        let resp = req.json::<BaseResponse<EndPointResponse>>().await?;
        println!("{:?}", resp.data);

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct EndPointResponse {
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "ClientConfig")]
    pub client_config: ClientConfig,
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
