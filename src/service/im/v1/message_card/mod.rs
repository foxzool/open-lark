use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use crate::impl_full_service;

/// 消息卡片服务
pub struct MessageCardService {
    pub config: Config,
}

/// 更新消息卡片请求
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchMessageCardRequest {
    /// 卡片内容
    pub card: Value,
    /// 令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

// 接入统一 Service 抽象（IM v1 - MessageCardService）
impl_full_service!(MessageCardService, "im.message_card", "v1");

/// 延时更新消息卡片请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DelayUpdateMessageCardRequest {
    /// 延时时间(秒)
    pub delay: i32,
    /// 卡片内容
    pub card: Value,
    /// 令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// 发送仅特定人可见的消息卡片请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SendVisibleMessageCardRequest {
    /// 可见用户ID列表
    pub open_ids: Vec<String>,
    /// 卡片内容
    pub card: Value,
}

/// 发送仅特定人可见的消息卡片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVisibleMessageCardResponse {
    /// 消息ID
    pub message_id: String,
}

impl ApiResponseTrait for SendVisibleMessageCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MessageCardService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 更新应用发送的消息卡片
    pub async fn patch(
        &self,
        message_id: &str,
        request: PatchMessageCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V1_UPDATE_MESSAGE,
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 延时更新消息卡片
    pub async fn delay_update(
        &self,
        message_id: &str,
        request: DelayUpdateMessageCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V1_MESSAGE_DELAY_UPDATE,
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 发送仅特定人可见的消息卡片
    pub async fn send_visible(
        &self,
        message_id: &str,
        request: SendVisibleMessageCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SendVisibleMessageCardResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V1_MESSAGE_URGENT_APP,
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除仅特定人可见的消息卡片
    pub async fn delete_visible(
        &self,
        message_id: &str,
        open_ids: Vec<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let request = serde_json::json!({
            "open_ids": open_ids
        });

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V1_MESSAGE_URGENT_APP,
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
