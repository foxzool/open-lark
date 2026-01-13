//! 更新卡片实体配置
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card/settings

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::cardkit_v1_card_settings,
};

/// 更新卡片实体配置请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardSettingsBody {
    /// 卡片 ID
    pub card_id: String,
    /// 设置内容（结构以官方文档为准）
    pub settings: serde_json::Value,
}

/// 更新卡片实体配置响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardSettingsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for UpdateCardSettingsResponse {}

/// 更新卡片实体配置请求
pub struct UpdateCardSettingsRequest {
    config: Config,
}

impl UpdateCardSettingsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card/settings
    pub async fn execute(
        self,
        body: UpdateCardSettingsBody,
    ) -> SDKResult<UpdateCardSettingsResponse> {
        if body.card_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "card_id 不能为空",
                "card_id 不能为空",
            ));
        }

        // url: PATCH:/open-apis/cardkit/v1/cards/:card_id/settings
        let url = cardkit_v1_card_settings(&body.card_id);
        let req: ApiRequest<UpdateCardSettingsResponse> =
            ApiRequest::patch(url).body(serialize_params(&body, "更新卡片实体配置")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新卡片实体配置")
    }
}
