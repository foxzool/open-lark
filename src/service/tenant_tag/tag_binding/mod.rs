#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use open_lark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::,
{,
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait,
    config::Config,
        constants::AccessTokenType,
        endpoints::tenant_tag::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    service::tenant_tag::models::{TagBinding, UserIdType}
};
/// 标签绑定服务
pub struct TagBindingService {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 绑定标签到群请求
#[derive(Debug, Clone)]
pub struct CreateTagBindingRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 解绑标签与群请求
#[derive(Debug, Clone)]
pub struct UpdateTagBindingRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl TagBindingService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 查询实体与标签的绑定关系
    pub async fn get(
        &self,
        request: GetTagBindingRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTagBindingResponse>> {,
let mut query_params = HashMap::new();
        query_params.insert("entity_id", request.entity_id);
        query_params.insert("entity_type", request.entity_type);
if let Some(tag_id) = request.tag_id {,
            query_params.insert("tag_id", tag_id);
if let Some(page_size) = request.page_size {,
            query_params.insert("page_size", page_size.to_string());
if let Some(page_token) = request.page_token {,
            query_params.insert("page_token", page_token);
if let Some(user_id_type) = request.user_id_type {,
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: TENANT_TAG_V1_TAG_BINDINGS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 绑定标签到群
    pub async fn create(
        &self,
        request: CreateTagBindingRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateTagBindingResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: TENANT_TAG_V1_TAG_BINDINGS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 解绑标签与群
    pub async fn update(
        &self,
        request: UpdateTagBindingRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateTagBindingResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::PUT,
            api_path: TENANT_TAG_V1_TAG_BINDINGS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}
}}}}}}}}}}}}}