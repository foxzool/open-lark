use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 新建文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileRequest {
    /// 文件夹 token
    #[serde(skip)]
    pub folder_token: String,
    /// 文件类型: doc, sheet, bitable
    pub r#type: String,
    /// 标题
    pub title: String,
}

impl CreateFileRequest {
    /// 创建新的 CreateFileRequest
    pub fn new(folder_token: impl Into<String>, type_: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            folder_token: folder_token.into(),
            r#type: type_.into(),
            title: title.into(),
        }
    }
}

/// 新建文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileResponse {
    /// url
    pub url: String,
    /// token
    pub token: String,
    /// revision
    pub revision: i64,
}

impl ApiResponseTrait for CreateFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件
///
/// 根据 folderToken 创建 Doc、 Sheet 或 Bitable。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create-online-document
pub async fn create(
    request: CreateFileRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateFileResponse>> {
    let api_endpoint = CcmDriveExplorerApiOld::File(request.folder_token.clone());
    let mut api_request: ApiRequest<CreateFileResponse> = ApiRequest::post(&api_endpoint.to_url())
        .json_body(&request);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file_request() {
        let request = CreateFileRequest::new("folder_token", "doc", "title");
        assert_eq!(request.folder_token, "folder_token");
        assert_eq!(request.r#type, "doc");
    }
}
