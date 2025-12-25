/// 查找单元格
///
/// 在指定范围内查找符合条件的单元格。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/find
/// doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/find
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::super::models::*;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

impl ApiResponseTrait for FindResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查找单元格
pub async fn find_cells(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: FindParams,
) -> SDKResult<FindResponse> {
    let api_endpoint = SheetsApiV3::FindCells(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<FindResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "查找单元格")?);

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查找单元格")
}

