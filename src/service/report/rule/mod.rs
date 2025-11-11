#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use open_lark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::report::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    service::report::models::{PageResponse, ReportRule}
};
/// 规则管理服务
pub struct RuleService {
}

impl RuleService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 查询规则
    ///,
/// 查询汇报规则列表，支持分页和条件筛选。
    ///,
/// # Arguments
    ///,
/// * `request` - 规则查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回规则列表
pub async fn query(,
        &self,
        request: RuleQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleQueryResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: REPORT_V1_RULES_QUERY.to_string(),
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
if let Some(rule_type) = request.rule_type {,
            api_req.query_params.insert("rule_type", rule_type);
if let Some(status) = request.status {,
            api_req.query_params.insert("status", status);
if let Some(creator) = request.creator {,
            api_req.query_params.insert("creator", creator);
if let Some(name) = request.name {,
            api_req.query_params.insert("name", name);
if let Some(start_time) = request.start_time {,
            api_req
.query_params
                .insert("start_time", start_time.to_string());
if let Some(end_time) = request.end_time {,
            api_req
.query_params
                .insert("end_time", end_time.to_string());
        Transport::request(api_req, &self.config, option).await,
/// 规则查询请求
#[derive(Debug, Clone)]
pub struct RuleQueryRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}}}}}}}}}