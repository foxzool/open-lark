#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use open_lark_core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpointshttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::apass::models::{
        EnvironmentVariable, EnvironmentVariableGetRequest, EnvironmentVariableQueryRequest,
        PageResponse,
};
/// 环境变量服务
pub struct EnvironmentVariableService {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 环境变量获取响应
#[derive(Debug, Clone)]
}
pub struct EnvironmentVariableGetResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl EnvironmentVariableService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 查询环境变量列表
    ///,
/// 该接口用于查询应用的环境变量列表。
    ///,
/// # 参数
    ///,
/// - `request`: 环境变量查询请求参数
    /// - `option`: 可选的请求配置
pub async fn query_environment_variables(,
        &self,
        request: EnvironmentVariableQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EnvironmentVariableQueryResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::APASS_V1_ENVIRONMENT_VARIABLE_QUERY,
                "app_id",
                &request.app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
// 添加查询参数
        if let Some(page_size) = request.page_size {,
api_req
                .query_params
                .insert("page_size", page_size.to_string());
if let Some(page_token) = request.page_token {,
            api_req.query_params.insert("page_token", page_token);
        Transport::request(api_req, &self.config, option).await,
/// 查询环境变量详情
    ///,
/// 该接口用于查询指定环境变量的详情信息。
    ///,
/// # 参数
    ///,
/// - `request`: 环境变量获取请求参数
    /// - `option`: 可选的请求配置
pub async fn get_environment_variable(,
        &self,
        request: EnvironmentVariableGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EnvironmentVariableGetResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_ENVIRONMENT_VARIABLE_GET,
                &[
                    ("app_id", &request.app_id),
                    ("variable_name", &request.variable_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}
}}}}}}}}}}}}}