//! Bitable App Form API 服务实现
//!
//! 提供多维表格表单管理相关的API服务，包括：
//! - 表单的查询、更新、删除
//! - 表单问题和字段管理
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 多维表格表单管理服务
#[derive(Debug, Clone)]
pub struct AppFormService {
    config: Config,
}

impl AppFormService {
    /// 创建新的表单管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取表单
    ///
    /// 获取指定表单的详细信息
    ///
    /// # 参数
    /// * `request` - 获取表单请求
    ///
    /// # 返回
    /// 返回表单信息
    pub async fn get_form(&self, request: &GetFormRequest) -> SDKResult<GetFormResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "获取表单: app_token={}, table_id={}, form_id={}",
            request.app_token,
            request.table_id,
            request.form_id
        );

        // 构建API请求
        let api_req = ApiRequest::get(format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
            request.app_token, request.table_id, request.form_id
        ))
        .query(HashMap::new());

        // 发送请求
        let resp = Transport::<GetFormResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "获取表单完成: app_token={}, table_id={}, form_id={}",
            request.app_token,
            request.table_id,
            request.form_id
        );

        Ok(response)
    }

    /// 列出表单问题
    ///
    /// 获取指定表单中的问题列表
    ///
    /// # 参数
    /// * `request` - 列出表单问题请求
    ///
    /// # 返回
    /// 返回表单问题列表
    pub async fn list_form_questions(
        &self,
        request: &ListFormQuestionsRequest,
    ) -> SDKResult<ListFormQuestionsResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "列出表单问题: app_token={}, table_id={}, form_id={}",
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
            "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions",
            request.app_token, request.table_id, request.form_id
        ))
        .query(query_params);

        // 发送请求
        let resp =
            Transport::<ListFormQuestionsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出表单问题完成: app_token={}, table_id={}, form_id={}, count={}",
            request.app_token,
            request.table_id,
            request.form_id,
            response.questions.as_ref().map(|q| q.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 更新表单问题
    ///
    /// 更新指定表单中的问题配置
    ///
    /// # 参数
    /// * `request` - 更新表单问题请求
    ///
    /// # 返回
    /// 返回更新后的表单问题信息
    pub async fn patch_form_question(
        &self,
        request: &PatchFormQuestionRequest,
    ) -> SDKResult<PatchFormQuestionResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新表单问题: app_token={}, table_id={}, form_id={}, question_id={}",
            request.app_token,
            request.table_id,
            request.form_id,
            request.question_id
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("title".to_string(), serde_json::to_value(&request.title)?);
        body.insert(
            "question_type".to_string(),
            serde_json::to_value(&request.question_type)?,
        );
        body.insert(
            "required".to_string(),
            serde_json::to_value(&request.required)?,
        );

        if let Some(ref description) = request.description {
            body.insert(
                "description".to_string(),
                serde_json::to_value(description)?,
            );
        }
        if let Some(ref options) = request.options {
            body.insert("options".to_string(), serde_json::to_value(options)?);
        }
        if let Some(ref validation) = request.validation {
            body.insert("validation".to_string(), serde_json::to_value(validation)?);
        }

        // 构建API请求
        let api_req = ApiRequest::post(format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}/questions/{}",
            request.app_token, request.table_id, request.form_id, request.question_id
        ))
        .body(serde_json::to_value(body)?)
        .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<PatchFormQuestionResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新表单问题完成: app_token={}, table_id={}, form_id={}, question_id={}",
            request.app_token,
            request.table_id,
            request.form_id,
            request.question_id
        );

        Ok(response)
    }

    /// 更新表单元数据
    ///
    /// 更新表单的基本信息和设置
    ///
    /// # 参数
    /// * `request` - 更新表单元数据请求
    ///
    /// # 返回
    /// 返回更新后的表单信息
    pub async fn patch_form_meta(
        &self,
        request: &PatchFormMetaRequest,
    ) -> SDKResult<PatchFormMetaResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新表单元数据: app_token={}, table_id={}, form_id={}",
            request.app_token,
            request.table_id,
            request.form_id
        );

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref title) = request.title {
            body.insert("title".to_string(), serde_json::to_value(title)?);
        }
        if let Some(ref description) = request.description {
            body.insert(
                "description".to_string(),
                serde_json::to_value(description)?,
            );
        }
        if let Some(ref settings) = request.settings {
            body.insert("settings".to_string(), serde_json::to_value(settings)?);
        }

        // 构建API请求
        let api_req = ApiRequest::post(format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
            request.app_token, request.table_id, request.form_id
        ))
        .body(serde_json::to_value(body)?)
        .query(HashMap::new());

        // 发送请求
        let resp = Transport::<PatchFormMetaResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新表单元数据完成: app_token={}, table_id={}, form_id={}",
            request.app_token,
            request.table_id,
            request.form_id
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct GetFormRequestBuilder {
    request: GetFormRequest,
}

impl GetFormRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        form_id: impl Into<String>,
    ) -> Self {
        Self {
            request: GetFormRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                form_id: form_id.into(),
            },
        }
    }

    pub async fn execute(self, service: &AppFormService) -> SDKResult<GetFormResponse> {
        service.get_form(&self.request).await
    }
}

pub struct PatchFormQuestionRequestBuilder {
    request: PatchFormQuestionRequest,
}

impl PatchFormQuestionRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        form_id: impl Into<String>,
        question_id: impl Into<String>,
        title: impl Into<String>,
        question_type: impl Into<String>,
        required: bool,
    ) -> Self {
        Self {
            request: PatchFormQuestionRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                form_id: form_id.into(),
                question_id: question_id.into(),
                title: title.into(),
                description: None,
                question_type: question_type.into(),
                required,
                options: None,
                validation: None,
            },
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn options(mut self, options: Vec<QuestionOption>) -> Self {
        self.request.options = Some(options);
        self
    }

    pub fn validation(mut self, validation: ValidationRule) -> Self {
        self.request.validation = Some(validation);
        self
    }

    pub async fn execute(self, service: &AppFormService) -> SDKResult<PatchFormQuestionResponse> {
        service.patch_form_question(&self.request).await
    }
}

pub struct PatchFormMetaRequestBuilder {
    request: PatchFormMetaRequest,
}

impl PatchFormMetaRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        form_id: impl Into<String>,
    ) -> Self {
        Self {
            request: PatchFormMetaRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                form_id: form_id.into(),
                title: None,
                description: None,
                settings: None,
            },
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn settings(mut self, settings: FormSettings) -> Self {
        self.request.settings = Some(settings);
        self
    }

    pub async fn execute(self, service: &AppFormService) -> SDKResult<PatchFormMetaResponse> {
        service.patch_form_meta(&self.request).await
    }
}
