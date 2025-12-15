/// 获取导出任务
///
/// 获取导出任务的状态和结果。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/export_tasks/:task_token
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取导出任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportTaskRequest {
    /// 任务token
    pub task_token: String,
}

impl GetExportTaskRequest {
    /// 创建获取导出任务请求
    ///
    /// # 参数
    /// * `task_token` - 任务token
    pub fn new(task_token: impl Into<String>) -> Self {
        Self {
            task_token: task_token.into(),
        }
    }
}

/// 获取导出任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportTaskResponse {
    /// 导出任务信息
    pub data: Option<ExportTaskInfo>,
}

impl ApiResponseTrait for GetExportTaskResponse {
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
    /// 导出文件token
    pub export_file_token: Option<String>,
    /// 创建时间
    pub created_at: String,
    /// 完成时间
    pub completed_at: Option<String>,
    /// 错误信息
    pub error: Option<String>,
}

/// 获取导出任务
///
/// 获取导出任务的状态和结果。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/export_tasks/:task_token
pub async fn get_export_task(
    request: GetExportTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetExportTaskResponse>> {
    // 创建API请求
    let url = DriveApi::GetExportTask(request.task_token.clone()).to_url();
    let mut api_request: ApiRequest<GetExportTaskResponse> =
        ApiRequest::get(&url);

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
    fn test_get_export_task_request_builder() {
        let request = GetExportTaskRequest::new("task_token");
        assert_eq!(request.task_token, "task_token");
    }

    #[test]
    fn test_export_task_info_structure() {
        let task_info = ExportTaskInfo {
            task_token: "task_123".to_string(),
            status: "completed".to_string(),
            file_token: "file_456".to_string(),
            r#type: "docx".to_string(),
            file_extension: "docx".to_string(),
            name: Some("导出文档".to_string()),
            export_file_token: Some("export_file_789".to_string()),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            completed_at: Some("2023-01-01T01:00:00Z".to_string()),
            error: None,
        };

        assert_eq!(task_info.task_token, "task_123");
        assert_eq!(task_info.status, "completed");
        assert_eq!(task_info.export_file_token, Some("export_file_789".to_string()));
        assert_eq!(task_info.completed_at, Some("2023-01-01T01:00:00Z".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetExportTaskResponse::data_format(), ResponseFormat::Data);
    }
}