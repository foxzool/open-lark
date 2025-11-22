//! Bitable App Workflow API 服务实现
//!
//! 提供多维表格工作流管理相关的API服务，包括：
//! - 工作流的查询、更新
//! - 工作流状态管理
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use serde_json::Value;
use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 多维表格工作流管理服务
#[derive(Debug, Clone)]
pub struct AppWorkflowService {
    config: Config,
}

impl AppWorkflowService {
    /// 创建新的工作流管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 列出工作流
    ///
    /// 获取指定应用中的工作流列表
    ///
    /// # 参数
    /// * `request` - 列出工作流请求
    ///
    /// # 返回
    /// 返回工作流列表
    pub async fn list_workflow(
        &self,
        request: &ListWorkflowRequest,
    ) -> SDKResult<ListWorkflowResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("列出工作流: app_token={}", request.app_token);

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!("/open-apis/bitable/v1/apps/{}/workflows", request.app_token))
            .query(query_params);

        // 发送请求
        let resp = Transport::<ListWorkflowResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出工作流完成: app_token={}, count={}",
            request.app_token,
            response.workflows.as_ref().map(|w| w.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 更新工作流
    ///
    /// 更新指定应用中的工作流
    ///
    /// # 参数
    /// * `request` - 更新工作流请求
    ///
    /// # 返回
    /// 返回更新后的工作流信息
    pub async fn update_workflow(
        &self,
        request: &UpdateWorkflowRequest,
    ) -> SDKResult<UpdateWorkflowResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新工作流: app_token={}, workflow_id={}",
            request.app_token,
            request.workflow_id
        );

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref name) = request.name {
            body.insert("name".to_string(), serde_json::to_value(name)?);
        }
        if let Some(ref description) = request.description {
            body.insert(
                "description".to_string(),
                serde_json::to_value(description)?,
            );
        }
        if let Some(ref config) = request.config {
            body.insert("config".to_string(), serde_json::to_value(config)?);
        }
        if let Some(ref status) = request.status {
            body.insert("status".to_string(), serde_json::to_value(status)?);
        }

        // 构建API请求
        let api_req = ApiRequest::put(format!(
                "/open-apis/bitable/v1/apps/{}/workflows/{}",
                request.app_token, request.workflow_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<UpdateWorkflowResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新工作流完成: app_token={}, workflow_id={}",
            request.app_token,
            request.workflow_id
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct ListWorkflowRequestBuilder {
    request: ListWorkflowRequest,
}

impl ListWorkflowRequestBuilder {
    pub fn new(app_token: impl Into<String>) -> Self {
        Self {
            request: ListWorkflowRequest {
                app_token: app_token.into(),
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

    pub async fn execute(self, service: &AppWorkflowService) -> SDKResult<ListWorkflowResponse> {
        service.list_workflow(&self.request).await
    }
}

pub struct UpdateWorkflowRequestBuilder {
    request: UpdateWorkflowRequest,
}

impl UpdateWorkflowRequestBuilder {
    pub fn new(app_token: impl Into<String>, workflow_id: impl Into<String>) -> Self {
        Self {
            request: UpdateWorkflowRequest {
                app_token: app_token.into(),
                workflow_id: workflow_id.into(),
                name: None,
                description: None,
                config: None,
                status: None,
            },
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn config(mut self, config: Value) -> Self {
        self.request.config = Some(config);
        self
    }

    pub fn status(mut self, status: WorkflowStatus) -> Self {
        self.request.status = Some(status);
        self
    }

    pub async fn execute(self, service: &AppWorkflowService) -> SDKResult<UpdateWorkflowResponse> {
        service.update_workflow(&self.request).await
    }
}
