#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use reqwest::Method;
use crate::core::SDKResult;
use serde::{Deserialize, Serialize};
use crate::core::SDKResult;
use open_lark_core::core::api_req::ApiRequest;
use crate::core::SDKResult;
use std::collections::HashMap;
use crate::core::SDKResult;
use crate::{
use crate::core::SDKResult;
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{approval::*, EndpointBuilderhttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::approval::models::{DepartmentIdType, UserIdType}
};
/// 三方审批实例服务
pub struct ExternalInstanceService {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 校验三方审批实例请求
#[derive(Debug, Clone)]
}
pub struct CheckExternalInstanceRequest {

impl ExternalInstanceService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 同步三方审批实例
    pub async fn create(
        &self,
        request: CreateExternalInstanceRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateExternalInstanceResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
if let Some(department_id_type) = department_id_type {,
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: APPROVAL_V4_EXTERNAL_INSTANCES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 校验三方审批实例
    pub async fn check(
        &self,
        instance_code: &str,
        request: CheckExternalInstanceRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_EXTERNAL_INSTANCE_CHECK,
                "instance_code",
                instance_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}
}}}}}}}}}}}