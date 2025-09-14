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
    service::mail::models::{MailGroupMember, UserIdType},
};

/// 邮件组管理员服务
pub struct MailGroupManagerService {
    pub config: Config,
}

/// 批量创建邮件组管理员请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateManagersRequest {
    /// 管理员列表
    pub managers: Vec<MailGroupMember>,
}

/// 批量创建邮件组管理员响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateManagersResponse {
    /// 创建的管理员列表
    pub managers: Vec<MailGroupMember>,
}

impl ApiResponseTrait for BatchCreateManagersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除邮件组管理员请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteManagersRequest {
    /// 管理员ID列表
    pub manager_ids: Vec<String>,
}

/// 批量获取邮件组管理员响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListManagersResponse {
    /// 管理员列表
    pub managers: Vec<MailGroupMember>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListManagersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MailGroupManagerService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量创建邮件组管理员
    pub async fn batch_create(
        &self,
        mailgroup_id: &str,
        request: BatchCreateManagersRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchCreateManagersResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP_MANAGERS_BATCH_CREATE,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量删除邮件组管理员
    pub async fn batch_delete(
        &self,
        mailgroup_id: &str,
        request: BatchDeleteManagersRequest,
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
                Endpoints::MAIL_V1_MAILGROUP_MANAGERS_BATCH_DELETE,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量获取邮件组管理员
    pub async fn list(
        &self,
        mailgroup_id: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListManagersResponse>> {
        let mut query_params = HashMap::new();
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP_MANAGERS,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
