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
#[derive(Debug, Clone)]
pub struct UpdateCardSettingsRequest {
    config: Config,
    card_id: Option<String>,
    settings: Option<serde_json::Value>,
}

impl UpdateCardSettingsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            settings: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card/settings
    pub async fn execute(
        self,
        body: UpdateCardSettingsBody,
    ) -> SDKResult<UpdateCardSettingsResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(settings) = self.settings {
            body.settings = settings;
        }

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

/// 更新卡片实体配置请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardSettingsRequestBuilder {
    request: UpdateCardSettingsRequest,
    card_id: Option<String>,
    settings: Option<serde_json::Value>,
}

impl UpdateCardSettingsRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardSettingsRequest::new(config),
            card_id: None,
            settings: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置配置
    pub fn settings(mut self, settings: impl Into<serde_json::Value>) -> Self {
        self.settings = Some(settings.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateCardSettingsRequest {
        UpdateCardSettingsRequest {
            config: self.request.config,
            card_id: self.card_id,
            settings: self.settings,
        }
    }
}
