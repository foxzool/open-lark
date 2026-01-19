/// 更新筛选
///
/// 更新子表筛选范围中的列筛选条件。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/update
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

/// 更新筛选请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterRequest {
    pub col: String,
    pub condition: Condition,
}

/// 更新筛选响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFilterResponse {}

impl ApiResponseTrait for UpdateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选
pub async fn update_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: UpdateFilterRequest,
) -> SDKResult<UpdateFilterResponse> {
    update_filter_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 更新筛选（带选项）
pub async fn update_filter_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: UpdateFilterRequest,
    option: RequestOption,
) -> SDKResult<UpdateFilterResponse> {
    let api_endpoint =
        SheetsApiV3::UpdateFilter(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<UpdateFilterResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&params, "更新筛选")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新筛选")
}
