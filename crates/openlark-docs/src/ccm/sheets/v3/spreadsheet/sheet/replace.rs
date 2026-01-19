/// 替换单元格
///
/// 按照指定的条件查找指定范围内的单元格并替换值。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/replace
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use super::super::models::*;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

impl ApiResponseTrait for FindReplaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 替换单元格
pub async fn replace_cells(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: FindReplaceParams,
) -> SDKResult<FindReplaceResponse> {
    let api_endpoint =
        SheetsApiV3::ReplaceCells(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<FindReplaceResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "替换单元格")?);

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "替换单元格")
}
