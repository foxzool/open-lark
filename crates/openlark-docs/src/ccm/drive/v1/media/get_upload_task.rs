/// 获取媒体上传任务
///
/// 获取媒体文件上传任务的详细信息，包括任务状态和上传地址。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/medias/upload_tasks/:task_id
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取媒体上传任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUploadTaskRequest {
    /// 上传任务ID
    pub task_id: String,
}

impl GetUploadTaskRequest {
    /// 创建获取媒体上传任务请求
    ///
    /// # 参数
    /// * `task_id` - 上传任务ID
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
        }
    }

    /// 设置上传任务ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = task_id.into();
        self
    }
}

/// 获取媒体上传任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUploadTaskResponse {
    /// 上传任务信息
    pub data: Option<UploadTaskInfo>,
}

impl ApiResponseTrait for GetUploadTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadUrl {
    /// 分片编号
    pub part_number: i32,
    /// 上传地址
    pub upload_url: String,
}

/// 获取媒体上传任务
///
/// 获取媒体文件上传任务的详细信息，包括任务状态和上传地址。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/medias/upload_tasks/:task_id
pub async fn get_upload_task(
    request: GetUploadTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetUploadTaskResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::MediaUploadTask(request.task_id);

    // 创建API请求
    let mut api_request: ApiRequest<GetUploadTaskResponse> =
        ApiRequest::get(&api_endpoint.to_url());

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
    fn test_get_upload_task_request_builder() {
        let request = GetUploadTaskRequest::new("task_id");

        assert_eq!(request.task_id, "task_id");
    }

    #[test]
    fn test_get_upload_task_request_builder_chain() {
        let request = GetUploadTaskRequest::new("task_id_1")
            .task_id("task_id_2");

        assert_eq!(request.task_id, "task_id_2");
    }

    #[test]
    fn test_upload_task_info_structure() {
        let media_file = MediaFile {
            file_name: "test_file.pdf".to_string(),
            r#type: "pdf".to_string(),
            size: 1024000,
            media_type: Some("application/pdf".to_string()),
        };

        let upload_url = UploadUrl {
            part_number: 1,
            upload_url: "https://upload.example.com/part1".to_string(),
        };

        let upload_task_info = UploadTaskInfo {
            task_id: "task_123".to_string(),
            status: "processing".to_string(),
            media_file: media_file.clone(),
            upload_urls: vec![upload_url.clone()],
        };

        assert_eq!(upload_task_info.task_id, "task_123");
        assert_eq!(upload_task_info.status, "processing");
        assert_eq!(upload_task_info.media_file.file_name, "test_file.pdf");
        assert_eq!(upload_task_info.upload_urls[0].part_number, 1);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetUploadTaskResponse::data_format(), ResponseFormat::Data);
    }
}