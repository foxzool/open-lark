/// 获取导入任务
///
/// 获取导入任务的详细信息，包括任务状态、进度和结果。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/import_tasks/:task_token
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取导入任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportTaskRequest {
    /// 任务token
    pub task_token: String,
}

impl GetImportTaskRequest {
    /// 创建获取导入任务请求
    ///
    /// # 参数
    /// * `task_token` - 任务token
    pub fn new(task_token: impl Into<String>) -> Self {
        Self {
            task_token: task_token.into(),
        }
    }
}

/// 获取导入任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportTaskResponse {
    /// 导入任务信息
    pub task: ImportTaskInfo,
    /// 操作结果
    pub result: String,
}

impl ApiResponseTrait for GetImportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 导入任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTaskInfo {
    /// 任务token
    pub task_token: String,
    /// 任务状态
    pub status: String,
    /// 文件token
    pub file_token: Option<String>,
    /// 导入类型
    pub r#type: String,
    /// 文件名
    pub file_name: String,
    /// 创建时间
    pub created_at: String,
    /// 完成时间
    pub completed_at: Option<String>,
    /// 错误信息
    pub error: Option<String>,
}

/// 获取导入任务
///
/// 获取导入任务的详细信息，包括任务状态、进度和结果。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/import_tasks/:task_token
pub async fn get_import_task(
    request: GetImportTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetImportTaskResponse>> {
    // 创建API请求
    let url = DriveApi::GetImportTask(request.task_token.clone()).to_url();
    let mut api_request: ApiRequest<GetImportTaskResponse> =
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
    fn test_get_import_task_request_builder() {
        let request = GetImportTaskRequest::new("task_token");
        assert_eq!(request.task_token, "task_token");
    }

    #[test]
    fn test_import_task_info_structure() {
        let task_info = ImportTaskInfo {
            task_token: "task_123".to_string(),
            status: "completed".to_string(),
            file_token: Some("file_456".to_string()),
            r#type: "docx".to_string(),
            file_name: "imported.docx".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            completed_at: Some("2023-01-01T00:05:00Z".to_string()),
            error: None,
        };

        assert_eq!(task_info.task_token, "task_123");
        assert_eq!(task_info.status, "completed");
        assert_eq!(task_info.file_token, Some("file_456".to_string()));
        assert_eq!(task_info.completed_at, Some("2023-01-01T00:05:00Z".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetImportTaskResponse::data_format(), ResponseFormat::Data);
    }
}