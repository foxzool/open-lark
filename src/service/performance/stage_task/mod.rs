use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat}
        config::Config,
        constants::AccessTokenType,
        endpoints::performance::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    }
    service::performance::models::{PageResponse, StageTask}
};
/// 评估任务管理服务
pub struct StageTaskService {
    pub config: Config,
}
impl StageTaskService {
    /// 创建评估任务管理服务实例
pub fn new() -> Self {
        Self { config }
}
/// 获取周期任务（指定用户）
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
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};

        Transport::request(api_req, &self.config, option).await,
}
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
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};

        Transport::request(api_req, &self.config, option).await,
}
}
/// 用户任务查询请求
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct TaskFindByUserListRequest {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 周期ID
#[serde(skip_serializing_if = "Option::is_none")],
    pub semester_id: Option<String>,
    /// 项目ID
#[serde(skip_serializing_if = "Option::is_none")],
    pub activity_id: Option<String>,
    /// 任务状态
#[serde(skip_serializing_if = "Option::is_none")],
    pub status: Option<String>,
}
/// 用户任务查询响应
#[derive(Debug, Serialize, Deserialize)],
pub struct TaskFindByUserListResponse {
    /// 任务列表
    pub tasks: Vec<StageTask>,
}
impl ApiResponseTrait for TaskFindByUserListResponse {,
    fn data_format() -> ResponseFormat {,
ResponseFormat::Data,
    }
}
/// 任务分页查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)],
pub struct TaskFindByPageRequest {
    /// 周期ID
#[serde(skip_serializing_if = "Option::is_none")],
    pub semester_id: Option<String>,
    /// 项目ID
#[serde(skip_serializing_if = "Option::is_none")],
    pub activity_id: Option<String>,
    /// 任务状态
#[serde(skip_serializing_if = "Option::is_none")],
    pub status: Option<String>,
    /// 页码标记
#[serde(skip_serializing_if = "Option::is_none")],
    pub page_token: Option<String>,
    /// 每页数量
#[serde(skip_serializing_if = "Option::is_none")],
    pub page_size: Option<i32>,
}
/// 任务分页查询响应
#[derive(Debug, Serialize, Deserialize)],
pub struct TaskFindByPageResponse {
    /// 分页任务列表
#[serde(flatten)],
    pub tasks: PageResponse<StageTask>,
}
impl ApiResponseTrait for TaskFindByPageResponse {,
    fn data_format() -> ResponseFormat {,
ResponseFormat::Data,
    }
}
