/// 查询异步任务状态
///
/// 查询异步任务状态，用于获取长时间运行任务的执行结果。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/task_check

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 查询异步任务状态请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTaskStatusParams {
    /// 任务ID
    pub task_id: String,
    /// 任务类型
    pub task_type: Option<String>,
}

/// 查询异步任务状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTaskStatusResponse {
    /// 任务状态信息
    pub data: Option<TaskStatusData>,
}

/// 任务状态数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusData {
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
    /// 结果数据
    pub data: Option<serde_json::Value>,
    /// 文件token（如果适用）
    pub file_token: Option<String>,
    /// 操作类型
    pub operation_type: Option<String>,
}

impl ApiResponseTrait for CheckTaskStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询异步任务状态请求
pub struct CheckTaskStatusRequest {
    config: Config,
}

impl CheckTaskStatusRequest {
    /// 创建新的请求实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行查询异步任务状态操作
    pub async fn execute(self, params: CheckTaskStatusParams) -> SDKResult<CheckTaskStatusResponse> {
        // 验证必填字段
        validate_required_field("任务ID", Some(&params.task_id), "任务ID不能为空")?;

        let api_endpoint = DriveApi::TaskCheck(params.task_id.clone());
        let mut api_request: ApiRequest<CheckTaskStatusResponse> = ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        if let Some(task_type) = &params.task_type {
            api_request = api_request.query("task_type", task_type);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "查询异步任务状态")
    }
}

// ==================== 构建器模式 ====================

/// 查询异步任务状态构建器
pub struct CheckTaskStatusBuilder {
    request: CheckTaskStatusParams,
}

impl CheckTaskStatusBuilder {
    /// 创建新的构建器
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            request: CheckTaskStatusParams {
                task_id: task_id.into(),
                task_type: None,
            },
        }
    }

    /// 设置任务类型
    pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
        self.request.task_type = Some(task_type.into());
        self
    }

    /// 执行查询异步任务状态操作
    pub async fn execute(self, service: &TaskService) -> SDKResult<CheckTaskStatusResponse> {
        service.check_task_status(self.request).await
    }
}

/// 任务服务
pub struct TaskService {
    config: Config,
}

impl TaskService {
    /// 创建任务服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 查询异步任务状态
    /// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/task_check
    pub async fn check_task_status(&self, params: CheckTaskStatusParams) -> SDKResult<CheckTaskStatusResponse> {
        CheckTaskStatusRequest::new(self.config.clone()).execute(params).await
    }

    /// 创建查询异步任务状态构建器
    pub fn check_task_status_builder(&self, task_id: impl Into<String>) -> CheckTaskStatusBuilder {
        CheckTaskStatusBuilder::new(task_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = TaskService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_check_task_status_builder() {
        let builder = CheckTaskStatusBuilder::new("task_123")
            .task_type("import");

        assert_eq!(builder.request.task_id, "task_123");
        assert_eq!(builder.request.task_type, Some("import".to_string()));
    }

    #[test]
    fn test_check_task_status_params() {
        let params = CheckTaskStatusParams {
            task_id: "task_456".to_string(),
            task_type: Some("export".to_string()),
        };

        assert_eq!(params.task_id, "task_456");
        assert_eq!(params.task_type, Some("export"));
    }

    #[test]
    fn test_response_trait_implementation() {
        assert_eq!(CheckTaskStatusResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_serialization() {
        let params = CheckTaskStatusParams {
            task_id: "task_789".to_string(),
            task_type: Some("convert".to_string()),
        };

        let serialized = serde_json::to_string(&params).unwrap();
        let deserialized: CheckTaskStatusParams = serde_json::from_str(&serialized).unwrap();

        assert_eq!(params.task_id, deserialized.task_id);
        assert_eq!(params.task_type, deserialized.task_type);
    }

    #[test]
    fn test_task_status_data() {
        let result = TaskResult {
            data: Some(serde_json::json!({"test": "data"})),
            file_token: Some("file_token_123".to_string()),
            operation_type: Some("import".to_string()),
        };

        let data = TaskStatusData {
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
    fn test_builder_default() {
        let builder = CheckTaskStatusBuilder::new("task_test");
        assert_eq!(builder.request.task_id, "task_test");
        assert_eq!(builder.request.task_type, None);
    }

    #[test]
    fn test_multiple_task_types() {
        let import_builder = CheckTaskStatusBuilder::new("import_task")
            .task_type("import");

        let export_builder = CheckTaskStatusBuilder::new("export_task")
            .task_type("export");

        let convert_builder = CheckTaskStatusBuilder::new("convert_task")
            .task_type("convert");

        assert_eq!(import_builder.request.task_type, Some("import"));
        assert_eq!(export_builder.request.task_type, Some("export"));
        assert_eq!(convert_builder.request.task_type, Some("convert"));
    }
}