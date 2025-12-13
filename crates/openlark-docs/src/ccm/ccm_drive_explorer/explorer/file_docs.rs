/// 获取文档文件信息
///
/// 此接口用于获取指定文档文件的详细信息，包括内容、版本等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/explorer/file_docs
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{
    requests::FileDocsRequest,
    responses::FileDocsData,
};

/// 获取文档文件信息
pub async fn file_docs(
    request: FileDocsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<FileDocsData> {
    use crate::common::api_endpoints::CcmDriveExplorerApi;

    // 使用CcmDriveExplorerApi枚举生成API端点
    let api_endpoint = CcmDriveExplorerApi::FileDocs(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<FileDocsResponse> = ApiRequest::get(&api_endpoint.to_url());

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

/// 获取文档文件信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDocsResponse {
    /// 文档文件信息
    pub data: Option<FileDocsData>,
}

impl ApiResponseTrait for FileDocsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
