use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::application::models::{UserIdType, *},
};

/// 应用信息管理服务
pub struct ApplicationService {
    config: Config,
}

impl ApplicationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 转移应用所有者
    pub async fn transfer_owner(
        &self,
        app_id: &str,
        request: TransferOwnerRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_TRANSFER_OWNER,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新应用协作者
    pub async fn update_collaborators(
        &self,
        app_id: &str,
        request: UpdateCollaboratorsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_COLLABORATORS,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用协作者列表
    pub async fn get_collaborators(
        &self,
        app_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetCollaboratorsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_COLLABORATORS,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用信息
    pub async fn get(
        &self,
        app_id: &str,
        lang: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetApplicationResponse>> {
        let mut query_params = HashMap::new();
        if let Some(lang) = lang {
            query_params.insert("lang", lang);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_GET,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用版本信息
    pub async fn get_version(
        &self,
        app_id: &str,
        version_id: &str,
        lang: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAppVersionResponse>> {
        let mut query_params = HashMap::new();
        if let Some(lang) = lang {
            query_params.insert("lang", lang);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::application::APPLICATION_V6_APP_VERSION_GET,
                &[("app_id", app_id), ("version_id", version_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用版本列表
    pub async fn list_versions(
        &self,
        app_id: &str,
        lang: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListAppVersionsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(lang) = lang {
            query_params.insert("lang", lang);
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_VERSIONS,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用版本中开发者申请的通讯录权限范围
    pub async fn contacts_range_suggest(
        &self,
        app_id: &str,
        version_id: &str,
        lang: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ContactsRangeSuggestResponse>> {
        let mut query_params = HashMap::new();
        if let Some(lang) = lang {
            query_params.insert("lang", lang);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::application::APPLICATION_V6_APP_VERSION_CONTACTS_RANGE_SUGGEST,
                &[("app_id", app_id), ("version_id", version_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查看待审核的应用列表
    pub async fn underaudit_list(
        &self,
        lang: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UnderauditListResponse>> {
        let mut query_params = HashMap::new();
        if let Some(lang) = lang {
            query_params.insert("lang", lang);
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: crate::core::endpoints::application::APPLICATION_V6_APPS_UNDERAUDITLIST
                .to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新应用审核状态
    pub async fn update_audit_status(
        &self,
        app_id: &str,
        request: UpdateAuditStatusRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_AUDIT,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新应用分组信息
    pub async fn update_group(
        &self,
        app_id: &str,
        request: UpdateGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_GROUP,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for ApplicationService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "application"
    }

    fn service_version() -> &'static str {
        "v6"
    }
}

// 请求响应模型

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferOwnerRequest {
    pub owner_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCollaboratorsRequest {
    pub collaborators: Vec<AppCollaborator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCollaboratorsResponse {
    pub collaborators: Vec<AppCollaborator>,
}

impl ApiResponseTrait for GetCollaboratorsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetApplicationResponse {
    pub application: Application,
}

impl ApiResponseTrait for GetApplicationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAppVersionResponse {
    pub version: AppVersion,
}

impl ApiResponseTrait for GetAppVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAppVersionsResponse {
    pub versions: Vec<AppVersion>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for ListAppVersionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactsRangeSuggestResponse {
    pub contacts_range: ContactsRange,
}

impl ApiResponseTrait for ContactsRangeSuggestResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnderauditListResponse {
    pub applications: Vec<Application>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for UnderauditListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAuditStatusRequest {
    pub status: AuditResult,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    pub group_id: String,
}
