/// 删除筛选
///
/// 删除子表的筛选。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/delete
/// doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 删除筛选响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFilterResponse {}

impl ApiResponseTrait for DeleteFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选
pub async fn delete_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<DeleteFilterResponse> {
    let api_endpoint =
        SheetsApiV3::DeleteFilter(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<DeleteFilterResponse> = ApiRequest::delete(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除筛选")
}

