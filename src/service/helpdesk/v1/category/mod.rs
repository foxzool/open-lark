use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use std::collections::HashMap;
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{helpdesk::*, EndpointBuilderhttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::helpdesk::models::{Category, UserIdType}
};
/// 知识库分类服务
pub struct CategoryService {
}
    pub config: Config,
/// 创建知识库分类请求
#[derive(Debug, Clone)]
pub struct CreateCategoryRequest {
}
    /// 分类名称
    pub name: String,
    /// 描述
#[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 父分类ID
#[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
/// 创建知识库分类响应
#[derive(Debug, Clone)]
pub struct CreateCategoryResponse {
}
    /// 创建的分类
    pub category: Category,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 获取知识库分类响应
#[derive(Debug, Clone)]
}
pub struct GetCategoryResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 更新知识库分类请求
#[derive(Debug, Clone)]
}
pub struct UpdateCategoryRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 获取全部知识库分类响应
#[derive(Debug, Clone)]
}
pub struct ListCategoriesResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl CategoryService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 创建知识库分类
    pub async fn create(
        &self,
        request: CreateCategoryRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateCategoryResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: HELPDESK_V1_CATEGORIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取知识库分类
    pub async fn get(
        &self,
        category_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetCategoryResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_CATEGORY_GET,
                "category_id",
                category_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 更新知识库分类详情
    pub async fn patch(
        &self,
        category_id: &str,
        request: UpdateCategoryRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateCategoryResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_CATEGORY_GET,
                "category_id",
                category_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 删除知识库分类详情
    pub async fn delete(
        &self,
        category_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_CATEGORY_GET,
                "category_id",
                category_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取全部知识库分类
    pub async fn list(
        &self,
        user_id_type: Option<UserIdType>,
        parent_id: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListCategoriesResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
if let Some(parent_id) = parent_id {,
            query_params.insert("parent_id", parent_id.to_string());
if let Some(page_token) = page_token {,
            query_params.insert("page_token", page_token.to_string());
if let Some(page_size) = page_size {,
            query_params.insert("page_size", page_size.to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: HELPDESK_V1_CATEGORIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}
}}}}}}}}}}}}}}}}}}}}}}}}}}}