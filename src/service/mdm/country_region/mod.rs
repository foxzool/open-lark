#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::mdm::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    service::mdm::models::{CountryRegion, PageResponse}
};
/// 国家/地区管理服务
pub struct CountryRegionService {
}
    pub config: Config,
impl CountryRegionService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 根据主数据编码批量查询国家/地区
    ///,
/// 根据提供的主数据编码列表批量查询国家/地区信息。
    ///,
/// # Arguments
    ///,
/// * `request` - 批量查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回国家/地区信息列表
pub async fn get(,
        &self,
        request: CountryRegionGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CountryRegionGetResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: MDM_V1_COUNTRY_REGIONS_BATCH_GET.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 分页批量查询国家/地区
    ///,
/// 分页查询所有国家/地区信息，支持条件筛选。
    ///,
/// # Arguments
    ///,
/// * `request` - 分页查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回分页的国家/地区信息列表
pub async fn list(,
        &self,
        request: CountryRegionListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CountryRegionListResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: MDM_V1_COUNTRY_REGIONS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
// 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert("page_size", page_size.to_string());
if let Some(status) = request.status {,
            api_req.query_params.insert("status", status);
if let Some(region_type) = request.region_type {,
            api_req.query_params.insert("region_type", region_type);
if let Some(name) = request.name {,
            api_req.query_params.insert("name", name);
        Transport::request(api_req, &self.config, option).await,
impl Service for CountryRegionService {,
    fn config(&self) -> &Config {,
&self.config,
    fn service_name() -> &'static str {,
        "country_region",
fn service_version() -> &'static str {,
        "v1",
/// 国家/地区批量查询请求
#[derive(Debug, Clone)]
}
pub struct CountryRegionGetRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 国家/地区分页查询请求
#[derive(Debug, Clone)]
}
pub struct CountryRegionListRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}}}}}}}}}}}}}}