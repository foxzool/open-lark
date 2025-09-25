use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{admin, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_full_service,
    service::admin::models::{
        BadgeGrant, BadgeGrantCreateRequest, BadgeGrantDeleteRequest, BadgeGrantGetRequest,
        BadgeGrantListRequest, BadgeGrantUpdateRequest, PageResponse,
    },
};

/// 勋章授予名单管理服务
pub struct BadgeGrantService {
    pub config: Config,
}

// Service 抽象接入：Admin BadgeGrantService
impl_full_service!(BadgeGrantService, "admin.badge_grant", "v1");

/// 勋章授予名单创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantCreateResponse {
    /// 创建的授予名单信息
    #[serde(flatten)]
    pub grant: BadgeGrant,
}

impl ApiResponseTrait for BadgeGrantCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 勋章授予名单删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantDeleteResponse {
    /// 删除结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 删除时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<String>,
}

impl ApiResponseTrait for BadgeGrantDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 勋章授予名单更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantUpdateResponse {
    /// 更新的授予名单信息
    #[serde(flatten)]
    pub grant: BadgeGrant,
}

impl ApiResponseTrait for BadgeGrantUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 勋章授予名单列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<BadgeGrant>,
}

impl ApiResponseTrait for BadgeGrantListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 勋章授予名单详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantGetResponse {
    /// 授予名单详细信息
    #[serde(flatten)]
    pub grant: BadgeGrant,
}

impl ApiResponseTrait for BadgeGrantGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BadgeGrantService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建授予名单
    ///
    /// 该接口用于创建勋章授予名单。
    ///
    /// # 参数
    ///
    /// - `request`: 授予名单创建请求参数
    /// - `option`: 可选的请求配置
    pub async fn create_badge_grant(
        &self,
        request: BadgeGrantCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeGrantCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: admin::ADMIN_V1_BADGE_GRANTS_CREATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除授予名单
    ///
    /// 该接口用于删除指定的勋章授予名单。
    ///
    /// # 参数
    ///
    /// - `request`: 授予名单删除请求参数
    /// - `option`: 可选的请求配置
    pub async fn delete_badge_grant(
        &self,
        request: BadgeGrantDeleteRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeGrantDeleteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                admin::ADMIN_V1_BADGE_GRANTS_OPERATION,
                "grant_id",
                &request.grant_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改授予名单
    ///
    /// 该接口用于修改指定勋章授予名单的信息。
    ///
    /// # 参数
    ///
    /// - `request`: 授予名单更新请求参数
    /// - `option`: 可选的请求配置
    pub async fn update_badge_grant(
        &self,
        request: BadgeGrantUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeGrantUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                admin::ADMIN_V1_BADGE_GRANTS_OPERATION,
                "grant_id",
                &request.grant_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&serde_json::json!({
                "name": request.name,
                "description": request.description,
                "user_list": request.user_list,
                "effective_time": request.effective_time,
                "expiry_time": request.expiry_time,
                "time_zone": request.time_zone
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取授予名单列表
    ///
    /// 该接口用于获取勋章授予名单列表。
    ///
    /// # 参数
    ///
    /// - `request`: 授予名单列表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_badge_grants(
        &self,
        request: BadgeGrantListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeGrantListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: admin::ADMIN_V1_BADGE_GRANTS_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(badge_id) = request.badge_id {
            api_req.query_params.insert("badge_id", badge_id);
        }
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(name) = request.name {
            api_req.query_params.insert("name", name);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取授予名单详情
    ///
    /// 该接口用于获取指定勋章授予名单的详细信息。
    ///
    /// # 参数
    ///
    /// - `request`: 授予名单查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn get_badge_grant(
        &self,
        request: BadgeGrantGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeGrantGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                admin::ADMIN_V1_BADGE_GRANTS_OPERATION,
                "grant_id",
                &request.grant_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
