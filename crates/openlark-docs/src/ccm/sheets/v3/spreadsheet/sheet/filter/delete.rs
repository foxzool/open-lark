/// 删除筛选
///
/// 删除工作表中的数据筛选条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde_json::json;

// 导入序列化支持
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};
use serde::{Deserialize, Serialize};

/// 删除筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterResponse {
    /// 删除结果
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选
///
/// 删除工作表中的数据筛选条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/delete
pub async fn delete_filter(
    config: &Config,
    spreadsheet_token: &str,
    filter_id: &str,
) -> SDKResult<DeleteFilterResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::DeleteFilter(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<DeleteFilterResponse> = ApiRequest::post(&api_endpoint.to_url())
        .body(json!({
            "filter_view_id": filter_id
        }));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除筛选")
}
