use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 查询筛选视图
///
/// 查询子表内所有的筛选视图基本信息，包括 id、name 和 range
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/query
use serde::{Deserialize, Serialize};

use super::create::*;
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 查询筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterViewsResponse {
    /// 筛选视图列表
    pub data: Option<QueryFilterViewsData>,
}

/// 查询筛选视图数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterViewsData {
    /// 筛选视图项目
    pub items: Vec<FilterViewData>,
}

impl ApiResponseTrait for QueryFilterViewsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询筛选视图
///
/// 查询子表内所有的筛选视图基本信息，包括 id、name 和 range。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/query
pub async fn query_filter_views(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<QueryFilterViewsResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint =
        CcmSheetApiOld::QueryFilterViews(spreadsheet_token.to_string(), sheet_id.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<QueryFilterViewsResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查询筛选视图")
}
