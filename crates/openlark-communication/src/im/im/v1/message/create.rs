//! 发送消息
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message/create

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::ReceiveIdType,
};

/// 发送消息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageBody {
    /// 消息接收者 ID，类型与 receive_id_type 一致
    pub receive_id: String,
    /// 消息类型
    pub msg_type: String,
    /// 消息内容（JSON 字符串）
    pub content: String,
    /// 幂等 uuid（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// 发送消息请求
pub struct CreateMessageRequest {
    config: Config,
    receive_id_type: Option<ReceiveIdType>,
}

impl CreateMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            receive_id_type: None,
        }
    }

    /// 接收者 ID 类型（查询参数，必填）
    pub fn receive_id_type(mut self, receive_id_type: ReceiveIdType) -> Self {
        self.receive_id_type = Some(receive_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message/create
    pub async fn execute(self, body: CreateMessageBody) -> SDKResult<serde_json::Value> {
        validate_required!(body.receive_id, "receive_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");
        validate_required!(body.content, "content 不能为空");
        let receive_id_type = self.receive_id_type.ok_or_else(|| {
            openlark_core::error::validation_error(
                "receive_id_type 不能为空".to_string(),
                "发送消息需要指定 receive_id_type".to_string(),
            )
        })?;

        // url: POST:/open-apis/im/v1/messages
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(IM_V1_MESSAGES)
            .query("receive_id_type", receive_id_type.as_str())
            .body(serialize_params(&body, "发送消息")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "发送消息")
    }
}
