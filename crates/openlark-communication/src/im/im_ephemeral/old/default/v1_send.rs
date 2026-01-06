//! 发送仅特定人可见的消息卡片
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/send-message-cards-that-are-only-visible-to-certain-people

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};

const EPHEMERAL_V1_SEND: &str = "/open-apis/ephemeral/v1/send";

/// 发送仅特定人可见的消息卡片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEphemeralCardBody {
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub msg_type: String,
    pub card: serde_json::Value,
}

/// 发送仅特定人可见的消息卡片响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEphemeralCardResponse {
    pub message_id: String,
}

impl ApiResponseTrait for SendEphemeralCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发送仅特定人可见的消息卡片请求
pub struct SendEphemeralCardRequest {
    config: Config,
}

impl SendEphemeralCardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/send-message-cards-that-are-only-visible-to-certain-people
    pub async fn execute(
        self,
        body: SendEphemeralCardBody,
    ) -> SDKResult<SendEphemeralCardResponse> {
        validate_required!(body.chat_id, "chat_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");

        if body.open_id.is_none() && body.user_id.is_none() && body.email.is_none() {
            return Err(error::validation_error(
                "open_id/user_id/email 不能同时为空".to_string(),
                "open_id、user_id、email 至少需要填写一个".to_string(),
            ));
        }

        // url: POST:/open-apis/ephemeral/v1/send
        let req: ApiRequest<SendEphemeralCardResponse> = ApiRequest::post(EPHEMERAL_V1_SEND)
            .body(serialize_params(&body, "发送仅特定人可见的消息卡片")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "发送仅特定人可见的消息卡片")
    }
}
