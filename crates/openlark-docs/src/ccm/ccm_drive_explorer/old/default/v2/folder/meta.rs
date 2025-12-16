use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 获取文件夹元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderMetaRequest {
    /// 文件夹 token
    #[serde(skip)]
    pub folder_token: String,
}

impl GetFolderMetaRequest {
    /// 创建新的 GetFolderMetaRequest
    pub fn new(folder_token: impl Into<String>) -> Self {
        Self {
            folder_token: folder_token.into(),
        }
    }
}

/// 获取文件夹元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderMetaResponse {
    /// id
    pub id: String,
    /// token
    pub token: String,
    /// name
    pub name: String,
    /// description
    pub description: String,
    /// parentId
    pub parentId: String,
    /// parentToken
    pub parentToken: String,
    /// ownId
    pub ownId: String,
    /// ownerId
    pub ownerId: String,
    /// createUid
    pub createUid: String,
    /// editUid
    pub editUid: String,
}

impl ApiResponseTrait for GetFolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹元数据
///
/// 根据 folderToken 获取该文件夹的元信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-folder-meta
pub async fn meta(
    request: GetFolderMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetFolderMetaResponse>> {
    let api_endpoint = CcmDriveExplorerApiOld::FolderMeta(request.folder_token.clone());
    let mut api_request: ApiRequest<GetFolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_folder_meta_request() {
        let request = GetFolderMetaRequest::new("folder_token");
        assert_eq!(request.folder_token, "folder_token");
    }
}
