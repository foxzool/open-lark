/// 分片上传素材-完成上传
///
/// 触发完成上传。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_finish
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 分片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartInfo {
    /// 分片编号
    pub part_number: i32,
    /// ETag
    pub etag: String,
}

/// 完成上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishRequest {
    /// 上传事务ID
    pub transaction_id: String,
    /// 分片信息列表
    pub parts: Vec<PartInfo>,
}

/// 媒体文件上传完成信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishInfo {
    /// 文件token
    pub file_token: String,
    /// 文件名
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件大小
    pub size: i64,
    /// 父文件夹token
    pub parent_folder_token: String,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub modify_time: String,
}

/// 完成上传响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishResponse {
    /// 上传完成的文件信息
    pub data: Option<UploadFinishInfo>,
}

impl ApiResponseTrait for UploadFinishResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 分片上传素材-完成上传
///
/// 触发完成上传。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_finish
pub async fn upload_finish(
    request: UploadFinishRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UploadFinishResponse>> {
    // 构建API端点
    let url = "/open-apis/drive/v1/medias/upload_finish";

    // 创建API请求
    let mut api_request: ApiRequest<UploadFinishResponse> =
        ApiRequest::post(url)
            .body(serde_json::to_value(request)?);

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
    fn test_upload_finish_request() {
        let part_info = PartInfo {
            part_number: 1,
            etag: "etag_media_123456".to_string(),
        };

        let request = UploadFinishRequest {
            transaction_id: "txn_media_123456".to_string(),
            parts: vec![part_info],
        };

        assert_eq!(request.transaction_id, "txn_media_123456");
        assert_eq!(request.parts.len(), 1);
        assert_eq!(request.parts[0].part_number, 1);
    }

    #[test]
    fn test_upload_finish_info() {
        let file_info = UploadFinishInfo {
            file_token: "media_token".to_string(),
            name: "uploaded_image.jpg".to_string(),
            r#type: "image".to_string(),
            size: 2048576, // 2MB
            parent_folder_token: "parent_token".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            modify_time: "2023-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(file_info.file_token, "media_token");
        assert_eq!(file_info.name, "uploaded_image.jpg");
        assert_eq!(file_info.size, 2048576);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UploadFinishResponse::data_format(), ResponseFormat::Data);
    }
}