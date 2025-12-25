/// 获取筛选视图
///
/// 获取指定筛选视图 id 的名字和筛选范围。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/get
/// doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::FilterView;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 获取筛选视图响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterViewResponse {
    pub filter_view: FilterView,
}

impl ApiResponseTrait for GetFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选视图
pub async fn get_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<GetFilterViewResponse> {
    let api_endpoint = SheetsApiV3::GetFilterView(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<GetFilterViewResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取筛选视图")
}

