#![allow(unused_variables, unused_unsafe)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::{cloud_docs::*, EndpointBuilder};
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    }
    impl_executable_builder_owned,
};
/// 获取任务结果请求,
#[derive(Clone, Debug)]
pub struct GetTaskRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 任务id,
#[serde(skip)]
    task_id: String}
impl GetTaskRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct GetTaskRequestBuilder {
    request: GetTaskRequest}
impl GetTaskRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    GetTaskRequestBuilder,
    super::cloud_docs::wiki::v2::task::TaskService,
    GetTaskRequest,
    GetTaskResponse,
    get,
);
/// 任务状态
#[derive(Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
/// 进行中,
    Processing,
    /// 成功
    Success,
    /// 失败
    Failed}
/// 移动结果,
#[derive(Clone, Debug)]
pub struct MoveResult {
    /// 原始文档token
    pub obj_token: String,
    /// 知识空间中的节点token
    pub node_token: String,
    /// 文档标题
    pub title: Option<String>,
    /// 文档类型
    pub obj_type: Option<String>}
/// 任务详细信息,
#[derive(Clone, Debug)]
pub struct TaskDetail {
    /// 任务id
    pub task_id: String,
    /// 任务状态
    pub status: TaskStatus,
    /// 知识空间id
    pub space_id: Option<String>,
    /// 已处理的文档数量
    pub processed_count: Option<i32>,
    /// 总文档数量
    pub total_count: Option<i32>,
    /// 移动成功的结果列表
    pub move_results: Option<Vec<MoveResult>>,
    /// 错误信息
    pub error_message: Option<String>,
    /// 创建时间（毫秒时间戳）
    pub create_time: Option<String>,
    /// 完成时间（毫秒时间戳）
    pub finish_time: Option<String>}
/// 获取任务结果响应,
#[derive(Clone, Debug)]
pub struct GetTaskResponse {
    /// 任务详细信息
    pub task: TaskDetail,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取任务结果,
pub async fn get_task(
    request: GetTaskRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<GetTaskResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.set_api_path(EndpointBuilder::replace_param(,
        WIKI_V2_TASK_GET,
        "task_id",
        &request.task_id,
    ));
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp)}

impl TaskStatus {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl TaskDetail {
    pub fn new(config: Config) -> Self {
        Self { config }
}        }
None,
    }
/// 是否有错误,
    pub fn w+.*{
self.error_message.is_some()}
/// 获取成功移动的文档数量,
    pub fn w+.*{
self.move_results,
            .as_ref()
            .map_or(0, |results| results.len())}
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_get_task_request_builder() {
let request = GetTaskRequest::builder().task_id("taskxxxxxx").build();
        assert_eq!(request.task_id, "taskxxxxxx");
#[test]
    fn test_task_status_methods() {
assert!(TaskStatus::Success.is_finished());
        assert!(TaskStatus::Failed.is_finished());
assert!(!TaskStatus::Processing.is_finished());
        assert!(TaskStatus::Success.is_success());
assert!(!TaskStatus::Failed.is_success());
        assert!(TaskStatus::Failed.is_failed());
assert!(!TaskStatus::Success.is_failed());
        assert!(TaskStatus::Processing.is_processing());
assert!(!TaskStatus::Success.is_processing());
    }
