/// 获取筛选条件
///
/// 获取筛选视图某列的筛选条件信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/get
/// doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
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
    let api_endpoint = SheetsApiV3::GetFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
        condition_id.to_string(),
    );
    let api_request: ApiRequest<GetFilterConditionResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取筛选条件")
}

