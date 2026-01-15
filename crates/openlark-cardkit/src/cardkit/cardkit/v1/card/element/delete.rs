//! 删除组件
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card-element/delete

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::DeleteCardElementResponse;
use crate::common::api_utils::extract_response_data;
use crate::endpoints::cardkit_v1_card_element;

/// 删除组件请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteCardElementBody {
    pub card_id: String,
    pub element_id: String,
}

/// 删除组件请求
#[derive(Debug, Clone)]
pub struct DeleteCardElementRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
}

impl DeleteCardElementRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            element_id: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card-element/delete
    pub async fn execute(
        self,
        body: DeleteCardElementBody,
    ) -> SDKResult<DeleteCardElementResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(element_id) = self.element_id {
            body.element_id = element_id;
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

        // url: DELETE:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<DeleteCardElementResponse> =
            ApiRequest::delete(cardkit_v1_card_element(&body.card_id, &body.element_id));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除组件")
    }
}

/// 删除组件请求构建器
#[derive(Debug, Clone)]
pub struct DeleteCardElementRequestBuilder {
    request: DeleteCardElementRequest,
    card_id: Option<String>,
    element_id: Option<String>,
}

impl DeleteCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteCardElementRequest::new(config),
            card_id: None,
            element_id: None,
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

    /// 构建请求
    pub fn build(self) -> DeleteCardElementRequest {
        DeleteCardElementRequest {
            config: self.request.config,
            card_id: self.card_id,
            element_id: self.element_id,
        }
    }
}
