/// 更新筛选视图
///
/// 更新筛选视图的名字或者筛选范围。名字长度不超过100，不能重复即子表内唯一；筛选范围不超过子表的最大范围。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/patch
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

/// 更新筛选视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewRequest {
    /// 筛选视图名称，可选，长度不超过100
    pub title: Option<String>,
    /// 筛选范围，可选
    pub range: Option<String>,
}

/// 更新筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewResponse {
    /// 筛选视图信息
    pub data: Option<FilterViewData>,
}

impl ApiResponseTrait for UpdateFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选视图
///
/// 更新筛选视图的名字或者筛选范围。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/patch
pub async fn update_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    params: UpdateFilterViewRequest,
) -> SDKResult<UpdateFilterViewResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UpdateFilterView(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<UpdateFilterViewResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(json!(params));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新筛选视图")
}
