/// 导出任务API服务实现
///
/// 提供文档导出任务相关的API服务，包括：
/// - 创建导出任务
/// - 查询导出任务结果
/// - 下载导出文件
use std::collections::HashMap;

use openlark_core::{
    api::{ApiRequest, HttpMethod}, config::Config, constants::AccessTokenType, error::{validation_error, LarkAPIError},
    http::Transport, SDKResult,
};

use super::models::*;

use serde_json::json;

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
            .map_err(|e| validation_error("request", &format!("请求参数验证失败: {}", e)))?;

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
        use crate::common::api_endpoints::DriveApi;
        let api_endpoint = DriveApi::CreateExportTask;
        let api_req = ApiRequest {
            method: HttpMethod::Post,
            url: api_endpoint.to_url(),
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
            .map_err(|e| validation_error("request", &format!("请求参数验证失败: {}", e)))?;

        log::info!("查询导出任务结果: ticket={}", request.ticket);

        // 构建查询参数
