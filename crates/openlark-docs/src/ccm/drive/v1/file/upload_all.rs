/// 上传文件
///
/// 向云空间指定目录下上传一个小文件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/upload_all
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 上传文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileRequest {
    /// 父文件夹token
    pub parent_folder_token: String,
    /// 文件名
    pub file_name: String,
    /// 文件内容（base64编码）
    pub file_content: String,
    /// 文件大小
    pub size: i64,
    /// 文件类型（MIME）
    pub content_type: Option<String>,
}

/// 文件上传响应信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileInfo {
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

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileResponse {
    /// 上传的文件信息
    pub data: Option<UploadFileInfo>,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传文件
///
/// 向云空间指定目录下上传一个小文件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/upload_all
pub async fn upload_file(
    request: UploadFileRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UploadFileResponse>> {
    // 构建API端点
    let url = "/open-apis/drive/v1/files/upload_all";

    // 创建API请求
    let mut api_request: ApiRequest<UploadFileResponse> =
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
    fn test_upload_file_request() {
        let request = UploadFileRequest {
            parent_folder_token: "folder_token".to_string(),
            file_name: "test.txt".to_string(),
            file_content: "SGVsbG8gV29ybGQ=".to_string(), // "Hello World" in base64
            size: 11,
            content_type: Some("text/plain".to_string()),
        };

        assert_eq!(request.file_name, "test.txt");
        assert_eq!(request.size, 11);
        assert_eq!(request.content_type, Some("text/plain".to_string()));
    }

    #[test]
    fn test_upload_file_info_structure() {
        let file_info = UploadFileInfo {
            file_token: "file_token".to_string(),
            name: "test.txt".to_string(),
            r#type: "file".to_string(),
            size: 11,
            parent_folder_token: "parent_token".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            modify_time: "2023-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(file_info.file_token, "file_token");
        assert_eq!(file_info.name, "test.txt");
        assert_eq!(file_info.size, 11);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UploadFileResponse::data_format(), ResponseFormat::Data);
    }
}