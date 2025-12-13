/// 创建筛选条件
///
/// 在筛选视图的筛选范围的某一列创建筛选条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/create
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

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// 字段索引
    pub column_index: i32,
    /// 筛选操作符
    pub operator: String,
    /// 筛选值
    pub value: Option<serde_json::Value>,
    /// 是否隐藏值
    pub hide_if_true: Option<bool>,
}

/// 筛选条件组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConditionGroup {
    /// 组内条件之间的关系：And 或 Or
    pub conjunction: String,
    /// 条件列表
    pub conditions: Vec<FilterCondition>,
}

/// 创建筛选条件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterConditionRequest {
    /// 筛选条件列表
    pub condition_groups: Vec<FilterConditionGroup>,
}

/// 创建筛选条件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterConditionResponse {
    /// 筛选条件信息
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选条件
///
/// 在筛选视图的筛选范围的某一列创建筛选条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/spreadsheet-sheet-filter_view-condition/create
pub async fn create_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    params: CreateFilterConditionRequest,
) -> SDKResult<CreateFilterConditionResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::CreateFilterCondition(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        filter_view_id.to_string(),
    );

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<CreateFilterConditionResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(json!(params));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建筛选条件")
}
