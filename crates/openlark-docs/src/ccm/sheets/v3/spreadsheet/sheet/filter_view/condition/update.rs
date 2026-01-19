/// 更新筛选条件
///
/// 更新筛选视图范围的某列的筛选条件，condition_id 即为列的字母号。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/update
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

/// 更新筛选条件请求体（字段在文档中标注为非必填，可按需传入）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFilterConditionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<Vec<String>>,
}

/// 更新筛选条件响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterConditionResponse {
    pub condition: Condition,
}

impl ApiResponseTrait for UpdateFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选条件
pub async fn update_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
    params: UpdateFilterConditionRequest,
) -> SDKResult<UpdateFilterConditionResponse> {
    let api_endpoint = SheetsApiV3::UpdateFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
        condition_id.to_string(),
    );
    let api_request: ApiRequest<UpdateFilterConditionResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&params, "更新筛选条件")?);

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新筛选条件")
}
