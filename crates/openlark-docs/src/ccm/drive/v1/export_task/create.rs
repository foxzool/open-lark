use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 创建导出任务请求
#[derive(Debug, Serialize, Default)]
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

/// PDF导出配置
#[derive(Debug, Serialize, Default)]
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

/// 导出页面范围
#[derive(Debug, Serialize, Default)]
pub struct PageRange {
    /// 开始页
    pub start: i32,
    /// 结束页
    pub end: i32,
}

/// 创建导出任务响应
#[derive(Debug, Deserialize, Default)]
pub struct CreateExportTaskResponse {
    /// 导出任务信息
    pub task: ExportTaskInfo,
    /// 操作结果
    pub result: String,
}

/// 导出任务信息
#[derive(Debug, Deserialize, Default)]
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
/// docPath: https://open.feishu.cn/open-apis/drive/v1/export_tasks
pub async fn create_export_task(
    request: CreateExportTaskRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateExportTaskResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/export_tasks",
        config.base_url
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::POST,
        headers: vec![],
        query_params: vec![],
        body: Some(serde_json::to_vec(&request)?),
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_create_export_task() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = CreateExportTaskRequest {
            file_token: "test_file_token".to_string(),
            r#type: "docx".to_string(),
            file_extension: "docx".to_string(),
            name: Some("导出文档".to_string()),
            include_deleted_comment: Some(false),
            pdf_options: Some(PdfExportOptions {
                page_size: Some("A4".to_string()),
                page_orientation: Some("portrait".to_string()),
                include_comments: Some(false),
                width: None,
                height: None,
                margin: Some(1.0),
            }),
            page_range: Some(PageRange {
                start: 1,
                end: 10,
            }),
        };

        let result = create_export_task(request, &config, None).await;
        assert!(result.is_ok());
    }
}