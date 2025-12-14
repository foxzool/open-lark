use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取导入任务请求
#[derive(Debug, Serialize, Default)]
pub struct GetImportTaskRequest {
    /// 任务token
    pub task_token: String,
}

/// 获取导入任务响应
#[derive(Debug, Deserialize, Default)]
pub struct GetImportTaskResponse {
    /// 导入任务信息
    pub task: ImportTaskInfo,
    /// 操作结果
    pub result: String,
}

/// 导入任务信息
#[derive(Debug, Deserialize, Default)]
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
/// docPath: https://open.feishu.cn/open-apis/drive/v1/import_tasks/:task_token
pub async fn get_import_task(
    request: GetImportTaskRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetImportTaskResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/import_tasks/{}",
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
    async fn test_get_import_task() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetImportTaskRequest {
            task_token: "test_task_token".to_string(),
        };

        let result = get_import_task(request, &config, None).await;
        assert!(result.is_ok());
    }
}