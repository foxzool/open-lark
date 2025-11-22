//! Card Element API 服务实现
//!
//! 提供卡片组件管理相关的API服务，包括：
//! - 卡片组件的创建、更新、删除
//! - 组件属性的局部更新
//! - 流式文本更新
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 卡片组件管理服务
#[derive(Debug, Clone)]
pub struct CardElementService {
    config: Config,
}

impl CardElementService {
    /// 创建新的卡片组件管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建卡片组件
    ///
    /// 在指定卡片中创建新的组件
    ///
    /// # 参数
    /// * `request` - 创建卡片组件请求
    ///
    /// # 返回
    /// 返回创建的组件信息
    pub async fn create_card_element(
        &self,
        request: &CreateCardElementRequest,
    ) -> SDKResult<CreateCardElementResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "创建卡片组件: card_id={}, element_type={:?}",
            request.card_id,
            request.element_type
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("element".to_string(), request.element.clone());

        if let Some(ref element_type) = request.element_type {
            body.insert(
                "element_type".to_string(),
                serde_json::to_value(element_type)?,
            );
        }
        if let Some(ref tag) = request.tag {
            body.insert("tag".to_string(), serde_json::to_value(tag)?);
        }

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::POST,
            url: format!("/open-apis/cardkit/v1/cards/{}/elements", request.card_id),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(
                &body
            ))),
            query: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<CreateCardElementResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "创建卡片组件完成: card_id={}, element_id={:?}",
            request.card_id,
            response.element_id
        );

        Ok(response)
    }

    /// 更新卡片组件
    ///
    /// 全量更新指定组件的配置
    ///
    /// # 参数
    /// * `request` - 更新卡片组件请求
    ///
    /// # 返回
    /// 返回更新后的组件信息
    pub async fn update_card_element(
        &self,
        request: &UpdateCardElementRequest,
    ) -> SDKResult<UpdateCardElementResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新卡片组件: card_id={}, element_id={}",
            request.card_id,
            request.element_id
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("element".to_string(), request.element.clone());

        if let Some(ref update_mask) = request.update_mask {
            body.insert(
                "update_mask".to_string(),
                serde_json::to_value(update_mask)?,
            );
        }

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::PUT,
            url: format!(
                "/open-apis/cardkit/v1/cards/{}/elements/{}",
                request.card_id, request.element_id
            ),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(
                &body
            ))),
            query: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<UpdateCardElementResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新卡片组件完成: card_id={}, element_id={}",
            request.card_id,
            request.element_id
        );

        Ok(response)
    }

    /// 局部更新卡片组件
    ///
    /// 局部更新指定组件的属性
    ///
    /// # 参数
    /// * `request` - 局部更新卡片组件请求
    ///
    /// # 返回
    /// 返回更新后的组件信息
    pub async fn patch_card_element(
        &self,
        request: &PatchCardElementRequest,
    ) -> SDKResult<PatchCardElementResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "局部更新卡片组件: card_id={}, element_id={}",
            request.card_id,
            request.element_id
        );

        // 构建请求体
        let body = serde_json::to_value(&request.properties)?;

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::PATCH,
            url: format!(
                "/open-apis/cardkit/v1/cards/{}/elements/{}",
                request.card_id, request.element_id
            ),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(
                body
            ))),
            timeout: None,
            _phantom: std::marker::PhantomData,
        };

        // 发送请求
        let resp =
            Transport::<PatchCardElementResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "局部更新卡片组件完成: card_id={}, element_id={}",
            request.card_id,
            request.element_id
        );

        Ok(response)
    }

    /// 更新组件内容
    ///
    /// 更新指定组件的内容，支持流式更新
    ///
    /// # 参数
    /// * `request` - 更新组件内容请求
    ///
    /// # 返回
    /// 返回更新结果
    pub async fn update_card_element_content(
        &self,
        request: &UpdateCardElementContentRequest,
    ) -> SDKResult<UpdateCardElementContentResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新组件内容: card_id={}, element_id={}, append={:?}, stream={:?}",
            request.card_id,
            request.element_id,
            request.append,
            request.stream
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("content".to_string(), request.content.clone());

        if let Some(append) = request.append {
            body.insert("append".to_string(), serde_json::to_value(append)?);
        }
        if let Some(stream) = request.stream {
            body.insert("stream".to_string(), serde_json::to_value(stream)?);
        }

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::PUT,
            url: format!(
                "/open-apis/cardkit/v1/cards/{}/elements/{}/content",
                request.card_id, request.element_id
            ),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(
                &body
            ))),
            query: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<UpdateCardElementContentResponse>::request(api_req, &self.config, None)
                .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新组件内容完成: card_id={}, element_id={}, success={:?}",
            request.card_id,
            request.element_id,
            response.success
        );

        Ok(response)
    }

    /// 删除卡片组件
    ///
    /// 删除指定的卡片组件
    ///
    /// # 参数
    /// * `request` - 删除卡片组件请求
    ///
    /// # 返回
    /// 返回删除结果
    pub async fn delete_card_element(
        &self,
        request: &DeleteCardElementRequest,
    ) -> SDKResult<DeleteCardElementResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "删除卡片组件: card_id={}, element_id={}",
            request.card_id,
            request.element_id
        );

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::DELETE,
            url: format!(
                "/open-apis/cardkit/v1/cards/{}/elements/{}",
                request.card_id, request.element_id
            ),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!([]))),
            query: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<DeleteCardElementResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除卡片组件完成: card_id={}, element_id={}, success={:?}",
            request.card_id,
            request.element_id,
            response.success
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateCardElementRequestBuilder {
    request: CreateCardElementRequest,
}

impl CreateCardElementRequestBuilder {
    pub fn new(card_id: impl Into<String>, element: Value) -> Self {
        Self {
            request: CreateCardElementRequest {
                card_id: card_id.into(),
                element,
                element_type: None,
                tag: None,
            },
        }
    }

    pub fn element_type(mut self, element_type: impl Into<String>) -> Self {
        self.request.element_type = Some(element_type.into());
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.request.tag = Some(tag.into());
        self
    }

    pub async fn execute(
        self,
        service: &CardElementService,
    ) -> SDKResult<CreateCardElementResponse> {
        service.create_card_element(&self.request).await
    }
}

pub struct UpdateCardElementRequestBuilder {
    request: UpdateCardElementRequest,
}

impl UpdateCardElementRequestBuilder {
    pub fn new(card_id: impl Into<String>, element_id: impl Into<String>, element: Value) -> Self {
        Self {
            request: UpdateCardElementRequest {
                card_id: card_id.into(),
                element_id: element_id.into(),
                element,
                update_mask: None,
            },
        }
    }

    pub fn update_mask(mut self, update_mask: Vec<String>) -> Self {
        self.request.update_mask = Some(update_mask);
        self
    }

    pub async fn execute(
        self,
        service: &CardElementService,
    ) -> SDKResult<UpdateCardElementResponse> {
        service.update_card_element(&self.request).await
    }
}

pub struct PatchCardElementRequestBuilder {
    request: PatchCardElementRequest,
}

impl PatchCardElementRequestBuilder {
    pub fn new(
        card_id: impl Into<String>,
        element_id: impl Into<String>,
        properties: Value,
    ) -> Self {
        Self {
            request: PatchCardElementRequest {
                card_id: card_id.into(),
                element_id: element_id.into(),
                properties,
            },
        }
    }

    pub async fn execute(
        self,
        service: &CardElementService,
    ) -> SDKResult<PatchCardElementResponse> {
        service.patch_card_element(&self.request).await
    }
}

pub struct UpdateCardElementContentRequestBuilder {
    request: UpdateCardElementContentRequest,
}

impl UpdateCardElementContentRequestBuilder {
    pub fn new(card_id: impl Into<String>, element_id: impl Into<String>, content: Value) -> Self {
        Self {
            request: UpdateCardElementContentRequest {
                card_id: card_id.into(),
                element_id: element_id.into(),
                content,
                append: None,
                stream: None,
            },
        }
    }

    pub fn append(mut self, append: bool) -> Self {
        self.request.append = Some(append);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.request.stream = Some(stream);
        self
    }

    pub async fn execute(
        self,
        service: &CardElementService,
    ) -> SDKResult<UpdateCardElementContentResponse> {
        service.update_card_element_content(&self.request).await
    }
}

pub struct DeleteCardElementRequestBuilder {
    request: DeleteCardElementRequest,
}

impl DeleteCardElementRequestBuilder {
    pub fn new(card_id: impl Into<String>, element_id: impl Into<String>) -> Self {
        Self {
            request: DeleteCardElementRequest {
                card_id: card_id.into(),
                element_id: element_id.into(),
            },
        }
    }

    pub async fn execute(
        self,
        service: &CardElementService,
    ) -> SDKResult<DeleteCardElementResponse> {
        service.delete_card_element(&self.request).await
    }
}
