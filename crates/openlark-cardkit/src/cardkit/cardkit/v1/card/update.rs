//! 全量更新卡片实体
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card/update

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::cardkit_v1_card,
};

/// 全量更新卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardBody {
    /// 卡片 ID
    pub card_id: String,
    /// 卡片内容
    pub card_content: serde_json::Value,
    /// 卡片类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// 更新掩码（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<Vec<String>>,
}

/// 全量更新卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for UpdateCardResponse {}

/// 全量更新卡片实体请求
pub struct UpdateCardRequest {
    config: Config,
}

impl UpdateCardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card/update
    pub async fn execute(self, body: UpdateCardBody) -> SDKResult<UpdateCardResponse> {
        if body.card_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "card_id 不能为空",
                "card_id 不能为空",
            ));
        }

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id
        let url = cardkit_v1_card(&body.card_id);
        let req: ApiRequest<UpdateCardResponse> =
            ApiRequest::put(url).body(serialize_params(&body, "全量更新卡片实体")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "全量更新卡片实体")
    }
}
