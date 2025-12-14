use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 创建媒体上传任务请求
#[derive(Debug, Serialize, Default)]
pub struct CreateUploadTaskRequest {
    /// 父目录的token
    pub parent_type: String,
    /// 父目录的token
    pub parent_token: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小
    pub size: u64,
    /// 文件类型
    pub r#type: String,
    /// 检查项
    pub check_items: Vec<CheckItem>,
    /// 媒体类型
    pub media_type: Option<String>,
}

/// 检查项配置
#[derive(Debug, Serialize, Default)]
pub struct CheckItem {
    /// 检查类型
    pub r#type: String,
}

/// 媒体文件信息
#[derive(Debug, Serialize, Default)]
pub struct MediaFile {
    /// 文件名称
    pub file_name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件大小
    pub size: u64,
    /// 媒体类型
    pub media_type: Option<String>,
}

/// 创建媒体上传任务响应
#[derive(Debug, Deserialize, Default)]
pub struct CreateUploadTaskResponse {
    /// 上传任务信息
    pub task: UploadTaskInfo,
    /// 操作结果
    pub result: String,
}

/// 上传任务信息
#[derive(Debug, Deserialize, Default)]
pub struct UploadTaskInfo {
    /// 任务ID
    pub task_id: String,
    /// 任务状态
    pub status: String,
    /// 媒体文件信息
    pub media_file: MediaFile,
}

/// 创建媒体上传任务
/// docPath: https://open.feishu.cn/open-apis/drive/v1/medias/upload_tasks
pub async fn create_upload_task(
    request: CreateUploadTaskRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateUploadTaskResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/medias/upload_tasks",
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
    async fn test_create_upload_task() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = CreateUploadTaskRequest {
            parent_type: "explorer".to_string(),
            parent_token: "test_parent_token".to_string(),
            file_name: "test_video.mp4".to_string(),
            size: 1024000,
            r#type: "video".to_string(),
            check_items: vec![CheckItem {
                r#type: "virus".to_string(),
            }],
            media_type: Some("video/mp4".to_string()),
        };

        let result = create_upload_task(request, &config, None).await;
        assert!(result.is_ok());
    }
}