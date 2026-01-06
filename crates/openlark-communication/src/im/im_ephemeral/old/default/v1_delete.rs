//! 删除仅特定人可见的消息卡片
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/delete-message-cards-that-are-only-visible-to-certain-people

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;

const EPHEMERAL_V1_DELETE: &str = "/open-apis/ephemeral/v1/delete";

/// 删除仅特定人可见的消息卡片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEphemeralCardBody {
    pub message_id: String,
}

/// 删除仅特定人可见的消息卡片请求
pub struct DeleteEphemeralCardRequest {
    config: Config,
}

impl DeleteEphemeralCardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口响应体仅包含 code/msg（不含 data 字段），因此此处仅以 code==0 判断成功。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/delete-message-cards-that-are-only-visible-to-certain-people
    pub async fn execute(self, body: DeleteEphemeralCardBody) -> SDKResult<()> {
        validate_required!(body.message_id, "message_id 不能为空");

        // url: POST:/open-apis/ephemeral/v1/delete
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(EPHEMERAL_V1_DELETE)
            .body(serialize_params(&body, "删除仅特定人可见的消息卡片")?);

        let resp: openlark_core::api::Response<serde_json::Value> =
            Transport::request(req, &self.config, None).await?;
        if resp.is_success() {
            Ok(())
        } else {
            Err(error::validation_error(
                format!("删除仅特定人可见的消息卡片失败: {}", resp.code()),
                resp.msg().to_string(),
            ))
        }
    }
}
