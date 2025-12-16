use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 新建文件夹请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    /// 文件夹 token
    #[serde(skip)]
    pub folder_token: String,
    /// 标题
    pub title: String,
}

impl CreateFolderRequest {
    /// 创建新的 CreateFolderRequest
    pub fn new(folder_token: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            folder_token: folder_token.into(),
            title: title.into(),
        }
    }
}

/// 新建文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderResponse {
    /// url
    pub url: String,
    /// token
    pub token: String,
    /// revision
    pub revision: i64,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件夹
///
/// 根据 folderToken 在该 folder 下创建文件夹。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/create-a-new-folder
pub async fn create(
    request: CreateFolderRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateFolderResponse>> {
    let api_endpoint = CcmDriveExplorerApiOld::Folder(request.folder_token.clone());
    let mut api_request: ApiRequest<CreateFolderResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_create_folder_request() {
        let request = CreateFolderRequest::new("folder_token", "title");
        assert_eq!(request.folder_token, "folder_token");
        assert_eq!(request.title, "title");
    }
}
