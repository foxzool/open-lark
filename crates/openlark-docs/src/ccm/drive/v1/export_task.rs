use openlark_core::config::Config;
use openlark_core::error::SDKError;
use crate::response::SDKResult;
use crate::service_trait::Service;
use crate::transport::Transport;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建导出任务API端点
pub const ENDPOINT_CREATE_EXPORT_TASK: &str = "/open-apis/drive/v1/export_tasks";

/// 创建导出任务请求体
///
/// 提供文件导出功能，支持将飞书云文档导出为各种格式
/// 支持多种文件格式和导出选项配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExportTaskRequest {
    /// 文件令牌
    /// 要导出的文件或文件夹的令牌
    pub file_token: String,
    /// 导出格式
    /// 支持的导出格式：pdf、docx、xlsx、png、jpg等
    pub export_format: String,
    /// 文件类型
    /// 源文件的类型：doc、sheet、mindnote、file等
    pub file_type: String,
    /// 导出选项
    /// 可选的导出参数配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ExportOptions>,
}

/// 导出选项配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    /// 导出质量
    /// 用于图片导出：high、medium、low
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    /// 页面范围
    /// 用于文档导出：all、current、1-5等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_range: Option<String>,
    /// 是否包含注释
    /// true：包含注释，false：不包含注释
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_comments: Option<bool>,
    /// 水印设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<WatermarkSettings>,
}

/// 水印设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatermarkSettings {
    /// 水印文字
    pub text: String,
    /// 水印透明度
    /// 0.0-1.0之间的数值
    pub opacity: f32,
    /// 水印旋转角度
    /// 旋转角度，单位：度
    pub rotation: i32,
}

impl CreateExportTaskRequest {
    /// 创建新的导出任务请求
    ///
    /// # 参数
    /// * `file_token` - 文件令牌
    /// * `export_format` - 导出格式
    /// * `file_type` - 文件类型
    pub fn new(file_token: impl Into<String>, export_format: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            export_format: export_format.into(),
            file_type: file_type.into(),
            options: None,
        }
    }

    /// 创建导出任务请求的构建器
    pub fn builder() -> CreateExportTaskBuilder {
        CreateExportTaskBuilder::default()
    }

    /// 设置导出选项
    pub fn with_options(mut self, options: ExportOptions) -> Self {
        self.options = Some(options);
        self
    }
}

/// 创建导出任务请求构建器
///
/// 提供流畅的API来构建导出任务请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct CreateExportTaskBuilder {
    file_token: Option<String>,
    export_format: Option<String>,
    file_type: Option<String>,
    options: Option<ExportOptions>,
}

impl CreateExportTaskBuilder {
    /// 设置文件令牌
    ///
    /// # 参数
    /// * `file_token` - 文件令牌
    ///
    /// # 示例
    /// ```rust
    /// let builder = CreateExportTaskBuilder::default()
    ///     .file_token("file_token_123");
    /// ```
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置导出格式
    ///
    /// # 参数
    /// * `export_format` - 导出格式
    ///
    /// # 示例
    /// ```rust
    /// let builder = CreateExportTaskBuilder::default()
    ///     .export_format("pdf");
    /// ```
    pub fn export_format(mut self, export_format: impl Into<String>) -> Self {
        self.export_format = Some(export_format.into());
        self
    }

    /// 设置文件类型
    ///
    /// # 参数
    /// * `file_type` - 文件类型
    ///
    /// # 示例
    /// ```rust
    /// let builder = CreateExportTaskBuilder::default()
    ///     .file_type("doc");
    /// ```
    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    /// 设置导出选项
    ///
    /// # 参数
    /// * `options` - 导出选项
    ///
    /// # 示例
    /// ```rust
    /// let options = ExportOptions {
    ///     quality: Some("high".to_string()),
    ///     include_comments: Some(true),
    ///     watermark: None,
    /// };
    /// let builder = CreateExportTaskBuilder::default()
    ///     .options(options);
    /// ```
    pub fn options(mut self, options: ExportOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// 构建导出任务请求
    ///
    /// # 返回
    /// 成功返回导出任务请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果文件令牌为空，返回错误
    /// * 如果导出格式无效，返回错误
    /// * 如果文件类型无效，返回错误
    pub fn build(self) -> SDKResult<CreateExportTaskRequest> {
        // 验证文件令牌
        if let Some(file_token) = &self.file_token {
            if file_token.trim().is_empty() {
                return Err(SDKError::ValidationError("文件令牌不能为空".to_string()));
            }
        } else {
            return Err(SDKError::ValidationError("文件令牌是必填参数".to_string()));
        }

        // 验证导出格式
        if let Some(export_format) = &self.export_format {
            let valid_formats = [
                "pdf", "docx", "xlsx", "pptx", "png", "jpg", "jpeg",
                "svg", "txt", "md", "csv", "html", "rtf"
            ];
            if !valid_formats.contains(&export_format.as_str()) {
                return Err(SDKError::ValidationError(
                    format!("不支持的导出格式: {}。支持的格式: {}",
                           export_format, valid_formats.join(", "))
                ));
            }
        } else {
            return Err(SDKError::ValidationError("导出格式是必填参数".to_string()));
        }

        // 验证文件类型
        if let Some(file_type) = &self.file_type {
            let valid_types = ["doc", "sheet", "mindnote", "file", "folder", "bitable", "wiki"];
            if !valid_types.contains(&file_type.as_str()) {
                return Err(SDKError::ValidationError(
                    format!("不支持的文件类型: {}。支持的类型: {}",
                           file_type, valid_types.join(", "))
                ));
            }
        } else {
            return Err(SDKError::ValidationError("文件类型是必填参数".to_string()));
        }

        // 验证导出选项
        if let Some(options) = &self.options {
            if let Some(quality) = &options.quality {
                let valid_qualities = ["high", "medium", "low"];
                if !valid_qualities.contains(&quality.as_str()) {
                    return Err(SDKError::ValidationError(
                        format!("不支持的导出质量: {}。支持的质量: {}",
                               quality, valid_qualities.join(", "))
                    ));
                }
            }

            if let Some(watermark) = &options.watermark {
                if watermark.opacity < 0.0 || watermark.opacity > 1.0 {
                    return Err(SDKError::ValidationError(
                        "水印透明度必须在0.0-1.0之间".to_string()
                    ));
                }
                if watermark.text.trim().is_empty() {
                    return Err(SDKError::ValidationError(
                        "水印文字不能为空".to_string()
                    ));
                }
            }
        }

        Ok(CreateExportTaskRequest {
            file_token: self.file_token.unwrap(),
            export_format: self.export_format.unwrap(),
            file_type: self.file_type.unwrap(),
            options: self.options,
        })
    }
}

/// 创建导出任务响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExportTaskResponse {
    /// 任务ID
    /// 导出任务的唯一标识符
    pub task_id: String,
    /// 任务状态
    /// pending: 等待中, processing: 处理中, success: 成功, failed: 失败
    pub status: String,
    /// 进度百分比
    /// 0-100的整数，表示导出进度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// 创建时间
    /// 任务创建的时间戳
    pub created_at: String,
    /// 预计完成时间
    /// 预计的任务完成时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_finish_time: Option<String>,
    /// 导出文件URL
    /// 任务完成后，导出文件的下载链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_url: Option<String>,
    /// 过期时间
    /// 导出文件链接的过期时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
}

impl CreateExportTaskResponse {
    /// 创建新的导出任务响应
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    /// * `status` - 任务状态
    /// * `created_at` - 创建时间
    pub fn new(task_id: impl Into<String>, status: impl Into<String>, created_at: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            status: status.into(),
            created_at: created_at.into(),
            progress: None,
            estimated_finish_time: None,
            export_url: None,
            expire_time: None,
        }
    }

    /// 获取任务ID
    ///
    /// # 返回
    /// 任务ID
    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    /// 获取任务状态
    ///
    /// # 返回
    /// 任务状态
    pub fn status(&self) -> &str {
        &self.status
    }

    /// 获取进度百分比
    ///
    /// # 返回
    /// 进度百分比，如果未提供则返回None
    pub fn progress(&self) -> Option<i32> {
        self.progress
    }

    /// 获取创建时间
    ///
    /// # 返回
    /// 创建时间
    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    /// 获取导出文件URL
    ///
    /// # 返回
    /// 导出文件URL，如果任务未完成则返回None
    pub fn export_url(&self) -> Option<&str> {
        self.export_url.as_deref()
    }

    /// 检查任务是否已完成
    ///
    /// # 返回
    /// true表示任务已完成，false表示进行中或等待中
    pub fn is_completed(&self) -> bool {
        self.status == "success"
    }

    /// 检查任务是否失败
    ///
    /// # 返回
    /// true表示任务失败，false表示进行中或成功
    pub fn is_failed(&self) -> bool {
        self.status == "failed"
    }
}

/// 创建导出任务构建器
///
/// 提供流畅的API来创建导出任务，支持方法链调用和完整的错误处理
#[derive(Clone, Debug)]
pub struct CreateExportTaskBuilder {
    service: Arc<DriveServiceV1>,
    request: CreateExportTaskRequest,
}

impl CreateExportTaskBuilder {
    /// 创建新的导出任务构建器
    ///
    /// # 参数
    /// * `service` - 云盘服务实例
    /// * `request` - 导出任务请求
    pub(crate) fn new(service: Arc<DriveServiceV1>, request: CreateExportTaskRequest) -> Self {
        Self { service, request }
    }

    /// 执行导出任务创建操作
    ///
    /// 向飞书API发送POST请求来创建导出任务
    ///
    /// # 返回
    /// * `Ok(CreateExportTaskResponse)` - 创建成功，返回任务信息
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
    /// use open_lark::service::cloud_docs::drive::v1::export_task::{CreateExportTaskRequest, CreateExportTaskResponse};
    ///
    /// async fn export_file_example(
    ///     service: Arc<DriveServiceV1>,
    /// ) -> Result<CreateExportTaskResponse, Box<dyn std::error::Error>> {
    ///     let request = CreateExportTaskRequest::builder()
    ///         .file_token("file_token_123")
    ///         .export_format("pdf")
    ///         .file_type("doc")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .create_export_task_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("导出任务创建成功，任务ID: {}", response.task_id());
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<CreateExportTaskResponse> {
        let url = self.service.config().build_url(ENDPOINT_CREATE_EXPORT_TASK);

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
        let response_data: CreateExportTaskResponse = serde_json::from_value(response)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        Ok(response_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_export_task_request_builder() {
        // 测试正常构建
        let request = CreateExportTaskRequest::builder()
            .file_token("file_token_123")
            .export_format("pdf")
            .file_type("doc")
            .build()
            .unwrap();

        assert_eq!(request.file_token, "file_token_123");
        assert_eq!(request.export_format, "pdf");
        assert_eq!(request.file_type, "doc");
        assert!(request.options.is_none());
    }

    #[test]
    fn test_create_export_task_request_with_options() {
        let options = ExportOptions {
            quality: Some("high".to_string()),
            page_range: Some("1-5".to_string()),
            include_comments: Some(true),
            watermark: Some(WatermarkSettings {
                text: "机密文档".to_string(),
                opacity: 0.3,
                rotation: 45,
            }),
        };

        let request = CreateExportTaskRequest::builder()
            .file_token("file_token_123")
            .export_format("pdf")
            .file_type("doc")
            .options(options)
            .build()
            .unwrap();

        assert!(request.options.is_some());
        let opts = request.options.unwrap();
        assert_eq!(opts.quality, Some("high".to_string()));
        assert_eq!(opts.page_range, Some("1-5".to_string()));
        assert_eq!(opts.include_comments, Some(true));
        assert!(opts.watermark.is_some());
    }

    #[test]
    fn test_create_export_task_request_validation() {
        // 测试必填参数缺失
        let result = CreateExportTaskRequest::builder().build();
        assert!(result.is_err());

        // 测试空文件令牌
        let result = CreateExportTaskRequest::builder()
            .file_token("")
            .export_format("pdf")
            .file_type("doc")
            .build();
        assert!(result.is_err());

        // 测试无效导出格式
        let result = CreateExportTaskRequest::builder()
            .file_token("file_token_123")
            .export_format("invalid_format")
            .file_type("doc")
            .build();
        assert!(result.is_err());

        // 测试无效文件类型
        let result = CreateExportTaskRequest::builder()
            .file_token("file_token_123")
            .export_format("pdf")
            .file_type("invalid_type")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_export_formats() {
        let valid_formats = ["pdf", "docx", "xlsx", "pptx", "png", "jpg", "jpeg"];

        for format in valid_formats.iter() {
            let request = CreateExportTaskRequest::builder()
                .file_token("file_token_123")
                .export_format(*format)
                .file_type("doc")
                .build();
            assert!(request.is_ok(), "Format {} should be valid", format);
        }
    }

    #[test]
    fn test_valid_file_types() {
        let valid_types = ["doc", "sheet", "mindnote", "file", "folder"];

        for file_type in valid_types.iter() {
            let request = CreateExportTaskRequest::builder()
                .file_token("file_token_123")
                .export_format("pdf")
                .file_type(*file_type)
                .build();
            assert!(request.is_ok(), "File type {} should be valid", file_type);
        }
    }

    #[test]
    fn test_export_options_validation() {
        // 测试无效的质量设置
        let options = ExportOptions {
            quality: Some("invalid_quality".to_string()),
            page_range: None,
            include_comments: None,
            watermark: None,
        };

        let result = CreateExportTaskRequest::builder()
            .file_token("file_token_123")
            .export_format("pdf")
            .file_type("doc")
            .options(options)
            .build();
        assert!(result.is_err());

        // 测试无效的水印透明度
        let watermark = WatermarkSettings {
            text: "测试".to_string(),
            opacity: 1.5, // 超出范围
            rotation: 0,
        };

        let options = ExportOptions {
            quality: None,
            page_range: None,
            include_comments: None,
            watermark: Some(watermark),
        };

        let result = CreateExportTaskRequest::builder()
            .file_token("file_token_123")
            .export_format("pdf")
            .file_type("doc")
            .options(options)
            .build();
        assert!(result.is_err());

        // 测试空水印文字
        let watermark = WatermarkSettings {
            text: "".to_string(), // 空文字
            opacity: 0.5,
            rotation: 0,
        };

        let options = ExportOptions {
            quality: None,
            page_range: None,
            include_comments: None,
            watermark: Some(watermark),
        };

        let result = CreateExportTaskRequest::builder()
            .file_token("file_token_123")
            .export_format("pdf")
            .file_type("doc")
            .options(options)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_export_task_response() {
        let response = CreateExportTaskResponse::new(
            "task_123",
            "success",
            "2024-01-01T00:00:00Z"
        );

        assert_eq!(response.task_id(), "task_123");
        assert_eq!(response.status(), "success");
        assert_eq!(response.created_at(), "2024-01-01T00:00:00Z");
        assert!(response.progress().is_none());
        assert!(response.export_url().is_none());
        assert!(response.is_completed());
        assert!(!response.is_failed());
    }

    #[test]
    fn test_create_export_task_request_new() {
        let request = CreateExportTaskRequest::new(
            "file_token_123",
            "pdf",
            "doc"
        );

        assert_eq!(request.file_token, "file_token_123");
        assert_eq!(request.export_format, "pdf");
        assert_eq!(request.file_type, "doc");
        assert!(request.options.is_none());
    }

    #[test]
    fn test_with_options() {
        let options = ExportOptions {
            quality: Some("medium".to_string()),
            page_range: Some("all".to_string()),
            include_comments: Some(false),
            watermark: None,
        };

        let request = CreateExportTaskRequest::new(
            "file_token_123",
            "pdf",
            "doc"
        ).with_options(options.clone());

        assert!(request.options.is_some());
        let opts = request.options.unwrap();
        assert_eq!(opts.quality, options.quality);
        assert_eq!(opts.page_range, options.page_range);
        assert_eq!(opts.include_comments, options.include_comments);
    }

    #[test]
    fn test_multiple_valid_requests() {
        let test_cases = vec![
            ("file_token_1", "pdf", "doc"),
            ("file_token_2", "xlsx", "sheet"),
            ("file_token_3", "png", "mindnote"),
        ];

        for (token, format, file_type) in test_cases {
            let request = CreateExportTaskRequest::builder()
                .file_token(token)
                .export_format(format)
                .file_type(file_type)
                .build()
                .unwrap();

            assert_eq!(request.file_token, token);
            assert_eq!(request.export_format, format);
            assert_eq!(request.file_type, file_type);
        }
    }

    #[test]
    fn test_export_settings_creation() {
        let watermark = WatermarkSettings {
            text: "内部文档".to_string(),
            opacity: 0.2,
            rotation: 30,
        };

        let options = ExportOptions {
            quality: Some("high".to_string()),
            page_range: Some("1-10".to_string()),
            include_comments: Some(true),
            watermark: Some(watermark),
        };

        assert_eq!(options.quality, Some("high".to_string()));
        assert_eq!(options.page_range, Some("1-10".to_string()));
        assert_eq!(options.include_comments, Some(true));
        assert!(options.watermark.is_some());

        let w = options.watermark.unwrap();
        assert_eq!(w.text, "内部文档");
        assert_eq!(w.opacity, 0.2);
        assert_eq!(w.rotation, 30);
    }

    #[test]
    fn test_create_export_task_request_serialization() {
        let request = CreateExportTaskRequest::builder()
            .file_token("file_token_123")
            .export_format("pdf")
            .file_type("doc")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: CreateExportTaskRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.file_token, request.file_token);
        assert_eq!(parsed.export_format, request.export_format);
        assert_eq!(parsed.file_type, request.file_type);
    }

    #[test]
    fn test_create_export_task_response_serialization() {
        let response = CreateExportTaskResponse::new(
            "task_123",
            "processing",
            "2024-01-01T00:00:00Z"
        );

        let json = serde_json::to_string(&response).unwrap();
        let parsed: CreateExportTaskResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.task_id(), response.task_id());
        assert_eq!(parsed.status(), response.status());
        assert_eq!(parsed.created_at(), response.created_at());
    }

    #[test]
    fn test_response_status_methods() {
        // 测试成功状态
        let success_response = CreateExportTaskResponse::new(
            "task_123",
            "success",
            "2024-01-01T00:00:00Z"
        );
        assert!(success_response.is_completed());
        assert!(!success_response.is_failed());

        // 测试失败状态
        let failed_response = CreateExportTaskResponse::new(
            "task_456",
            "failed",
            "2024-01-01T00:00:00Z"
        );
        assert!(!failed_response.is_completed());
        assert!(failed_response.is_failed());

        // 测试进行中状态
        let processing_response = CreateExportTaskResponse::new(
            "task_789",
            "processing",
            "2024-01-01T00:00:00Z"
        );
        assert!(!processing_response.is_completed());
        assert!(!processing_response.is_failed());
    }

    #[test]
    fn test_edge_cases() {
        // 测试最大有效质量设置
        let valid_qualities = ["high", "medium", "low"];
        for quality in valid_qualities.iter() {
            let options = ExportOptions {
                quality: Some(quality.to_string()),
                page_range: None,
                include_comments: None,
                watermark: None,
            };

            let request = CreateExportTaskRequest::builder()
                .file_token("file_token_123")
                .export_format("pdf")
                .file_type("doc")
                .options(options)
                .build();
            assert!(request.is_ok(), "Quality {} should be valid", quality);
        }

        // 测试边界水印透明度值
        let boundary_opacities = [0.0, 0.5, 1.0];
        for opacity in boundary_opacities.iter() {
            let watermark = WatermarkSettings {
                text: "测试".to_string(),
                opacity: *opacity,
                rotation: 0,
            };

            let options = ExportOptions {
                quality: None,
                page_range: None,
                include_comments: None,
                watermark: Some(watermark),
            };

            let request = CreateExportTaskRequest::builder()
                .file_token("file_token_123")
                .export_format("pdf")
                .file_type("doc")
                .options(options)
                .build();
            assert!(request.is_ok(), "Opacity {} should be valid", opacity);
        }
    }

    #[test]
    fn test_builder_defaults() {
        let builder = CreateExportTaskBuilder::default();
        assert!(builder.file_token.is_none());
        assert!(builder.export_format.is_none());
        assert!(builder.file_type.is_none());
        assert!(builder.options.is_none());
    }

    #[test]
    fn test_watermark_settings_validation() {
        // 测试有效的旋转角度
        let valid_rotations = [-180, -90, 0, 45, 90, 180];
        for rotation in valid_rotations.iter() {
            let watermark = WatermarkSettings {
                text: "测试水印".to_string(),
                opacity: 0.5,
                rotation: *rotation,
            };

            let options = ExportOptions {
                quality: None,
                page_range: None,
                include_comments: None,
                watermark: Some(watermark),
            };

            let request = CreateExportTaskRequest::builder()
                .file_token("file_token_123")
                .export_format("pdf")
                .file_type("doc")
                .options(options)
                .build();
            assert!(request.is_ok(), "Rotation {} should be valid", rotation);
        }
    }
}