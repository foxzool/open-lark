//! 转发话题
//!
//! docPath: https://open.feishu.cn/document/im-v1/message/forward-2

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_THREADS,
    im::im::v1::message::models::ReceiveIdType,
};

/// 转发话题请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardThreadBody {
    pub receive_id: String,
}

/// 转发话题请求
pub struct ForwardThreadRequest {
    config: Config,
    thread_id: String,
    receive_id_type: Option<ReceiveIdType>,
    uuid: Option<String>,
}

impl ForwardThreadRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            thread_id: String::new(),
            receive_id_type: None,
            uuid: None,
        }
    }

    /// 待转发的话题 ID（路径参数）
    pub fn thread_id(mut self, thread_id: impl Into<String>) -> Self {
        self.thread_id = thread_id.into();
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
    /// docPath: https://open.feishu.cn/document/im-v1/message/forward-2
    pub async fn execute(self, body: ForwardThreadBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: ForwardThreadBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.thread_id, "thread_id 不能为空");
        validate_required!(body.receive_id, "receive_id 不能为空");
        let receive_id_type = self.receive_id_type.ok_or_else(|| {
            error::validation_error(
                "receive_id_type 不能为空".to_string(),
                "转发话题需要指定 receive_id_type".to_string(),
            )
        })?;

        // url: POST:/open-apis/im/v1/threads/:thread_id/forward
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/{}/forward", IM_V1_THREADS, self.thread_id))
                .query("receive_id_type", receive_id_type.as_str())
                .body(serialize_params(&body, "转发话题")?);

        if let Some(uuid) = self.uuid {
            req = req.query("uuid", uuid);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "转发话题")
    }
}
