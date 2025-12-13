/// 获取文件夹子内容
///
/// 此接口用于获取指定文件夹下的子文件和子文件夹列表。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/explorer/folder_children
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{
    requests::FolderChildrenRequest,
    responses::FolderChildrenData,
};

/// 获取文件夹子内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderChildrenResponse {
    /// 文件夹子内容列表
    pub data: Option<FolderChildrenData>,
}

impl ApiResponseTrait for FolderChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹子内容
pub async fn folder_children(
    request: FolderChildrenRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<FolderChildrenData> {
    use crate::common::api_endpoints::CcmDriveExplorerApi;

    // 使用CcmDriveExplorerApi枚举生成API端点
    let api_endpoint = CcmDriveExplorerApi::FolderChildren(request.folder_token.clone());

    // 创建查询参数
    let mut query_params = Vec::new();

    if let Some(page_size) = &request.page_size {
        query_params.push(("page_size", page_size.to_string()));
    }
    if let Some(page_token) = &request.page_token {
        query_params.push(("page_token", page_token.clone()));
    }
    if let Some(order_by) = &request.order_by {
        query_params.push(("order_by", order_by.clone()));
    }
    if let Some(direction) = &request.direction {
        query_params.push(("direction", direction.clone()));
    }

    // 创建API请求
    let mut api_request: ApiRequest<FolderChildrenResponse> =
        ApiRequest::get(&api_endpoint.to_url_with_params(&query_params));

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
