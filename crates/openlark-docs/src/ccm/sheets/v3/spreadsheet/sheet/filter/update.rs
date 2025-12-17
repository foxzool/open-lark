/// 更新筛选
///
/// 更新现有的数据筛选条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/update
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::{CreateFilterRequest, FilterData};
use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

// 导入序列化支持
use serde::{Deserialize, Serialize};

/// 更新筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterResponse {
    /// 筛选信息
    pub data: Option<FilterData>,
}

impl ApiResponseTrait for UpdateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选
///
/// 更新现有的数据筛选条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/update
pub async fn update_filter(
    config: &Config,
    spreadsheet_token: &str,
    filter_id: &str,
    params: CreateFilterRequest,
) -> SDKResult<UpdateFilterResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateFilter(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let mut request_body = serde_json::json!({
        "filter_view_id": filter_id
    });

    // 合并其他参数
    if let Ok(params_obj) = serde_json::to_value(params) {
        if let Some(params_map) = params_obj.as_object() {
            for (key, value) in params_map {
                request_body[key] = value.clone();
            }
        }
    }

    let api_request: ApiRequest<UpdateFilterResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(request_body);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新筛选")
}
