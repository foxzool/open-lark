/// 查询筛选条件
///
/// 查询一个筛选视图的所有筛选条件，返回筛选视图的筛选范围内的筛选条件。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/query
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

/// 查询筛选条件响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterConditionsResponse {
    pub items: Vec<Condition>,
}

impl ApiResponseTrait for QueryFilterConditionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询筛选条件
pub async fn query_filter_conditions(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<QueryFilterConditionsResponse> {
    query_filter_conditions_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        RequestOption::default(),
    )
    .await
}

/// 查询筛选条件（支持请求选项）
pub async fn query_filter_conditions_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    option: RequestOption,
) -> SDKResult<QueryFilterConditionsResponse> {
    let api_endpoint = SheetsApiV3::QueryFilterConditions(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<QueryFilterConditionsResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "查询筛选条件")
}
