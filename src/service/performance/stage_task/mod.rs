use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::performance::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    service::performance::models::{PageResponse, StageTask}
};
/// 评估任务管理服务
pub struct StageTaskService {
}

impl StageTaskService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取周期任务（指定用户）
    ///,
/// 根据用户列表获取其周期任务信息，支持按周期ID、项目ID、任务状态等条件筛选。
    /// 用于批量查询指定用户的评估任务，便于任务管理和进度跟踪。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/stage_task/find_by_user_list
pub async fn find_tasks_by_user_list(,
        &self,
        request: TaskFindByUserListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<TaskFindByUserListResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_V1_STAGE_TASK_FIND_BY_USER_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取周期任务（全部用户）
    ///,
/// 分页获取全部用户的周期任务信息，支持按周期ID、项目ID、任务状态等条件筛选。
    ///用于获取全局的任务视图，便于统计分析和绩效管理。,
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/stage_task/find_by_page
pub async fn find_tasks_by_page(,
        &self,
        request: TaskFindByPageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<TaskFindByPageResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_V1_STAGE_TASK_FIND_BY_PAGE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 用户任务查询请求
#[derive(Debug, Clone)]
pub struct TaskFindByUserListRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 任务分页查询请求
#[derive(Debug, Clone)]
}
pub struct TaskFindByPageRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}}}}}