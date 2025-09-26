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
    service::helpdesk::models::{Notification, UserIdType},
};

/// 推送中心服务
pub struct NotificationService {
    pub config: Config,
}

/// 创建推送请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    /// 标题
    pub title: String,
    /// 内容
    pub content: String,
    /// 目标用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_users: Option<Vec<String>>,
    /// 计划发送时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

/// 创建推送响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationResponse {
    /// 创建的推送
    pub notification: Notification,
}

impl ApiResponseTrait for CreateNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新推送请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNotificationRequest {
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 目标用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_users: Option<Vec<String>>,
    /// 计划发送时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

/// 更新推送响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNotificationResponse {
    /// 更新后的推送
    pub notification: Notification,
}

impl ApiResponseTrait for UpdateNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询推送响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetNotificationResponse {
    /// 推送详情
    pub notification: Notification,
}

impl ApiResponseTrait for GetNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 预览推送响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewNotificationResponse {
    /// 预览内容
    pub preview: String,
}

impl ApiResponseTrait for PreviewNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl NotificationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建推送
    pub async fn create(
        &self,
        request: CreateNotificationRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateNotificationResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: HELPDESK_V1_NOTIFICATIONS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新推送
    pub async fn patch(
        &self,
        notification_id: &str,
        request: UpdateNotificationRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateNotificationResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_NOTIFICATION_UPDATE,
                "notification_id",
                notification_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询推送
    pub async fn get(
        &self,
        notification_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetNotificationResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_NOTIFICATION_GET,
                "notification_id",
                notification_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 预览推送
    pub async fn preview(
        &self,
        notification_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PreviewNotificationResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_NOTIFICATION_PREVIEW,
                "notification_id",
                notification_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 提交审核
    pub async fn submit_approve(
        &self,
        notification_id: &str,
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
                HELPDESK_V1_NOTIFICATION_SUBMIT_APPROVE,
                "notification_id",
                notification_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 取消审核
    pub async fn cancel_approve(
        &self,
        notification_id: &str,
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
                HELPDESK_V1_NOTIFICATION_CANCEL_APPROVE,
                "notification_id",
                notification_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 执行推送
    pub async fn execute_send(
        &self,
        notification_id: &str,
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
                HELPDESK_V1_NOTIFICATION_EXECUTE_SEND,
                "notification_id",
                notification_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 取消推送
    pub async fn cancel_send(
        &self,
        notification_id: &str,
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
                HELPDESK_V1_NOTIFICATION_CANCEL_SEND,
                "notification_id",
                notification_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
