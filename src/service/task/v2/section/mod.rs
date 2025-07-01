use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::task::models::{Section, Task, UserIdType},
};

/// 自定义分组服务
pub struct SectionService {
    pub config: Config,
}

/// 创建自定义分组请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSectionRequest {
    /// 分组名称
    pub name: String,
    /// 分组所属的清单GUID
    pub tasklist_guid: String,
}

/// 创建自定义分组响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSectionResponse {
    /// 创建的分组
    pub section: Section,
}

impl ApiResponseTrait for CreateSectionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新自定义分组请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSectionRequest {
    /// 分组名称
    pub name: String,
}

/// 更新自定义分组响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSectionResponse {
    /// 更新后的分组
    pub section: Section,
}

impl ApiResponseTrait for UpdateSectionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取自定义分组响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSectionResponse {
    /// 分组详情
    pub section: Section,
}

impl ApiResponseTrait for GetSectionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 自定义分组列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListSectionsResponse {
    /// 分组列表
    pub items: Vec<Section>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListSectionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 自定义分组任务列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SectionTasksResponse {
    /// 任务列表
    pub items: Vec<Task>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for SectionTasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SectionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建自定义分组
    pub async fn create(
        &self,
        request: CreateSectionRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateSectionResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/task/v2/sections".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取自定义分组详情
    pub async fn get(
        &self,
        section_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetSectionResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/task/v2/sections/{section_guid}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新自定义分组
    pub async fn patch(
        &self,
        section_guid: &str,
        request: UpdateSectionRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateSectionResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/task/v2/sections/{section_guid}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除自定义分组
    pub async fn delete(
        &self,
        section_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: format!("/open-apis/task/v2/sections/{section_guid}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取自定义分组列表
    pub async fn list(
        &self,
        tasklist_guid: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListSectionsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }
        query_params.insert("tasklist_guid".to_string(), tasklist_guid.to_string());
        if let Some(page_size) = page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token".to_string(), page_token.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/task/v2/sections".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取自定义分组任务列表
    #[allow(clippy::too_many_arguments)]
    pub async fn tasks(
        &self,
        section_guid: &str,
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
    ) -> SDKResult<BaseResponse<SectionTasksResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token".to_string(), page_token.to_string());
        }
        if let Some(completed) = completed {
            query_params.insert("completed".to_string(), completed.to_string());
        }
        if let Some(created_from) = created_from {
            query_params.insert("created_from".to_string(), created_from.to_string());
        }
        if let Some(created_to) = created_to {
            query_params.insert("created_to".to_string(), created_to.to_string());
        }
        if let Some(updated_from) = updated_from {
            query_params.insert("updated_from".to_string(), updated_from.to_string());
        }
        if let Some(updated_to) = updated_to {
            query_params.insert("updated_to".to_string(), updated_to.to_string());
        }
        if let Some(due_from) = due_from {
            query_params.insert("due_from".to_string(), due_from.to_string());
        }
        if let Some(due_to) = due_to {
            query_params.insert("due_to".to_string(), due_to.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/task/v2/sections/{section_guid}/tasks"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
