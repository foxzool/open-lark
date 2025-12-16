use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 获取我的空间（根文件夹）元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRootFolderMetaRequest {}

impl GetRootFolderMetaRequest {
    /// 创建新的 GetRootFolderMetaRequest
    pub fn new() -> Self {
        Self {}
    }
}

/// 获取我的空间（根文件夹）元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRootFolderMetaResponse {
    /// token
    pub token: String,
    /// id
    pub id: String,
    /// user_id
    pub user_id: i64,
}

impl ApiResponseTrait for GetRootFolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取我的空间（根文件夹）元数据
///
/// 获取 "我的空间" 的元信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta
pub async fn meta(
    _request: GetRootFolderMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetRootFolderMetaResponse>> {
    let api_endpoint = CcmDriveExplorerApiOld::RootFolderMeta;
    let mut api_request: ApiRequest<GetRootFolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_root_folder_meta_request() {
        let _request = GetRootFolderMetaRequest::new();
    }
}
