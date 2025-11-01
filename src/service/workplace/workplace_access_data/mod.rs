use reqwest::Method;
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
        trait_system::Service,
        SDKResult,
};
service::workplace::models::{,
        CustomWorkplaceAccessData, CustomWorkplaceWidgetAccessData, PageResponse,
        WorkplaceAccessData}
};
/// 工作台访问数据服务
pub struct WorkplaceAccessDataService {
}
    pub config: Config,
impl WorkplaceAccessDataService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取工作台访问数据
    ///,
/// 获取工作台的访问数据统计信息，支持按时间范围、用户、部门等维度查询。
    ///,
/// # Arguments
    ///,
/// * `request` - 访问数据查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回工作台访问数据列表
pub async fn search(,
        &self,
        request: AccessDataSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<AccessDataSearchResponse> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: WORKPLACE_ACCESS_DATA_SEARCH.to_string(),
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
if let Some(start_time) = request.start_time {,
            api_req
.query_params
                .insert(QueryParams::START_TIME, start_time.to_string());
if let Some(end_time) = request.end_time {,
            api_req
.query_params
                .insert(QueryParams::END_TIME, end_time.to_string());
if let Some(user_id) = request.user_id {,
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
if let Some(department_id) = request.department_id {,
            api_req
.query_params
                .insert(QueryParams::DEPARTMENT_ID, department_id);
if let Some(access_type) = request.access_type {,
            api_req
.query_params
                .insert(QueryParams::ACCESS_TYPE, access_type);
let api_resp: BaseResponse<AccessDataSearchResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 获取定制工作台访问数据
    ///,
/// 获取定制工作台的访问数据统计信息。
    ///,
/// # Arguments
    ///,
/// * `request` - 定制工作台访问数据查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回定制工作台访问数据列表
pub async fn search_custom(,
        &self,
        request: CustomAccessDataSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CustomAccessDataSearchResponse> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: WORKPLACE_CUSTOM_ACCESS_DATA_SEARCH.to_string(),
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
if let Some(start_time) = request.start_time {,
            api_req
.query_params
                .insert(QueryParams::START_TIME, start_time.to_string());
if let Some(end_time) = request.end_time {,
            api_req
.query_params
                .insert(QueryParams::END_TIME, end_time.to_string());
if let Some(user_id) = request.user_id {,
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
if let Some(custom_workplace_id) = request.custom_workplace_id {,
            api_req
.query_params
                .insert(QueryParams::CUSTOM_WORKPLACE_ID, custom_workplace_id);
let api_resp: BaseResponse<CustomAccessDataSearchResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 获取定制工作台小组件访问数据
    ///,
/// 获取定制工作台小组件的访问数据统计信息。
    ///,
/// # Arguments
    ///,
/// * `request` - 定制工作台小组件访问数据查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回定制工作台小组件访问数据列表
pub async fn search_custom_widget(,
        &self,
        request: CustomWidgetAccessDataSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CustomWidgetAccessDataSearchResponse> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: WORKPLACE_WIDGET_ACCESS_DATA_SEARCH.to_string(),
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
if let Some(start_time) = request.start_time {,
            api_req
.query_params
                .insert(QueryParams::START_TIME, start_time.to_string());
if let Some(end_time) = request.end_time {,
            api_req
.query_params
                .insert(QueryParams::END_TIME, end_time.to_string());
if let Some(user_id) = request.user_id {,
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
if let Some(custom_workplace_id) = request.custom_workplace_id {,
            api_req
.query_params
                .insert(QueryParams::CUSTOM_WORKPLACE_ID, custom_workplace_id);
if let Some(widget_id) = request.widget_id {,
            api_req
.query_params
                .insert(QueryParams::WIDGET_ID, widget_id);
let api_resp: BaseResponse<CustomWidgetAccessDataSearchResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    impl Service for WorkplaceAccessDataService {,
    fn config(&self) -> &Config {,
&self.configfn service_name() -> &'static str {,
        "workplace_access_data"fn service_version() -> &'static str {,
        "v1"/// 工作台访问数据查询请求
#[derive(Debug, Clone)]
    /// 页码标记
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 开始时间戳
#[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳
#[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 用户ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 部门ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 访问类型筛选
#[serde(skip_serializing_if = "Option::is_none")]
}    pub access_type: Option<String>}
pub struct AccessDataSearchRequest {
impl AccessDataSearchRequest {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 工作台访问数据查询请求Builder
#[derive(Default)]
pub struct AccessDataSearchRequestBuilder {
    inner: AccessDataSearchRequest}
impl AccessDataSearchRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 工作台访问数据查询响应
#[derive(Debug, Clone)]
pub struct AccessDataSearchResponse {
    /// 工作台访问数据列表
#[serde(flatten)]
}    pub access_data: PageResponse<WorkplaceAccessData>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 定制工作台访问数据查询请求
#[derive(Debug, Clone)]
    /// 页码标记
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 开始时间戳
#[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳
#[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 用户ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 定制工作台ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
}    pub custom_workplace_id: Option<String>/// 定制工作台访问数据查询响应
pub struct CustomAccessDataSearchRequest {
#[derive(Debug, Clone)]
pub struct CustomAccessDataSearchResponse {
    /// 定制工作台访问数据列表
#[serde(flatten)]
}    pub access_data: PageResponse<CustomWorkplaceAccessData>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 定制工作台小组件访问数据查询请求
#[derive(Debug, Clone)]
    /// 页码标记
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 开始时间戳
#[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间戳
#[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 用户ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 定制工作台ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
    pub custom_workplace_id: Option<String>,
    /// 小组件ID筛选
#[serde(skip_serializing_if = "Option::is_none")]
}    pub widget_id: Option<String>}
pub struct CustomWidgetAccessDataSearchRequest {
impl CustomWidgetAccessDataSearchRequest {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 定制工作台小组件访问数据查询请求Builder
#[derive(Default)]
pub struct CustomWidgetAccessDataSearchRequestBuilder {
    inner: CustomWidgetAccessDataSearchRequest}
impl CustomWidgetAccessDataSearchRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 定制工作台小组件访问数据查询响应
#[derive(Debug, Clone)]
pub struct CustomWidgetAccessDataSearchResponse {
    /// 定制工作台小组件访问数据列表
#[serde(flatten)]
}    pub access_data: PageResponse<CustomWorkplaceWidgetAccessData>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}