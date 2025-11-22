//! Bitable App Table Form Field API 服务实现
//!
//! 提供多维表格表单字段管理相关的API服务，包括：
//! - 表单字段的列表查询和更新
//! - 表单字段类型配置
//! - 表单字段验证规则设置
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 多维表格表单字段管理服务
#[derive(Debug, Clone)]
pub struct AppTableFormFieldService {
    config: Config,
}

impl AppTableFormFieldService {
    /// 创建新的表单字段管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 列出表单字段
    ///
    /// 获取指定表单中的字段列表
    ///
    /// # 参数
    /// * `request` - 列出表单字段请求
    ///
    /// # 返回
    /// 返回表单字段列表
    pub async fn list_form_field(
        &self,
        request: &ListFormFieldRequest,
    ) -> SDKResult<ListFormFieldResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "列出表单字段: app_token={}, table_id={}, form_id={}",
            request.app_token,
            request.table_id,
            request.form_id
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
                "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/fields",
                request.app_token, request.table_id, request.form_id
            ))
            .query(query_params);

        // 发送请求
        let resp = Transport::<ListFormFieldResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出表单字段完成: app_token={}, table_id={}, form_id={}, count={}",
            request.app_token,
            request.table_id,
            request.form_id,
            response.fields.as_ref().map(|f| f.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 更新表单字段
    ///
    /// 批量更新表单字段的配置信息
    ///
    /// # 参数
    /// * `request` - 更新表单字段请求
    ///
    /// # 返回
    /// 返回更新后的表单字段信息
    pub async fn patch_form_field(
        &self,
        request: &PatchFormFieldRequest,
    ) -> SDKResult<PatchFormFieldResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新表单字段: app_token={}, table_id={}, form_id={}, field_count={}",
            request.app_token,
            request.table_id,
            request.form_id,
            request.fields.len()
        );

        // 构建请求体
        let body = serde_json::to_value(request)?;

        // 构建API请求
        let api_req = ApiRequest::put(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/fields",
                request.app_token, request.table_id, request.form_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<PatchFormFieldResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新表单字段完成: app_token={}, table_id={}, form_id={}",
            request.app_token,
            request.table_id,
            request.form_id
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct ListFormFieldRequestBuilder {
    request: ListFormFieldRequest,
}

impl ListFormFieldRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        form_id: impl Into<String>,
    ) -> Self {
        Self {
            request: ListFormFieldRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                form_id: form_id.into(),
                page_size: None,
                page_token: None,
            },
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(
        self,
        service: &AppTableFormFieldService,
    ) -> SDKResult<ListFormFieldResponse> {
        service.list_form_field(&self.request).await
    }
}

pub struct PatchFormFieldRequestBuilder {
    request: PatchFormFieldRequest,
}

impl PatchFormFieldRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        form_id: impl Into<String>,
    ) -> Self {
        Self {
            request: PatchFormFieldRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                form_id: form_id.into(),
                fields: Vec::new(),
            },
        }
    }

    pub fn field(mut self, field: FormFieldUpdate) -> Self {
        self.request.fields.push(field);
        self
    }

    pub fn fields(mut self, fields: Vec<FormFieldUpdate>) -> Self {
        self.request.fields = fields;
        self
    }

    pub async fn execute(
        self,
        service: &AppTableFormFieldService,
    ) -> SDKResult<PatchFormFieldResponse> {
        service.patch_form_field(&self.request).await
    }
}
