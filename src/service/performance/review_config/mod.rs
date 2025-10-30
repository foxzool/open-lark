use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::performance::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    service::performance::models::{
        Activity, AdditionalInformation, Metric, MetricField, MetricTag, MetricTemplate,
        PageResponse, ReviewItem, ReviewTemplate, Reviewee, Semester, TagQuestionConfig, UserGroup,
};
/// 后台配置管理服务
pub struct ReviewConfigService {
}
    pub config: Config,
impl ReviewConfigService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取周期列表
    ///,
/// 分页获取绩效周期列表。
    ///,
/// # Arguments
    ///,
/// * `request` - 周期列表查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回周期列表
pub async fn list_semesters(,
        &self,
        request: SemesterListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SemesterListResponse>> {,
let mut api_req = ApiRequest {,
            http_method: Method::GET,
            api_path: PERFORMANCE_SEMESTER_LIST.to_string(),
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
        Transport::request(api_req, &self.config, option).await,
/// 获取项目列表
    ///,
/// 查询指定周期下的绩效项目列表。
    ///,
/// # Arguments
    ///,
/// * `request` - 项目查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回项目列表
pub async fn query_activities(,
        &self,
        request: ActivityQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ActivityQueryResponse>> {,
let mut api_req = ApiRequest {,
            http_method: Method::GET,
            api_path: PERFORMANCE_ACTIVITIES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
// 添加查询参数
        if let Some(semester_id) = request.semester_id {
            api_req.query_params.insert("semester_id", semester_id);
if let Some(page_token) = request.page_token {,
            api_req.query_params.insert("page_token", page_token);
if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert("page_size", page_size.to_string());
        Transport::request(api_req, &self.config, option).await,
/// 批量查询补充信息
    ///,
/// 查询指定项目下的补充信息。
    ///,
/// # Arguments
    ///,
/// * `request` - 补充信息查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回补充信息列表
pub async fn query_additional_information(,
        &self,
        request: AdditionalInfoQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AdditionalInfoQueryResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_ADDITIONAL_INFO_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 批量导入补充信息
    ///,
/// 批量导入或更新补充信息。
    ///,
/// # Arguments
    ///,
/// * `request` - 补充信息导入请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回导入结果
pub async fn import_additional_information(,
        &self,
        request: AdditionalInfoImportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AdditionalInfoImportResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_ADDITIONAL_INFO_IMPORT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 批量删除补充信息
    ///,
/// 批量删除指定的补充信息。
    ///,
/// # Arguments
    ///,
/// * `request` - 补充信息删除请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回删除结果
pub async fn delete_additional_information(,
        &self,
        request: AdditionalInfoDeleteRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AdditionalInfoDeleteResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_ADDITIONAL_INFO_DELETE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 更新人员组成员
    ///,
/// 更新指定人员组的成员列表。
    ///,
/// # Arguments
    ///,
/// * `request` - 人员组成员更新请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回更新结果
pub async fn write_user_group_members(,
        &self,
        request: UserGroupWriteRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserGroupWriteResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_USER_GROUP_WRITE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取被评估人信息
    ///,
/// 查询指定项目下的被评估人信息。
    ///,
/// # Arguments
    ///,
/// * `request` - 被评估人查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回被评估人信息列表
pub async fn query_reviewees(,
        &self,
        request: RevieweeQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RevieweeQueryResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_REVIEWEES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取评估模板配置
    ///,
/// 查询指定项目的评估模板配置。
    ///,
/// # Arguments
    ///,
/// * `request` - 评估模板查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回评估模板配置
pub async fn query_review_templates(,
        &self,
        request: ReviewTemplateQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ReviewTemplateQueryResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_REVIEW_TEMPLATES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取评估项列表
    ///,
/// 查询指定评估模板的评估项列表。
    ///,
/// # Arguments
    ///,
/// * `request` - 评估项查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回评估项列表
pub async fn query_review_items(,
        &self,
        request: ReviewItemQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ReviewItemQueryResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_REVIEW_ITEMS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取标签填写题配置
    ///,
/// 查询标签填写题的详细配置。
    ///,
/// # Arguments
    ///,
/// * `request` - 标签题配置查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回标签题配置
pub async fn query_tag_question_configs(,
        &self,
        request: TagQuestionConfigQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<TagQuestionConfigQueryResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_TAG_QUESTIONS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取指标列表
    ///,
/// 查询可用的指标列表。
    ///,
/// # Arguments
    ///,
/// * `request` - 指标查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回指标列表
pub async fn query_metrics(,
        &self,
        request: MetricQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricQueryResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_METRICS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取指标模板列表
    ///,
/// 查询指标模板配置。
    ///,
/// # Arguments
    ///,
/// * `request` - 指标模板查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回指标模板列表
pub async fn query_metric_templates(,
        &self,
        request: MetricTemplateQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricTemplateQueryResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_METRIC_TEMPLATES_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取指标字段列表
    ///,
/// 查询指标的字段配置。
    ///,
/// # Arguments
    ///,
/// * `request` - 指标字段查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回指标字段列表
pub async fn query_metric_fields(,
        &self,
        request: MetricFieldQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricFieldQueryResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_METRIC_FIELDS_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取指标标签列表
    ///,
/// 查询可用的指标标签。
    ///,
/// # Arguments
    ///,
/// * `request` - 指标标签查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回指标标签列表
pub async fn list_metric_tags(,
        &self,
        request: MetricTagListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricTagListResponse>> {,
let mut api_req = ApiRequest {,
            http_method: Method::GET,
            api_path: PERFORMANCE_METRIC_TAGS.to_string(),
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
        Transport::request(api_req, &self.config, option).await,
impl Service for ReviewConfigService {,
    fn config(&self) -> &Config {,
&self.config,
    fn service_name() -> &'static str {,
        "review_config",
fn service_version() -> &'static str {,
        "v1",
/// 周期列表查询请求
#[derive(Debug, Clone)]
}
pub struct SemesterListRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 项目查询请求
#[derive(Debug, Clone)]
}
pub struct ActivityQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 补充信息查询请求
#[derive(Debug, Clone)]
}
pub struct AdditionalInfoQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 补充信息导入请求
#[derive(Debug, Clone)]
}
pub struct AdditionalInfoImportRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 补充信息删除请求
#[derive(Debug, Clone)]
}
pub struct AdditionalInfoDeleteRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 人员组成员更新请求
#[derive(Debug, Clone)]
}
pub struct UserGroupWriteRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 被评估人查询请求
#[derive(Debug, Clone)]
}
pub struct RevieweeQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 评估模板查询请求
#[derive(Debug, Clone)]
}
pub struct ReviewTemplateQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 评估项查询请求
#[derive(Debug, Clone)]
}
pub struct ReviewItemQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 标签题配置查询请求
#[derive(Debug, Clone)]
}
pub struct TagQuestionConfigQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 指标查询请求
#[derive(Debug, Clone)]
}
pub struct MetricQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 指标模板查询请求
#[derive(Debug, Clone)]
}
pub struct MetricTemplateQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 指标字段查询请求
#[derive(Debug, Clone)]
}
pub struct MetricFieldQueryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 指标标签列表查询请求
#[derive(Debug, Clone)]
}
pub struct MetricTagListRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}