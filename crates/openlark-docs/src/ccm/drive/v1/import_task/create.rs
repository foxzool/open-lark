use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 创建导入任务请求
#[derive(Debug, Serialize, Default)]
pub struct CreateImportTaskRequest {
    /// 父目录的token
    pub parent_type: String,
    /// 父目录的token
    pub parent_token: String,
    /// 文件名
    pub file_name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件token（用于从云空间导入）
    pub file_token: Option<String>,
    /// 上传任务token（用于从上传任务导入）
    pub upload_task_token: Option<String>,
    /// 导入选项
    pub options: Option<ImportOptions>,
}

/// 导入选项
#[derive(Debug, Serialize, Default)]
pub struct ImportOptions {
    /// 是否覆盖同名文件
    pub overwrite: Option<bool>,
    /// 导入模式
    pub mode: Option<String>,
    /// 目标类型
    pub target_type: Option<String>,
}

/// 创建导入任务响应
#[derive(Debug, Deserialize, Default)]
pub struct CreateImportTaskResponse {
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
}

/// 创建导入任务
/// docPath: https://open.feishu.cn/open-apis/drive/v1/import_tasks
pub async fn create_import_task(
    request: CreateImportTaskRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateImportTaskResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/import_tasks",
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
    async fn test_create_import_task() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = CreateImportTaskRequest {
            parent_type: "explorer".to_string(),
            parent_token: "test_parent_token".to_string(),
            file_name: "imported_doc.docx".to_string(),
            r#type: "docx".to_string(),
            file_token: Some("test_file_token".to_string()),
            upload_task_token: None,
            options: Some(ImportOptions {
                overwrite: Some(false),
                mode: Some("create".to_string()),
                target_type: Some("doc".to_string()),
            }),
        };

        let result = create_import_task(request, &config, None).await;
        assert!(result.is_ok());
    }
}