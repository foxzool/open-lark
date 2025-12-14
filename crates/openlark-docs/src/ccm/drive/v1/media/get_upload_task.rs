use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取媒体上传任务请求
#[derive(Debug, Serialize, Default)]
pub struct GetUploadTaskRequest {
    /// 上传任务ID
    pub task_id: String,
}

/// 获取媒体上传任务响应
#[derive(Debug, Deserialize, Default)]
pub struct GetUploadTaskResponse {
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
    /// 上传地址列表
    pub upload_urls: Vec<UploadUrl>,
}

/// 媒体文件信息
#[derive(Debug, Deserialize, Default)]
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

/// 上传地址信息
#[derive(Debug, Deserialize, Default)]
pub struct UploadUrl {
    /// 分片编号
    pub part_number: i32,
    /// 上传地址
    pub upload_url: String,
}

/// 获取媒体上传任务
/// docPath: https://open.feishu.cn/open-apis/drive/v1/medias/upload_tasks/:task_id
pub async fn get_upload_task(
    request: GetUploadTaskRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetUploadTaskResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/medias/upload_tasks/{}",
        config.base_url, request.task_id
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
    async fn test_get_upload_task() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetUploadTaskRequest {
            task_id: "test_task_id".to_string(),
        };

        let result = get_upload_task(request, &config, None).await;
        assert!(result.is_ok());
    }
}