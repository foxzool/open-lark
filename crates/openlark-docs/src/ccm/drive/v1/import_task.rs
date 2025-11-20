use openlark_core::config::Config;
use openlark_core::error::SDKError;
use crate::response::SDKResult;
use crate::service_trait::Service;
use crate::transport::Transport;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建导入任务API端点
pub const ENDPOINT_CREATE_IMPORT_TASK: &str = "/open-apis/drive/v1/import_tasks";

/// 创建导入任务请求体
///
/// 提供文件导入功能，支持将外部文件导入到飞书云文档
/// 支持多种文件格式和导入选项配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskRequest {
    /// 文件类型
    /// 支持的文件类型：doc、docx、pdf、xlsx、csv、txt、md等
    pub file_type: String,
    /// 文件名称
    /// 包含文件扩展名的完整文件名
    pub file_name: String,
    /// 文件URL
    /// 公开可访问的文件URL，支持HTTP/HTTPS协议
    pub file_url: String,
    /// 父文件夹令牌
    /// 目标文件夹的令牌，用于指定导入位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_token: Option<String>,
    /// 覆盖模式
    /// true：覆盖同名文件，false：重命名文件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}

impl CreateImportTaskRequest {
    /// 创建新的导入任务请求
    ///
    /// # 参数
    /// * `file_type` - 文件类型
    /// * `file_name` - 文件名称
    /// * `file_url` - 文件URL
    pub fn new(file_type: impl Into<String>, file_name: impl Into<String>, file_url: impl Into<String>) -> Self {
        Self {
            file_type: file_type.into(),
            file_name: file_name.into(),
            file_url: file_url.into(),
            parent_folder_token: None,
            overwrite: None,
        }
    }

    /// 创建导入任务请求的构建器
    pub fn builder() -> CreateImportTaskBuilder {
        CreateImportTaskBuilder::default()
    }
}

/// 创建导入任务请求构建器
///
/// 提供流畅的API来构建导入任务请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct CreateImportTaskBuilder {
    file_type: Option<String>,
    file_name: Option<String>,
    file_url: Option<String>,
    parent_folder_token: Option<String>,
    overwrite: Option<bool>,
}

impl CreateImportTaskBuilder {
    /// 设置文件类型
    ///
    /// # 参数
    /// * `file_type` - 文件类型
    ///
    /// # 示例
    /// ```rust
    /// let builder = CreateImportTaskBuilder::default()
    ///     .file_type("docx");
    /// ```
    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    /// 设置文件名称
    ///
    /// # 参数
    /// * `file_name` - 文件名称
    ///
    /// # 示例
    /// ```rust
    /// let builder = CreateImportTaskBuilder::default()
    ///     .file_name("document.docx");
    /// ```
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 设置文件URL
    ///
    /// # 参数
    /// * `file_url` - 文件URL
    ///
    /// # 示例
    /// ```rust
    /// let builder = CreateImportTaskBuilder::default()
    ///     .file_url("https://example.com/file.docx");
    /// ```
    pub fn file_url(mut self, file_url: impl Into<String>) -> Self {
        self.file_url = Some(file_url.into());
        self
    }

    /// 设置父文件夹令牌
    ///
    /// # 参数
    /// * `parent_folder_token` - 父文件夹令牌
    ///
    /// # 示例
    /// ```rust
    /// let builder = CreateImportTaskBuilder::default()
    ///     .parent_folder_token("folder_token_123");
    /// ```
    pub fn parent_folder_token(mut self, parent_folder_token: impl Into<String>) -> Self {
        self.parent_folder_token = Some(parent_folder_token.into());
        self
    }

    /// 设置覆盖模式
    ///
    /// # 参数
    /// * `overwrite` - 是否覆盖同名文件
    ///
    /// # 示例
    /// ```rust
    /// let builder = CreateImportTaskBuilder::default()
    ///     .overwrite(true);
    /// ```
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    /// 构建导入任务请求
    ///
    /// # 返回
    /// 成功返回导入任务请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果文件类型为空，返回错误
    /// * 如果文件名称为空，返回错误
    /// * 如果文件URL为空，返回错误
    /// * 如果文件类型不支持，返回错误
    /// * 如果文件URL格式无效，返回错误
    pub fn build(self) -> SDKResult<CreateImportTaskRequest> {
        // 验证必填参数
        if let Some(ref file_type) = self.file_type {
            if file_type.is_empty() {
                return Err(SDKError::ValidationError("文件类型不能为空".to_string()));
            }

            // 验证支持的文件类型
            let supported_types = [
                "doc", "docx", "pdf", "txt", "md", "csv", "xlsx", "xls", "ppt", "pptx",
                "png", "jpg", "jpeg", "gif", "bmp", "svg", "mp3", "mp4", "avi", "mov",
                "zip", "rar", "7z", "tar", "gz"
            ];
            if !supported_types.contains(&file_type.as_str()) {
                return Err(SDKError::ValidationError(
                    format!("不支持的文件类型: {}", file_type),
                ));
            }
        } else {
            return Err(SDKError::ValidationError("文件类型不能为空".to_string()));
        }

        if let Some(ref file_name) = self.file_name {
            if file_name.is_empty() {
                return Err(SDKError::ValidationError("文件名称不能为空".to_string()));
            }
        } else {
            return Err(SDKError::ValidationError("文件名称不能为空".to_string()));
        }

        if let Some(ref file_url) = self.file_url {
            if file_url.is_empty() {
                return Err(SDKError::ValidationError("文件URL不能为空".to_string()));
            }

            // 验证URL格式
            if !file_url.starts_with("http://") && !file_url.starts_with("https://") {
                return Err(SDKError::ValidationError(
                    "文件URL必须以http://或https://开头".to_string(),
                ));
            }
        } else {
            return Err(SDKError::ValidationError("文件URL不能为空".to_string()));
        }

        Ok(CreateImportTaskRequest {
            file_type: self.file_type.unwrap(),
            file_name: self.file_name.unwrap(),
            file_url: self.file_url.unwrap(),
            parent_folder_token: self.parent_folder_token,
            overwrite: self.overwrite,
        })
    }
}

/// 创建导入任务响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskResponse {
    /// 任务ID
    /// 导入任务的唯一标识符，用于查询任务状态
    pub task_id: String,
    /// 任务状态
    /// pending: 待处理，processing: 处理中，completed: 已完成，failed: 失败
    pub status: String,
    /// 文件令牌
    /// 导入成功后的文件令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    /// 错误信息
    /// 任务失败时的错误描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 创建时间
    /// 任务创建的时间戳（毫秒）
    pub created_time: u64,
    /// 完成时间
    /// 任务完成的时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<u64>,
}

impl CreateImportTaskResponse {
    /// 创建新的导入任务响应
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    /// * `status` - 任务状态
    /// * `created_time` - 创建时间
    pub fn new(task_id: String, status: String, created_time: u64) -> Self {
        Self {
            task_id,
            status,
            file_token: None,
            error_message: None,
            created_time,
            completed_time: None,
        }
    }

    /// 获取任务ID
    ///
    /// # 返回
    /// 任务ID字符串
    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    /// 获取任务状态
    ///
    /// # 返回
    /// 任务状态字符串
    pub fn status(&self) -> &str {
        &self.status
    }

    /// 获取文件令牌
    ///
    /// # 返回
    /// 文件令牌，如果任务未完成则返回None
    pub fn file_token(&self) -> Option<&str> {
        self.file_token.as_deref()
    }

    /// 获取错误信息
    ///
    /// # 返回
    /// 错误信息，如果任务成功则返回None
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// 获取创建时间
    ///
    /// # 返回
    /// 创建时间戳（毫秒）
    pub fn created_time(&self) -> u64 {
        self.created_time
    }

    /// 获取完成时间
    ///
    /// # 返回
    /// 完成时间戳（毫秒），如果任务未完成则返回None
    pub fn completed_time(&self) -> Option<u64> {
        self.completed_time
    }

    /// 检查任务是否完成
    ///
    /// # 返回
    /// true表示任务已完成，false表示任务正在进行或失败
    pub fn is_completed(&self) -> bool {
        self.status == "completed"
    }

    /// 检查任务是否失败
    ///
    /// # 返回
    /// true表示任务失败，false表示任务成功或正在进行
    pub fn is_failed(&self) -> bool {
        self.status == "failed"
    }

    /// 检查任务是否正在进行
    ///
    /// # 返回
    /// true表示任务正在进行，false表示任务已完成或失败
    pub fn is_processing(&self) -> bool {
        self.status == "processing"
    }

    /// 检查任务是否待处理
    ///
    /// # 返回
    /// true表示任务待处理，false表示任务已开始处理
    pub fn is_pending(&self) -> bool {
        self.status == "pending"
    }
}

/// 创建导入任务构建器
///
/// 提供流畅的API来创建导入任务，支持方法链调用和完整的错误处理
#[derive(Clone, Debug)]
pub struct CreateImportTaskBuilder {
    service: Arc<DriveServiceV1>,
    request: CreateImportTaskRequest,
}

impl CreateImportTaskBuilder {
    /// 创建新的导入任务构建器
    ///
    /// # 参数
    /// * `service` - 云盘服务实例
    /// * `request` - 导入任务请求
    pub(crate) fn new(service: Arc<DriveServiceV1>, request: CreateImportTaskRequest) -> Self {
        Self { service, request }
    }

    /// 执行导入任务创建操作
    ///
    /// 向飞书API发送POST请求来创建文件导入任务
    ///
    /// # 返回
    /// * `Ok(CreateImportTaskResponse)` - 创建成功，返回任务信息
    /// * `Err(SDKError)` - 创建失败，返回错误信息
    ///
    /// # 错误类型
    /// * `SDKError::NetworkError` - 网络请求失败
    /// * `SDKError::ApiError` - API调用失败，包含错误码和消息
    /// * `SDKError::SerializationError` - 响应序列化失败
    /// * `SDKError::AuthenticationError` - 身份验证失败
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::drive::v1::import_task::{CreateImportTaskRequest, CreateImportTaskResponse};
    ///
    /// async fn import_file_example(
    ///     service: Arc<DriveServiceV1>,
    /// ) -> Result<CreateImportTaskResponse, Box<dyn std::error::Error>> {
    ///     let request = CreateImportTaskRequest::builder()
    ///         .file_type("docx")
    ///         .file_name("重要文档.docx")
    ///         .file_url("https://example.com/document.docx")
    ///         .parent_folder_token("folder_token_123")
    ///         .overwrite(true)
    ///         .build()?;
    ///
    ///     let response = service
    ///         .create_import_task_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("导入任务创建成功，任务ID: {}", response.task_id());
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<CreateImportTaskResponse> {
        let url = self.service.config().build_url(ENDPOINT_CREATE_IMPORT_TASK);

        // 构建请求体
        let body = serde_json::to_value(&self.request)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        // 发送HTTP POST请求
        let response = self
            .service
            .transport()
            .post(&url, Some(body))
            .await?;

        // 解析响应体
        let response_data: CreateImportTaskResponse = serde_json::from_value(response)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        Ok(response_data)
    }
}

// 云盘服务的占位符，实际应该在主服务文件中定义
pub struct DriveServiceV1 {
    config: Config,
}

impl Service for DriveServiceV1 {
    fn config(&self) -> &Config {
        &self.config
    }

    fn transport(&self) -> &dyn Transport {
        todo!("Transport implementation needed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_import_task_request_builder() {
        // 测试正常构建
        let request = CreateImportTaskRequest::builder()
            .file_type("docx")
            .file_name("测试文档.docx")
            .file_url("https://example.com/document.docx")
            .parent_folder_token("folder_token_123")
            .overwrite(true)
            .build()
            .unwrap();

        assert_eq!(request.file_type, "docx");
        assert_eq!(request.file_name, "测试文档.docx");
        assert_eq!(request.file_url, "https://example.com/document.docx");
        assert_eq!(request.parent_folder_token, Some("folder_token_123".to_string()));
        assert_eq!(request.overwrite, Some(true));
    }

    #[test]
    fn test_create_import_task_request_builder_minimal() {
        let request = CreateImportTaskRequest::builder()
            .file_type("pdf")
            .file_name("report.pdf")
            .file_url("https://example.com/report.pdf")
            .build()
            .unwrap();

        assert_eq!(request.file_type, "pdf");
        assert_eq!(request.file_name, "report.pdf");
        assert_eq!(request.file_url, "https://example.com/report.pdf");
        assert_eq!(request.parent_folder_token, None);
        assert_eq!(request.overwrite, None);
    }

    #[test]
    fn test_create_import_task_request_validation() {
        // 测试文件类型为空
        let result = CreateImportTaskRequest::builder()
            .file_type("")
            .file_name("test.docx")
            .file_url("https://example.com/test.docx")
            .build();
        assert!(result.is_err());

        // 测试文件名称为空
        let result = CreateImportTaskRequest::builder()
            .file_type("docx")
            .file_name("")
            .file_url("https://example.com/test.docx")
            .build();
        assert!(result.is_err());

        // 测试文件URL为空
        let result = CreateImportTaskRequest::builder()
            .file_type("docx")
            .file_name("test.docx")
            .file_url("")
            .build();
        assert!(result.is_err());

        // 测试无效的文件类型
        let result = CreateImportTaskRequest::builder()
            .file_type("invalid_type")
            .file_name("test.invalid")
            .file_url("https://example.com/test.invalid")
            .build();
        assert!(result.is_err());

        // 测试无效的URL格式
        let result = CreateImportTaskRequest::builder()
            .file_type("docx")
            .file_name("test.docx")
            .file_url("ftp://example.com/test.docx")
            .build();
        assert!(result.is_err());

        // 测试不以http开头的URL
        let result = CreateImportTaskRequest::builder()
            .file_type("docx")
            .file_name("test.docx")
            .file_url("example.com/test.docx")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_import_task_request_supported_file_types() {
        let supported_types = [
            "doc", "docx", "pdf", "txt", "md", "csv", "xlsx", "xls", "ppt", "pptx",
            "png", "jpg", "jpeg", "gif", "bmp", "svg", "mp3", "mp4", "avi", "mov",
            "zip", "rar", "7z", "tar", "gz"
        ];

        for file_type in supported_types.iter() {
            let request = CreateImportTaskRequest::builder()
                .file_type(*file_type)
                .file_name(format!("test.{}", file_type))
                .file_url(format!("https://example.com/test.{}", file_type))
                .build()
                .unwrap();

            assert_eq!(request.file_type, *file_type);
        }
    }

    #[test]
    fn test_create_import_task_request_new() {
        let request = CreateImportTaskRequest::new(
            "xlsx",
            "数据表.xlsx",
            "https://example.com/data.xlsx",
        );

        assert_eq!(request.file_type, "xlsx");
        assert_eq!(request.file_name, "数据表.xlsx");
        assert_eq!(request.file_url, "https://example.com/data.xlsx");
        assert_eq!(request.parent_folder_token, None);
        assert_eq!(request.overwrite, None);
    }

    #[test]
    fn test_create_import_task_response() {
        let response = CreateImportResponse::new(
            "task_123".to_string(),
            "pending".to_string(),
            1704067200000,
        );

        assert_eq!(response.task_id(), "task_123");
        assert_eq!(response.status(), "pending");
        assert_eq!(response.created_time(), 1704067200000);
        assert_eq!(response.file_token(), None);
        assert_eq!(response.error_message(), None);
        assert_eq!(response.completed_time(), None);
        assert!(!response.is_completed());
        assert!(!response.is_failed());
        assert!(!response.is_processing());
        assert!(response.is_pending());
    }

    #[test]
    fn test_create_import_task_response_with_file_token() {
        let response = CreateImportResponse::new(
            "task_123".to_string(),
            "completed".to_string(),
            1704067200000,
        );
        response.file_token = Some("file_token_456".to_string());
        response.completed_time = Some(1704067201000);

        assert_eq!(response.task_id(), "task_123");
        assert_eq!(response.status(), "completed");
        assert_eq!(response.file_token(), Some("file_token_456"));
        assert_eq!(response.completed_time(), Some(1704067201000));
        assert!(response.is_completed());
        assert!(!response.is_failed());
        assert!(!response.is_processing());
        assert!(!response.is_pending());
    }

    #[test]
    fn test_create_import_task_response_with_error() {
        let response = CreateImportResponse::new(
            "task_123".to_string(),
            "failed".to_string(),
            1704067200000,
        );
        response.error_message = Some("文件格式不支持".to_string());

        assert_eq!(response.task_id(), "task_123");
        assert_eq!(response.status(), "failed");
        assert_eq!(response.error_message(), Some("文件格式不支持"));
        assert_eq!(response.file_token(), None);
        assert_eq!(response.completed_time(), None);
        assert!(!response.is_completed());
        assert!(response.is_failed());
        assert!(!response.is_processing());
        assert!(!response.is_pending());
    }

    #[test]
    fn test_multiple_valid_requests() {
        let test_cases = vec![
            // 最小请求
            ("txt", "note.txt", "https://example.com/note.txt", None, None),
            // 带父文件夹
            ("pdf", "报告.pdf", "https://example.com/report.pdf", Some("folder_parent"), None),
            // 覆盖模式
            ("docx", "文档.docx", "https://example.com/document.docx", None, Some(false)),
            // 完整请求
            ("xlsx", "数据表.xlsx", "https://example.com/data.xlsx", Some("folder_root"), Some(true)),
        ];

        for (file_type, file_name, file_url, parent_token, overwrite) in test_cases {
            let mut builder = CreateImportTaskRequest::builder()
                .file_type(file_type)
                .file_name(file_name)
                .file_url(file_url);

            if let Some(token) = parent_token {
                builder = builder.parent_folder_token(token);
            }

            if let Some(overwrite) = overwrite {
                builder = builder.overwrite(overwrite);
            }

            let request = builder.build().unwrap();
            assert_eq!(request.file_type, file_type);
            assert_eq!(request.file_name, file_name);
            assert_eq!(request.file_url, file_url);
            assert_eq!(request.parent_folder_token, parent_token.map(|s| s.to_string()));
            assert_eq!(request.overwrite, overwrite);
        }
    }

    #[test]
    fn test_create_import_task_request_serialization() {
        let request = CreateImportTaskRequest::builder()
            .file_type("docx")
            .file_name("重要文档.docx")
            .file_url("https://example.com/document.docx")
            .parent_folder_token("folder_token_123")
            .overwrite(true)
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: CreateImportTaskRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.file_type, request.file_type);
        assert_eq!(parsed.file_name, request.file_name);
        assert_eq!(parsed.file_url, request.file_url);
        assert_eq!(parsed.parent_folder_id, request.parent_folder_token);
        assert_eq!(parsed.overwrite, request.overwrite);
    }

    #[test]
    fn test_create_import_task_response_serialization() {
        let mut response = CreateImportResponse::new(
            "task_123".to_string(),
            "completed".to_string(),
            1704067200000,
        );
        response.file_token = Some("file_token_456".to_string());
        response.completed_time = Some(1704067201000);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: CreateImportResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.task_id, "task_123");
        assert_eq!(parsed.status, "completed");
        assert_eq!(parsed.file_token, Some("file_token_456"));
        assert_eq!(parsed.completed_time, Some(1704067201000));
    }

    #[test]
    fn test_url_validation() {
        // 测试有效的URL格式
        let valid_urls = [
            "http://example.com/file.docx",
            "https://example.com/file.pdf",
            "https://sub.example.com/path/to/file.xlsx",
            "http://192.168.1.1/document.docx",
        ];

        for url in valid_urls.iter() {
            let result = CreateImportTaskRequest::builder()
                .file_type("docx")
                .file_name("test.docx")
                .file_url(url)
                .build();

            assert!(result.is_ok(), "URL {} should be valid", url);
        }

        // 测试无效的URL格式
        let invalid_urls = [
            "ftp://example.com/file.docx",
            "file://example.com/file.docx",
            "example.com/file.docx",
            "://example.com/file.docx",
            "ws://example.com/file.docx",
        ];

        for url in invalid_urls.iter() {
            let result = CreateImportTaskRequest::builder()
                .file_type("docx")
                .file_name("test.docx")
                .file_url(url)
                .build();

            assert!(result.is_err(), "URL {} should be invalid", url);
        }
    }
}