/// 删除筛选条件
///
/// 删除筛选视图的筛选范围某一列的筛选条件。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 删除筛选条件响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFilterConditionResponse {}

impl ApiResponseTrait for DeleteFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选条件
pub async fn delete_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
) -> SDKResult<DeleteFilterConditionResponse> {
    delete_filter_condition_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        condition_id,
        RequestOption::default(),
    )
    .await
}

/// 删除筛选条件（支持请求选项）
pub async fn delete_filter_condition_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
    option: RequestOption,
) -> SDKResult<DeleteFilterConditionResponse> {
    let api_endpoint = SheetsApiV3::DeleteFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
        condition_id.to_string(),
    );
    let api_request: ApiRequest<DeleteFilterConditionResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "删除筛选条件")
}
