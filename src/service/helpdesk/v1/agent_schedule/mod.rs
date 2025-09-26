use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{helpdesk::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::helpdesk::models::{AgentSchedule, UserIdType},
};

/// 客服工作日程服务
pub struct AgentScheduleService {
    pub config: Config,
}

/// 创建客服工作日程请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAgentScheduleRequest {
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 重复模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_type: Option<String>,
}

/// 创建客服工作日程响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAgentScheduleResponse {
    /// 创建的工作日程
    pub schedule: AgentSchedule,
}

impl ApiResponseTrait for CreateAgentScheduleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新客服工作日程请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAgentScheduleRequest {
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 重复模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_type: Option<String>,
}

/// 更新客服工作日程响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAgentScheduleResponse {
    /// 更新后的工作日程
    pub schedule: AgentSchedule,
}

impl ApiResponseTrait for UpdateAgentScheduleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取客服工作日程响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAgentScheduleResponse {
    /// 工作日程
    pub schedule: AgentSchedule,
}

impl ApiResponseTrait for GetAgentScheduleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取全部客服工作日程响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAgentSchedulesResponse {
    /// 工作日程列表
    pub schedules: Vec<AgentSchedule>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListAgentSchedulesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AgentScheduleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建客服工作日程
    pub async fn create(
        &self,
        agent_id: &str,
        request: CreateAgentScheduleRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateAgentScheduleResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_AGENT_SCHEDULES,
                "agent_id",
                agent_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除客服工作日程
    pub async fn delete(
        &self,
        agent_id: &str,
        schedule_id: &str,
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
                &EndpointBuilder::replace_param(
                    HELPDESK_V1_AGENT_SCHEDULE_DELETE,
                    "agent_id",
                    agent_id,
                ),
                "schedule_id",
                schedule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新客服工作日程
    pub async fn patch(
        &self,
        agent_id: &str,
        schedule_id: &str,
        request: UpdateAgentScheduleRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateAgentScheduleResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                &EndpointBuilder::replace_param(
                    HELPDESK_V1_AGENT_SCHEDULE_DELETE,
                    "agent_id",
                    agent_id,
                ),
                "schedule_id",
                schedule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询指定客服工作日程
    pub async fn get(
        &self,
        agent_id: &str,
        schedule_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAgentScheduleResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                &EndpointBuilder::replace_param(
                    HELPDESK_V1_AGENT_SCHEDULE_DELETE,
                    "agent_id",
                    agent_id,
                ),
                "schedule_id",
                schedule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询全部客服工作日程
    pub async fn list(
        &self,
        agent_id: &str,
        user_id_type: Option<UserIdType>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListAgentSchedulesResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token.to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_AGENT_SCHEDULES,
                "agent_id",
                agent_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
