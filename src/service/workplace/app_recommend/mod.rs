#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTraitconfig::Config,
        constants::AccessTokenType,
        endpoints::workplace::*,
        http::Transport,
        query_params::QueryParams,
        req_option::RequestOption,
        standard_response::StandardResponse,
        SDKResult,
};
    service::workplace::models::{AppRecommendRule, FavouriteApp, PageResponse, RecommendedApp}
};
/// 应用推荐服务
pub struct AppRecommendService {
}
    pub config: Config,
impl AppRecommendService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取用户自定义常用的应用
    ///,
/// 获取当前用户自定义设置的常用应用列表。
    ///,
/// # Arguments
    ///,
/// * `request` - 常用应用查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回用户常用应用列表
pub async fn get_favourite_apps(,
        &self,
        request: FavouriteAppsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<FavouriteAppsResponse> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: WORKPLACE_APP_RECOMMEND_FAVOURITE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()};
// 添加查询参数
        if let Some(page_token) = request.page_token {,
api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
if let Some(user_id) = request.user_id {,
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
let api_resp: BaseResponse<FavouriteAppsResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 获取管理员推荐的应用
    ///,
/// 获取管理员设置的推荐应用列表。
    ///,
/// # Arguments
    ///,
/// * `request` - 推荐应用查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回管理员推荐应用列表
pub async fn get_recommended_apps(,
        &self,
        request: RecommendedAppsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<RecommendedAppsResponse> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: WORKPLACE_APP_RECOMMEND_RECOMMEND.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()};
// 添加查询参数
        if let Some(page_token) = request.page_token {,
api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
if let Some(user_id) = request.user_id {,
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
if let Some(department_id) = request.department_id {,
            api_req
.query_params
                .insert(QueryParams::DEPARTMENT_ID, department_id);
let api_resp: BaseResponse<RecommendedAppsResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 获取当前设置的推荐规则列表
    ///,
/// 获取当前系统中配置的应用推荐规则列表。
    ///,
/// # Arguments
    ///,
/// * `request` - 推荐规则查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回推荐规则列表
pub async fn list_recommend_rules(,
        &self,
        request: RecommendRulesListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<RecommendRulesListResponse> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: WORKPLACE_APP_RECOMMEND_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()};
// 添加查询参数
        if let Some(page_token) = request.page_token {,
api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
if let Some(rule_type) = request.rule_type {,
            api_req
.query_params
                .insert(QueryParams::RULE_TYPE, rule_type);
if let Some(status) = request.status {,
            api_req.query_params.insert(QueryParams::STATUS, status);
let api_resp: BaseResponse<RecommendRulesListResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 常用应用查询请求
#[derive(Debug, Clone)]
pub struct FavouriteAppsRequest {
    /// 页码标记
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
}    pub user_id: Option<String>/// 常用应用查询响应
#[derive(Debug, Clone)]
pub struct FavouriteAppsResponse {
    /// 常用应用列表
#[serde(flatten)]
}    pub favourite_apps: PageResponse<FavouriteApp>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 推荐应用查询请求
#[derive(Debug, Clone)]
    /// 页码标记
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 部门ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
}    pub department_id: Option<String>}
pub struct RecommendedAppsRequest {
impl RecommendedAppsRequest {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 推荐应用查询请求Builder
#[derive(Default)]
pub struct RecommendedAppsRequestBuilder {
    inner: RecommendedAppsRequest}
impl RecommendedAppsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 推荐应用查询响应
#[derive(Debug, Clone)]
pub struct RecommendedAppsResponse {
    /// 推荐应用列表
#[serde(flatten)]
}    pub recommended_apps: PageResponse<RecommendedApp>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 推荐规则列表查询请求
#[derive(Debug, Clone)]
    /// 页码标记
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 规则类型筛选
#[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则状态筛选
#[serde(skip_serializing_if = "Option::is_none")]
}    pub status: Option<String>}
pub struct RecommendRulesListRequest {
impl RecommendRulesListRequest {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 推荐规则列表查询请求Builder
#[derive(Default)]
pub struct RecommendRulesListRequestBuilder {
    inner: RecommendRulesListRequest}
impl RecommendRulesListRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 推荐规则列表查询响应
#[derive(Debug, Clone)]
pub struct RecommendRulesListResponse {
    /// 推荐规则列表
#[serde(flatten)]
}    pub recommend_rules: PageResponse<AppRecommendRule>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}
}}}}}}}}}}}}}}}}}}}}}