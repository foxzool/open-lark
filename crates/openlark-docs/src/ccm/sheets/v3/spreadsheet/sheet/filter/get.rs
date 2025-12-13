use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取筛选
///
/// 获取工作表中的数据筛选信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/get
use serde::{Deserialize, Serialize};

use super::{FilterData, FilterRange, FilterCondition};
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

use serde_json::json;

/// 获取筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterResponse {
    /// 筛选信息
    pub data: Option<FilterData>,
}

impl ApiResponseTrait for GetFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选
///
/// 获取工作表中的数据筛选信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/get
pub async fn get_filter(
    config: &Config,
    spreadsheet_token: &str,
    filter_id: &str,
) -> SDKResult<GetFilterResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::GetFilter(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<GetFilterResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(json!({
            "filter_view_id": filter_id
        }));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取筛选")
}
