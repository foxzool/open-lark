/// 查询筛选视图
///
/// 查询子表内所有的筛选视图基本信息，包括 id、name 和 range。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/query
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::FilterView;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 查询筛选视图响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterViewsResponse {
    pub items: Vec<FilterView>,
}

impl ApiResponseTrait for QueryFilterViewsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询筛选视图
pub async fn query_filter_views(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<QueryFilterViewsResponse> {
    let api_endpoint =
        SheetsApiV3::QueryFilterViews(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<QueryFilterViewsResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查询筛选视图")
}
