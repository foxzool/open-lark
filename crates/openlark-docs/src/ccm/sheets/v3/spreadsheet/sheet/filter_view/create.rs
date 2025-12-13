/// 创建筛选视图
///
/// 根据传入的参数创建一个筛选视图。Id 和 名字可选，不填的话会默认生成；range 必填。
/// Id 长度为10，由 0-9、a-z、A-Z 组合生成。名字长度不超过100。单个子表内的筛选视图个数不超过 150。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/create
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

/// 创建筛选视图请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewRequest {
    /// 筛选视图ID，可选，不填会自动生成
    pub filter_view_id: Option<String>,
    /// 筛选视图名称，可选，长度不超过100
    pub title: Option<String>,
    /// 筛选范围，必填
    pub range: String,
}

/// 创建筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewResponse {
    /// 筛选视图信息
    pub data: Option<FilterViewData>,
}

/// 筛选视图数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterViewData {
    /// 筛选视图ID
    pub filter_view_id: String,
    /// 筛选视图标题
    pub title: String,
    /// 筛选范围
    pub range: String,
}

impl ApiResponseTrait for CreateFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选视图
///
/// 根据传入的参数创建一个筛选视图。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/create
pub async fn create_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFilterViewRequest,
) -> SDKResult<CreateFilterViewResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint =
        CcmSheetApiOld::CreateFilterView(spreadsheet_token.to_string(), sheet_id.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<CreateFilterViewResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(json!(params));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建筛选视图")
}
