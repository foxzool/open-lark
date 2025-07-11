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
    service::im::v1::models::{Pin, UserIdType},
};

/// Pin消息服务
pub struct PinService {
    pub config: Config,
}

/// Pin信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePinResponse {
    /// Pin信息
    pub pin: PinInfo,
}

/// Pin信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PinInfo {
    /// Pin ID
    pub pin_id: String,
    /// 消息ID
    pub message_id: String,
    /// 聊天ID
    pub chat_id: String,
    /// Pin创建者
    pub operator_id: String,
    /// 创建时间
    pub create_time: String,
}

impl ApiResponseTrait for CreatePinResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取Pin消息列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPinResponse {
    /// Pin消息列表
    pub pins: Vec<Pin>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListPinResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PinService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Pin消息
    pub async fn create(
        &self,
        message_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreatePinResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("message_id".to_string(), message_id.to_string());
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/im/v1/pins".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 移除Pin消息
    pub async fn delete(
        &self,
        pin_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let query_params = if let Some(user_id_type) = user_id_type {
            HashMap::from([(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            )])
        } else {
            HashMap::new()
        };

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: format!("/open-apis/im/v1/pins/{pin_id}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取群内Pin消息
    pub async fn list(
        &self,
        chat_id: &str,
        user_id_type: Option<UserIdType>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListPinResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("chat_id".to_string(), chat_id.to_string());
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
            query_params.insert("page_token".to_string(), page_token);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/im/v1/pins".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
