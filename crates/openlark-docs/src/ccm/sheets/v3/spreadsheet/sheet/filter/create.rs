/// 创建筛选
///
/// 在子表内创建筛选。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/create
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

/// 创建筛选请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterRequest {
    pub range: String,
    pub col: String,
    pub condition: Condition,
}

/// 创建筛选响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFilterResponse {}

impl ApiResponseTrait for CreateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选
pub async fn create_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFilterRequest,
) -> SDKResult<CreateFilterResponse> {
    let api_endpoint =
        SheetsApiV3::CreateFilter(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<CreateFilterResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建筛选")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "创建筛选")
}
