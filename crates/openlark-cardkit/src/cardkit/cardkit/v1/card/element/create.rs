//! 新增组件
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card-element/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::cardkit_v1_card_elements,
};

/// 新增组件请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateCardElementBody {
    /// 卡片 ID
    pub card_id: String,
    /// 组件定义
    pub element: serde_json::Value,
}

/// 新增组件请求
pub struct CreateCardElementRequest {
    config: Config,
}

impl CreateCardElementRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card-element/create
    pub async fn execute(self, body: CreateCardElementBody) -> SDKResult<serde_json::Value> {
        if body.card_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "card_id 不能为空",
                "card_id 不能为空",
            ));
        }

        // url: POST:/open-apis/cardkit/v1/cards/:card_id/elements
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(cardkit_v1_card_elements(&body.card_id))
                .body(serialize_params(&body, "新增组件")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "新增组件")
    }
}
