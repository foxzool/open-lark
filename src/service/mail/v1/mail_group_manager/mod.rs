#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use reqwest::Method;
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
    service::mail::models::{MailGroupMember, UserIdType}
};
/// 邮件组管理员服务
pub struct MailGroupManagerService {
}
    pub config: Config,
/// 批量创建邮件组管理员请求
#[derive(Debug, Clone)]
pub struct BatchCreateManagersRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 批量删除邮件组管理员请求
#[derive(Debug, Clone)]
}
pub struct BatchDeleteManagersRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl MailGroupManagerService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 批量创建邮件组管理员
    pub async fn batch_create(
        &self,
        mailgroup_id: &str,
        request: BatchCreateManagersRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchCreateManagersResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP_MANAGERS_BATCH_CREATE,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 批量删除邮件组管理员
    pub async fn batch_delete(
        &self,
        mailgroup_id: &str,
        request: BatchDeleteManagersRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP_MANAGERS_BATCH_DELETE,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 批量获取邮件组管理员
    pub async fn list(
        &self,
        mailgroup_id: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListManagersResponse>> {,
let mut query_params = HashMap::new();
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
if let Some(page_token) = page_token {,
            query_params.insert("page_token", page_token);
if let Some(user_id_type) = user_id_type {,
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_MAILGROUP_MANAGERS,
                "mailgroup_id",
                mailgroup_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}
}}}}}}}}}}}}}}}}