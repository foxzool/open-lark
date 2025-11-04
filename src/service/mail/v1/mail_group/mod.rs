#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use open_lark_core::core::api_req::ApiRequest;
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpointshttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::mail::models::{MailGroup, UserIdType}
};
/// 邮件组管理服务
pub struct MailGroupService {
}
    pub config: Config,
/// 创建邮件组请求
#[derive(Debug, Clone)]
pub struct CreateMailGroupRequest {
}
    /// 邮件组名称
    pub name: String,
    /// 邮件组邮箱
    pub email: String,
    /// 描述
#[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否允许外部发送
#[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_send: Option<bool>,
/// 创建邮件组响应
#[derive(Debug, Clone)]
pub struct CreateMailGroupResponse {
}
    /// 创建的邮件组
    pub mailgroup: MailGroup,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 修改邮件组请求
#[derive(Debug, Clone)]
}
pub struct UpdateMailGroupRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 查询指定邮件组响应
#[derive(Debug, Clone)]
}
pub struct GetMailGroupResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 批量获取邮件组响应
#[derive(Debug, Clone)]
}
pub struct ListMailGroupsResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl MailGroupService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 创建邮件组
    pub async fn create(
        &self,
        request: CreateMailGroupRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateMailGroupResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: Endpoints::MAIL_V1_MAILGROUPS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 删除邮件组
    pub async fn delete(
        &self,
        mailgroup_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 修改邮件组部分信息
    pub async fn patch(
        &self,
        mailgroup_id: &str,
        request: UpdateMailGroupRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateMailGroupResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 修改邮件组全部信息
    pub async fn update(
        &self,
        mailgroup_id: &str,
        request: UpdateMailGroupRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateMailGroupResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 查询指定邮件组
    pub async fn get(
        &self,
        mailgroup_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetMailGroupResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 批量获取邮件组
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListMailGroupsResponse>> {,
let mut query_params = HashMap::new();
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
if let Some(page_token) = page_token {,
            query_params.insert("page_token", page_token);
if let Some(user_id_type) = user_id_type {,
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: Endpoints::MAIL_V1_MAILGROUPS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}