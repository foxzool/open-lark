use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::task::models::{Task, TaskMember, Tasklist, UserIdType},
};

/// 清单服务
pub struct TasklistService {
    pub config: Config,
}

/// 创建清单请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTasklistRequest {
    /// 清单名称
    pub name: String,
    /// 清单成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskMember>>,
}

/// 创建清单响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTasklistResponse {
    /// 创建的清单
    pub tasklist: Tasklist,
}

impl ApiResponseTrait for CreateTasklistResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新清单请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTasklistRequest {
    /// 清单名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 更新清单响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTasklistResponse {
    /// 更新后的清单
    pub tasklist: Tasklist,
}

impl ApiResponseTrait for UpdateTasklistResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取清单响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTasklistResponse {
    /// 清单详情
    pub tasklist: Tasklist,
}

impl ApiResponseTrait for GetTasklistResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 清单列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListTasklistsResponse {
    /// 清单列表
    pub items: Vec<Tasklist>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListTasklistsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加清单成员请求
#[derive(Debug, Serialize, Deserialize)]
pub struct AddTasklistMembersRequest {
    /// 成员列表
    pub members: Vec<TaskMember>,
}

/// 添加清单成员响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AddTasklistMembersResponse {
    /// 成员列表
    pub members: Vec<TaskMember>,
}

impl ApiResponseTrait for AddTasklistMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移除清单成员请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveTasklistMembersRequest {
    /// 成员ID列表
    pub members: Vec<String>,
}

/// 移除清单成员响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveTasklistMembersResponse {
    /// 成员列表
    pub members: Vec<TaskMember>,
}

impl ApiResponseTrait for RemoveTasklistMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 清单任务列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TasklistTasksResponse {
    /// 任务列表
    pub items: Vec<Task>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for TasklistTasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TasklistService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建清单
    pub async fn create(
        &self,
        request: CreateTasklistRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateTasklistResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::TASK_V2_TASKLISTS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取清单详情
    pub async fn get(
        &self,
        tasklist_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTasklistResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASKLIST_GET,
                "tasklist_guid",
                tasklist_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新清单
    pub async fn patch(
        &self,
        tasklist_guid: &str,
        request: UpdateTasklistRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateTasklistResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASKLIST_GET,
                "tasklist_guid",
                tasklist_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除清单
    pub async fn delete(
        &self,
        tasklist_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASKLIST_GET,
                "tasklist_guid",
                tasklist_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 添加清单成员
    pub async fn add_members(
        &self,
        tasklist_guid: &str,
        request: AddTasklistMembersRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AddTasklistMembersResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASKLIST_ADD_MEMBERS,
                "tasklist_guid",
                tasklist_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 移除清单成员
    pub async fn remove_members(
        &self,
        tasklist_guid: &str,
        request: RemoveTasklistMembersRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RemoveTasklistMembersResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASKLIST_REMOVE_MEMBERS,
                "tasklist_guid",
                tasklist_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取清单任务列表
    #[allow(clippy::too_many_arguments)]
    pub async fn tasks(
        &self,
        tasklist_guid: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        completed: Option<bool>,
        created_from: Option<&str>,
        created_to: Option<&str>,
        updated_from: Option<&str>,
        updated_to: Option<&str>,
        due_from: Option<&str>,
        due_to: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<TasklistTasksResponse>> {
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
        if let Some(completed) = completed {
            query_params.insert("completed", completed.to_string());
        }
        if let Some(created_from) = created_from {
            query_params.insert("created_from", created_from.to_string());
        }
        if let Some(created_to) = created_to {
            query_params.insert("created_to", created_to.to_string());
        }
        if let Some(updated_from) = updated_from {
            query_params.insert("updated_from", updated_from.to_string());
        }
        if let Some(updated_to) = updated_to {
            query_params.insert("updated_to", updated_to.to_string());
        }
        if let Some(due_from) = due_from {
            query_params.insert("due_from", due_from.to_string());
        }
        if let Some(due_to) = due_to {
            query_params.insert("due_to", due_to.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASKLIST_TASKS,
                "tasklist_guid",
                tasklist_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取清单列表
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListTasklistsResponse>> {
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
            api_path: Endpoints::TASK_V2_TASKLISTS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for TasklistService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "tasklist"
    }

    fn service_version() -> &'static str {
        "v2"
    }
}
