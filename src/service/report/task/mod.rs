#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::report::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    service::report::models::{PageResponse, ReportTask}
};
/// 任务管理服务
pub struct TaskService {
}

impl TaskService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 查询任务
    ///,
/// 查询汇报任务列表，支持分页和条件筛选。
    ///,
/// # Arguments
    ///,
/// * `request` - 任务查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回任务列表
pub async fn query(,
        &self,
        request: TaskQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<TaskQueryResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: REPORT_V1_TASKS_QUERY.to_string(),
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
if let Some(rule_id) = request.rule_id {,
            api_req.query_params.insert("rule_id", rule_id);
if let Some(status) = request.status {,
            api_req.query_params.insert("status", status);
if let Some(task_type) = request.task_type {,
            api_req.query_params.insert("task_type", task_type);
if let Some(reporter_id) = request.reporter_id {,
            api_req.query_params.insert("reporter_id", reporter_id);
if let Some(start_time) = request.start_time {,
            api_req
.query_params
                .insert("start_time", start_time.to_string());
if let Some(end_time) = request.end_time {,
            api_req
.query_params
                .insert("end_time", end_time.to_string());
        Transport::request(api_req, &self.config, option).await,
/// 任务查询请求
#[derive(Debug, Clone)]
pub struct TaskQueryRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}}}}}}}}}