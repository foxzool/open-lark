/// 创建筛选条件
///
/// 在筛选视图的筛选范围的某一列创建筛选条件。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/create
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

/// 创建筛选条件请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterConditionRequest {
    /// 设置筛选条件的列，用字母表示（如 "E"）
    pub condition_id: String,
    pub filter_type: String,
    pub compare_type: String,
    pub expected: Vec<String>,
}

/// 创建筛选条件响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterConditionResponse {
    pub condition: Condition,
}

impl ApiResponseTrait for CreateFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选条件
pub async fn create_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    params: CreateFilterConditionRequest,
) -> SDKResult<CreateFilterConditionResponse> {
    let api_endpoint = SheetsApiV3::CreateFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<CreateFilterConditionResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建筛选条件")?);

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建筛选条件")
}
