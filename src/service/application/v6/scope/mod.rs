#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{
    core::{,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    service::application::models::*,
};
/// 应用权限管理服务
pub struct ScopeService {
}

impl ScopeService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 向管理员申请授权
    pub async fn apply(
        &self,
        app_id: &str,
        request: ApplyScopeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_SCOPE_APPLY,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 查询租户授权状态
    pub async fn list(
        &self,
        app_id: &str,
        lang: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListScopeResponse>> {,
let mut query_params = HashMap::new();
        if let Some(lang) = lang {
            query_params.insert("lang", lang);
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_SCOPE_GET,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
// 请求响应模型
#[derive(Debug, Clone)]
pub struct ApplyScopeRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}}}