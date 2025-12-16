use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 获取文件夹下的文档清单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderChildrenRequest {
    /// 文件夹 token
    #[serde(skip)]
    pub folder_token: String,
    /// types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

impl GetFolderChildrenRequest {
    /// 创建新的 GetFolderChildrenRequest
    pub fn new(folder_token: impl Into<String>) -> Self {
        Self {
            folder_token: folder_token.into(),
            types: None,
        }
    }

    /// 设置 types
    pub fn types(mut self, types: Vec<String>) -> Self {
        self.types = Some(types);
        self
    }
}

/// 获取文件夹下的文档清单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderChildrenResponse {
    /// children
    pub children: std::collections::HashMap<String, serde_json::Value>, // Using Map as structure variable depending on type key
}

impl ApiResponseTrait for GetFolderChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹下的文档清单
///
/// 根据 folderToken 获取该文件夹的文档清单。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/get-folder-children
pub async fn children(
    request: GetFolderChildrenRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetFolderChildrenResponse>> {
    let api_endpoint = CcmDriveExplorerApiOld::FolderChildren(request.folder_token.clone());
    let mut api_request: ApiRequest<GetFolderChildrenResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(types) = &request.types {
        api_request = api_request.query("types", &types.join(","));
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_folder_children_request() {
        let request = GetFolderChildrenRequest::new("folder_token");
        assert_eq!(request.folder_token, "folder_token");
    }
}
