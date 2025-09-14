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
        SDKResult,
    },
    service::mail::models::{SubscriptionStatus, UserIdType},
};

/// 事件订阅服务
pub struct EventService {
    pub config: Config,
}

/// 订阅事件请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeEventRequest {
    /// 事件类型
    pub event_type: String,
    /// 回调URL
    pub callback_url: String,
}

/// 订阅事件响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeEventResponse {
    /// 订阅状态
    pub subscription: SubscriptionStatus,
}

impl ApiResponseTrait for SubscribeEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取订阅状态响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubscriptionResponse {
    /// 订阅状态
    pub subscription: SubscriptionStatus,
}

impl ApiResponseTrait for GetSubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EventService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 订阅事件
    pub async fn subscribe(
        &self,
        user_mailbox_id: &str,
        request: SubscribeEventRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SubscribeEventResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取订阅状态
    pub async fn subscription(
        &self,
        user_mailbox_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetSubscriptionResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIPTION,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 取消订阅
    pub async fn unsubscribe(
        &self,
        user_mailbox_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_EVENTS_UNSUBSCRIBE,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
