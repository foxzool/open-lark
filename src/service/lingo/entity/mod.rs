use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{
            EndpointBuilder, LINGO_ENTITY_CREATE, LINGO_ENTITY_GET, LINGO_ENTITY_HIGHLIGHT,
            LINGO_ENTITY_MATCH, LINGO_ENTITY_SEARCH, LINGO_ENTITY_UPDATE,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    service::lingo::models::{
        Entity, EntityMatchResult, EntitySearchResult, HighlightResult, OuterInfo, PageResponse,
        RelatedMeta,
};
/// 词条管理服务
pub struct EntityService {
}
    pub config: Config,
impl EntityService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建免审词条
    ///,
/// 通过此接口创建的词条，无需经过词典管理员审核，直接写入词库。
    ///,
/// # Arguments
    ///,
/// * `request` - 词条创建请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回创建的词条信息
pub async fn create_entity(,
        &self,
        request: EntityCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityCreateResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: LINGO_ENTITY_CREATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 更新免审词条
    ///,
/// 通过此接口更新已有词条，无需经过词典管理员审核，直接写入词库。
    ///,
/// # Arguments
    ///,
/// * `entity_id` - 词条ID
    /// * `request` - 词条更新请求
/// * `option` - 请求选项，可选
    ///,
/// # Returns
    ///,
/// 返回更新后的词条信息
    pub async fn update_entity(
        &self,
        entity_id: &str,
        request: EntityUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityUpdateResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(LINGO_ENTITY_UPDATE, "{entity_id}", entity_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 删除免审词条
    ///,
/// 通过词条ID删除已有词条。
    ///,
/// # Arguments
    ///,
/// * `entity_id` - 词条ID
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回删除结果
pub async fn delete_entity(,
        &self,
        entity_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityDeleteResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(LINGO_ENTITY_UPDATE, "{entity_id}", entity_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取词条详情
    ///,
/// 通过词条ID获取词条的详细信息。
    ///,
/// # Arguments
    ///,
/// * `entity_id` - 词条ID
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回词条详细信息
pub async fn get_entity(,
        &self,
        entity_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityGetResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(LINGO_ENTITY_GET, "{entity_id}", entity_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取词条列表
    ///,
/// 分页获取词条列表，支持多种筛选条件。
    ///,
/// # Arguments
    ///,
/// * `request` - 词条列表查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回词条列表
pub async fn list_entities(,
        &self,
        request: EntityListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityListResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: LINGO_ENTITY_CREATE.to_string(),
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
if let Some(repo_id) = request.repo_id {,
            api_req.query_params.insert("repo_id", repo_id);
if let Some(classification_id) = request.classification_id {,
            api_req
.query_params
                .insert("classification_id", classification_id);
if let Some(creator) = request.creator {,
            api_req.query_params.insert("creator", creator);
        Transport::request(api_req, &self.config, option).await,
/// 精准搜索词条
    ///,
/// 传入关键词，与词条的名称和别名精准匹配。
    ///,
/// # Arguments
    ///,
/// * `request` - 词条精准搜索请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回匹配的词条列表
pub async fn match_entities(,
        &self,
        request: EntityMatchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityMatchResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: LINGO_ENTITY_MATCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 模糊搜索词条
    ///,
/// 传入关键词，对词条的名称、别名和释义等信息进行模糊匹配。
    ///,
/// # Arguments
    ///,
/// * `request` - 词条模糊搜索请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回搜索结果列表
pub async fn search_entities(,
        &self,
        request: EntitySearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntitySearchResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: LINGO_ENTITY_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 词条高亮
    ///,
/// 传入一段文本，返回文本中匹配到的词条位置信息，可用于高亮显示。
    ///,
/// # Arguments
    ///,
/// * `request` - 词条高亮请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回高亮信息
pub async fn highlight_entities(,
        &self,
        request: EntityHighlightRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityHighlightResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: LINGO_ENTITY_HIGHLIGHT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 词条创建请求
#[derive(Debug, Clone)]
pub struct EntityCreateRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 词条更新请求
#[derive(Debug, Clone)]
}
pub struct EntityUpdateRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 词条删除响应
#[derive(Debug, Clone)]
}
pub struct EntityDeleteResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 词条详情查询响应
#[derive(Debug, Clone)]
}
pub struct EntityGetResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 词条列表查询请求
#[derive(Debug, Clone)]
}
pub struct EntityListRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 词条精准搜索请求
#[derive(Debug, Clone)]
}
pub struct EntityMatchRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 词条模糊搜索请求
#[derive(Debug, Clone)]
}
pub struct EntitySearchRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 词条高亮请求
#[derive(Debug, Clone)]
}
pub struct EntityHighlightRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}