//! 任务管理服务
//!
//! 提供飞书知识库异步任务的管理功能，包括：
//! - 获取任务执行状态
//! - 查询任务结果和进度
//! - 错误信息和调试信息获取
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 任务状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    /// 等待执行
    Pending,
    /// 执行中
    Running,
    /// 执行成功
    Success,
    /// 执行失败
    Failed,
    /// 执行超时
    Timeout,
    /// 已取消
    Cancelled,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "pending"),
            TaskStatus::Running => write!(f, "running"),
            TaskStatus::Success => write!(f, "success"),
            TaskStatus::Failed => write!(f, "failed"),
            TaskStatus::Timeout => write!(f, "timeout"),
            TaskStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// 任务错误信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskError {
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 错误详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, serde_json::Value>>,
}

impl Default for TaskError {
    fn default() -> Self {
        Self {
            code: None,
            message: None,
            details: None,
        }
    }
}

/// 任务结果数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TaskResult {
    /// 成功时的结果数据
    Success(serde_json::Value),
    /// 失败时的错误信息
    Error(TaskError),
}

impl Default for TaskResult {
    fn default() -> Self {
        TaskResult::Success(serde_json::Value::Null)
    }
}

/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskInfo {
    /// 任务ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// 任务进度（0-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// 任务类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 完成时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    /// 任务结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TaskResult>,
    /// 错误信息（兼容字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TaskError>,
    /// 额外数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<HashMap<String, serde_json::Value>>,
}

impl Default for TaskInfo {
    fn default() -> Self {
        Self {
            task_id: None,
            status: None,
            progress: None,
            task_type: None,
            create_time: None,
            update_time: None,
            finish_time: None,
            result: None,
            error: None,
            extra_data: None,
        }
    }
}

/// 获取任务结果请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTaskResultRequest {
    /// 任务ID
    pub task_id: String,
}

impl GetTaskResultRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `task_id`: 任务ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::task::GetTaskResultRequest;
    ///
    /// let request = GetTaskResultRequest::new("task_123");
    /// ```
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
        }
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 参数验证通过
    /// - `Err(String)`: 参数验证失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::task::GetTaskResultRequest;
    ///
    /// let request = GetTaskResultRequest::new("task_123");
    /// assert!(request.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.task_id.trim().is_empty() {
            return Err("任务ID不能为空".to_string());
        }
        if self.task_id.len() > 200 {
            return Err("任务ID长度不能超过200个字符".to_string());
        }
        Ok(())
    }
}

/// 获取任务结果响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTaskResultResponse {
    /// 任务信息
    pub task: TaskInfo,
}

impl ApiResponseTrait for GetTaskResultResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 任务管理服务
#[derive(Clone, Debug)]
pub struct TaskService {
    config: Config,
}

impl TaskService {
    /// 创建任务管理服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::wiki::v2::task::TaskService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = TaskService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取任务结果
    ///
    /// 根据任务ID查询任务的执行状态、进度和结果。
    /// 适用于查询异步操作的执行情况，如文档导入、批量处理等。
    ///
    /// # 参数
    /// * `req` - 获取任务结果请求
    ///
    /// # 返回值
    /// 返回任务的详细信息和结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::task::{TaskService, GetTaskResultRequest};
    ///
    /// let service = TaskService::new(config);
    /// let request = GetTaskResultRequest::new("task_123");
    ///
    /// let result = service.get_task_result(&request).await?;
    /// match result.task.status {
    ///     Some(TaskStatus::Success) => println!("任务完成: {:?}", result.task.result),
    ///     Some(TaskStatus::Running) => println!("任务进行中: {}%", result.task.progress.unwrap_or(0)),
    ///     Some(TaskStatus::Failed) => println!("任务失败: {:?}", result.task.error),
    ///     _ => println!("任务状态未知"),
    /// }
    /// ```
    pub async fn get_task_result(
        &self,
        req: &GetTaskResultRequest,
    ) -> SDKResult<GetTaskResultResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取任务结果: task_id={}", req.task_id);

        // 构建动态端点路径
        let endpoint = openlark_core::endpoints::Endpoints::WIKI_V2_TASK_GET
            .replace("{}", &req.task_id);

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // GET请求无body
            
        };

        let resp = Transport::<GetTaskResultResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "任务结果获取完成: task_id={}, status={:?}",
            req.task_id,
            response.task.status
        );

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 获取任务结果构建器
#[derive(Clone, Debug)]
pub struct GetTaskResultBuilder {
    request: GetTaskResultRequest,
}

impl Default for GetTaskResultBuilder {
    fn default() -> Self {
        Self {
            request: GetTaskResultRequest {
                task_id: String::new(),
            },
        }
    }
}

impl GetTaskResultBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置任务ID
    ///
    /// # 参数
    /// - `task_id`: 任务ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::task::GetTaskResultBuilder;
    ///
    /// let builder = GetTaskResultBuilder::new().task_id("task_123");
    /// ```
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.request.task_id = task_id.into();
        self
    }

    /// 执行获取任务结果操作
    ///
    /// # 参数
    /// - `service`: 任务管理服务实例
    ///
    /// # 返回值
    /// 返回任务的详细信息和结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::task::{TaskService, GetTaskResultBuilder};
    ///
    /// let service = TaskService::new(config);
    ///
    /// let result = GetTaskResultBuilder::new()
    ///     .task_id("task_123")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(self, service: &TaskService) -> SDKResult<GetTaskResultResponse> {
        service.get_task_result(&self.request).await
    }
}

impl TaskService {
    /// 创建获取任务结果构建器
    ///
    /// # 返回值
    /// 返回获取任务结果构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::task::TaskService;
    ///
    /// let service = TaskService::new(config);
    /// let builder = service.get_task_result_builder();
    /// ```
    pub fn get_task_result_builder(&self) -> GetTaskResultBuilder {
        GetTaskResultBuilder::new()
    }
}

// ==================== 单元测试 ====================

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
    fn test_task_status_enum() {
        // 测试所有任务状态
        let statuses = vec![
            TaskStatus::Pending,
            TaskStatus::Running,
            TaskStatus::Success,
            TaskStatus::Failed,
            TaskStatus::Timeout,
            TaskStatus::Cancelled,
        ];

        for status in &statuses {
            let display_str = format!("{}", status);
            assert!(!display_str.is_empty());
        }

        // 测试默认值
        let default_status = TaskStatus::default();
        assert_eq!(default_status, TaskStatus::Pending);

        // 测试相等性比较
        assert_eq!(TaskStatus::Success, TaskStatus::Success);
        assert_ne!(TaskStatus::Success, TaskStatus::Failed);
    }

    #[test]
    fn test_task_status_serialization() {
        let status = TaskStatus::Success;
        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: TaskStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_task_error_default_creation() {
        let error = TaskError::default();
        assert_eq!(error.code, None);
        assert_eq!(error.message, None);
        assert_eq!(error.details, None);
    }

    #[test]
    fn test_task_error_with_data() {
        let mut details = HashMap::new();
        details.insert(
            "field".to_string(),
            serde_json::Value::String("value".to_string()),
        );

        let error = TaskError {
            code: Some("ERROR_CODE".to_string()),
            message: Some("错误信息".to_string()),
            details: Some(details),
        };

        assert_eq!(error.code, Some("ERROR_CODE".to_string()));
        assert_eq!(error.message, Some("错误信息".to_string()));
        assert!(error.details.is_some());
    }

    #[test]
    fn test_task_result_variants() {
        // 测试成功结果
        let success_result = TaskResult::Success(serde_json::json!({"key": "value"}));
        if let TaskResult::Success(value) = success_result {
            assert_eq!(
                value.get("key"),
                Some(&serde_json::Value::String("value".to_string()))
            );
        }

        // 测试错误结果
        let error = TaskError {
            code: Some("ERROR".to_string()),
            message: Some("Error message".to_string()),
            
        };
        let error_result = TaskResult::Error(error);
        if let TaskResult::Error(err) = error_result {
            assert_eq!(err.code, Some("ERROR".to_string()));
        }

        // 测试默认值
        let default_result = TaskResult::default();
        if let TaskResult::Success(value) = default_result {
            assert_eq!(value, serde_json::Value::Null);
        }
    }

    #[test]
    fn test_task_info_default_creation() {
        let task_info = TaskInfo::default();
        assert_eq!(task_info.task_id, None);
        assert_eq!(task_info.status, None);
        assert_eq!(task_info.progress, None);
        assert_eq!(task_info.task_type, None);
        assert_eq!(task_info.create_time, None);
        assert_eq!(task_info.update_time, None);
        assert_eq!(task_info.finish_time, None);
        assert_eq!(task_info.result, None);
        assert_eq!(task_info.error, None);
        assert_eq!(task_info.extra_data, None);
    }

    #[test]
    fn test_task_info_with_data() {
        let mut extra_data = HashMap::new();
        extra_data.insert(
            "batch_id".to_string(),
            serde_json::Value::String("batch_123".to_string()),
        );

        let task_info = TaskInfo {
            task_id: Some("task_123".to_string()),
            status: Some(TaskStatus::Success),
            progress: Some(100),
            task_type: Some("document_import".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-01T00:05:00Z".to_string()),
            finish_time: Some("2023-01-01T00:05:00Z".to_string()),
            result: Some(TaskResult::Success(
                serde_json::json!({"imported_count": 10}),
            )),
            error: None,
            extra_data: Some(extra_data),
        };

        assert_eq!(task_info.task_id, Some("task_123".to_string()));
        assert_eq!(task_info.status, Some(TaskStatus::Success));
        assert_eq!(task_info.progress, Some(100));
        assert_eq!(task_info.task_type, Some("document_import".to_string()));
        assert_eq!(
            task_info.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            task_info.update_time,
            Some("2023-01-01T00:05:00Z".to_string())
        );
        assert_eq!(
            task_info.finish_time,
            Some("2023-01-01T00:05:00Z".to_string())
        );
        assert!(task_info.result.is_some());
        assert!(task_info.extra_data.is_some());
    }

    #[test]
    fn test_get_task_result_request() {
        let request = GetTaskResultRequest::new("task_123");
        assert_eq!(request.task_id, "task_123");
    }

    #[test]
    fn test_get_task_result_request_validation() {
        // 测试正常情况
        let valid_request = GetTaskResultRequest::new("task_123");
        assert!(valid_request.validate().is_ok());

        // 测试空task_id
        let empty_request = GetTaskResultRequest::new("");
        assert!(empty_request.validate().is_err());

        // 测试空白字符
        let whitespace_request = GetTaskResultRequest::new("   ");
        assert!(whitespace_request.validate().is_err());

        // 测试长度超限
        let long_request = GetTaskResultRequest::new(&"a".repeat(201));
        assert!(long_request.validate().is_err());

        // 测试长度边界
        let boundary_request = GetTaskResultRequest::new(&"a".repeat(200));
        assert!(boundary_request.validate().is_ok());
    }

    #[test]
    fn test_get_task_result_builder() {
        let builder = GetTaskResultBuilder::new().task_id("task_123");
        assert_eq!(builder.request.task_id, "task_123");
    }

    #[test]
    fn test_get_task_result_builder_default() {
        let builder = GetTaskResultBuilder::default();
        assert_eq!(builder.request.task_id, "");
    }

    #[test]
    fn test_response_default_creation() {
        let response = GetTaskResultResponse::default();
        assert_eq!(response.task.task_id, None);
        assert_eq!(response.task.status, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut response = GetTaskResultResponse::default();
        response.task = TaskInfo {
            task_id: Some("task_abc".to_string()),
            status: Some(TaskStatus::Success),
            
        };

        assert_eq!(response.task.task_id, Some("task_abc".to_string()));
        assert_eq!(response.task.status, Some(TaskStatus::Success));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(GetTaskResultResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = GetTaskResultRequest::new("task_123");
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetTaskResultRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.task_id, deserialized.task_id);
    }

    #[test]
    fn test_response_serialization() {
        let mut response = GetTaskResultResponse::default();
        response.task = TaskInfo {
            task_id: Some("task_abc".to_string()),
            status: Some(TaskStatus::Success),
            progress: Some(100),
            
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetTaskResultResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.task.task_id, deserialized.task.task_id);
        assert_eq!(response.task.status, deserialized.task.status);
        assert_eq!(response.task.progress, deserialized.task.progress);
    }

    #[test]
    fn test_comprehensive_task_info_scenarios() {
        // 测试失败任务
        let failed_task = TaskInfo {
            task_id: Some("task_failed".to_string()),
            status: Some(TaskStatus::Failed),
            progress: Some(75),
            error: Some(TaskError {
                code: Some("TIMEOUT".to_string()),
                message: Some("任务执行超时".to_string()),
                
            }),
            
        };

        assert_eq!(failed_task.task_id, Some("task_failed".to_string()));
        assert_eq!(failed_task.status, Some(TaskStatus::Failed));
        assert_eq!(failed_task.progress, Some(75));
        assert!(failed_task.error.is_some());

        // 测试进行中任务
        let running_task = TaskInfo {
            task_id: Some("task_running".to_string()),
            status: Some(TaskStatus::Running),
            progress: Some(45),
            task_type: Some("batch_export".to_string()),
            
        };

        assert_eq!(running_task.status, Some(TaskStatus::Running));
        assert_eq!(running_task.progress, Some(45));
        assert_eq!(running_task.task_type, Some("batch_export".to_string()));

        // 测试取消任务
        let cancelled_task = TaskInfo {
            task_id: Some("task_cancelled".to_string()),
            status: Some(TaskStatus::Cancelled),
            progress: Some(20),
            finish_time: Some("2023-01-01T00:02:00Z".to_string()),
            
        };

        assert_eq!(cancelled_task.status, Some(TaskStatus::Cancelled));
        assert_eq!(cancelled_task.progress, Some(20));
        assert!(cancelled_task.finish_time.is_some());
    }

    #[test]
    fn test_builder_chain_calls() {
        let builder = GetTaskResultBuilder::new()
            .task_id("task_123")
            .task_id("task_456"); // 覆盖之前的值

        assert_eq!(builder.request.task_id, "task_456");
    }

    #[test]
    fn test_request_validation_edge_cases() {
        // 测试仅包含空白字符的task_id
        let whitespace_request = GetTaskResultRequest::new("  \t\n  ");
        assert!(whitespace_request.validate().is_err());

        // 测试中文字符（虽然可能不常见，但应该能处理）
        let chinese_request = GetTaskResultRequest::new("任务_123");
        assert!(chinese_request.validate().is_ok());

        // 测试包含特殊字符的task_id
        let special_chars_request = GetTaskResultRequest::new("task_abc-123_xyz");
        assert!(special_chars_request.validate().is_ok());
    }

    #[test]
    fn test_endpoint_constant() {
        // 测试端点常量是否正确定义
        assert_eq!(
            openlark_core::endpoints::Endpoints::WIKI_V2_TASK_GET,
            "/open-apis/wiki/v2/tasks/{}"
        );
    }

    #[test]
    fn test_task_error_serialization() {
        let mut details = HashMap::new();
        details.insert(
            "retry_count".to_string(),
            serde_json::Value::Number(serde_json::Number::from(3)),
        );

        let error = TaskError {
            code: Some("NETWORK_ERROR".to_string()),
            message: Some("网络连接失败".to_string()),
            details: Some(details),
        };

        let serialized = serde_json::to_string(&error).unwrap();
        let deserialized: TaskError = serde_json::from_str(&serialized).unwrap();

        assert_eq!(error.code, deserialized.code);
        assert_eq!(error.message, deserialized.message);
        assert!(deserialized.details.is_some());
    }

    #[test]
    fn test_task_result_with_complex_data() {
        // 测试包含复杂结果数据的任务
        let complex_result = serde_json::json!({
            "processed_items": 150,
            "failed_items": 2,
            "warnings": ["Item 45 had minor issues"],
            "statistics": {
                "processing_time": 120.5,
                "memory_usage": "256MB"
            }
        });

        let task_with_complex_result = TaskInfo {
            task_id: Some("task_complex".to_string()),
            status: Some(TaskStatus::Success),
            result: Some(TaskResult::Success(complex_result)),
            
        };

        if let Some(TaskResult::Success(data)) = task_with_complex_result.result {
            assert!(data.get("processed_items").is_some());
            assert!(data.get("statistics").is_some());
        }
    }
}
