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
    service::mail::models::{MailGroup, UserIdType},
};

/// 邮件组管理服务
pub struct MailGroupService {
    pub config: Config,
}

/// 创建邮件组请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMailGroupRequest {
    /// 邮件组名称
    pub name: String,
    /// 邮件组邮箱
    pub email: String,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否允许外部发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_send: Option<bool>,
}

/// 创建邮件组响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMailGroupResponse {
    /// 创建的邮件组
    pub mailgroup: MailGroup,
}

impl ApiResponseTrait for CreateMailGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改邮件组请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMailGroupRequest {
    /// 邮件组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否允许外部发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_send: Option<bool>,
}

/// 修改邮件组响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMailGroupResponse {
    /// 修改后的邮件组
    pub mailgroup: MailGroup,
}

impl ApiResponseTrait for UpdateMailGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询指定邮件组响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetMailGroupResponse {
    /// 邮件组信息
    pub mailgroup: MailGroup,
}

impl ApiResponseTrait for GetMailGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取邮件组响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMailGroupsResponse {
    /// 邮件组列表
    pub mailgroups: Vec<MailGroup>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListMailGroupsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MailGroupService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建邮件组
    pub async fn create(
        &self,
        request: CreateMailGroupRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateMailGroupResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::MAIL_V1_MAILGROUPS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除邮件组
    pub async fn delete(
        &self,
        mailgroup_id: &str,
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
                Endpoints::MAIL_V1_MAILGROUP,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改邮件组部分信息
    pub async fn patch(
        &self,
        mailgroup_id: &str,
        request: UpdateMailGroupRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateMailGroupResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP,
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

    /// 修改邮件组全部信息
    pub async fn update(
        &self,
        mailgroup_id: &str,
        request: UpdateMailGroupRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateMailGroupResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP,
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

    /// 查询指定邮件组
    pub async fn get(
        &self,
        mailgroup_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetMailGroupResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量获取邮件组
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListMailGroupsResponse>> {
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
            api_path: Endpoints::MAIL_V1_MAILGROUPS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
