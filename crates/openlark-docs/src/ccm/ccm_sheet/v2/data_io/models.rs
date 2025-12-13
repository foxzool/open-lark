/// CCM Sheet V2 数据读写模型
use serde::{Deserialize, Serialize};

/// 读取单个范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadSingleRangeParams {
    /// 范围参数，如 "Sheet1!A1:C10"
    #[serde(rename = "value_range")]
    pub value_range: String,
    /// 是否返回格式化值
    #[serde(rename = "value_render_option")]
    pub value_render_option: Option<String>,
    /// 日期格式化
    #[serde(rename = "date_render_option")]
    pub date_render_option: Option<String>,
}

/// 读取单个范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadSingleRangeResponse {
    /// 范围数据
    pub data: Option<RangeData>,
}

/// 范围数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeData {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 范围信息
    #[serde(rename = "value_range")]
    pub value_range: String,
    /// 主要维度
    #[serde(rename = "major_dimension")]
    pub major_dimension: String,
    /// 行数据
    pub values: Vec<Vec<serde_json::Value>>,
}

/// 读取多个范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadMultipleRangesParams {
    /// 范围列表
    pub ranges: Vec<String>,
    /// 是否返回格式化值
    #[serde(rename = "value_render_option")]
    pub value_render_option: Option<String>,
    /// 日期格式化
    #[serde(rename = "date_render_option")]
    pub date_render_option: Option<String>,
}

/// 读取多个范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadMultipleRangesResponse {
    /// 范围数据列表
    pub data: Option<MultipleRangeData>,
}

/// 多个范围数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleRangeData {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 范围数据列表
    #[serde(rename = "value_ranges")]
    pub value_ranges: Vec<RangeData>,
}

/// 写入单个范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteSingleRangeParams {
    /// 范围参数，如 "Sheet1!A1:C10"
    #[serde(rename = "range")]
    pub range: String,
    /// 主要维度
    #[serde(rename = "major_dimension")]
    pub major_dimension: Option<String>,
    /// 行数据
    pub values: Vec<Vec<serde_json::Value>>,
}

/// 写入单个范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteSingleRangeResponse {
    /// 更新结果
    pub data: Option<UpdateRangeResult>,
}

/// 更新范围结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRangeResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 更新的范围
    #[serde(rename = "updated_range")]
    pub updated_range: String,
    /// 更新的行数
    #[serde(rename = "updated_rows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updated_columns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updated_cells")]
    pub updated_cells: i32,
}

/// 批量写入范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchWriteRangesParams {
    /// 写入数据
    pub data: Vec<BatchWriteData>,
    /// 是否包含格式
    #[serde(rename = "include_style")]
    pub include_style: Option<bool>,
}

/// 批量写入数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchWriteData {
    /// 数据范围
    #[serde(rename = "data_range")]
    pub data_range: String,
    /// 主要维度
    #[serde(rename = "major_dimension")]
    pub major_dimension: String,
    /// 行数据
    pub values: Vec<Vec<serde_json::Value>>,
}

/// 批量写入范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchWriteRangesResponse {
    /// 批量更新结果
    pub data: Option<BatchUpdateResult>,
}

/// 批量更新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 更新结果列表
    #[serde(rename = "updated_ranges")]
    pub updated_ranges: Vec<UpdateRangeResult>,
    /// 总更新行数
    #[serde(rename = "total_updated_rows")]
    pub total_updated_rows: i32,
    /// 总更新列数
    #[serde(rename = "total_updated_columns")]
    pub total_updated_columns: i32,
    /// 总更新单元格数
    #[serde(rename = "total_updated_cells")]
    pub total_updated_cells: i32,
    /// 是否包含格式
    #[serde(rename = "total_updated_styles")]
    pub total_updated_styles: i32,
}

/// 追加数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendValuesParams {
    /// 追加范围
    #[serde(rename = "range")]
    pub range: String,
    /// 主要维度
    #[serde(rename = "major_dimension")]
    pub major_dimension: Option<String>,
    /// 追加的行数据
    pub values: Vec<Vec<serde_json::Value>>,
}

/// 追加数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendValuesResponse {
    /// 追加结果
    pub data: Option<AppendResult>,
}

/// 追加结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 更新的范围
    #[serde(rename = "updated_range")]
    pub updated_range: String,
    /// 更新的行数
    #[serde(rename = "updated_rows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updated_columns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updated_cells")]
    pub updated_cells: i32,
}

/// 插入数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertValuesParams {
    /// 插入范围
    #[serde(rename = "range")]
    pub range: String,
    /// 主要维度
    #[serde(rename = "major_dimension")]
    pub major_dimension: Option<String>,
    /// 插入的行数据
    pub values: Vec<Vec<serde_json::Value>>,
}

/// 插入数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertValuesResponse {
    /// 插入结果
    pub data: Option<InsertResult>,
}

/// 插入结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 更新的范围
    #[serde(rename = "updated_range")]
    pub updated_range: String,
    /// 更新的行数
    #[serde(rename = "updated_rows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updated_columns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updated_cells")]
    pub updated_cells: i32,
}

/// 写入图片请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesImageParams {
    /// 范围参数，如 "Sheet1!A1:C10"
    #[serde(rename = "range")]
    pub range: String,
    /// 主要维度
    #[serde(rename = "major_dimension")]
    pub major_dimension: Option<String>,
    /// 图片数据
    pub values: Vec<ImageValue>,
}

/// 图片值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageValue {
    /// 图片数据
    #[serde(rename = "image_key")]
    pub image_key: String,
    /// 替代文本
    #[serde(rename = "alternative_text")]
    pub alternative_text: Option<String>,
}

/// 写入图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesImageResponse {
    /// 图片写入结果
    pub data: Option<ImageResult>,
}

/// 图片写入结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResult {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 更新的范围
    #[serde(rename = "updated_range")]
    pub updated_range: String,
    /// 更新的行数
    #[serde(rename = "updated_rows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updated_columns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updated_cells")]
    pub updated_cells: i32,
}
