use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{helpdesk::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::helpdesk::models::{TicketMessage, UserIdType},
};

/// 工单消息服务
pub struct TicketMessageService {
    pub config: Config,
}

/// 发送工单消息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTicketMessageRequest {
    /// 消息内容
    pub content: String,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
}

/// 发送工单消息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTicketMessageResponse {
    /// 创建的消息
    pub message: TicketMessage,
}

impl ApiResponseTrait for CreateTicketMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取工单消息详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListTicketMessagesResponse {
    /// 消息列表
    pub messages: Vec<TicketMessage>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListTicketMessagesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 向群内发送消息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupMessageRequest {
    /// 消息内容
    pub content: String,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
}

/// 向群内发送消息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupMessageResponse {
    /// 创建的消息
    pub message: TicketMessage,
}

impl ApiResponseTrait for CreateGroupMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TicketMessageService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 发送工单消息
    ///
    /// 该接口用于向指定工单发送消息。
    ///
    /// # 参数
    ///
    /// - `ticket_id`: 工单ID
    /// - `request`: 消息请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 工单不存在
    pub async fn create(
        &self,
        ticket_id: &str,
        request: CreateTicketMessageRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateTicketMessageResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_TICKET_MESSAGES,
                "ticket_id",
                ticket_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取工单消息详情
    ///
    /// 该接口用于获取指定工单的消息列表。
    ///
    /// # 参数
    ///
    /// - `ticket_id`: 工单ID
    /// - `user_id_type`: 用户ID类型
    /// - `page_token`: 分页标记
    /// - `page_size`: 分页大小
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 工单不存在
    pub async fn list(
        &self,
        ticket_id: &str,
        user_id_type: Option<UserIdType>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListTicketMessagesResponse>> {
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
                HELPDESK_V1_TICKET_MESSAGES,
                "ticket_id",
                ticket_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 服务台机器人向工单绑定的群内发送消息
    ///
    /// 该接口用于服务台机器人向工单绑定的群内发送消息。
    ///
    /// # 参数
    ///
    /// - `ticket_id`: 工单ID
    /// - `request`: 消息请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 工单不存在
    /// - 群组不存在
    pub async fn create_group_message(
        &self,
        ticket_id: &str,
        request: CreateGroupMessageRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateGroupMessageResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_TICKET_BOT_MESSAGES,
                "ticket_id",
                ticket_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
