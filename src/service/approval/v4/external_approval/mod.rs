#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use reqwest::Method;
use SDKResult;
use serde::{Deserialize, Serialize};
use SDKResult;
use openlark_core::api_req::ApiRequest;
use SDKResult;
use std::collections::HashMap;
use SDKResult;
use crate::{
use SDKResult;
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{approval::*, EndpointBuilderhttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::approval::models::{DepartmentIdType, UserIdType}
};
/// 三方审批定义服务
pub struct ExternalApprovalService {
}
    pub config: Config,
/// 创建三方审批定义请求
#[derive(Debug, Clone)]
pub struct CreateExternalApprovalRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 获取三方审批定义响应
#[derive(Debug, Clone)]
}
pub struct GetExternalApprovalResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl ExternalApprovalService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 创建三方审批定义
    pub async fn create(
        &self,
        request: CreateExternalApprovalRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateExternalApprovalResponse>> {,
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
            api_path: APPROVAL_V4_EXTERNAL_APPROVALS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 查看指定三方审批定义
    pub async fn get(
        &self,
        approval_code: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetExternalApprovalResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_EXTERNAL_APPROVAL_GET,
                "approval_code",
                approval_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}
}}}}}}}}}}}}}