//! Pin 消息相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// Pin 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pin {
    pub message_id: String,
    pub chat_id: String,
    pub operator_id: String,
    pub operator_id_type: String,
    pub create_time: String,
}

/// Pin 消息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePinBody {
    pub message_id: String,
}

/// Pin 消息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePinResponse {
    pub pin: Pin,
}

impl ApiResponseTrait for CreatePinResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群内 Pin 消息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPinsResponse {
    #[serde(default)]
    pub items: Option<Vec<Pin>>,
    pub has_more: bool,
    #[serde(default)]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListPinsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

