/// 创建文件夹
///
/// 此接口用于在指定位置创建新的文件夹。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/explorer/folder
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{
    requests::FolderRequest,
    responses::FolderData,
};

/// 创建文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderResponse {
    /// 创建的文件夹信息
    pub data: Option<FolderData>,
}

impl ApiResponseTrait for FolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文件夹
pub async fn folder(
    request: FolderRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<FolderData> {
    use crate::common::api_endpoints::CcmDriveExplorerApi;
    use serde_json::json;

    // 使用CcmDriveExplorerApi枚举生成API端点
    let api_endpoint = CcmDriveExplorerApi::Folder;

    // 构建请求体
    let body = json!({
        "parent_folder_token": request.folder_token,
        "name": request.name
    });

    // 创建API请求
    let mut api_request: ApiRequest<FolderResponse> = ApiRequest::post(&api_endpoint.to_url())
        .body(body);

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
