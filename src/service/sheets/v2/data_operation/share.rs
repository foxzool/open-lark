use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};

#[derive(Serialize, Debug, Default)]
pub(crate) struct ValueRangeRequest {
    /// 插入范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见
    /// 在线表格开发指南，range所表示的范围需要大于等于values占用的范围。
    pub(crate) range: String,
    /// 需要写入的值，如要写入公式、超链接、email、@人等，可详看附录sheet 支持写入数据类型
    pub(crate) values: Value,
}

#[derive(Deserialize, Debug, Default)]
#[allow(dead_code)]
pub(crate) struct ValueRangeResponse {
    /// 插入维度
    #[serde(rename = "majorDimension")]
    pub(crate) major_dimension: String,
    /// 插入范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见
    /// 在线表格开发指南，range所表示的范围需要大于等于values占用的范围。
    pub(crate) range: String,
    /// 需要写入的值，如要写入公式、超链接、email、@人等，可详看附录sheet 支持写入数据类型
    pub(crate) values: Value,
    /// sheet 的版本号
    pub(crate) revision: i32,
}

/// 追加数据响应体
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct AppendDataResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    spreed_sheet_token: String,
    /// 写入的范围
    #[serde(rename = "tableRange")]
    table_range: String,
    /// sheet 的版本号
    revision: i32,
    /// 追加数据的范围、行列数等
    updates: AppendDataUpdate,
}

/// 追加数据的范围、行列数等
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct AppendDataUpdate {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    spreed_sheet_token: String,
    /// 写入的范围
    #[serde(rename = "updatedRange")]
    updated_range: String,
    /// 写入的行数
    #[serde(rename = "updatedRows")]
    updated_rows: i32,
    /// 写入的列数
    #[serde(rename = "updatedColumns")]
    updated_columns: i32,
    /// 写入的单元格总数
    #[serde(rename = "updatedCells")]
    updated_cells: i32,
    /// sheet 的版本号
    revision: i32,
}

impl ApiResponseTrait for AppendDataResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 值与范围
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ReadRangeValueRange {
    /// 插入维度
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
    /// 返回数据的范围，为空时表示查询范围没有数据
    pub range: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 查询得到的值
    pub values: Value,
}
