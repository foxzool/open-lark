#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use open_lark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
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
    service::sheets::v3::SpreadsheetService,
};
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 增加保护范围请求,
#[derive(Debug, Clone)]
pub struct AddProtectRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 保护范围设置
    protect_range: ProtectRangeData}
impl AddProtectRangeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct AddProtectRangeRequestBuilder {
    request: AddProtectRangeRequest}
impl AddProtectRangeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    AddProtectRangeRequestBuilder,
    SpreadsheetService,
    AddProtectRangeRequest,
    BaseResponse<AddProtectRangeResponseData>,
    add_protect_range,
);
/// 保护范围数据
#[derive(Debug, Clone)]
pub struct ProtectRangeData {
    /// 保护范围的维度
    pub dimension: String,
    /// 保护工作表 ID
    pub sheet_id: String,
    /// 保护起始位置
    pub start_index: i32,
    /// 保护结束位置
    pub end_index: i32,
    /// 保护范围 ID（仅在获取时返回）,
#[serde(skip_serializing_if = "Option::is_none")]
    pub protect_id: Option<String>,
    /// 锁定用户,
#[serde(skip_serializing_if = "Option::is_none")]
    pub lock_info: Option<String>}
impl ProtectRangeData {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建行保护范围,
    pub fn row_range(sheet_id: impl ToString, start_row: i32, end_row: i32) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::new("ROWS", sheet_id, start_row, end_row)}
/// 创建列保护范围,
    pub fn column_range(sheet_id: impl ToString, start_col: i32, end_col: i32) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::new("COLUMNS", sheet_id, start_col, end_col)}
/// 增加保护范围响应体最外层,
#[derive(Debug, Clone)]
pub struct AddProtectRangeResponseData {
    /// 保护范围 ID
    pub protect_id: String,
    /// 保护范围信息,
#[serde(flatten)]
    pub protect_range: ProtectRangeData,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use super::*;
use serde_json::json;
    #[test]
fn test_protect_range_data_creation() {
        let protect_range = ProtectRangeData::row_range("Sheet1", 1, 10);
        assert_eq!(protect_range.dimension, "ROWS");
        assert_eq!(protect_range.sheet_id, "Sheet1");
        assert_eq!(protect_range.start_index, 1);
        assert_eq!(protect_range.end_index, 10);
#[test]
    fn test_add_protect_range_response() {
let json = json!({,
            "protect_id": "protect_001",
            "dimension": "COLUMNS",
            "sheet_id": "Sheet1",
            "start_index": 1,
            "end_index": 5,
            "lock_info": "user@example.com"});
let response: AddProtectRangeResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.protect_id, "protect_001");
        assert_eq!(response.protect_range.dimension, "COLUMNS");
