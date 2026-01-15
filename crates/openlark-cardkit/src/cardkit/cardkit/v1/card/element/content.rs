//! 流式更新文本
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card-element/content

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::UpdateCardElementContentResponse;
use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::cardkit_v1_card_element_content;

/// 流式更新文本请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateCardElementContentBody {
    /// 卡片 ID
    pub card_id: String,
    /// 组件 ID
    pub element_id: String,
    /// 内容
    pub content: serde_json::Value,
}

/// 流式更新文本请求
#[derive(Debug, Clone)]
pub struct UpdateCardElementContentRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
    content: Option<serde_json::Value>,
}

impl UpdateCardElementContentRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            element_id: None,
            content: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card-element/content
    pub async fn execute(
        self,
        body: UpdateCardElementContentBody,
    ) -> SDKResult<UpdateCardElementContentResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(element_id) = self.element_id {
            body.element_id = element_id;
        }
        if let Some(content) = self.content {
            body.content = content;
        }

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
        let req: ApiRequest<UpdateCardElementContentResponse> =
            ApiRequest::put(cardkit_v1_card_element_content(&body.card_id, &body.element_id))
                .body(serialize_params(&body, "流式更新文本")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "流式更新文本")
    }
}

/// 流式更新文本请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardElementContentRequestBuilder {
    request: UpdateCardElementContentRequest,
    card_id: Option<String>,
    element_id: Option<String>,
    content: Option<serde_json::Value>,
}

impl UpdateCardElementContentRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardElementContentRequest::new(config),
            card_id: None,
            element_id: None,
            content: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置组件 ID
    pub fn element_id(mut self, element_id: impl Into<String>) -> Self {
        self.element_id = Some(element_id.into());
        self
    }

    /// 设置内容
    pub fn content(mut self, content: impl Into<serde_json::Value>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateCardElementContentRequest {
        UpdateCardElementContentRequest {
            config: self.request.config,
            card_id: self.card_id,
            element_id: self.element_id,
            content: self.content,
        }
    }
}
