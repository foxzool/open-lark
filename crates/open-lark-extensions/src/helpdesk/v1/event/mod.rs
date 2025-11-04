use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::helpdesk::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::helpdesk::models::UserIdType,
};

/// 事件订阅服务
pub struct EventService {
    pub config: Config,
}

/// 订阅服务台事件请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeEventRequest {
    /// 事件类型列表
    pub events: Vec<String>,
    /// 回调URL
    pub callback_url: String,
}

/// 订阅服务台事件响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeEventResponse {
    /// 订阅ID
    pub subscription_id: String,
}

impl ApiResponseTrait for SubscribeEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 取消订阅服务台事件请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeEventRequest {
    /// 订阅ID
    pub subscription_id: String,
}

impl EventService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 订阅服务台事件
    ///
    /// 该接口用于订阅服务台相关事件。
    ///
    /// # 参数
    ///
    /// - `request`: 订阅请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 回调URL无效
    pub async fn subscribe(
        &self,
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
            api_path: HELPDESK_V1_EVENTS_SUBSCRIBE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 取消订阅服务台事件
    ///
    /// 该接口用于取消订阅服务台事件。
    ///
    /// # 参数
    ///
    /// - `request`: 取消订阅请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 订阅不存在
    pub async fn unsubscribe(
        &self,
        request: UnsubscribeEventRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: HELPDESK_V1_EVENTS_UNSUBSCRIBE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
