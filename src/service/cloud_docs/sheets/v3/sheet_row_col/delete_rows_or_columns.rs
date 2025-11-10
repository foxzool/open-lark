#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use reqwest::Method;
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
use super::insert_rows_or_columns::DimensionRange;
impl SheetRowColService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除行列请求,
#[derive(Debug, Clone)]
pub struct DeleteRowsOrColumnsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 删除位置的维度信息
    dimension_range: DimensionRange}
impl DeleteRowsOrColumnsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteRowsOrColumnsRequestBuilder {
    request: DeleteRowsOrColumnsRequest}
impl DeleteRowsOrColumnsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除行列响应体最外层,
#[derive(Debug, Clone)]
pub struct DeleteRowsOrColumnsResponseData {
    /// 删除行列后的信息
    pub delete_range: DeleteRangeInfo,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 删除范围信息,
#[derive(Debug, Clone)]
pub struct DeleteRangeInfo {
    /// 删除的维度
    pub dimension: String,
    /// 删除的起始位置
    pub start_index: i32,
    /// 删除的结束位置
    pub end_index: i32,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use super::DeleteRowsOrColumnsResponseData;
    #[test]
fn test_delete_rows_or_columns_response() {
        let json = json!({,
"delete_range": {,
                "dimension": "ROWS",
                "start_index": 2,
                "end_index": 4}
        });
let response: DeleteRowsOrColumnsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.delete_range.dimension, "ROWS");
        assert_eq!(response.delete_range.start_index, 2);
        assert_eq!(response.delete_range.end_index, 4);
// 实现ExecutableBuilder trait,
impl_executable_builder_owned!(
    DeleteRowsOrColumnsRequestBuilder,
    SheetRowColService,
    DeleteRowsOrColumnsRequest,
    BaseResponse<DeleteRowsOrColumnsResponseData>,
    delete_rows_or_columns,
);
