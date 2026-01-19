//! 转发消息
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message/forward

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::ReceiveIdType,
};

/// 转发消息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardMessageBody {
    pub receive_id: String,
}

/// 转发消息请求
pub struct ForwardMessageRequest {
    config: Config,
    message_id: String,
    receive_id_type: Option<ReceiveIdType>,
    uuid: Option<String>,
}

impl ForwardMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            receive_id_type: None,
            uuid: None,
        }
    }

    /// 待转发的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 消息接收者 ID 类型（查询参数，必填）
    pub fn receive_id_type(mut self, receive_id_type: ReceiveIdType) -> Self {
        self.receive_id_type = Some(receive_id_type);
        self
    }

    /// 幂等 uuid（查询参数，可选）
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message/forward
    pub async fn execute(self, body: ForwardMessageBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: ForwardMessageBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(body.receive_id, "receive_id 不能为空");
        let receive_id_type = self.receive_id_type.ok_or_else(|| {
            error::validation_error(
                "receive_id_type 不能为空".to_string(),
                "转发消息需要指定 receive_id_type".to_string(),
            )
        })?;

        // url: POST:/open-apis/im/v1/messages/:message_id/forward
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/{}/forward", IM_V1_MESSAGES, self.message_id))
                .query("receive_id_type", receive_id_type.as_str())
                .body(serialize_params(&body, "转发消息")?);

        if let Some(uuid) = self.uuid {
            req = req.query("uuid", uuid);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "转发消息")
    }
}
