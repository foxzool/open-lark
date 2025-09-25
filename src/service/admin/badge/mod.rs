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
        query_params::QueryParams,
        req_option::RequestOption,
        SDKResult,
    },
    impl_full_service,
    service::admin::models::{
        Badge, BadgeCreateRequest, BadgeGetRequest, BadgeImageUploadRequest,
        BadgeImageUploadResult, BadgeListRequest, BadgeUpdateRequest, PageResponse,
    },
};

/// 勋章管理服务
pub struct BadgeService {
    pub config: Config,
}

// Service 抽象接入：Admin BadgeService
impl_full_service!(BadgeService, "admin.badge", "v1");

/// 勋章创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeCreateResponse {
    /// 创建的勋章信息
    #[serde(flatten)]
    pub badge: Badge,
}

impl ApiResponseTrait for BadgeCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 勋章更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeUpdateResponse {
    /// 更新的勋章信息
    #[serde(flatten)]
    pub badge: Badge,
}

impl ApiResponseTrait for BadgeUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 勋章图片上传响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeImageUploadResponse {
    /// 图片上传结果
    #[serde(flatten)]
    pub upload_result: BadgeImageUploadResult,
}

impl ApiResponseTrait for BadgeImageUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 勋章列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<Badge>,
}

impl ApiResponseTrait for BadgeListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 勋章详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGetResponse {
    /// 勋章详细信息
    #[serde(flatten)]
    pub badge: Badge,
}

impl ApiResponseTrait for BadgeGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BadgeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建勋章
    ///
    /// 该接口用于创建企业勋章。
    ///
    /// # 参数
    ///
    /// - `request`: 勋章创建请求参数
    /// - `option`: 可选的请求配置
    pub async fn create_badge(
        &self,
        request: BadgeCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: admin::ADMIN_V1_BADGES_CREATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改勋章信息
    ///
    /// 该接口用于修改指定勋章的信息。
    ///
    /// # 参数
    ///
    /// - `request`: 勋章更新请求参数
    /// - `option`: 可选的请求配置
    pub async fn update_badge(
        &self,
        request: BadgeUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                admin::ADMIN_V1_BADGES_OPERATION,
                "badge_id",
                &request.badge_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&serde_json::json!({
                "name": request.name,
                "description": request.description,
                "detail_description": request.detail_description,
                "show_detail_time": request.show_detail_time,
                "image_key": request.image_key,
                "i18n_name": request.i18n_name,
                "i18n_description": request.i18n_description,
                "i18n_detail_description": request.i18n_detail_description
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 上传勋章图片
    ///
    /// 该接口用于上传勋章图片并获取图片key。
    ///
    /// # 参数
    ///
    /// - `request`: 图片上传请求参数
    /// - `option`: 可选的请求配置
    pub async fn upload_badge_image(
        &self,
        request: BadgeImageUploadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeImageUploadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: admin::ADMIN_V1_BADGES_IMAGE_UPLOAD.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取勋章列表
    ///
    /// 该接口用于获取企业勋章列表。
    ///
    /// # 参数
    ///
    /// - `request`: 勋章列表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_badges(
        &self,
        request: BadgeListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: admin::ADMIN_V1_BADGES_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }
        if let Some(name) = request.name {
            api_req.query_params.insert(QueryParams::NAME, name);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取勋章详情
    ///
    /// 该接口用于获取指定勋章的详细信息。
    ///
    /// # 参数
    ///
    /// - `request`: 勋章查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn get_badge(
        &self,
        request: BadgeGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BadgeGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                admin::ADMIN_V1_BADGES_OPERATION,
                "badge_id",
                &request.badge_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
