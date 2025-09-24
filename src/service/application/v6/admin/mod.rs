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
        SDKResult,
    },
    service::application::models::*,
};

/// 应用管理服务
pub struct AdminService {
    config: Config,
}

impl AdminService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取企业安装的应用
    pub async fn list_installed_apps(
        &self,
        user_id_type: Option<UserIdType>,
        lang: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListInstalledAppsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
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
            api_path: crate::core::endpoints::application::APPLICATION_V6_ADMIN_APPS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取用户可用的应用
    pub async fn get_user_available_apps(
        &self,
        user_id: &str,
        user_id_type: Option<UserIdType>,
        lang: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetUserAvailableAppsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
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
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_USER_AVAILABLE_APPS,
                "user_id",
                user_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用通讯录权限范围配置
    pub async fn contacts_range_configuration(
        &self,
        app_id: &str,
        department_id_type: Option<DepartmentIdType>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ContactsRangeConfigurationResponse>> {
        let mut query_params = HashMap::new();
        if let Some(department_id_type) = department_id_type {
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_GET,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新应用通讯录权限范围配置
    pub async fn update_contacts_range_configuration(
        &self,
        app_id: &str,
        request: UpdateContactsRangeConfigurationRequest,
        department_id_type: Option<DepartmentIdType>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(department_id_type) = department_id_type {
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_SET,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用在企业内的可用范围
    pub async fn get_app_availability(
        &self,
        app_id: &str,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAppAvailabilityResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(department_id_type) = department_id_type {
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_APP_VISIBILITY,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询用户或部门是否在应用的可用或禁用名单
    pub async fn check_white_black_list(
        &self,
        app_id: &str,
        request: CheckWhiteBlackListRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CheckWhiteBlackListResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(department_id_type) = department_id_type {
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_APP_CHECK_WHITE_BLACK_LIST,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新应用可用范围
    pub async fn update_app_availability(
        &self,
        app_id: &str,
        request: UpdateAppAvailabilityRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(department_id_type) = department_id_type {
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_APP_VISIBILITY_CREATE,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 启停用应用
    pub async fn enable_disable_app(
        &self,
        app_id: &str,
        request: EnableDisableAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_APP_ENABLE,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询应用管理员列表
    pub async fn list_app_admins(
        &self,
        app_id: &str,
        user_id_type: Option<UserIdType>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListAppAdminsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
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
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_APP_ADMINS,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用管理员管理范围
    pub async fn get_app_admin_permissions(
        &self,
        app_id: &str,
        user_id: &str,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAppAdminPermissionsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(department_id_type) = department_id_type {
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_APP_ADMIN_MANAGEMENT_PERMISSIONS,
                &[("app_id", app_id), ("user_id", user_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 校验应用管理员
    pub async fn verify_app_admin(
        &self,
        app_id: &str,
        user_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<VerifyAppAdminResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::application::APPLICATION_V6_ADMIN_APP_ADMIN_VERIFY,
                &[("app_id", app_id), ("user_id", user_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

// 请求响应模型

#[derive(Debug, Serialize, Deserialize)]
pub struct ListInstalledAppsResponse {
    pub apps: Vec<Application>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for ListInstalledAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserAvailableAppsResponse {
    pub apps: Vec<Application>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for GetUserAvailableAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactsRangeConfigurationResponse {
    pub contacts_range: ContactsRange,
}

impl ApiResponseTrait for ContactsRangeConfigurationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContactsRangeConfigurationRequest {
    pub contacts_range: ContactsRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAppAvailabilityResponse {
    pub availability: AppAvailability,
}

impl ApiResponseTrait for GetAppAvailabilityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckWhiteBlackListRequest {
    pub user_list: Option<Vec<String>>,
    pub department_list: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckWhiteBlackListResponse {
    pub is_in_list: bool,
}

impl ApiResponseTrait for CheckWhiteBlackListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppAvailabilityRequest {
    pub availability: AppAvailability,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnableDisableAppRequest {
    pub enable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAppAdminsResponse {
    pub admins: Vec<AppAdmin>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for ListAppAdminsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAppAdminPermissionsResponse {
    pub permissions: Vec<AdminPermission>,
}

impl ApiResponseTrait for GetAppAdminPermissionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyAppAdminResponse {
    pub is_admin: bool,
}

impl ApiResponseTrait for VerifyAppAdminResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
