use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取筛选条件
///
/// 获取筛选视图某列的筛选条件信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/get
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 获取筛选条件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterConditionResponse {
    /// 筛选条件信息
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选条件
///
/// 获取筛选视图某列的筛选条件信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/get
pub async fn get_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
) -> SDKResult<GetFilterConditionResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
        condition_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<GetFilterConditionResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取筛选条件")
}
