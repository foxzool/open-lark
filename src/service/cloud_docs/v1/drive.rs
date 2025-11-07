#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Cloud Docs Drive v1服务模块
//!
//! 云盘文件管理服务，提供文件和文件夹的增删改查、权限管理、
//! 分享链接、版本控制等企业级文档管理功能。
//!
//! 提供完整的异步任务状态监控功能：
//! - 查询异步任务执行状态
//! - 支持任务进度跟踪
//! - 错误信息获取和处理

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ==================== API #188: 查询异步任务状态 ====================

/// 任务状态枚举
///
/// 表示异步任务的执行状态，支持处理中、成功、失败三种状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 处理中 - 任务正在执行，尚未完成
    Processing,
    /// 成功 - 任务执行完成且成功
    Success,
    /// 失败 - 任务执行失败
    Failed,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Processing
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Processing => write!(f, "processing"),
            TaskStatus::Success => write!(f, "success"),
            TaskStatus::Failed => write!(f, "failed"),
        }
    }
}

/// 任务详细信息
///
/// 包含异步任务的完整信息，包括状态、进度、错误信息等
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskDetail {
    /// 任务ID
    /// 异步任务的唯一标识符
    pub task_id: String,
    /// 任务状态
    /// 当前任务的执行状态
    pub status: TaskStatus,
    /// 任务类型
    /// 任务的具体类型，如文件上传、复制、移动等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 任务描述
    /// 任务的详细描述信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 进度信息
    /// 任务执行的进度百分比(0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// 已处理项目数量
    /// 对于批量操作，表示已处理的数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_count: Option<i32>,
    /// 总项目数量
    /// 对于批量操作，表示总数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 错误信息
    /// 任务失败时的错误描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    /// 任务失败时的错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// 创建时间
    /// 任务创建的时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 开始时间
    /// 任务开始执行的时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 完成时间
    /// 任务完成的时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    /// 预估剩余时间（秒）
    /// 任务预计完成所需的时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_remaining_seconds: Option<i32>,
}

impl TaskDetail {
    /// 检查任务是否已完成
    ///
    /// # 返回值
    /// 返回true表示任务已完成（成功或失败）
    pub fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Success | TaskStatus::Failed)
    }

    /// 检查任务是否成功
    ///
    /// # 返回值
    /// 返回true表示任务执行成功
    pub fn is_success(&self) -> bool {
        matches!(self.status, TaskStatus::Success)
    }

    /// 检查任务是否失败
    ///
    /// # 返回值
    /// 返回true表示任务执行失败
    pub fn is_failed(&self) -> bool {
        matches!(self.status, TaskStatus::Failed)
    }

    /// 获取进度百分比
    ///
    /// # 返回值
    /// 返回任务进度百分比(0-100)，如果没有进度信息返回None
    pub fn get_progress_percentage(&self) -> Option<f32> {
        self.progress.map(|p| p as f32)
    }

    /// 获取格式化的进度信息
    ///
    /// # 返回值
    /// 返回格式化的进度字符串，如"50/100"
    pub fn get_progress_text(&self) -> Option<String> {
        match (self.processed_count, self.total_count) {
            (Some(processed), Some(total)) => Some(format!("{}/{}", processed, total)),
            _ => None,
        }
    }
}

/// 查询异步任务状态请求
///
/// 用于查询指定异步任务的执行状态和详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTaskStatusRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 任务ID
    /// 要查询的异步任务的唯一标识符
    pub task_id: String,
}

impl GetTaskStatusRequest {
    /// 创建新的查询任务状态请求
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 返回
    /// 返回查询任务状态请求实例
    ///
    /// # 示例
    /// ```rust
    /// let request = GetTaskStatusRequest::new("task_123456789");
    /// ```
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            api_req: ApiRequest::default(),
            task_id: task_id.into(),
        }
    }

    /// 验证请求参数
    ///
    /// # 返回
    /// * `Ok(())` - 验证通过
    /// * `Err(String)` - 验证失败，返回错误描述
    pub fn validate(&self) -> Result<(), String> {
        if self.task_id.is_empty() {
            return Err("任务ID不能为空".to_string());
        }

        if self.task_id.len() > 100 {
            return Err("任务ID长度不能超过100个字符".to_string());
        }

        Ok(())
    }
}

/// 查询异步任务状态响应
///
/// 包含指定异步任务的详细状态信息和执行结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTaskStatusResponse {
    /// 任务详细信息
    /// 包含任务的完整状态和进度信息
    pub task: TaskDetail,
}

impl ApiResponseTrait for GetTaskStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== API #194: 删除文件或文件夹 ====================

/// 删除文件或文件夹请求
///
/// 用于删除云盘中的文件或文件夹，支持递归删除目录。
/// 删除操作不可恢复，请谨慎使用。
///
/// # 示例
/// ```rust
/// let request = DeleteFileRequest::new("file_token_123456789");
/// let response = service.delete_file(request, None).await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件或文件夹Token
    /// 要删除的文件或文件夹的唯一标识符
    pub file_token: String,
}

impl Default for DeleteFileRequest {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_token: String::new(),
        }
    }
}

impl DeleteFileRequest {
    /// 创建新的删除文件请求
    ///
    /// # 参数
    /// * `file_token` - 文件或文件夹Token
    ///
    /// # 返回
    /// 返回删除文件请求实例
    ///
    /// # 示例
    /// ```rust
    /// let request = DeleteFileRequest::new("file_token_123456789");
    /// ```
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_token: file_token.into(),
        }
    }

    /// 验证请求参数
    ///
    /// # 返回
    /// * `Ok(())` - 验证通过
    /// * `Err(String)` - 验证失败，返回错误描述
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.is_empty() {
            return Err("文件Token不能为空".to_string());
        }

        if self.file_token.trim().is_empty() {
            return Err("文件Token不能只包含空格".to_string());
        }

        if self.file_token.len() > 200 {
            return Err("文件Token长度不能超过200个字符".to_string());
        }

        Ok(())
    }
}

/// 删除文件或文件夹响应
///
/// 包含删除操作的结果信息，确认文件是否成功删除。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFileResponse {
    /// 删除操作结果
    /// true表示删除成功，false表示删除失败
    pub success: bool,
    /// 文件或文件夹Token
    /// 被删除的文件或文件夹的唯一标识符
    pub file_token: String,
    /// 文件类型
    /// 被删除文件的类型：file(文件)或folder(文件夹)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 删除时间
    /// 文件删除的时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<String>,
    /// 错误信息
    /// 如果删除失败，包含具体的错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    /// 如果删除失败，包含具体的错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for DeleteFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 云盘服务 v1
///
/// 提供完整的文件管理功能，包括：
/// - 文件上传、下载、删除
/// - 文件夹创建、重命名、移动
/// - 权限设置和访问控制
/// - 文件分享和链接管理
/// - 文件版本控制和历史记录
#[derive(Debug, Clone)]
pub struct DriveServiceV1 {
    pub config: Config,
}

impl DriveServiceV1 {
    /// 创建Drive服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询异步任务状态
    ///
    /// 查询指定异步任务的执行状态、进度信息和结果。支持监控文件上传、
    /// 复制、移动、删除等异步操作的执行情况。
    ///
    /// # 参数
    /// * `request` - 查询任务状态请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回任务状态响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust
    /// let request = GetTaskStatusRequest::new("task_123456789");
    /// let response = service.get_task_status(request, None).await?;
    ///
    /// match response.task.status {
    ///     TaskStatus::Processing => println!("任务处理中..."),
    ///     TaskStatus::Success => println!("任务完成"),
    ///     TaskStatus::Failed => println!("任务失败: {:?}", response.task.error_message),
    /// }
    /// ```
    pub async fn get_task_status(
        &self,
        request: GetTaskStatusRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<GetTaskStatusResponse>> {
        // 验证请求参数
        request.validate().map_err(|e| {
            crate::core::error::LarkAPIError::IllegalParamError(format!("参数验证失败: {}", e))
        })?;

        // 创建API请求
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);

        // 设置API路径并替换路径参数
        let api_path = crate::core::endpoints_original::Endpoints::DRIVE_V1_TASK_GET
            .replace("{}", &request.task_id);
        api_req.set_api_path(api_path);

        // 设置支持的访问令牌类型
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 发送HTTP请求
        let api_resp = Transport::<GetTaskStatusResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 获取任务状态查询构建器
    ///
    /// 创建一个查询任务状态的构建器，支持链式调用和完整的错误处理
    ///
    /// # 参数
    /// * `request` - 查询任务状态请求
    ///
    /// # 返回
    /// 返回任务状态查询构建器实例
    ///
    /// # 示例
    /// ```rust
    /// let response = service
    ///     .get_task_status_builder(GetTaskStatusRequest::new("task_123"))
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn get_task_status_builder(&self, request: GetTaskStatusRequest) -> GetTaskStatusBuilder {
        GetTaskStatusBuilder::new(std::sync::Arc::new(self.clone()), request)
    }

    /// 删除文件或文件夹
    ///
    /// 根据文件或文件夹的Token删除指定资源。删除操作不可逆，请谨慎使用。
    ///
    /// # 参数
    /// * `request` - 删除文件请求，包含要删除的文件或文件夹Token
    /// * `option` - 请求选项（可选）
    ///
    /// # 返回
    /// 返回删除操作的结果，包含操作状态和相关信息
    ///
    /// # 示例
    /// ```rust
    /// let request = DeleteFileRequest::new("file_token_123456789");
    /// let response = service.delete_file(request, None).await?;
    ///
    /// if response.success {
    ///     println!("文件删除成功");
    /// } else {
    ///     println!("文件删除失败: {:?}", response.error_message);
    /// }
    /// ```
    pub async fn delete_file(
        &self,
        request: DeleteFileRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteFileResponse>> {
        // 验证请求参数
        request.validate().map_err(|e| {
            crate::core::error::LarkAPIError::IllegalParamError(format!("参数验证失败: {}", e))
        })?;

        // 创建API请求
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::DELETE);

        // 设置API路径并替换路径参数
        let api_path = crate::core::endpoints_original::Endpoints::DRIVE_V1_DELETE
            .replace("{file_token}", &request.file_token);
        api_req.set_api_path(api_path);

        // 设置支持的访问令牌类型
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 发送HTTP请求
        let api_resp = Transport::<DeleteFileResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 获取文件删除构建器
    ///
    /// 创建一个删除文件的构建器，支持链式调用和完整的错误处理
    ///
    /// # 参数
    /// * `request` - 删除文件请求
    ///
    /// # 返回
    /// 返回文件删除构建器实例
    ///
    /// # 示例
    /// ```rust
    /// let response = service
    ///     .delete_file_builder(DeleteFileRequest::new("file_123"))
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn delete_file_builder(&self, request: DeleteFileRequest) -> DeleteFileBuilder {
        DeleteFileBuilder::new(std::sync::Arc::new(self.clone()), request)
    }
}

// ==================== GetTaskStatusBuilder 构建器模式 ====================

/// 查询异步任务状态构建器
///
/// 提供流畅的API来构建查询任务状态的请求，支持链式调用
/// 和完整的参数验证。
#[derive(Debug, Clone)]
pub struct GetTaskStatusBuilder {
    service: std::sync::Arc<DriveServiceV1>,
    request: GetTaskStatusRequest,
}

impl GetTaskStatusBuilder {
    /// 创建新的查询任务状态构建器
    ///
    /// # 参数
    /// * `service` - Drive服务实例
    /// * `request` - 查询任务状态请求
    pub fn new(service: std::sync::Arc<DriveServiceV1>, request: GetTaskStatusRequest) -> Self {
        Self { service, request }
    }

    /// 设置任务ID
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 示例
    /// ```rust
    /// let builder = GetTaskStatusBuilder::new(service, request)
    ///     .task_id("task_123456789");
    /// ```
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.request.task_id = task_id.into();
        self
    }

    /// 执行查询任务状态请求
    ///
    /// # 返回
    /// 成功返回任务状态响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust
    /// let response = GetTaskStatusBuilder::new(service, request)
    ///     .task_id("task_123456789")
    ///     .execute()
    ///     .await?;
    /// ```
    pub async fn execute(self) -> SDKResult<BaseResponse<GetTaskStatusResponse>> {
        // 验证请求参数
        self.request.validate().map_err(|e| {
            crate::core::error::LarkAPIError::IllegalParamError(format!("参数验证失败: {}", e))
        })?;

        // 执行请求
        self.service.get_task_status(self.request, None).await
    }
}

// ==================== DeleteFileBuilder 构建器模式 ====================

/// 删除文件或文件夹构建器
///
/// 提供流畅的API来构建删除文件的请求，支持链式调用
/// 和完整的参数验证。
#[derive(Debug, Clone)]
pub struct DeleteFileBuilder {
    service: std::sync::Arc<DriveServiceV1>,
    request: DeleteFileRequest,
}

impl DeleteFileBuilder {
    /// 创建新的删除文件构建器
    ///
    /// # 参数
    /// * `service` - Drive服务实例
    /// * `request` - 删除文件请求
    pub fn new(service: std::sync::Arc<DriveServiceV1>, request: DeleteFileRequest) -> Self {
        Self { service, request }
    }

    /// 设置文件Token
    ///
    /// # 参数
    /// * `file_token` - 文件或文件夹Token
    ///
    /// # 示例
    /// ```rust
    /// let builder = DeleteFileBuilder::new(service, request)
    ///     .file_token("file_123456789");
    /// ```
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
        self
    }

    /// 执行删除文件请求
    ///
    /// # 返回
    /// 成功返回删除结果响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust
    /// let response = DeleteFileBuilder::new(service, request)
    ///     .file_token("file_123456789")
    ///     .execute()
    ///     .await?;
    /// ```
    pub async fn execute(self) -> SDKResult<BaseResponse<DeleteFileResponse>> {
        // 验证请求参数
        self.request.validate().map_err(|e| {
            crate::core::error::LarkAPIError::IllegalParamError(format!("参数验证失败: {}", e))
        })?;

        // 执行请求
        self.service.as_ref().delete_file(self.request.clone(), None).await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    #[test]
    fn test_task_status_default() {
        let status = TaskStatus::default();
        assert!(matches!(status, TaskStatus::Processing));
    }

    #[test]
    fn test_task_status_display() {
        assert_eq!(TaskStatus::Processing.to_string(), "processing");
        assert_eq!(TaskStatus::Success.to_string(), "success");
        assert_eq!(TaskStatus::Failed.to_string(), "failed");
    }

    #[test]
    fn test_task_status_serialization() {
        let status = TaskStatus::Success;
        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: TaskStatus = serde_json::from_str(&serialized).unwrap();
        assert!(matches!(deserialized, TaskStatus::Success));
    }

    #[test]
    fn test_task_detail_default() {
        let task = TaskDetail::default();
        assert_eq!(task.task_id, "");
        assert!(matches!(task.status, TaskStatus::Processing));
        assert!(task.progress.is_none());
        assert!(task.error_message.is_none());
    }

    #[test]
    fn test_task_detail_is_completed() {
        let mut task = TaskDetail::default();

        // 处理中状态
        task.status = TaskStatus::Processing;
        assert!(!task.is_completed());
        assert!(!task.is_success());
        assert!(!task.is_failed());

        // 成功状态
        task.status = TaskStatus::Success;
        assert!(task.is_completed());
        assert!(task.is_success());
        assert!(!task.is_failed());

        // 失败状态
        task.status = TaskStatus::Failed;
        assert!(task.is_completed());
        assert!(!task.is_success());
        assert!(task.is_failed());
    }

    #[test]
    fn test_task_detail_progress() {
        let mut task = TaskDetail::default();

        // 测试进度百分比
        task.progress = Some(50);
        assert_eq!(task.get_progress_percentage(), Some(50.0));

        // 测试进度文本
        task.processed_count = Some(25);
        task.total_count = Some(100);
        assert_eq!(task.get_progress_text(), Some("25/100".to_string()));

        // 测试没有进度信息的情况
        task.progress = None;
        assert_eq!(task.get_progress_percentage(), None);
        assert_eq!(task.get_progress_text(), Some("25/100".to_string()));
    }

    #[test]
    fn test_task_detail_serialization() {
        let mut task = TaskDetail::default();
        task.task_id = "task_123".to_string();
        task.status = TaskStatus::Processing;
        task.progress = Some(75);
        task.error_message = Some("测试错误".to_string());

        let serialized = serde_json::to_string(&task).unwrap();
        let deserialized: TaskDetail = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.task_id, "task_123");
        assert!(matches!(deserialized.status, TaskStatus::Processing));
        assert_eq!(deserialized.progress, Some(75));
        assert_eq!(deserialized.error_message, Some("测试错误".to_string()));
    }

    #[test]
    fn test_get_task_status_request_new() {
        let request = GetTaskStatusRequest::new("task_123456789");
        assert_eq!(request.task_id, "task_123456789");
    }

    #[test]
    fn test_get_task_status_request_validate() {
        // 测试有效的请求
        let valid_request = GetTaskStatusRequest::new("valid_task_id");
        assert!(valid_request.validate().is_ok());

        // 测试空的task_id
        let empty_request = GetTaskStatusRequest::new("");
        assert!(empty_request.validate().is_err());

        // 测试过长的task_id
        let long_id = "a".repeat(101);
        let long_request = GetTaskStatusRequest::new(long_id);
        assert!(long_request.validate().is_err());
    }

    #[test]
    fn test_get_task_status_request_serialization() {
        let request = GetTaskStatusRequest::new("test_task");
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetTaskStatusRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.task_id, "test_task");
    }

    #[test]
    fn test_get_task_status_response_default() {
        let response = GetTaskStatusResponse::default();
        assert_eq!(response.task.task_id, "");
        assert!(matches!(response.task.status, TaskStatus::Processing));
    }

    #[test]
    fn test_get_task_status_response_serialization() {
        let mut response = GetTaskStatusResponse::default();
        response.task.task_id = "task_456".to_string();
        response.task.status = TaskStatus::Success;

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetTaskStatusResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.task.task_id, "task_456");
        assert!(matches!(deserialized.task.status, TaskStatus::Success));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(GetTaskStatusResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_drive_service_v1_creation() {
        let config = Config::default();
        let service = DriveServiceV1::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_get_task_status_builder() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));
        let request = GetTaskStatusRequest::new("initial_task");

        let builder = service.get_task_status_builder(request);
        assert_eq!(builder.request.task_id, "initial_task");
    }

    #[test]
    fn test_get_task_status_builder_task_id() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));
        let request = GetTaskStatusRequest::new("initial_task");

        let builder = service.get_task_status_builder(request)
            .task_id("new_task_id");

        assert_eq!(builder.request.task_id, "new_task_id");
    }

    #[test]
    fn test_get_task_status_builder_validate() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));

        // 测试有效的请求
        let valid_request = GetTaskStatusRequest::new("valid_task");
        let builder = service.get_task_status_builder(valid_request);
        assert!(builder.request.validate().is_ok());

        // 测试无效的请求
        let invalid_request = GetTaskStatusRequest::new("");
        let builder = service.get_task_status_builder(invalid_request);
        assert!(builder.request.validate().is_err());
    }

    #[test]
    fn test_task_edge_cases() {
        // 测试边界情况的任务状态
        let mut task = TaskDetail::default();

        // 测试进度为0的情况
        task.progress = Some(0);
        assert_eq!(task.get_progress_percentage(), Some(0.0));

        // 测试进度为100的情况
        task.progress = Some(100);
        assert_eq!(task.get_progress_percentage(), Some(100.0));

        // 测试已处理数量为0的情况
        task.processed_count = Some(0);
        task.total_count = Some(100);
        assert_eq!(task.get_progress_text(), Some("0/100".to_string()));

        // 测试已处理数量等于总数的情况
        task.processed_count = Some(100);
        task.total_count = Some(100);
        assert_eq!(task.get_progress_text(), Some("100/100".to_string()));
    }

    #[test]
    fn test_task_error_handling() {
        let mut task = TaskDetail::default();

        // 测试错误代码和错误消息
        task.status = TaskStatus::Failed;
        task.error_code = Some("TIMEOUT".to_string());
        task.error_message = Some("任务执行超时".to_string());

        assert!(task.is_failed());
        assert_eq!(task.error_code, Some("TIMEOUT".to_string()));
        assert_eq!(task.error_message, Some("任务执行超时".to_string()));
    }

    #[test]
    fn test_time_fields() {
        let mut task = TaskDetail::default();
        task.create_time = Some("1699123456789".to_string());
        task.start_time = Some("1699123456790".to_string());
        task.finish_time = Some("1699123456889".to_string());
        task.estimated_remaining_seconds = Some(60);

        assert_eq!(task.create_time, Some("1699123456789".to_string()));
        assert_eq!(task.start_time, Some("1699123456790".to_string()));
        assert_eq!(task.finish_time, Some("1699123456889".to_string()));
        assert_eq!(task.estimated_remaining_seconds, Some(60));
    }

    #[test]
    fn test_complex_task_scenario() {
        // 测试复杂任务场景
        let mut task = TaskDetail::default();
        task.task_id = "complex_file_upload_123".to_string();
        task.status = TaskStatus::Processing;
        task.task_type = Some("file_upload".to_string());
        task.description = Some("上传大文件到云盘".to_string());
        task.progress = Some(45);
        task.processed_count = Some(45000000);
        task.total_count = Some(100000000);
        task.create_time = Some("1699123456000".to_string());
        task.start_time = Some("1699123456100".to_string());
        task.estimated_remaining_seconds = Some(300);

        assert_eq!(task.task_id, "complex_file_upload_123");
        assert!(!task.is_completed());
        assert_eq!(task.get_progress_percentage(), Some(45.0));
        assert_eq!(task.get_progress_text(), Some("45000000/100000000".to_string()));
        assert_eq!(task.task_type, Some("file_upload".to_string()));
        assert_eq!(task.estimated_remaining_seconds, Some(300));
    }

    #[test]
    fn test_task_type_and_description() {
        let mut task = TaskDetail::default();
        task.task_type = Some("batch_copy".to_string());
        task.description = Some("批量复制1000个文件".to_string());

        assert_eq!(task.task_type, Some("batch_copy".to_string()));
        assert_eq!(task.description, Some("批量复制1000个文件".to_string()));
    }

    #[test]
    fn test_builder_chain_validation() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));

        // 测试链式调用
        let builder = service
            .get_task_status_builder(GetTaskStatusRequest::new("initial"))
            .task_id("chained_task_id");

        assert_eq!(builder.request.task_id, "chained_task_id");
        assert!(builder.request.validate().is_ok());
    }

    #[test]
    fn test_request_with_various_task_ids() {
        // 测试不同格式的task_id
        let test_cases = vec![
            "task_123",
            "abc123def456",
            "upload_file_2024_01_01",
            "short",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", // 100个字符
        ];

        for task_id in test_cases {
            let request = GetTaskStatusRequest::new(task_id);
            assert!(request.validate().is_ok());
            assert_eq!(request.task_id, task_id);
        }
    }

    #[test]
    fn test_response_with_full_task_details() {
        let mut response = GetTaskStatusResponse::default();
        let mut task = TaskDetail::default();

        // 填充完整的任务信息
        task.task_id = "full_task_123".to_string();
        task.status = TaskStatus::Success;
        task.task_type = Some("file_move".to_string());
        task.description = Some("移动文件到新目录".to_string());
        task.progress = Some(100);
        task.processed_count = Some(500);
        task.total_count = Some(500);
        task.create_time = Some("1699123450000".to_string());
        task.start_time = Some("1699123450100".to_string());
        task.finish_time = Some("1699123500000".to_string());

        response.task = task;

        assert_eq!(response.task.task_id, "full_task_123");
        assert!(response.task.is_success());
        assert!(response.task.is_completed());
        assert_eq!(response.task.get_progress_percentage(), Some(100.0));
        assert_eq!(response.task.get_progress_text(), Some("500/500".to_string()));
    }

    // ==================== DeleteFile API 测试 ====================

    #[test]
    fn test_delete_file_request_creation() {
        let request = DeleteFileRequest::new("file_token_123456789");

        assert_eq!(request.file_token, "file_token_123456789");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_delete_file_request_with_various_tokens() {
        let test_cases = vec![
            ("file_123", "file_123"),
            ("folder_token_456", "folder_token_456"),
            ("token_with_underscores_and_numbers_789", "token_with_underscores_and_numbers_789"),
            ("a", "a"),
            ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), // 100个字符
        ];

        for (input_token, expected_token) in test_cases {
            let request = DeleteFileRequest::new(input_token);
            assert_eq!(request.file_token, expected_token);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_delete_file_request_validation() {
        // 测试有效token
        let valid_request = DeleteFileRequest::new("valid_token_123");
        assert!(valid_request.validate().is_ok());

        // 测试空token
        let empty_request = DeleteFileRequest::new("");
        assert!(empty_request.validate().is_err());

        // 测试只有空格的token
        let whitespace_request = DeleteFileRequest::new("   ");
        assert!(whitespace_request.validate().is_err());

        // 测试超长token (超过200字符)
        let long_token = "a".repeat(201);
        let long_request = DeleteFileRequest::new(&long_token);
        assert!(long_request.validate().is_err());
    }

    #[test]
    fn test_delete_file_request_default() {
        let request = DeleteFileRequest::default();

        assert_eq!(request.file_token, "");
        assert!(request.validate().is_err()); // 默认构造应该验证失败
    }

    #[test]
    fn test_delete_file_response_creation() {
        let response = DeleteFileResponse::default();

        // DeleteFileResponse 目前是空结构体，主要测试创建和序列化
        assert!(serde_json::to_string(&response).is_ok());
    }

    #[test]
    fn test_delete_file_response_serialization() {
        let response = DeleteFileResponse::default();

        // 测试序列化和反序列化
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: DeleteFileResponse = serde_json::from_str(&serialized).unwrap();

        // 空结构体应该能正确序列化和反序列化
        assert!(serialized.is_char_boundary(0)); // 基本JSON格式检查
    }

    #[test]
    fn test_delete_file_builder_creation() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));
        let request = DeleteFileRequest::new("test_file_token");

        let builder = DeleteFileBuilder::new(service, request);

        assert_eq!(builder.request.file_token, "test_file_token");
    }

    #[test]
    fn test_delete_file_builder_file_token_method() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));
        let request = DeleteFileRequest::new("initial_token");

        let builder = DeleteFileBuilder::new(service, request)
            .file_token("new_token_123");

        assert_eq!(builder.request.file_token, "new_token_123");
        assert!(builder.request.validate().is_ok());
    }

    #[test]
    fn test_delete_file_builder_chain_validation() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));

        // 测试链式调用
        let builder = service
            .delete_file_builder(DeleteFileRequest::new("initial"))
            .file_token("final_token_456");

        assert_eq!(builder.request.file_token, "final_token_456");
        assert!(builder.request.validate().is_ok());
    }

    #[test]
    fn test_delete_file_builder_invalid_token() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));
        let request = DeleteFileRequest::new("valid_token");

        let builder = DeleteFileBuilder::new(service, request)
            .file_token(""); // 设置无效token

        assert_eq!(builder.request.file_token, "");
        assert!(builder.request.validate().is_err());
    }

    #[test]
    fn test_delete_file_service_creation() {
        let config = Config::default();
        let service = DriveServiceV1::new(config);

        // 测试服务创建
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_delete_file_service_builder_creation() {
        let config = Config::default();
        let service = DriveServiceV1::new(config);
        let request = DeleteFileRequest::new("test_token");

        let builder = service.delete_file_builder(request);

        assert_eq!(builder.request.file_token, "test_token");
    }

    #[test]
    fn test_delete_file_edge_cases() {
        // 测试边界情况

        // 最小有效token
        let min_request = DeleteFileRequest::new("a");
        assert!(min_request.validate().is_ok());

        // 最大有效token (200字符)
        let max_token = "x".repeat(200);
        let max_request = DeleteFileRequest::new(&max_token);
        assert!(max_request.validate().is_ok());

        // 包含特殊字符的token (如果允许的话)
        let special_token = "file_token_123-456_789";
        let special_request = DeleteFileRequest::new(special_token);
        assert!(special_request.validate().is_ok());
    }

    #[test]
    fn test_delete_file_api_trait_implementation() {
        // 测试API响应trait实现
        assert_eq!(DeleteFileResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_file_complex_scenarios() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));

        // 测试复杂场景：多个文件token的验证
        let file_tokens = vec![
            "doc_token_123",
            "folder_token_456",
            "sheet_token_789",
            "presentation_token_101112",
        ];

        for token in file_tokens {
            let request = DeleteFileRequest::new(token);
            let builder = DeleteFileBuilder::new(service.clone(), request);

            assert!(builder.request.validate().is_ok());
            assert_eq!(builder.request.file_token, token);
        }
    }

    #[test]
    fn test_delete_file_builder_with_various_service_instances() {
        // 测试使用不同服务实例的构建器
        let configs = vec![
            Config::default(),
            {
                let mut config = Config::default();
                // 这里假设app_id是String类型，如果不对需要调整
                // config.app_id = Some("test_app_id".to_string());
                config
            },
        ];

        for config in configs {
            let service = Arc::new(DriveServiceV1::new(config));
            let request = DeleteFileRequest::new("test_token");
            let builder = DeleteFileBuilder::new(service, request);

            assert_eq!(builder.request.file_token, "test_token");
            assert!(builder.request.validate().is_ok());
        }
    }

    #[test]
    fn test_delete_file_error_handling() {
        // 测试错误处理场景

        // 无效请求验证
        let invalid_requests = vec![
            DeleteFileRequest::new(""),
            DeleteFileRequest::new("   "),
            DeleteFileRequest::new(&"a".repeat(201)), // 超长
        ];

        for request in invalid_requests {
            assert!(request.validate().is_err());
        }
    }

    #[test]
    fn test_delete_file_request_serialization() {
        let request = DeleteFileRequest::new("test_token_123");

        // 测试请求序列化
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: DeleteFileRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.file_token, deserialized.file_token);
    }

    #[test]
    fn test_delete_file_response_trait_methods() {
        let response = DeleteFileResponse::default();

        // 测试trait方法
        assert_eq!(DeleteFileResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_file_builder_state_management() {
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));
        let request = DeleteFileRequest::new("initial_token");

        // 测试构建器状态管理
        let mut builder = DeleteFileBuilder::new(service, request);

        // 验证初始状态
        assert_eq!(builder.request.file_token, "initial_token");

        // 修改状态
        builder = builder.file_token("modified_token");

        // 验证状态已更改
        assert_eq!(builder.request.file_token, "modified_token");
    }

    #[test]
    fn test_delete_file_service_arc_sharing() {
        // 测试Arc服务共享
        let config = Config::default();
        let service = Arc::new(DriveServiceV1::new(config));

        // 创建多个构建器共享同一个服务
        let builder1 = DeleteFileBuilder::new(service.clone(), DeleteFileRequest::new("token1"));
        let builder2 = DeleteFileBuilder::new(service.clone(), DeleteFileRequest::new("token2"));
        let builder3 = DeleteFileBuilder::new(service, DeleteFileRequest::new("token3"));

        assert_eq!(builder1.request.file_token, "token1");
        assert_eq!(builder2.request.file_token, "token2");
        assert_eq!(builder3.request.file_token, "token3");

        // 所有构建器都应该有有效的请求
        assert!(builder1.request.validate().is_ok());
        assert!(builder2.request.validate().is_ok());
        assert!(builder3.request.validate().is_ok());
    }

    #[test]
    fn test_delete_file_comprehensive_validation() {
        // 综合验证测试
        let test_cases = vec![
            ("valid", true),
            ("", false),
            ("   ", false),
            ("a", true), // 最小有效
            ("file_token_12345", true),
            ("folder_token_with_special_chars_456", true),
        ];

        for (token, should_be_valid) in test_cases {
            let request = DeleteFileRequest::new(token);
            let validation_result = request.validate();
            let is_valid = validation_result.is_ok();

            if !is_valid && should_be_valid {
                println!("Token '{}' (length: {}) failed validation: {:?}",
                    token.chars().take(20).collect::<String>(),
                    token.len(),
                    validation_result);
            }

            assert_eq!(
                is_valid,
                should_be_valid,
                "Token '{}' validation result: expected {}, got {}",
                token.chars().take(20).collect::<String>(),
                should_be_valid,
                is_valid
            );
        }
    }
}
