use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::hire::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::hire::models::{Location, LocationQueryRequest, PageResponse};

/// 地址服务
pub struct LocationService {
    pub config: Config,
}

/// 地址列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationListResponse {
    /// 地址列表
    #[serde(flatten)]
    pub locations: PageResponse<Location>,
}

impl ApiResponseTrait for LocationListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl LocationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询地点列表
    ///
    /// 该接口用于查询系统中的地点列表信息，支持按地址类型、
    /// 父级地址等条件筛选，用于职位发布时选择工作地点。
    ///
    /// # 参数
    ///
    /// - `request`: 地址查询请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的地址列表，包括地址基本信息和层级关系
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::crate::hire::models::LocationQueryRequest;
    ///
    /// let request = LocationQueryRequest {
    ///     location_type: Some("city".to_string()),
    ///     parent_id: Some("province_123".to_string()),
    ///     page_size: Some(50),
    ///     page_token: None,
    /// };
    ///
    /// let response = client.hire.recruitment_config.location.query_locations(request, None).await?;
    /// ```
    pub async fn query_locations(
        &self,
        request: LocationQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<LocationListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_LOCATIONS_QUERY.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(location_type) = request.location_type {
            api_req.query_params.insert("location_type", location_type);
        }

        if let Some(parent_id) = request.parent_id {
            api_req.query_params.insert("parent_id", parent_id);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取地址列表
    ///
    /// 该接口用于获取系统支持的地址列表，包括国家、
    /// 省份、城市等各级地址信息。
    ///
    /// # 参数
    ///
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.hire.recruitment_config.location.list_locations(None).await?;
    /// ```
    pub async fn list_locations(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<LocationListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_LOCATIONS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
}
