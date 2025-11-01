use crate::core::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use std::collections::HashMap;
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpointshttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::task::models::{Section, Task, UserIdType}
};
/// 自定义分组服务
#[derive(Debug)]
pub struct SectionService {
}
    pub config: Config,
/// 创建自定义分组请求
#[derive(Debug, Clone)]
pub struct CreateSectionRequest {
}
    /// 分组名称
    pub name: String,
    /// 分组所属的清单GUID
    pub tasklist_guid: String,
/// 创建自定义分组响应
#[derive(Debug, Clone)]
pub struct CreateSectionResponse {
}
    /// 创建的分组
    pub section: Section,
impl ApiResponseTrait for.* {
}
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 更新自定义分组请求
#[derive(Debug, Clone)]
pub struct UpdateSectionRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 获取自定义分组响应
#[derive(Debug, Clone)]
}
pub struct GetSectionResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 自定义分组列表响应
#[derive(Debug, Clone)]
}
pub struct ListSectionsResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 自定义分组任务列表响应
#[derive(Debug, Clone)]
}
pub struct SectionTasksResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl SectionService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 创建自定义分组
    pub async fn create(
        &self,
        request: CreateSectionRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateSectionResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: Endpoints::TASK_V2_SECTIONS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取自定义分组详情
    pub async fn get(
        &self,
        section_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetSectionResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_SECTION_GET,
                "section_guid",
                section_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 更新自定义分组
    pub async fn patch(
        &self,
        section_guid: &str,
        request: UpdateSectionRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateSectionResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_SECTION_GET,
                "section_guid",
                section_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 删除自定义分组
    pub async fn delete(
        &self,
        section_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_SECTION_GET,
                "section_guid",
                section_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取自定义分组列表
    pub async fn list(
        &self,
        tasklist_guid: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListSectionsResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        query_params.insert("tasklist_guid", tasklist_guid.to_string());
if let Some(page_size) = page_size {,
            query_params.insert("page_size", page_size.to_string());
if let Some(page_token) = page_token {,
            query_params.insert("page_token", page_token.to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: Endpoints::TASK_V2_SECTIONS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取自定义分组任务列表
    #[allow(clippy::too_many_arguments)]
pub async fn tasks(,
        &self,
        section_guid: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        completed: Option<bool>,
        created_from: Option<&str>,
        created_to: Option<&str>,
        updated_from: Option<&str>,
        updated_to: Option<&str>,
        due_from: Option<&str>,
        due_to: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SectionTasksResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
if let Some(page_size) = page_size {,
            query_params.insert("page_size", page_size.to_string());
if let Some(page_token) = page_token {,
            query_params.insert("page_token", page_token.to_string());
if let Some(completed) = completed {,
            query_params.insert("completed", completed.to_string());
if let Some(created_from) = created_from {,
            query_params.insert("created_from", created_from.to_string());
if let Some(created_to) = created_to {,
            query_params.insert("created_to", created_to.to_string());
if let Some(updated_from) = updated_from {,
            query_params.insert("updated_from", updated_from.to_string());
if let Some(updated_to) = updated_to {,
            query_params.insert("updated_to", updated_to.to_string());
if let Some(due_from) = due_from {,
            query_params.insert("due_from", due_from.to_string());
if let Some(due_to) = due_to {,
            query_params.insert("due_to", due_to.to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_SECTION_TASKS,
                "section_guid",
                section_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}