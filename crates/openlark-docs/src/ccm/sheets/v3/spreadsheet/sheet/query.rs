/// 查询工作表
///
/// 查询指定电子表格中的所有工作表信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/query
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::super::models::*;
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};


impl ApiResponseTrait for QuerySheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询工作表
///
/// 查询指定电子表格中的所有工作表信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/query
pub async fn query_sheets(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<QuerySheetResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetSheet(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<QuerySheetResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!({}));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查询工作表")
}
