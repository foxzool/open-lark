//! 获取我的空间（根文件夹）元数据
//!
//! docPath: /document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta
//! doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

/// 获取我的空间（根文件夹）元数据请求
pub struct GetRootFolderMetaRequest {
    config: Config,
}

impl GetRootFolderMetaRequest {
    /// 创建获取我的空间（根文件夹）元数据请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 发送请求
    ///
    /// docPath: /document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta
    /// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta
    pub async fn send(self) -> SDKResult<GetRootFolderMetaResponse> {
        let api_endpoint = CcmDriveExplorerApiOld::RootFolderMeta;
        let api_request: ApiRequest<GetRootFolderMetaResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取我的空间（根文件夹）元数据")
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
    pub user_id: String,
}

impl ApiResponseTrait for GetRootFolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// 注：旧实现曾提供 `meta(request, config, option)` 形式函数调用；
// 目前统一为 `GetRootFolderMetaRequest { config }.send()` 以保持风格一致。
