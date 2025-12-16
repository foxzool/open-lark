use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 上传文件（一次性上传）
///
/// 上传指定文件到指定目录中，支持单次上传文件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/upload_all
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 上传文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAllRequest {
    /// 文件名
    pub file_name: String,
    /// 父文件夹token
    pub parent_folder_token: String,
    /// 文件内容的 base64 编码（实际上传时需要处理）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
}

impl UploadAllRequest {
    /// 创建上传请求
    pub fn new(file_name: impl Into<String>, parent_folder_token: impl Into<String>) -> Self {
        Self {
            file_name: file_name.into(),
            parent_folder_token: parent_folder_token.into(),
            file_content: None,
        }
    }

    /// 设置文件内容
    pub fn file_content(mut self, content: impl Into<String>) -> Self {
        self.file_content = Some(content.into());
        self
    }
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAllResponse {
    /// 文件token
    pub file_token: Option<String>,
}

impl ApiResponseTrait for UploadAllResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传文件（一次性上传）
pub async fn upload_all(
    request: UploadAllRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UploadAllResponse>> {
    let api_endpoint = DriveApi::UploadFile;

    let mut api_request: ApiRequest<UploadAllResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serde_json::json!(&request));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_all_request() {
        let request = UploadAllRequest::new("test.txt", "folder_token");
        assert_eq!(request.file_name, "test.txt");
        assert_eq!(request.parent_folder_token, "folder_token");
    }
}