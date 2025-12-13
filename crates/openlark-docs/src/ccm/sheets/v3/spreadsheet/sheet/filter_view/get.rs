use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取筛选视图
///
/// 获取指定筛选视图 id 的名字和筛选范围。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/get
use serde::{Deserialize, Serialize};

use super::create::*;
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 获取筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterViewResponse {
    /// 筛选视图信息
    pub data: Option<FilterViewData>,
}

impl ApiResponseTrait for GetFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选视图
///
/// 获取指定筛选视图 id 的名字和筛选范围。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/get
pub async fn get_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<GetFilterViewResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetFilterView(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<GetFilterViewResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取筛选视图")
}
