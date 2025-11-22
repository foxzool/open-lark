//! Bitable App Table Field API 服务实现
//!
//! 提供多维表格字段管理相关的API服务，包括：
//! - 字段的创建、查询、更新、删除
//! - 字段类型配置和属性设置
//! - 完整的错误处理和参数验证
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 多维表格字段管理服务
#[derive(Debug, Clone)]
pub struct AppTableFieldService {
    config: Config,
}

impl AppTableFieldService {
    /// 创建新的字段管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新增字段
    ///
    /// 在指定的数据表中新增一个字段
    ///
    /// # 参数
    /// * `request` - 新增字段请求
    ///
    /// # 返回
    /// 返回新创建的字段信息
    pub async fn create_field(
        &self,
        request: &CreateFieldRequest,
    ) -> SDKResult<CreateFieldResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "新增字段: app_token={}, table_id={}, field_name={}, field_type={}",
            request.app_token,
            request.table_id,
            request.field_name,
            request.field_type
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("field_name", Value::String(request.field_name.clone()));
        body.insert("field_type", Value::String(request.field_type.clone()));

        if let Some(ref property) = request.property {
            body.insert("property", property.clone());
        }
        if let Some(ref description) = request.description {
            body.insert("description", Value::String(description.clone()));
        }
        if let Some(is_required) = request.is_required {
            body.insert("is_required", Value::Bool(is_required));
        }

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/fields",
                request.app_token, request.table_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<CreateFieldResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "新增字段完成: app_token={}, table_id={}, field_name={}",
            request.app_token,
            request.table_id,
            request.field_name
        );

        Ok(response)
    }

    /// 更新字段
    ///
    /// 更新指定数据表中的字段
    ///
    /// # 参数
    /// * `request` - 更新字段请求
    ///
    /// # 返回
    /// 返回更新后的字段信息
    pub async fn update_field(
        &self,
        request: &UpdateFieldRequest,
    ) -> SDKResult<UpdateFieldResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新字段: app_token={}, table_id={}, field_id={}",
            request.app_token,
            request.table_id,
            request.field_id
        );

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref field_name) = request.field_name {
            body.insert("field_name", Value::String(field_name.clone()));
        }
        if let Some(ref property) = request.property {
            body.insert("property", property.clone());
        }
        if let Some(ref description) = request.description {
            body.insert("description", Value::String(description.clone()));
        }
        if let Some(is_required) = request.is_required {
            body.insert("is_required", Value::Bool(is_required));
        }

        // 构建API请求
        let api_req = ApiRequest::put(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}",
                request.app_token, request.table_id, request.field_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<UpdateFieldResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新字段完成: app_token={}, table_id={}, field_id={}",
            request.app_token,
            request.table_id,
            request.field_id
        );

        Ok(response)
    }

    /// 删除字段
    ///
    /// 删除指定数据表中的字段
    ///
    /// # 参数
    /// * `request` - 删除字段请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_field(
        &self,
        request: &DeleteFieldRequest,
    ) -> SDKResult<DeleteFieldResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "删除字段: app_token={}, table_id={}, field_id={}",
            request.app_token,
            request.table_id,
            request.field_id
        );

        // 构建API请求
        let api_req = ApiRequest::delete(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}",
                request.app_token, request.table_id, request.field_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<DeleteFieldResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除字段完成: app_token={}, table_id={}, field_id={}",
            request.app_token,
            request.table_id,
            request.field_id
        );

        Ok(response)
    }

    /// 列出字段
    ///
    /// 获取指定数据表中的字段列表
    ///
    /// # 参数
    /// * `request` - 列出字段请求
    ///
    /// # 返回
    /// 返回字段列表
    pub async fn list_fields(&self, request: &ListFieldsRequest) -> SDKResult<ListFieldsResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "列出字段: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/fields",
                request.app_token, request.table_id
            ))
            .query(query_params);

        // 发送请求
        let resp = Transport::<ListFieldsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出字段完成: app_token={}, table_id={}, count={}",
            request.app_token,
            request.table_id,
            response.fields.as_ref().map(|f| f.len()).unwrap_or(0)
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateFieldRequestBuilder {
    request: CreateFieldRequest,
}

impl CreateFieldRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        field_name: impl Into<String>,
        field_type: impl Into<String>,
    ) -> Self {
        Self {
            request: CreateFieldRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                field_name: field_name.into(),
                field_type: field_type.into(),
                property: None,
                description: None,
                is_required: None,
            },
        }
    }

    pub fn property(mut self, property: Value) -> Self {
        self.request.property = Some(property);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn is_required(mut self, is_required: bool) -> Self {
        self.request.is_required = Some(is_required);
        self
    }

    pub async fn execute(self, service: &AppTableFieldService) -> SDKResult<CreateFieldResponse> {
        service.create_field(&self.request).await
    }
}

pub struct UpdateFieldRequestBuilder {
    request: UpdateFieldRequest,
}

impl UpdateFieldRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        field_id: impl Into<String>,
    ) -> Self {
        Self {
            request: UpdateFieldRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                field_id: field_id.into(),
                field_name: None,
                property: None,
                description: None,
                is_required: None,
            },
        }
    }

    pub fn field_name(mut self, field_name: impl Into<String>) -> Self {
        self.request.field_name = Some(field_name.into());
        self
    }

    pub fn property(mut self, property: Value) -> Self {
        self.request.property = Some(property);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn is_required(mut self, is_required: bool) -> Self {
        self.request.is_required = Some(is_required);
        self
    }

    pub async fn execute(self, service: &AppTableFieldService) -> SDKResult<UpdateFieldResponse> {
        service.update_field(&self.request).await
    }
}
