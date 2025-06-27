use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::helpdesk::models::{Ticket, UserIdType},
};

/// 工单管理服务
pub struct TicketService {
    pub config: Config,
}

/// 创建服务台对话请求
#[derive(Debug, Serialize, Deserialize)]
pub struct StartServiceRequest {
    /// 用户开放平台ID
    pub open_id: String,
    /// 服务台ID
    pub helpdesk_id: String,
    /// 问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建服务台对话响应
#[derive(Debug, Serialize, Deserialize)]
pub struct StartServiceResponse {
    /// 聊天群ID
    pub chat_id: String,
    /// 工单信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<Ticket>,
}

impl ApiResponseTrait for StartServiceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取工单详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTicketResponse {
    /// 工单信息
    pub ticket: Ticket,
}

impl ApiResponseTrait for GetTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新工单请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTicketRequest {
    /// 工单状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 评论
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 更新工单响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTicketResponse {
    /// 更新后的工单信息
    pub ticket: Ticket,
}

impl ApiResponseTrait for UpdateTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取全部工单响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListTicketsResponse {
    /// 工单列表
    pub tickets: Vec<Ticket>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListTicketsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TicketService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建服务台对话
    pub async fn start_service(
        &self,
        request: StartServiceRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<StartServiceResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/helpdesk/v1/start_service".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询指定工单详情
    pub async fn get(
        &self,
        ticket_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTicketResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/helpdesk/v1/tickets/{}", ticket_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新工单详情
    pub async fn update(
        &self,
        ticket_id: &str,
        request: UpdateTicketRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateTicketResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: format!("/open-apis/helpdesk/v1/tickets/{}", ticket_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询全部工单详情
    pub async fn list(
        &self,
        user_id_type: Option<UserIdType>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListTicketsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token".to_string(), page_token.to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/helpdesk/v1/tickets".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    // TODO: 实现其他工单相关接口
    // - ticket_image: 获取工单内图像
    // - answer_user_query: 回复用户在工单里的提问
    // - customized_fields: 获取服务台自定义字段
}
