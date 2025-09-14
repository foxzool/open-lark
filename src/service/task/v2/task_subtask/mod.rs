use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::task::models::{Task, UserIdType},
};

/// 子任务服务
pub struct TaskSubtaskService {
    pub config: Config,
}

/// 创建子任务请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubtaskRequest {
    /// 子任务标题
    pub summary: String,
    /// 子任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建子任务响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubtaskResponse {
    /// 创建的子任务
    pub task: Task,
}

impl ApiResponseTrait for CreateSubtaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 子任务列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListSubtasksResponse {
    /// 子任务列表
    pub items: Vec<Task>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListSubtasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TaskSubtaskService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建子任务
    pub async fn create(
        &self,
        task_guid: &str,
        request: CreateSubtaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateSubtaskResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASK_SUBTASKS,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取任务的子任务列表
    pub async fn list(
        &self,
        task_guid: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListSubtasksResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASK_SUBTASKS,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
