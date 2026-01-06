//! 编辑消息
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message/update

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
};

/// 编辑消息请求体（仅支持 text/post）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageBody {
    pub msg_type: String,
    pub content: String,
}

/// 编辑消息请求
pub struct UpdateMessageRequest {
    config: Config,
    message_id: String,
}

impl UpdateMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 待编辑的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message/update
    pub async fn execute(self, body: UpdateMessageBody) -> SDKResult<serde_json::Value> {
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");
        validate_required!(body.content, "content 不能为空");

        // url: PUT:/open-apis/im/v1/messages/:message_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::put(format!("{}/{}", IM_V1_MESSAGES, self.message_id))
                .body(serialize_params(&body, "编辑消息")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "编辑消息")
    }
}
