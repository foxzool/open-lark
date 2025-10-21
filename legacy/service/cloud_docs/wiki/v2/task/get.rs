use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{cloud_docs::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 获取任务结果请求
#[derive(Debug, Serialize, Default)]
pub struct GetTaskRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 任务id
    #[serde(skip)]
    task_id: String,
}

impl GetTaskRequest {
    pub fn builder() -> GetTaskRequestBuilder {
        GetTaskRequestBuilder::default()
    }

    pub fn new(task_id: impl ToString) -> Self {
        Self {
            task_id: task_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct GetTaskRequestBuilder {
    request: GetTaskRequest,
}

impl GetTaskRequestBuilder {
    /// 任务id
    pub fn task_id(mut self, task_id: impl ToString) -> Self {
        self.request.task_id = task_id.to_string();
        self
    }

    pub fn build(mut self) -> GetTaskRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    GetTaskRequestBuilder,
    crate::service::cloud_docs::wiki::v2::task::TaskService,
    GetTaskRequest,
    GetTaskResponse,
    get
);

/// 任务状态
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 进行中
    Processing,
    /// 成功
    Success,
    /// 失败
    Failed,
}

/// 移动结果
#[derive(Debug, Deserialize)]
pub struct MoveResult {
    /// 原始文档token
    pub obj_token: String,
    /// 知识空间中的节点token
    pub node_token: String,
    /// 文档标题
    pub title: Option<String>,
    /// 文档类型
    pub obj_type: Option<String>,
}

/// 任务详细信息
#[derive(Debug, Deserialize)]
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
    pub finish_time: Option<String>,
}

/// 获取任务结果响应
#[derive(Debug, Deserialize)]
pub struct GetTaskResponse {
    /// 任务详细信息
    pub task: TaskDetail,
}

impl ApiResponseTrait for GetTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取任务结果
pub async fn get_task(
    request: GetTaskRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetTaskResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path =
        EndpointBuilder::replace_param(WIKI_V2_TASK_GET, "task_id", &request.task_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl TaskStatus {
    /// 是否已完成（成功或失败）
    pub fn is_finished(&self) -> bool {
        matches!(self, TaskStatus::Success | TaskStatus::Failed)
    }

    /// 是否成功
    pub fn is_success(&self) -> bool {
        matches!(self, TaskStatus::Success)
    }

    /// 是否失败
    pub fn is_failed(&self) -> bool {
        matches!(self, TaskStatus::Failed)
    }

    /// 是否进行中
    pub fn is_processing(&self) -> bool {
        matches!(self, TaskStatus::Processing)
    }
}

impl TaskDetail {
    /// 获取进度百分比
    pub fn progress_percentage(&self) -> Option<f32> {
        if let (Some(processed), Some(total)) = (self.processed_count, self.total_count) {
            if total > 0 {
                return Some((processed as f32 / total as f32) * 100.0);
            }
        }
        None
    }

    /// 是否有错误
    pub fn has_error(&self) -> bool {
        self.error_message.is_some()
    }

    /// 获取成功移动的文档数量
    pub fn success_count(&self) -> usize {
        self.move_results
            .as_ref()
            .map_or(0, |results| results.len())
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_get_task_request_builder() {
        let request = GetTaskRequest::builder().task_id("taskxxxxxx").build();

        assert_eq!(request.task_id, "taskxxxxxx");
    }

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
}
