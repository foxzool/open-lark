//! Report Task API 服务实现
//!
//! 提供报告任务管理相关的API服务，包括：
//! - 任务查询和过滤
use std::collections::HashMap;
//! - 任务状态跟踪
//! - 任务执行历史
//! - 完整的错误处理和参数验证


use openlark_core::{
    error::LarkAPIError,
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    api_req::ApiRequest,
    SDKResult,
};

use super::models::*;

/// 报告任务管理服务
#[derive(Debug, Clone)]
pub struct TaskService {
    config: Config,
}

impl TaskService {
    /// 创建新的报告任务管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询报告任务
    ///
    /// 查询符合条件的报告任务列表
    ///
    /// # 参数
    /// * `request` - 查询报告任务请求
    ///
    /// # 返回
    /// 返回报告任务列表数据
    pub async fn query_task(&self, request: &QueryTaskRequest) -> SDKResult<QueryTaskResponse> {
        // 验证请求参数
        request.validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("查询报告任务: task_type={:?}, status={:?}, page_size={:?}",
                  request.task_type, request.status, request.page_size);

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(ref task_type) = request.task_type {
            query_params.insert("task_type", task_type.clone());
        }
        if let Some(ref status) = request.status {
            query_params.insert("status", status.clone());
        }
        if let Some(ref rule_id) = request.rule_id {
            query_params.insert("rule_id", rule_id.clone());
        }
        if let Some(ref creator_id) = request.creator_id {
            query_params.insert("creator_id", creator_id.clone());
        }
        if let Some(start_time) = request.start_time {
            query_params.insert("start_time", start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            query_params.insert("end_time", end_time.to_string());
        }
        if let Some(ref user_id_type) = request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }
        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/report/v1/tasks/query".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        // 发送请求
        let resp = Transport::<QueryTaskResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("查询报告任务完成: total={}, has_more={}",
                  response.data.as_ref()
                      .map(|d| d.total)
                      .unwrap_or(0),
                  response.data.as_ref()
                      .map(|d| d.has_more)
                      .unwrap_or(false));

        Ok(response)
    }
}

// 构建器模式实现
pub struct QueryTaskRequestBuilder {
    request: QueryTaskRequest,
}

impl QueryTaskRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: QueryTaskRequest::default(),
        }
    }

    pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
        self.request.task_type = Some(task_type.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.request.status = Some(status.into());
        self
    }

    pub fn rule_id(mut self, rule_id: impl Into<String>) -> Self {
        self.request.rule_id = Some(rule_id.into());
        self
    }

    pub fn creator_id(mut self, creator_id: impl Into<String>) -> Self {
        self.request.creator_id = Some(creator_id.into());
        self
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.request.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.request.end_time = Some(end_time);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self, service: &TaskService) -> SDKResult<QueryTaskResponse> {
        service.query_task(&self.request).await
    }
}