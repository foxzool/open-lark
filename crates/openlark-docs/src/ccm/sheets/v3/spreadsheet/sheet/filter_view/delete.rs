use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 删除筛选视图
///
/// 删除指定 id 对应的筛选视图。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/delete
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 删除筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterViewResponse {
    /// 删除结果
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选视图
///
/// 删除指定 id 对应的筛选视图。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/delete
pub async fn delete_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<DeleteFilterViewResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DeleteFilterView(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<DeleteFilterViewResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除筛选视图")
}
