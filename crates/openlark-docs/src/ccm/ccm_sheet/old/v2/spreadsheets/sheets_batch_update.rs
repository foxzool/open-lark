/// 操作工作表 API
///
/// 操作工作表 - 根据 spreadsheetToken 更新工作表属性
///
/// 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/operate-sheets
///
/// # 参数说明
/// - `spreadsheet_token`: 电子表格令牌
/// - `params`: 操作工作表请求参数
///
/// # 返回值
/// 返回操作工作表的结果
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 操作工作表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperateSheetsParams {
    /// 工作表操作请求
    pub requests: Vec<SheetOperation>,
}

/// 工作表操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetOperation {
    /// 操作类型
    #[serde(rename = "operate_type")]
    pub operate_type: String,
    /// 工作表属性（可选）
    #[serde(rename = "sheet_properties", skip_serializing_if = "Option::is_none")]
    pub sheet_properties: Option<SheetProperties>,
}

/// 工作表属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProperties {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: i32,
    /// 工作表标题
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 索引
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 是否隐藏
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

/// 操作工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperateSheetsResponse {
    /// 更新结果
    pub data: Option<SheetUpdateResult>,
}

/// 工作表更新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetUpdateResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 更新的工作表结果列表
    #[serde(rename = "updated_sheets")]
    pub updated_sheets: Vec<UpdatedSheet>,
}

/// 更新的工作表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedSheet {
    /// 工作表属性
    #[serde(rename = "sheet_properties")]
    pub sheet_properties: SheetProperties,
}

impl ApiResponseTrait for OperateSheetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 操作工作表
///
/// 根据 spreadsheetToken 更新工作表属性。支持批量操作多个工作表。
/// 单次操作最多支持100个工作表。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::sheets_batch_update::*;
///
/// let params = OperateSheetsParams {
///     requests: vec![
///         SheetOperation {
///             operate_type: "update".to_string(),
///             sheet_properties: Some(SheetProperties {
///                 sheet_id: 0,
///                 title: Some("新标题".to_string()),
///                 index: Some(1),
///                 hidden: Some(false),
///             }),
///         }
///     ],
/// };
///
/// let result = operate_sheets(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn operate_sheets(
    config: &Config,
    spreadsheet_token: &str,
    params: OperateSheetsParams,
) -> SDKResult<OperateSheetsResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;

    if params.requests.is_empty() {
        return Err(openlark_core::error::CoreError::validation(
            "requests",
            "操作请求列表不能为空",
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::OperateSheets(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<OperateSheetsResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "操作工作表")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "操作工作表")
}
