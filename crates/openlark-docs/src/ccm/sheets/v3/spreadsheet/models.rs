/// Sheets电子表格数据模型
use serde::{Deserialize, Serialize};

/// 创建电子表格请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetParams {
    /// 电子表格标题，长度限制：1-200字符
    pub title: String,
    /// 工作表初始配置
    pub sheets: Option<Vec<SheetProperty>>,
    /// 时区设置
    pub time_zone: Option<String>,
    /// 语言设置
    pub locale: Option<String>,
    /// 文件夹路径
    pub folder_path: Option<String>,
}

/// 工作表属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProperty {
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表ID
    pub sheet_id: Option<String>,
    /// 工作表颜色
    pub sheet_color: Option<String>,
    /// 是否隐藏
    pub hidden: Option<bool>,
    /// 网格属性
    pub grid_properties: Option<GridProperties>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 冻结行数
    pub frozen_row_count: Option<i32>,
    /// 冻结列数
    pub frozen_column_count: Option<i32>,
    /// 是否隐藏网格线
    pub hide_gridlines: Option<bool>,
}

/// 创建电子表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetResponse {
    /// 电子表格信息
    pub data: Option<SpreadsheetData>,
}

/// 电子表格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetData {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 电子表格标题
    pub title: String,
    /// 时区
    pub time_zone: Option<String>,
    /// 语言
    pub locale: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
}

/// 更新电子表格请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetParams {
    /// 电子表格标题
    pub title: Option<String>,
    /// 时区设置
    pub time_zone: Option<String>,
    /// 语言设置
    pub locale: Option<String>,
}

/// 获取电子表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetResponse {
    /// 电子表格信息
    pub data: Option<SpreadsheetData>,
}

/// 工作表查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySheetResponse {
    /// 工作表信息
    pub data: Option<SheetQueryData>,
}

/// 工作表查询数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetQueryData {
    /// 工作表列表
    pub sheets: Option<Vec<SheetProperty>>,
}

/// 获取单个工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetResponse {
    /// 工作表信息
    pub data: Option<SheetProperty>,
}

/// 移动维度请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionParams {
    /// 移动的维度类型：ROWS（行）或 COLUMNS（列）
    pub dimension: String,
    /// 起始索引
    pub source_index: i32,
    /// 目标索引
    pub destination_index: i32,
}

/// 移动维度响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionResponse {
    /// 操作结果
    pub data: Option<serde_json::Value>,
}

/// 查找替换请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceParams {
    /// 查找内容
    pub find: String,
    /// 替换内容
    pub replacement: String,
    /// 查找范围
    pub range: Option<String>,
    /// 是否区分大小写
    pub case_sensitive: Option<bool>,
    /// 是否完全匹配
    pub match_entire_cell: Option<bool>,
}

/// 查找替换响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceResponse {
    /// 查找替换结果
    pub data: Option<FindReplaceData>,
}

/// 查找替换数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceData {
    /// 找到的单元格数量
    pub cells_updated: i32,
    /// 查找的数量
    pub cells_found: i32,
}

/// 更新电子表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetResponse {
    /// 更新结果
    pub data: Option<SpreadsheetData>,
}
