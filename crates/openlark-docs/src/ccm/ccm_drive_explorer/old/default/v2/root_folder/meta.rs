use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

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
    /// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta
    pub async fn send(self) -> SDKResult<GetRootFolderMetaResponse> {
        let api_endpoint = CcmDriveExplorerApiOld::RootFolderMeta;
        let api_request: ApiRequest<GetRootFolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

        let response: Response<GetRootFolderMetaResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("response", "响应数据为空")
        })
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

// 注：旧实现曾提供 `meta(request, config, option)` 形式函数调用；
// 目前统一为 `GetRootFolderMetaRequest { config }.send()` 以保持风格一致。
