/// 获取电子表格信息
///
/// 获取指定电子表格的详细信息，包括基本属性和配置。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::*;
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

impl ApiResponseTrait for GetSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取电子表格信息
///
/// 获取指定电子表格的详细信息，包括基本属性和配置。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/get
pub async fn get_spreadsheet(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<GetSpreadsheetResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetSpreadsheet(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<GetSpreadsheetResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取电子表格信息")
}
