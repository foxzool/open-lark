//! 更新组件
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card-element/update

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::cardkit_v1_card_element,
};

/// 更新组件请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateCardElementBody {
    pub card_id: String,
    pub element_id: String,
    pub element: serde_json::Value,
}

/// 更新组件请求
pub struct UpdateCardElementRequest {
    config: Config,
}

impl UpdateCardElementRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card-element/update
    pub async fn execute(self, body: UpdateCardElementBody) -> SDKResult<serde_json::Value> {
        if body.card_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "card_id 不能为空",
                "card_id 不能为空",
            ));
        }
        if body.element_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "element_id 不能为空",
                "element_id 不能为空",
            ));
        }

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::put(cardkit_v1_card_element(&body.card_id, &body.element_id))
                .body(serialize_params(&body, "更新组件")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新组件")
    }
}
