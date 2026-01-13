//! 删除组件
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card-element/delete

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::ensure_success, endpoints::cardkit_v1_card_element};

/// 删除组件请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteCardElementBody {
    pub card_id: String,
    pub element_id: String,
}

/// 删除组件请求
pub struct DeleteCardElementRequest {
    config: Config,
}

impl DeleteCardElementRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card-element/delete
    pub async fn execute(self, body: DeleteCardElementBody) -> SDKResult<()> {
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

        // url: DELETE:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete(cardkit_v1_card_element(&body.card_id, &body.element_id));

        let resp = Transport::request(req, &self.config, None).await?;
        ensure_success(resp, "删除组件")
    }
}
