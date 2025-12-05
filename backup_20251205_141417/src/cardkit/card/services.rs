//! Card API 服务实现
//!
//! 提供卡片实体管理相关的API服务，包括：
//! - 卡片的创建、更新、配置管理
//! - 批量更新操作
//! - 卡片ID类型转换
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 卡片实体管理服务
#[derive(Debug, Clone)]
pub struct CardService {
    config: Config,
}

impl CardService {
    /// 创建新的卡片管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建卡片
    ///
    /// 创建一个新的卡片实体
    ///
    /// # 参数
    /// * `request` - 创建卡片请求
    ///
    /// # 返回
    /// 返回创建的卡片信息
    pub async fn create_card(&self, request: &CreateCardRequest) -> SDKResult<CreateCardResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("创建卡片: card_type={:?}", request.card_type);

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("card_content".to_string(), request.card_content.clone());

        if let Some(ref card_type) = request.card_type {
            body.insert("card_type".to_string(), serde_json::to_value(card_type)?);
        }
        if let Some(ref template_id) = request.template_id {
            body.insert(
                "template_id".to_string(),
                serde_json::to_value(template_id)?,
            );
        }
        if let Some(temp) = request.temp {
            body.insert("temp".to_string(), serde_json::to_value(temp)?);
        }
        if let Some(temp_expire_time) = request.temp_expire_time {
            body.insert(
                "temp_expire_time".to_string(),
                serde_json::to_value(temp_expire_time)?,
            );
        }

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::POST,
            url: "/open-apis/cardkit/v1/cards".to_string(),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(
                &body
            ))),
            query: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp = Transport::<CreateCardResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("创建卡片完成: card_id={:?}", response.card_id);

        Ok(response)
    }

    /// 更新卡片
    ///
    /// 全量更新指定卡片的配置
    ///
    /// # 参数
    /// * `request` - 更新卡片请求
    ///
    /// # 返回
    /// 返回更新后的卡片信息
    pub async fn update_card(&self, request: &UpdateCardRequest) -> SDKResult<UpdateCardResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("更新卡片: card_id={}", request.card_id);

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("card_content".to_string(), request.card_content.clone());

        if let Some(ref card_type) = request.card_type {
            body.insert("card_type".to_string(), serde_json::to_value(card_type)?);
        }
        if let Some(ref update_mask) = request.update_mask {
            body.insert(
                "update_mask".to_string(),
                serde_json::to_value(update_mask)?,
            );
        }

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::PUT,
            url: format!("/open-apis/cardkit/v1/cards/{}", request.card_id),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(
                &body
            ))),
            query: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp = Transport::<UpdateCardResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("更新卡片完成: card_id={}", request.card_id);

        Ok(response)
    }

    /// 批量更新卡片
    ///
    /// 对指定卡片执行批量更新操作
    ///
    /// # 参数
    /// * `request` - 批量更新卡片请求
    ///
    /// # 返回
    /// 返回更新后的卡片信息
    pub async fn batch_update_card(
        &self,
        request: &BatchUpdateCardRequest,
    ) -> SDKResult<BatchUpdateCardResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "批量更新卡片: card_id={}, operation_count={}",
            request.card_id,
            request.operations.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert(
            "operations".to_string(),
            serde_json::to_value(&request.operations)?,
        );

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::POST,
            url: format!(
                "/open-apis/cardkit/v1/cards/{}/batch_update",
                request.card_id
            ),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(
                &body
            ))),
            query: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<BatchUpdateCardResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("批量更新卡片完成: card_id={}", request.card_id);

        Ok(response)
    }

    /// 更新卡片设置
    ///
    /// 更新指定卡片的全局设置
    ///
    /// # 参数
    /// * `request` - 更新卡片设置请求
    ///
    /// # 返回
    /// 返回更新后的卡片设置信息
    pub async fn update_card_settings(
        &self,
        request: &UpdateCardSettingsRequest,
    ) -> SDKResult<UpdateCardSettingsResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("更新卡片设置: card_id={}", request.card_id);

        // 构建请求体
        let body = serde_json::to_value(&request.settings)?;

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::PATCH,
            url: format!("/open-apis/cardkit/v1/cards/{}/settings", request.card_id),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(
                &body
            ))),
            query: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<UpdateCardSettingsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("更新卡片设置完成: card_id={}", request.card_id);

        Ok(response)
    }

    /// 转换卡片ID
    ///
    /// 将卡片ID从一种类型转换为另一种类型
    ///
    /// # 参数
    /// * `request` - 转换卡片ID请求
    ///
    /// # 返回
    /// 返回ID转换结果
    pub async fn convert_card_id(
        &self,
        request: &ConvertCardIdRequest,
    ) -> SDKResult<ConvertCardIdResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "转换卡片ID: source_type={}, target_type={}, count={}",
            request.source_id_type,
            request.target_id_type,
            request.card_ids.len()
        );

        // 构建请求体
        let body = serde_json::to_value(request)?;

        // 构建API请求
        let api_req = ApiRequest {
            method: reqwest::Method::POST,
            url: "/open-apis/cardkit/v1/cards/id_convert".to_string(),
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(
                &body
            ))),
            query: HashMap::new(),
            ..Default::default()
        };

        // 发送请求
        let resp = Transport::<ConvertCardIdResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "转换卡片ID完成: conversion_count={}",
            response.conversions.as_ref().map(|c| c.len()).unwrap_or(0)
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateCardRequestBuilder {
    request: CreateCardRequest,
}

impl CreateCardRequestBuilder {
    pub fn new(card_content: Value) -> Self {
        Self {
            request: CreateCardRequest {
                card_content,
                card_type: None,
                template_id: None,
                temp: None,
                temp_expire_time: None,
            },
        }
    }

    pub fn card_type(mut self, card_type: impl Into<String>) -> Self {
        self.request.card_type = Some(card_type.into());
        self
    }

    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.request.template_id = Some(template_id.into());
        self
    }

    pub fn temp(mut self, temp: bool) -> Self {
        self.request.temp = Some(temp);
        self
    }

    pub fn temp_expire_time(mut self, temp_expire_time: i32) -> Self {
        self.request.temp_expire_time = Some(temp_expire_time);
        self
    }

    pub async fn execute(self, service: &CardService) -> SDKResult<CreateCardResponse> {
        service.create_card(&self.request).await
    }
}

pub struct UpdateCardRequestBuilder {
    request: UpdateCardRequest,
}

impl UpdateCardRequestBuilder {
    pub fn new(card_id: impl Into<String>, card_content: Value) -> Self {
        Self {
            request: UpdateCardRequest {
                card_id: card_id.into(),
                card_content,
                card_type: None,
                update_mask: None,
            },
        }
    }

    pub fn card_type(mut self, card_type: impl Into<String>) -> Self {
        self.request.card_type = Some(card_type.into());
        self
    }

    pub fn update_mask(mut self, update_mask: Vec<String>) -> Self {
        self.request.update_mask = Some(update_mask);
        self
    }

    pub async fn execute(self, service: &CardService) -> SDKResult<UpdateCardResponse> {
        service.update_card(&self.request).await
    }
}

pub struct BatchUpdateCardRequestBuilder {
    request: BatchUpdateCardRequest,
}

impl BatchUpdateCardRequestBuilder {
    pub fn new(card_id: impl Into<String>) -> Self {
        Self {
            request: BatchUpdateCardRequest {
                card_id: card_id.into(),
                operations: Vec::new(),
            },
        }
    }

    pub fn operation(mut self, operation: CardOperation) -> Self {
        self.request.operations.push(operation);
        self
    }

    pub fn operations(mut self, operations: Vec<CardOperation>) -> Self {
        self.request.operations = operations;
        self
    }

    pub async fn execute(self, service: &CardService) -> SDKResult<BatchUpdateCardResponse> {
        service.batch_update_card(&self.request).await
    }
}

pub struct UpdateCardSettingsRequestBuilder {
    request: UpdateCardSettingsRequest,
}

impl UpdateCardSettingsRequestBuilder {
    pub fn new(card_id: impl Into<String>, settings: CardSettings) -> Self {
        Self {
            request: UpdateCardSettingsRequest {
                card_id: card_id.into(),
                settings,
            },
        }
    }

    pub async fn execute(self, service: &CardService) -> SDKResult<UpdateCardSettingsResponse> {
        service.update_card_settings(&self.request).await
    }
}

pub struct ConvertCardIdRequestBuilder {
    request: ConvertCardIdRequest,
}

impl ConvertCardIdRequestBuilder {
    pub fn new(
        source_id_type: impl Into<String>,
        target_id_type: impl Into<String>,
        card_ids: Vec<String>,
    ) -> Self {
        Self {
            request: ConvertCardIdRequest {
                source_id_type: source_id_type.into(),
                target_id_type: target_id_type.into(),
                card_ids,
            },
        }
    }

    pub async fn execute(self, service: &CardService) -> SDKResult<ConvertCardIdResponse> {
        service.convert_card_id(&self.request).await
    }
}
