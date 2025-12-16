use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 查询筛选条件
///
/// 查询一个筛选视图的所有筛选条件，返回筛选视图的筛选范围内的筛选条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/query
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 查询筛选条件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterConditionsResponse {
    /// 筛选条件信息
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for QueryFilterConditionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询筛选条件
///
/// 查询一个筛选视图的所有筛选条件，返回筛选视图的筛选范围内的筛选条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/query
pub async fn query_filter_conditions(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<QueryFilterConditionsResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::QueryFilterConditions(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<QueryFilterConditionsResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查询筛选条件")
}
