/// 创建导出任务
///
/// 创建文件导出任务，支持多种格式导出。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/export_tasks
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建导出任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExportTaskRequest {
    /// 文件token
    pub file_token: String,
    /// 导出类型
    pub r#type: String,
    /// 导出格式
    pub file_extension: String,
    /// 导出名称
    pub name: Option<String>,
    /// 是否包含已删除的评论
    pub include_deleted_comment: Option<bool>,
    /// PDF导出配置
    pub pdf_options: Option<PdfExportOptions>,
    /// 导出页面范围
    pub page_range: Option<PageRange>,
}

impl CreateExportTaskRequest {
    /// 创建创建导出任务请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `export_type` - 导出类型
    /// * `file_extension` - 导出格式
    pub fn new(
        file_token: impl Into<String>,
        export_type: impl Into<String>,
        file_extension: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            r#type: export_type.into(),
            file_extension: file_extension.into(),
            name: None,
            include_deleted_comment: None,
            pdf_options: None,
            page_range: None,
        }
    }

    /// 设置导出名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置是否包含已删除的评论
    pub fn include_deleted_comment(mut self, include: bool) -> Self {
        self.include_deleted_comment = Some(include);
        self
    }

    /// 设置PDF导出配置
    pub fn pdf_options(mut self, options: PdfExportOptions) -> Self {
        self.pdf_options = Some(options);
        self
    }

    /// 设置导出页面范围
    pub fn page_range(mut self, range: PageRange) -> Self {
        self.page_range = Some(range);
        self
    }
}

/// PDF导出配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfExportOptions {
    /// 页面大小
    pub page_size: Option<String>,
    /// 页面方向
    pub page_orientation: Option<String>,
    /// 是否包含评论
    pub include_comments: Option<bool>,
    /// 宽度
    pub width: Option<f64>,
    /// 高度
    pub height: Option<f64>,
    /// 边距
    pub margin: Option<f64>,
}

impl PdfExportOptions {
    /// 创建PDF导出配置
    pub fn new() -> Self {
        Self {
            page_size: None,
            page_orientation: None,
            include_comments: None,
            width: None,
            height: None,
            margin: None,
        }
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: impl Into<String>) -> Self {
        self.page_size = Some(size.into());
        self
    }

    /// 设置页面方向
    pub fn page_orientation(mut self, orientation: impl Into<String>) -> Self {
        self.page_orientation = Some(orientation.into());
        self
    }

    /// 设置是否包含评论
    pub fn include_comments(mut self, include: bool) -> Self {
        self.include_comments = Some(include);
        self
    }

    /// 设置宽度
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// 设置高度
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// 设置边距
    pub fn margin(mut self, margin: f64) -> Self {
        self.margin = Some(margin);
        self
    }
}

/// 导出页面范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRange {
    /// 开始页
    pub start: i32,
    /// 结束页
    pub end: i32,
}

impl PageRange {
    /// 创建页面范围
    ///
    /// # 参数
    /// * `start` - 开始页
    /// * `end` - 结束页
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}

/// 创建导出任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExportTaskResponse {
    /// 导出任务信息
    pub data: Option<ExportTaskInfo>,
}

impl ApiResponseTrait for CreateExportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 导出任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTaskInfo {
    /// 任务token
    pub task_token: String,
    /// 任务状态
    pub status: String,
    /// 文件token
    pub file_token: String,
    /// 导出类型
    pub r#type: String,
    /// 导出格式
    pub file_extension: String,
    /// 导出名称
    pub name: Option<String>,
    /// 创建时间
    pub created_at: String,
}

/// 创建导出任务
///
/// 创建文件导出任务，支持多种格式导出。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/export_tasks
pub async fn create_export_task(
    request: CreateExportTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateExportTaskResponse>> {
    // 构建请求体
    let mut body = json!({
        "fileToken": request.file_token,
        "type": request.r#type,
        "fileExtension": request.file_extension
    });

    if let Some(name) = &request.name {
        body["name"] = json!(name);
    }
    if let Some(include_deleted_comment) = request.include_deleted_comment {
        body["includeDeletedComment"] = json!(include_deleted_comment);
    }
    if let Some(pdf_options) = &request.pdf_options {
        body["pdfOptions"] = json!(pdf_options);
    }
    if let Some(page_range) = &request.page_range {
        body["pageRange"] = json!(page_range);
    }

    // 创建API请求
    let url = DriveApi::CreateExportTask.to_url();
    let mut api_request: ApiRequest<CreateExportTaskResponse> =
        ApiRequest::post(&url)
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_export_task_request_builder() {
        let request = CreateExportTaskRequest::new(
            "file_token",
            "docx",
            "docx"
        );

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.r#type, "docx");
        assert_eq!(request.file_extension, "docx");
        assert!(request.name.is_none());
        assert!(request.include_deleted_comment.is_none());
    }

    #[test]
    fn test_create_export_task_request_builder_chain() {
        let pdf_options = PdfExportOptions::new()
            .page_size("A4")
            .page_orientation("portrait")
            .include_comments(false);

        let page_range = PageRange::new(1, 10);

        let request = CreateExportTaskRequest::new(
            "file_token",
            "docx",
            "docx"
        )
        .name("导出文档")
        .include_deleted_comment(false)
        .pdf_options(pdf_options)
        .page_range(page_range);

        assert_eq!(request.name, Some("导出文档".to_string()));
        assert_eq!(request.include_deleted_comment, Some(false));
        assert!(request.pdf_options.is_some());
        assert!(request.page_range.is_some());
    }

    #[test]
    fn test_pdf_options_builder() {
        let pdf_options = PdfExportOptions::new()
            .page_size("A4")
            .page_orientation("portrait")
            .include_comments(true)
            .width(210.0)
            .height(297.0)
            .margin(1.0);

        assert_eq!(pdf_options.page_size, Some("A4".to_string()));
        assert_eq!(pdf_options.page_orientation, Some("portrait".to_string()));
        assert_eq!(pdf_options.include_comments, Some(true));
        assert_eq!(pdf_options.width, Some(210.0));
        assert_eq!(pdf_options.height, Some(297.0));
        assert_eq!(pdf_options.margin, Some(1.0));
    }

    #[test]
    fn test_page_range_builder() {
        let page_range = PageRange::new(1, 10);
        assert_eq!(page_range.start, 1);
        assert_eq!(page_range.end, 10);
    }

    #[test]
    fn test_export_task_info_structure() {
        let task_info = ExportTaskInfo {
            task_token: "task_123".to_string(),
            status: "processing".to_string(),
            file_token: "file_456".to_string(),
            r#type: "docx".to_string(),
            file_extension: "docx".to_string(),
            name: Some("导出文档".to_string()),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(task_info.task_token, "task_123");
        assert_eq!(task_info.status, "processing");
        assert_eq!(task_info.name, Some("导出文档".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateExportTaskResponse::data_format(), ResponseFormat::Data);
    }
}