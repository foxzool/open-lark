/// 创建媒体上传任务
///
/// 创建媒体文件的上传任务，支持分片上传大文件。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/medias/upload_tasks
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建媒体上传任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl CreateUploadTaskRequest {
    /// 创建创建媒体上传任务请求
    ///
    /// # 参数
    /// * `parent_type` - 父目录类型
    /// * `parent_token` - 父目录token
    /// * `file_name` - 文件名
    /// * `size` - 文件大小
    /// * `file_type` - 文件类型
    pub fn new(
        parent_type: impl Into<String>,
        parent_token: impl Into<String>,
        file_name: impl Into<String>,
        size: u64,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            parent_type: parent_type.into(),
            parent_token: parent_token.into(),
            file_name: file_name.into(),
            size,
            r#type: file_type.into(),
            check_items: vec![],
            media_type: None,
        }
    }

    /// 设置检查项
    pub fn check_items(mut self, check_items: Vec<CheckItem>) -> Self {
        self.check_items = check_items;
        self
    }

    /// 设置媒体类型
    pub fn media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }
}

/// 检查项配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckItem {
    /// 检查类型
    pub r#type: String,
}

impl CheckItem {
    /// 创建检查项
    ///
    /// # 参数
    /// * `check_type` - 检查类型
    pub fn new(check_type: impl Into<String>) -> Self {
        Self {
            r#type: check_type.into(),
        }
    }
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

/// 创建媒体上传任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUploadTaskResponse {
    /// 上传任务信息
    pub data: Option<UploadTaskInfo>,
}

impl ApiResponseTrait for CreateUploadTaskResponse {
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
}

/// 创建媒体上传任务
///
/// 创建媒体文件的上传任务，支持分片上传大文件。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/medias/upload_tasks
pub async fn create_upload_task(
    request: CreateUploadTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateUploadTaskResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::MediaUploadTasks;

    // 构建请求体
    let body = json!({
        "parentType": request.parent_type,
        "parentToken": request.parent_token,
        "fileName": request.file_name,
        "size": request.size,
        "type": request.r#type,
        "checkItems": request.check_items,
        "mediaType": request.media_type
    });

    // 创建API请求
    let mut api_request: ApiRequest<CreateUploadTaskResponse> =
        ApiRequest::post(&api_endpoint.to_url())
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
    fn test_create_upload_task_request_builder() {
        let request = CreateUploadTaskRequest::new(
            "explorer",
            "parent_token",
            "test_file.mp4",
            1024000,
            "video"
        );

        assert_eq!(request.parent_type, "explorer");
        assert_eq!(request.parent_token, "parent_token");
        assert_eq!(request.file_name, "test_file.mp4");
        assert_eq!(request.size, 1024000);
        assert_eq!(request.r#type, "video");
        assert!(request.check_items.is_empty());
    }

    #[test]
    fn test_create_upload_task_request_builder_chain() {
        let check_items = vec![CheckItem::new("virus"), CheckItem::new("content")];
        let request = CreateUploadTaskRequest::new(
            "explorer",
            "parent_token",
            "test_file.mp4",
            1024000,
            "video"
        )
        .check_items(check_items.clone())
        .media_type("video/mp4");

        assert_eq!(request.media_type, Some("video/mp4".to_string()));
        assert_eq!(request.check_items.len(), 2);
        assert_eq!(request.check_items[0].r#type, "virus");
    }

    #[test]
    fn test_check_item_builder() {
        let check_item = CheckItem::new("virus");
        assert_eq!(check_item.r#type, "virus");
    }

    #[test]
    fn test_media_file_structure() {
        let media_file = MediaFile {
            file_name: "test_file.pdf".to_string(),
            r#type: "pdf".to_string(),
            size: 1024000,
            media_type: Some("application/pdf".to_string()),
        };

        assert_eq!(media_file.file_name, "test_file.pdf");
        assert_eq!(media_file.size, 1024000);
        assert_eq!(media_file.media_type, Some("application/pdf".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateUploadTaskResponse::data_format(), ResponseFormat::Data);
    }
}