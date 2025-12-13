/// 获取表格文件信息
///
/// 此接口用于获取指定表格文件的详细信息，包括工作表、单元格等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/explorer/file_spreadsheets
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{
    requests::FileSpreadsheetsRequest,
    responses::FileSpreadsheetsData,
};

/// 获取表格文件信息
pub async fn file_spreadsheets(
    request: FileSpreadsheetsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<FileSpreadsheetsData> {
    use crate::common::api_endpoints::CcmDriveExplorerApi;

    // 使用CcmDriveExplorerApi枚举生成API端点
    let api_endpoint = CcmDriveExplorerApi::FileSpreadsheets(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<FileSpreadsheetsResponse> = ApiRequest::get(&api_endpoint.to_url());

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

/// 获取表格文件信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSpreadsheetsResponse {
    /// 表格文件信息
    pub data: Option<FileSpreadsheetsData>,
}

impl ApiResponseTrait for FileSpreadsheetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
