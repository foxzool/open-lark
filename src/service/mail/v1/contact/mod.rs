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
    service::mail::models::{Contact, UserIdType},
};

/// 邮箱联系人服务
pub struct ContactService {
    pub config: Config,
}

/// 创建邮箱联系人请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContactRequest {
    /// 联系人姓名
    pub name: String,
    /// 邮箱地址
    pub email: String,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

/// 创建邮箱联系人响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContactResponse {
    /// 创建的联系人
    pub contact: Contact,
}

impl ApiResponseTrait for CreateContactResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改邮箱联系人请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContactRequest {
    /// 联系人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

/// 修改邮箱联系人响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContactResponse {
    /// 修改后的联系人
    pub contact: Contact,
}

impl ApiResponseTrait for UpdateContactResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出邮箱联系人响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListContactsResponse {
    /// 联系人列表
    pub contacts: Vec<Contact>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListContactsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ContactService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建邮箱联系人
    pub async fn create(
        &self,
        user_mailbox_id: &str,
        request: CreateContactRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateContactResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_MAIL_CONTACTS,
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

    /// 删除邮箱联系人
    pub async fn delete(
        &self,
        user_mailbox_id: &str,
        contact_id: &str,
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
                    Endpoints::MAIL_V1_USER_MAILBOX_MAIL_CONTACT,
                    "user_mailbox_id",
                    user_mailbox_id,
                ),
                "contact_id",
                contact_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改邮箱联系人信息
    pub async fn patch(
        &self,
        user_mailbox_id: &str,
        contact_id: &str,
        request: UpdateContactRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateContactResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                &EndpointBuilder::replace_param(
                    Endpoints::MAIL_V1_USER_MAILBOX_MAIL_CONTACT,
                    "user_mailbox_id",
                    user_mailbox_id,
                ),
                "contact_id",
                contact_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 列出邮箱联系人
    pub async fn list(
        &self,
        user_mailbox_id: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListContactsResponse>> {
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
                Endpoints::MAIL_V1_USER_MAILBOX_MAIL_CONTACTS,
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
