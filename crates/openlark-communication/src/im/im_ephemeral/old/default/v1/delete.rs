//! 删除仅特定人可见的消息卡片
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/delete-message-cards-that-are-only-visible-to-certain-people

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::EPHEMERAL_V1_DELETE,
};

/// 删除仅特定人可见的消息卡片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEphemeralBody {
    /// 消息卡片 ID
    pub message_id: String,
}

/// 删除仅特定人可见的消息卡片请求
pub struct DeleteEphemeralRequest {
    config: Config,
}

impl DeleteEphemeralRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/delete-message-cards-that-are-only-visible-to-certain-people
    pub async fn execute(self, body: DeleteEphemeralBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: DeleteEphemeralBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(&body.message_id, "message_id 不能为空");

        // url: POST:/open-apis/ephemeral/v1/delete
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(EPHEMERAL_V1_DELETE)
            .body(serialize_params(&body, "删除仅特定人可见的消息卡片")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "删除仅特定人可见的消息卡片")
    }
}
