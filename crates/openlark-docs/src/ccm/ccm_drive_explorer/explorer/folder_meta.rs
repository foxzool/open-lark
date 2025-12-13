/// 获取文件夹元数据
///
/// 此接口用于获取指定文件夹的元数据信息，包括文件夹属性、权限等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/explorer/folder_meta
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{
    requests::FolderMetaRequest,
    responses::FolderMetaData,
};

/// 获取文件夹元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderMetaResponse {
    /// 文件夹元数据
    pub data: Option<FolderMetaData>,
}

impl ApiResponseTrait for FolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹元数据
pub async fn folder_meta(
    request: FolderMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<FolderMetaData> {
    use crate::common::api_endpoints::CcmDriveExplorerApi;

    // 使用CcmDriveExplorerApi枚举生成API端点
    let api_endpoint = CcmDriveExplorerApi::FolderMeta(request.folder_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<FolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    let response = Transport::request(api_request, config, None).await?;

    // 返回数据
    response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })
}
