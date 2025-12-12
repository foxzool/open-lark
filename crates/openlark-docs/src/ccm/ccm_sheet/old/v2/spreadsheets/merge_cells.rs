//! 合并单元格 API
//!
/// 合并单元格 - 根据 spreadsheetToken 和维度信息合并单元格
///
/// 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/merge-cells
///
/// # 参数说明
/// - `spreadsheet_token`: 电子表格令牌
/// - `params`: 合并单元格请求参数
///
/// # 返回值
/// 返回合并单元格的结果

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 合并单元格请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsParams {
    /// 合并范围
    #[serde(rename = "merge_type")]
    pub merge_type: String,
    /// 范围信息
    pub range: String,
    /// 合并方向（可选）
    #[serde(rename = "merge_direction", skip_serializing_if = "Option::is_none")]
    pub merge_direction: Option<String>,
}

/// 合并单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsResponse {
    /// 合并结果
    pub data: Option<MergeResult>,
}

/// 合并结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 合并的范围
    #[serde(rename = "merged_range")]
    pub merged_range: String,
    /// 合并的行数
    #[serde(rename = "merged_rows")]
    pub merged_rows: i32,
    /// 合并的列数
    #[serde(rename = "merged_columns")]
    pub merged_columns: i32,
    /// 合并的单元格数
    #[serde(rename = "merged_cells")]
    pub merged_cells: i32,
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 合并单元格
///
/// 根据 spreadsheetToken 和维度信息合并单元格；单次操作不超过5000行，100列。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::merge_cells::*;
///
/// let params = MergeCellsParams {
///     merge_type: "MERGE_ALL".to_string(),
///     range: "Sheet1!A1:C3".to_string(),
///     merge_direction: Some("ROWS".to_string()),
/// };
///
/// let result = merge_cells(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn merge_cells(
    config: &Config,
    spreadsheet_token: &str,
    params: MergeCellsParams,
) -> SDKResult<MergeCellsResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;
    validate_required_field("合并类型", Some(&params.merge_type), "合并类型不能为空")?;
    validate_required_field("范围", Some(&params.range), "范围不能为空")?;

    // 验证合并类型
    if !["MERGE_ALL", "MERGE_ROWS", "MERGE_COLUMNS", "UNMERGE"].contains(&params.merge_type.as_str()) {
        return Err(openlark_core::error::CoreError::validation(
            "merge_type",
            "合并类型必须是 MERGE_ALL、MERGE_ROWS、MERGE_COLUMNS 或 UNMERGE 之一"
        ));
    }

    // 验证合并方向（如果提供）
    if let Some(ref direction) = params.merge_direction {
        if !["ROWS", "COLUMNS"].contains(&direction.as_str()) {
            return Err(openlark_core::error::CoreError::validation(
                "merge_direction",
                "合并方向必须是 ROWS 或 COLUMNS"
            ));
        }
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::MergeCells(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<MergeCellsResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&params, "合并单元格")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "合并单元格")
}

/// 拆分单元格
///
/// 根据 spreadsheetToken 和维度信息拆分单元格；单次操作不超过5000行，100列。
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::ccm_sheet::old::v2::spreadsheets::merge_cells::*;
///
/// let params = MergeCellsParams {
///     merge_type: "UNMERGE".to_string(),
///     range: "Sheet1!A1:C3".to_string(),
/// };
///
/// let result = unmerge_cells(&config, "spreadsheet_token", params).await?;
/// ```
pub async fn unmerge_cells(
    config: &Config,
    spreadsheet_token: &str,
    params: MergeCellsParams,
) -> SDKResult<MergeCellsResponse> {
    // 验证必填字段
    validate_required_field("表格Token", Some(spreadsheet_token), "表格Token不能为空")?;
    validate_required_field("范围", Some(&params.range), "范围不能为空")?;

    // 强制设置合并类型为UNMERGE
    let mut unmerge_params = params;
    unmerge_params.merge_type = "UNMERGE".to_string();

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::UnmergeCells(spreadsheet_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<MergeCellsResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&unmerge_params, "拆分单元格")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "拆分单元格")
}