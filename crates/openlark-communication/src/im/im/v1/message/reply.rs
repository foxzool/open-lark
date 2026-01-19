//! 回复消息
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message/reply

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
};

/// 回复消息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyMessageBody {
    /// 消息内容（JSON 字符串）
    pub content: String,
    /// 消息类型
    pub msg_type: String,
    /// 是否以话题形式回复（可选，默认 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_in_thread: Option<bool>,
    /// 幂等 uuid（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// 回复消息请求
pub struct ReplyMessageRequest {
    config: Config,
    message_id: String,
}

impl ReplyMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 待回复的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message/reply
    pub async fn execute(self, body: ReplyMessageBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: ReplyMessageBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {

        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");
        validate_required!(body.content, "content 不能为空");

        // url: POST:/open-apis/im/v1/messages/:message_id/reply
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/{}/reply", IM_V1_MESSAGES, self.message_id))
                .body(serialize_params(&body, "回复消息")?);

        
        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "回复消息")
}
}
