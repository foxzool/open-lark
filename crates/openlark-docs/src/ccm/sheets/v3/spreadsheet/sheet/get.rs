/// 获取工作表信息
///
/// 获取指定工作表的详细信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::super::models::*;
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};


impl ApiResponseTrait for GetSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取工作表信息
///
/// 获取指定工作表的详细信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/get
pub async fn get_sheet(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<GetSheetResponse> {
    // 构建请求参数
    let params = serde_json::json!({
        "sheet_id": sheet_id
    });

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateSheet(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<GetSheetResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(params);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取工作表信息")
}
