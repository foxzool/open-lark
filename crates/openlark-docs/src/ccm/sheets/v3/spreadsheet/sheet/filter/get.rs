/// 获取筛选
///
/// 获取子表的详细筛选信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::SheetFilterInfo;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 获取筛选响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterResponse {
    pub sheet_filter_info: SheetFilterInfo,
}

impl ApiResponseTrait for GetFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选
pub async fn get_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<GetFilterResponse> {
    get_filter_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        RequestOption::default(),
    )
    .await
}

/// 获取筛选（带选项）
pub async fn get_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    option: RequestOption,
) -> SDKResult<GetFilterResponse> {
    let api_endpoint = SheetsApiV3::GetFilter(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<GetFilterResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取筛选")
}
