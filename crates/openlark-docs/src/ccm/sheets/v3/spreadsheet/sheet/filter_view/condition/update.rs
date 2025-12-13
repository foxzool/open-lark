/// 更新筛选条件
///
/// 更新筛选视图范围的某列的筛选条件，condition id 即为列的字母号。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/update
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde_json::json;

use super::create::*;
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

// 导入序列化支持
use serde::{Deserialize, Serialize};

/// 更新筛选条件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterConditionResponse {
    /// 筛选条件信息
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选条件
///
/// 更新筛选视图范围的某列的筛选条件，condition id 即为列的字母号。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/update
pub async fn update_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    condition_id: &str,
    params: CreateFilterConditionRequest,
) -> SDKResult<UpdateFilterConditionResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
        condition_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<UpdateFilterConditionResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(json!(params));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新筛选条件")
}
