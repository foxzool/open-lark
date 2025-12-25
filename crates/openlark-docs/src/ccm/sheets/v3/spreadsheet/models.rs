/// Sheets 电子表格 v3 数据模型
///
/// 注意：这些结构体对应「电子表格」相关接口响应体中的 data 字段结构。
use serde::{Deserialize, Serialize};

// ============================================================================
// spreadsheet
// ============================================================================

/// 创建电子表格请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSpreadsheetParams {
    /// 表格标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文件夹 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

/// 创建电子表格响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetResponse {
    pub spreadsheet: CreatedSpreadsheet,
}

/// 创建电子表格返回的表格信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedSpreadsheet {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    pub url: String,
    pub spreadsheet_token: String,
}

/// 修改电子表格属性请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSpreadsheetParams {
    /// 新的电子表格标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 修改电子表格属性响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSpreadsheetResponse {}

/// 获取电子表格信息响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetResponse {
    pub spreadsheet: SpreadsheetInfo,
}

/// 电子表格信息（获取电子表格信息接口返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetInfo {
    pub title: String,
    pub owner_id: String,
    /// 表格 token（等价 spreadsheet_token）
    pub token: String,
    pub url: String,
}

// ============================================================================
// spreadsheet.sheet
// ============================================================================

/// 获取工作表列表响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySheetResponse {
    pub sheets: Vec<Sheet>,
}

/// 查询工作表响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetResponse {
    pub sheet: Sheet,
}

/// 工作表属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub sheet_id: String,
    pub title: String,
    pub index: i32,
    pub hidden: bool,
    pub grid_properties: GridProperties,
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merges: Option<Vec<MergeRange>>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    pub frozen_row_count: i32,
    pub frozen_column_count: i32,
    pub row_count: i32,
    pub column_count: i32,
}

/// 合并单元格范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRange {
    pub start_row_index: i32,
    pub end_row_index: i32,
    pub start_column_index: i32,
    pub end_column_index: i32,
}

/// 移动行列请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionParams {
    /// 移动源位置信息
    pub source: DimensionSource,
    /// 移动的目标位置行或者列
    pub destination_index: i32,
}

/// 移动源位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionSource {
    /// ROWS 或 COLUMNS
    pub major_dimension: String,
    pub start_index: i32,
    pub end_index: i32,
}

/// 移动行列响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MoveDimensionResponse {}

// ============================================================================
// spreadsheet.sheet.find / replace
// ============================================================================

/// 查找条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCondition {
    pub range: String,
    pub match_case: bool,
    pub match_entire_cell: bool,
    pub search_by_regex: bool,
    pub include_formulas: bool,
}

/// 查找单元格请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindParams {
    pub find_condition: FindCondition,
    pub find: String,
}

/// 查找单元格响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindResponse {
    pub find_result: FindResult,
}

/// 替换单元格请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceParams {
    pub find_condition: FindCondition,
    pub find: String,
    pub replacement: String,
}

/// 替换单元格响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceResponse {
    pub replace_result: FindResult,
}

/// 查找/替换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindResult {
    pub matched_cells: Vec<String>,
    pub matched_formula_cells: Vec<String>,
    pub rows_count: i32,
}
