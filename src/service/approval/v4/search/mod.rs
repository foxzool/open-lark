use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::approval::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::approval::models::{ApprovalInstance, ApprovalTask, UserIdType},
};

/// 审批查询服务
pub struct SearchService {
    pub config: Config,
}

/// 查询实例列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchInstancesResponse {
    /// 实例列表
    pub instances: Vec<ApprovalInstance>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchInstancesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTasksResponse {
    /// 任务列表
    pub tasks: Vec<ApprovalTask>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchTasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询抄送列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCcResponse {
    /// 抄送列表
    pub cc_list: Vec<serde_json::Value>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchCcResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询审批ID响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchApprovalIdResponse {
    /// 审批定义列表
    pub approval_list: Vec<SearchApprovalItem>,
}

/// 审批项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchApprovalItem {
    /// 审批编码
    pub approval_code: String,
    /// 审批名称
    pub approval_name: String,
}

impl ApiResponseTrait for SearchApprovalIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 实例查询参数
#[derive(Debug, Default)]
pub struct SearchInstanceParams {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub approval_code: Option<String>,
    pub instance_status: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub user_id: Option<String>,
    pub user_id_type: Option<UserIdType>,
}

/// 任务查询参数
#[derive(Debug, Default)]
pub struct SearchTaskParams {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub approval_code: Option<String>,
    pub instance_code: Option<String>,
    pub task_status: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub user_id: Option<String>,
    pub user_id_type: Option<UserIdType>,
}

/// 抄送查询参数
#[derive(Debug, Default)]
pub struct SearchCcParams {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub approval_code: Option<String>,
    pub instance_code: Option<String>,
    pub user_id: Option<String>,
    pub user_id_type: Option<UserIdType>,
}

impl SearchService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询实例列表
    pub async fn instances(
        &self,
        params: Option<SearchInstanceParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchInstancesResponse>> {
        let mut query_params = HashMap::new();
        if let Some(params) = params {
            if let Some(page_size) = params.page_size {
                query_params.insert("page_size", page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                query_params.insert("page_token", page_token);
            }
            if let Some(approval_code) = params.approval_code {
                query_params.insert("approval_code", approval_code);
            }
            if let Some(instance_status) = params.instance_status {
                query_params.insert("instance_status", instance_status);
            }
            if let Some(start_time) = params.start_time {
                query_params.insert("start_time", start_time);
            }
            if let Some(end_time) = params.end_time {
                query_params.insert("end_time", end_time);
            }
            if let Some(user_id) = params.user_id {
                query_params.insert("user_id", user_id);
            }
            if let Some(user_id_type) = params.user_id_type {
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
            }
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: APPROVAL_V4_INSTANCES_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询任务列表
    pub async fn tasks(
        &self,
        params: Option<SearchTaskParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchTasksResponse>> {
        let mut query_params = HashMap::new();
        if let Some(params) = params {
            if let Some(page_size) = params.page_size {
                query_params.insert("page_size", page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                query_params.insert("page_token", page_token);
            }
            if let Some(approval_code) = params.approval_code {
                query_params.insert("approval_code", approval_code);
            }
            if let Some(instance_code) = params.instance_code {
                query_params.insert("instance_code", instance_code);
            }
            if let Some(task_status) = params.task_status {
                query_params.insert("task_status", task_status);
            }
            if let Some(start_time) = params.start_time {
                query_params.insert("start_time", start_time);
            }
            if let Some(end_time) = params.end_time {
                query_params.insert("end_time", end_time);
            }
            if let Some(user_id) = params.user_id {
                query_params.insert("user_id", user_id);
            }
            if let Some(user_id_type) = params.user_id_type {
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
            }
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: APPROVAL_V4_TASKS_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询抄送列表
    pub async fn cc(
        &self,
        params: Option<SearchCcParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchCcResponse>> {
        let mut query_params = HashMap::new();
        if let Some(params) = params {
            if let Some(page_size) = params.page_size {
                query_params.insert("page_size", page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                query_params.insert("page_token", page_token);
            }
            if let Some(approval_code) = params.approval_code {
                query_params.insert("approval_code", approval_code);
            }
            if let Some(instance_code) = params.instance_code {
                query_params.insert("instance_code", instance_code);
            }
            if let Some(user_id) = params.user_id {
                query_params.insert("user_id", user_id);
            }
            if let Some(user_id_type) = params.user_id_type {
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
            }
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: APPROVAL_V4_INSTANCES_SEARCH_CC.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询审批ID
    pub async fn approval_id(
        &self,
        approval_name: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchApprovalIdResponse>> {
        let mut query_params = HashMap::new();
        if let Some(approval_name) = approval_name {
            query_params.insert("approval_name", approval_name.to_string());
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: APPROVAL_V4_APPROVALS_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询用户的任务列表
    pub async fn user_tasks(
        &self,
        user_id: &str,
        params: Option<SearchTaskParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchTasksResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("user_id", user_id.to_string());

        if let Some(params) = params {
            if let Some(page_size) = params.page_size {
                query_params.insert("page_size", page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                query_params.insert("page_token", page_token);
            }
            if let Some(approval_code) = params.approval_code {
                query_params.insert("approval_code", approval_code);
            }
            if let Some(task_status) = params.task_status {
                query_params.insert("task_status", task_status);
            }
            if let Some(start_time) = params.start_time {
                query_params.insert("start_time", start_time);
            }
            if let Some(end_time) = params.end_time {
                query_params.insert("end_time", end_time);
            }
            if let Some(user_id_type) = params.user_id_type {
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
            }
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: APPROVAL_V4_TASKS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
