#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::performance::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    service::performance::models::{PerformanceResult, ReviewDetail}
};
/// 评估数据管理服务
pub struct ReviewDataService {
}
    pub config: Config,
impl ReviewDataService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取绩效结果
    ///,
/// 查询被评估人的绩效结果信息。
    ///,
/// # Arguments
    ///,
/// * `request` - 绩效结果查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回绩效结果列表
pub async fn query_results(,
        &self,
        request: ResultQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ResultQueryResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: PERFORMANCE_V1_REVIEW_DATA_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取绩效详情数据
    ///,
/// 查询绩效评估的详细数据，包括各评估项的具体回答。
    ///,
/// # Arguments
    ///,
/// * `request` - 绩效详情查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回绩效详情数据列表
pub async fn query_details(,
        &self,
        request: DetailQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DetailQueryResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: PERFORMANCE_V1_REVIEW_DATA_DETAILS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 绩效结果查询请求
#[derive(Debug, Clone)]
pub struct ResultQueryRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 绩效详情查询请求
#[derive(Debug, Clone)]
}
pub struct DetailQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    impl Service for ReviewDataService {,
    fn config(&self) -> &Config {,
&self.config,
    fn service_name() -> &'static str {,
        "review_data",
fn service_version() -> &'static str {,
        "v2",
}
}}}}}}}}}}}}}}