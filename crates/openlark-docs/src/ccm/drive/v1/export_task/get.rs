use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取导出任务请求
#[derive(Debug, Serialize, Default)]
pub struct GetExportTaskRequest {
    /// 任务token
    pub task_token: String,
}

/// 获取导出任务响应
#[derive(Debug, Deserialize, Default)]
pub struct GetExportTaskResponse {
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
/// docPath: https://open.feishu.cn/open-apis/drive/v1/export_tasks/:task_token
pub async fn get_export_task(
    request: GetExportTaskRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetExportTaskResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/export_tasks/{}",
        config.base_url, request.task_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::GET,
        headers: vec![],
        query_params: vec![],
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_export_task() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetExportTaskRequest {
            task_token: "test_task_token".to_string(),
        };

        let result = get_export_task(request, &config, None).await;
        assert!(result.is_ok());
    }
}