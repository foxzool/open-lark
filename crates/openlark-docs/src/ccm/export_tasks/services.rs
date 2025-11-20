//! 导出任务API服务实现
//!
//! 提供文档导出任务相关的API服务，包括：
//! - 创建导出任务
//! - 查询导出任务结果
//! - 下载导出文件
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 导出任务API服务
#[derive(Debug, Clone)]
pub struct ExportTasksService {
    config: Config,
}

impl ExportTasksService {
    /// 创建新的导出任务服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建导出任务
    pub async fn create_export_task(
        &self,
        request: &CreateExportTaskRequest,
    ) -> SDKResult<CreateExportTaskResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "创建导出任务: file_token={}, file_type={}, export_type={}",
            request.file_token,
            request.file_type,
            request.export_type
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("file_token".to_string(), request.file_token.clone());
        body.insert("file_type".to_string(), request.file_type.clone());
        body.insert("export_type".to_string(), request.export_type.clone());

        if let Some(ref export_name) = request.export_name {
            body.insert("export_name".to_string(), export_name.clone());
        }
        if let Some(ref locale) = request.locale {
            body.insert("locale".to_string(), locale.clone());
        }
        if let Some(include_comments) = request.include_comments {
            body.insert("include_comments".to_string(), include_comments.to_string());
        }
        if let Some(password_enable) = request.password_enable {
            body.insert("password_enable".to_string(), password_enable.to_string());
        }
        if let Some(ref password) = request.password {
            body.insert("password".to_string(), password.clone());
        }
        if let Some(attachment_separate) = request.attachment_separate {
            body.insert(
                "attachment_separate".to_string(),
                attachment_separate.to_string(),
            );
        }

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            url: "/open-apis/drive/v1/export_tasks".to_string(),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(&body)))?,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp =
            Transport::<CreateExportTaskResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "创建导出任务完成: file_token={}, ticket={:?}",
            request.file_token,
            response.ticket
        );

        Ok(response)
    }

    /// 查询导出任务结果
    pub async fn get_export_task(
        &self,
        request: &GetExportTaskRequest,
    ) -> SDKResult<GetExportTaskResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("查询导出任务结果: ticket={}", request.ticket);

        // 构建查询参数
        let query_params = HashMap::new();

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            url: format!("/open-apis/drive/v1/export_tasks/{}", request.ticket),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None,
            query_params,
            
        };

        // 发送请求
        let resp = Transport::<GetExportTaskResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "查询导出任务结果完成: ticket={}, status={:?}",
            request.ticket,
            response.result.as_ref().and_then(|r| r.status.as_ref())
        );

        Ok(response)
    }

    /// 下载导出文件
    pub async fn download_export_file(
        &self,
        request: &DownloadExportFileRequest,
    ) -> SDKResult<DownloadExportFileResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("下载导出文件: file_token={}", request.file_token);

        // 构建查询参数
        let query_params = HashMap::new();

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            url: format!(
                "/open-apis/drive/export_tasks/file/{}/download",
                request.file_token
            ),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None,
            query_params,
            
        };

        // 发送请求
        let resp =
            Transport::<DownloadExportFileResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "下载导出文件完成: file_token={}, file_size={:?}",
            request.file_token,
            response.file_size
        );

        Ok(response)
    }

    /// 创建创建导出任务构建器
    pub fn create_export_task_builder(&self) -> CreateExportTaskRequestBuilder {
        CreateExportTaskRequestBuilder::new()
    }

    /// 创建查询导出任务构建器
    pub fn get_export_task_builder(&self) -> GetExportTaskRequestBuilder {
        GetExportTaskRequestBuilder::new()
    }

    /// 创建下载导出文件构建器
    pub fn download_export_file_builder(&self) -> DownloadExportFileRequestBuilder {
        DownloadExportFileRequestBuilder::new()
    }
}

/// 创建导出任务请求构建器
#[derive(Debug, Clone)]
pub struct CreateExportTaskRequestBuilder {
    file_token: Option<String>,
    file_type: Option<String>,
    export_type: Option<String>,
    export_name: Option<String>,
    locale: Option<String>,
    include_comments: Option<bool>,
    password_enable: Option<bool>,
    password: Option<String>,
    attachment_separate: Option<bool>,
}

impl CreateExportTaskRequestBuilder {
    pub fn new() -> Self {
        Self {
            file_token: None,
            file_type: None,
            export_type: None,
            export_name: None,
            locale: None,
            include_comments: None,
            password_enable: None,
            password: None,
            attachment_separate: None,
        }
    }

    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    pub fn export_type(mut self, export_type: impl Into<String>) -> Self {
        self.export_type = Some(export_type.into());
        self
    }

    pub fn export_name(mut self, export_name: impl Into<String>) -> Self {
        self.export_name = Some(export_name.into());
        self
    }

    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    pub fn include_comments(mut self, include_comments: bool) -> Self {
        self.include_comments = Some(include_comments);
        self
    }

    pub fn password_enable(mut self, password_enable: bool) -> Self {
        self.password_enable = Some(password_enable);
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn attachment_separate(mut self, attachment_separate: bool) -> Self {
        self.attachment_separate = Some(attachment_separate);
        self
    }

    pub async fn execute(
        self,
        service: &ExportTasksService,
    ) -> SDKResult<CreateExportTaskResponse> {
        let request = CreateExportTaskRequest {
            file_token: self
                .file_token
                .ok_or_else(|| LarkAPIError::illegal_param("file_token is required".to_string()))?,
            file_type: self
                .file_type
                .ok_or_else(|| LarkAPIError::illegal_param("file_type is required".to_string()))?,
            export_type: self.export_type.ok_or_else(|| {
                LarkAPIError::illegal_param("export_type is required".to_string())
            })?,
            export_name: self.export_name,
            locale: self.locale,
            include_comments: self.include_comments,
            password_enable: self.password_enable,
            password: self.password,
            attachment_separate: self.attachment_separate,
        };
        service.create_export_task(&request).await
    }
}

impl Default for CreateExportTaskRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 查询导出任务请求构建器
#[derive(Debug, Clone)]
pub struct GetExportTaskRequestBuilder {
    ticket: Option<String>,
}

impl GetExportTaskRequestBuilder {
    pub fn new() -> Self {
        Self { ticket: None }
    }

    pub fn ticket(mut self, ticket: impl Into<String>) -> Self {
        self.ticket = Some(ticket.into());
        self
    }

    pub async fn execute(self, service: &ExportTasksService) -> SDKResult<GetExportTaskResponse> {
        let request = GetExportTaskRequest {
            ticket: self
                .ticket
                .ok_or_else(|| LarkAPIError::illegal_param("ticket is required".to_string()))?,
        };
        service.get_export_task(&request).await
    }
}

impl Default for GetExportTaskRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 下载导出文件请求构建器
#[derive(Debug, Clone)]
pub struct DownloadExportFileRequestBuilder {
    file_token: Option<String>,
}

impl DownloadExportFileRequestBuilder {
    pub fn new() -> Self {
        Self { file_token: None }
    }

    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    pub async fn execute(
        self,
        service: &ExportTasksService,
    ) -> SDKResult<DownloadExportFileResponse> {
        let request = DownloadExportFileRequest {
            file_token: self
                .file_token
                .ok_or_else(|| LarkAPIError::illegal_param("file_token is required".to_string()))?,
        };
        service.download_export_file(&request).await
    }
}

impl Default for DownloadExportFileRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
