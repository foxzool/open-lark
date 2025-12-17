/// 查询异步任务状态
///
/// 查询异步任务状态，用于获取长时间运行任务的执行结果。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/task_check

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 查询异步任务状态请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTaskStatusRequest {
    #[serde(skip)]
    config: Config,
    /// 任务ID
    pub task_id: String,
    /// 任务类型
    pub task_type: Option<String>,
}

impl CheckTaskStatusRequest {
    /// 创建新的请求实例
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self { 
            config,
            task_id: task_id.into(),
            task_type: None,
        }
    }

    /// 设置任务类型
    pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
        self.task_type = Some(task_type.into());
        self
    }

    /// 执行查询异步任务状态操作
    pub async fn execute(self) -> SDKResult<Response<CheckTaskStatusResponse>> {
        let api_endpoint = DriveApi::TaskCheck;
        let mut api_request = ApiRequest::<CheckTaskStatusResponse>::get(&api_endpoint.to_url())
            .query("task_id", &self.task_id);

        // 设置查询参数
        if let Some(task_type) = self.task_type {
            api_request = api_request.query("task_type", task_type);
        }

        Transport::request(api_request, &self.config, None).await
    }
}

/// 查询异步任务状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTaskStatusResponse {
    /// 任务ID
    pub task_id: String,
    /// 任务状态
    pub status: String,
    /// 任务进度
    pub progress: Option<f64>,
    /// 错误信息
    pub error_message: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 完成时间
    pub complete_time: Option<String>,
    /// 任务结果
    pub result: Option<TaskResult>,
}

/// 任务结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// 任务结果数据
    pub data: Option<serde_json::Value>,
    /// 文件token
    pub file_token: Option<String>,
    /// 操作类型
    pub operation_type: Option<String>,
}

// Alias for tests compatibility if needed, or update tests to use CheckTaskStatusResponse directly
pub type TaskStatusData = CheckTaskStatusResponse;

impl ApiResponseTrait for CheckTaskStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_task_status_request_builder() {
        let config = Config::default();
        let request = CheckTaskStatusRequest::new(config, "task_123")
            .task_type("import");

        assert_eq!(request.task_id, "task_123");
        assert_eq!(request.task_type, Some("import".to_string()));
    }

    #[test]
    fn test_task_status_data() {
        let result = TaskResult {
            data: Some(serde_json::json!({"test": "data"})),
            file_token: Some("file_token_123".to_string()),
            operation_type: Some("import".to_string()),
        };

        let data = CheckTaskStatusResponse {
            task_id: "task_456".to_string(),
            status: "completed".to_string(),
            progress: Some(100.0),
            error_message: None,
            create_time: "2023-12-01T00:00:00Z".to_string(),
            complete_time: Some("2023-12-01T00:05:00Z".to_string()),
            result: Some(result),
        };

        assert_eq!(data.task_id, "task_456");
        assert_eq!(data.status, "completed");
        assert_eq!(data.progress, Some(100.0));
        assert_eq!(data.create_time, "2023-12-01T00:00:00Z");
        assert_eq!(data.complete_time, Some("2023-12-01T00:05:00Z".to_string()));

        if let Some(result) = data.result {
            assert_eq!(result.file_token, Some("file_token_123".to_string()));
            assert_eq!(result.operation_type, Some("import".to_string()));
        }
    }

    #[test]
    fn test_response_trait_implementation() {
        assert_eq!(CheckTaskStatusResponse::data_format(), ResponseFormat::Data);
    }
}