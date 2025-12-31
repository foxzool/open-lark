/// 创建电子表格
///
/// 创建并初始化一个新的电子表格。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::*;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

impl ApiResponseTrait for CreateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建电子表格
///
/// 创建并初始化一个新的电子表格。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create
pub async fn create_spreadsheet(
    config: &Config,
    params: CreateSpreadsheetParams,
) -> SDKResult<CreateSpreadsheetResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = SheetsApiV3::CreateSpreadsheet;

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<CreateSpreadsheetResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建电子表格")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建电子表格")
}
