//! 流式更新文本
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card-element/content

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::cardkit_v1_card_element_content,
};

/// 流式更新文本请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateCardElementContentBody {
    pub card_id: String,
    pub element_id: String,
    pub content: serde_json::Value,
}

/// 流式更新文本请求
pub struct UpdateCardElementContentRequest {
    config: Config,
}

impl UpdateCardElementContentRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card-element/content
    pub async fn execute(self, body: UpdateCardElementContentBody) -> SDKResult<serde_json::Value> {
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

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id/content
        let req: ApiRequest<serde_json::Value> = ApiRequest::put(cardkit_v1_card_element_content(
            &body.card_id,
            &body.element_id,
        ))
        .body(serialize_params(&body, "流式更新文本")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "流式更新文本")
    }
}
