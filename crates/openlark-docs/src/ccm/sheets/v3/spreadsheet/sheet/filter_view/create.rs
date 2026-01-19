/// 创建筛选视图
///
/// 根据传入的参数创建一个筛选视图。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/create
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

/// 创建筛选视图请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewRequest {
    /// 自定义筛选视图 ID，不传由系统生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_view_id: Option<String>,
    /// 自定义筛选视图名称，不传由系统生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_view_name: Option<String>,
    /// 筛选视图的筛选范围（文档中标注“否”，但实际必填）
    pub range: String,
}

/// 创建筛选视图响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewResponse {
    pub filter_view: FilterView,
}

impl ApiResponseTrait for CreateFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选视图
pub async fn create_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFilterViewRequest,
) -> SDKResult<CreateFilterViewResponse> {
    create_filter_view_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 创建筛选视图（支持自定义选项）
pub async fn create_filter_view_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFilterViewRequest,
    option: RequestOption,
) -> SDKResult<CreateFilterViewResponse> {
    let api_endpoint =
        SheetsApiV3::CreateFilterView(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<CreateFilterViewResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建筛选视图")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "创建筛选视图")
}
