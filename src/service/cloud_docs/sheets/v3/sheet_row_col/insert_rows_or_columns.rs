#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
    service::sheets::v3::SheetRowColService,
};
impl SheetRowColService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 插入行列请求,
#[derive(Debug, Clone)]
pub struct InsertRowsOrColumnsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 插入位置的维度信息
    dimension_range: DimensionRange,
    /// 是否继承样式
    inherit_style: Option<String>}
impl InsertRowsOrColumnsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct InsertRowsOrColumnsRequestBuilder {
    request: InsertRowsOrColumnsRequest}
impl InsertRowsOrColumnsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 维度范围,
#[derive(Debug, Clone)]
pub struct DimensionRange {
    /// 维度类型：ROWS（行）或 COLUMNS（列）
    pub dimension: String,
    /// 起始索引
    pub start_index: i32,
    /// 结束索引
    pub end_index: i32,
/// 插入行列响应体最外层,
#[derive(Debug, Clone)]
pub struct InsertRowsOrColumnsResponseData {
    /// 插入行列后的信息
    pub insert_range: InsertRangeInfo,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 插入范围信息,
#[derive(Debug, Clone)]
pub struct InsertRangeInfo {
    /// 插入的维度
    pub dimension: String,
    /// 插入的起始位置
    pub start_index: i32,
    /// 插入的结束位置
    pub end_index: i32,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use super::InsertRowsOrColumnsResponseData;
    #[test]
fn test_insert_rows_or_columns_response() {
        let json = json!({,
"insert_range": {,
                "dimension": "COLUMNS",
                "start_index": 3,
                "end_index": 5}
        });
let response: InsertRowsOrColumnsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.insert_range.dimension, "COLUMNS");
        assert_eq!(response.insert_range.start_index, 3);
        assert_eq!(response.insert_range.end_index, 5);
// 实现ExecutableBuilder trait,
impl_executable_builder_owned!(
    InsertRowsOrColumnsRequestBuilder,
    SheetRowColService,
    InsertRowsOrColumnsRequest,
    BaseResponse<InsertRowsOrColumnsResponseData>,
    insert_rows_or_columns,
);
