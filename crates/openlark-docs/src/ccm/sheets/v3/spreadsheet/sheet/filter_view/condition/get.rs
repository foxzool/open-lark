/// 获取筛选条件
///
/// 获取筛选视图某列的筛选条件信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::Condition;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 获取筛选条件响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterConditionResponse {
    pub condition: Condition,
}

impl ApiResponseTrait for GetFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选条件
pub async fn get_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
) -> SDKResult<GetFilterConditionResponse> {
    get_filter_condition_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        condition_id,
        RequestOption::default(),
    )
    .await
}

/// 获取筛选条件（带请求选项）
pub async fn get_filter_condition_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
    option: RequestOption,
) -> SDKResult<GetFilterConditionResponse> {
    let api_endpoint = SheetsApiV3::GetFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
        condition_id.to_string(),
    );
    let api_request: ApiRequest<GetFilterConditionResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取筛选条件")
}
