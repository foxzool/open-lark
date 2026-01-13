//! 更新组件属性
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card-element/patch

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::cardkit_v1_card_element,
};

/// 更新组件属性请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatchCardElementBody {
    pub card_id: String,
    pub element_id: String,
    pub patch: serde_json::Value,
}

/// 更新组件属性请求
pub struct PatchCardElementRequest {
    config: Config,
}

impl PatchCardElementRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card-element/patch
    pub async fn execute(self, body: PatchCardElementBody) -> SDKResult<serde_json::Value> {
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

        // url: PATCH:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::patch(cardkit_v1_card_element(&body.card_id, &body.element_id))
                .body(serialize_params(&body, "更新组件属性")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新组件属性")
    }
}
