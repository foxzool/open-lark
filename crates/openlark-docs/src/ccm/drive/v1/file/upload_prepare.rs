/// 分片上传文件-预上传
///
/// 发送初始化请求获取上传事务ID和分块策略，目前是以4MB大小进行定长分片。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_prepare
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 分片上传预请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareRequest {
    /// 父文件夹token
    pub parent_folder_token: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小（字节）
    pub size: i64,
    /// 文件类型（MIME）
    pub content_type: Option<String>,
}

/// 分片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartInfo {
    /// 分片编号
    pub part_number: i32,
    /// 分片起始位置
    pub start_position: i64,
    /// 分片大小
    pub part_size: i32,
    /// 上传URL
    pub upload_url: String,
}

/// 分片上传准备响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareResponse {
    /// 上传事务ID
    pub transaction_id: String,
    /// 文件token
    pub file_token: String,
    /// 分片信息列表
    pub parts: Option<Vec<PartInfo>>,
}

impl ApiResponseTrait for UploadPrepareResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 分片上传文件-预上传
///
/// 发送初始化请求获取上传事务ID和分块策略，目前是以4MB大小进行定长分片。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_prepare
pub async fn upload_prepare(
    request: UploadPrepareRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UploadPrepareResponse>> {
    // 构建API端点
    let url = "/open-apis/drive/v1/files/upload_prepare";

    // 创建API请求
    let mut api_request: ApiRequest<UploadPrepareResponse> =
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
    fn test_upload_prepare_request() {
        let request = UploadPrepareRequest {
            parent_folder_token: "folder_token".to_string(),
            file_name: "large_file.zip".to_string(),
            size: 10485760, // 10MB
            content_type: Some("application/zip".to_string()),
        };

        assert_eq!(request.file_name, "large_file.zip");
        assert_eq!(request.size, 10485760);
        assert_eq!(request.content_type, Some("application/zip".to_string()));
    }

    #[test]
    fn test_part_info_structure() {
        let part_info = PartInfo {
            part_number: 1,
            start_position: 0,
            part_size: 4194304, // 4MB
            upload_url: "https://upload.example.com/part1".to_string(),
        };

        assert_eq!(part_info.part_number, 1);
        assert_eq!(part_info.start_position, 0);
        assert_eq!(part_info.part_size, 4194304);
        assert_eq!(part_info.upload_url, "https://upload.example.com/part1");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UploadPrepareResponse::data_format(), ResponseFormat::Data);
    }
}