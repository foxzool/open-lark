/// 更新筛选视图
///
/// 更新筛选视图的名字或者筛选范围。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/patch
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::FilterView;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 更新筛选视图请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

/// 更新筛选视图响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewResponse {
    pub filter_view: FilterView,
}

impl ApiResponseTrait for UpdateFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选视图
pub async fn update_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    params: UpdateFilterViewRequest,
) -> SDKResult<UpdateFilterViewResponse> {
    update_filter_view_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        filter_view_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 更新筛选视图（支持自定义选项）
pub async fn update_filter_view_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    params: UpdateFilterViewRequest,
    option: RequestOption,
) -> SDKResult<UpdateFilterViewResponse> {
    let api_endpoint = SheetsApiV3::PatchFilterView(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );
    let api_request: ApiRequest<UpdateFilterViewResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serialize_params(&params, "更新筛选视图")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新筛选视图")
}
