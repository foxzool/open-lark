/// 删除筛选视图
///
/// 删除指定 id 对应的筛选视图。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 删除筛选视图响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFilterViewResponse {}

impl ApiResponseTrait for DeleteFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选视图
pub async fn delete_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<DeleteFilterViewResponse> {
    delete_filter_view_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        RequestOption::default(),
    )
    .await
}

/// 删除筛选视图（支持请求选项）
pub async fn delete_filter_view_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    option: RequestOption,
) -> SDKResult<DeleteFilterViewResponse> {
    let api_endpoint = SheetsApiV3::DeleteFilterView(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<DeleteFilterViewResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "删除筛选视图")
}
