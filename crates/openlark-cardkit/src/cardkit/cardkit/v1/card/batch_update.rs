//! 局部更新卡片实体
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card/batch_update

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::cardkit_v1_card_batch_update,
};

/// 局部更新卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateCardBody {
    /// 卡片 ID
    pub card_id: String,
    /// 操作列表（结构以官方文档为准）
    pub operations: Vec<serde_json::Value>,
}

/// 局部更新卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchUpdateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for BatchUpdateCardResponse {}

/// 局部更新卡片实体请求
pub struct BatchUpdateCardRequest {
    config: Config,
}

impl BatchUpdateCardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card/batch_update
    pub async fn execute(self, body: BatchUpdateCardBody) -> SDKResult<BatchUpdateCardResponse> {
        if body.card_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "card_id 不能为空",
                "card_id 不能为空",
            ));
        }
        if body.operations.is_empty() {
            return Err(openlark_core::error::validation_error(
                "operations 不能为空",
                "operations 不能为空",
            ));
        }

        // url: POST:/open-apis/cardkit/v1/cards/:card_id/batch_update
        let url = cardkit_v1_card_batch_update(&body.card_id);
        let req: ApiRequest<BatchUpdateCardResponse> =
            ApiRequest::post(url).body(serialize_params(&body, "局部更新卡片实体")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "局部更新卡片实体")
    }
}
