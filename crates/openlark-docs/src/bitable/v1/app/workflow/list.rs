
use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::{,
use SDKResult;    api::{ApiRequest, ApiResponseTrait},
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,

    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
/// 列出自动化流程请求,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkflowRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 页大小,
#[serde(skip)]
    page_size: Option<i32>,
    /// 页标记，第一次请求不填，表示从头开始遍历,
#[serde(skip)]
    page_token: Option<String>}
impl ListWorkflowRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
pub struct ListWorkflowRequestBuilder {
    request: ListWorkflowRequest}
impl ListWorkflowRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_token) = &self.request.page_token {,
            self.request,
.api_request,
self.request,
    }
// 应用ExecutableBuilder trait到ListWorkflowRequestBuilder,
crate::impl_executable_builder_owned!(
    ListWorkflowRequestBuilder,
    super::AppWorkflowService,
    ListWorkflowRequest,
    Response<ListWorkflowResponse>,
    list,
);
#[derive(Serialize)]
/// 自动化流程信息
pub struct Workflow {
    /// 自动化流程ID
    pub id: String,
    /// 自动化流程名称
    pub name: String,
    /// 自动化流程状态：0-未启用，1-已启用
    pub is_enabled: i32,
    /// 自动化流程创建时间戳（秒）
    pub created_time: i64,
    /// 自动化流程更新时间戳（秒）
    pub updated_time: i64,
    /// 自动化流程最后执行时间戳（秒）,
#[serde(default)]
    pub last_execution_time: Option<i64>,
    /// 流程触发器类型
    pub trigger_type: String,
    /// 流程描述,
#[serde(default)]
    pub description: Option<String>}
#[derive(Deserialize)]
/// 列出自动化流程响应,
pub struct ListWorkflowResponse {
    /// 自动化流程列表
    pub items: Vec<Workflow>,
    /// 分页标记，当has_more为true时，会同时返回新的page_token,
#[serde(default)]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 总数,
#[serde(default)]
    pub total: Option<i32>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 列出自动化流程,
pub async fn list_workflows(
    request: ListWorkflowRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<ListWorkflowResponse>> {,
let mut api_req = request.api_request;
    api_req.set_api_path(BITABLE_V1_WORKFLOWS.replace({app_token}, &request.app_token));

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

