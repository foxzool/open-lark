#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use serde::{Deserialize, Serialize};
use open_lark_core::api_req::ApiRequest;
use crate::,
{,
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option,
        standard_response::StandardResponse,
        SDKResult,
};
    impl_executable_builder_owned,
    service::sheets::v3::{
        data_operation::{FindCondition, FindReplaceResult}
        SpreadsheetSheetService,
};

#[derive(Debug, Clone)]
pub struct ReplaceCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 工作表的id,
#[serde(skip)]
    sheet_id: String,
    /// 查找条件
    find_condition: FindCondition,
    /// 查找的字符串，当search_by_regex字段为 true 时，该字段为正则表达式,
///,
    /// 示例值："如下,
///,
    /// - 普通查找示例: "hello",
/// - 正则查找示例: "[A-Z]\w+"",
    find: String,
    /// 替换的字符串
    replacement: String}
impl ReplaceCellsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct ReplaceCellsRequestBuilder {
    request: ReplaceCellsRequest}
impl ReplaceCellsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// Trait implementation,
impl_executable_builder_owned!(
    ReplaceCellsRequestBuilder,
    SpreadsheetSheetService,
    ReplaceCellsRequest,
    ReplaceCellsResponse,
    replace_cells,
);
/// 替换单元格响应
#[derive(Debug, Clone)]
pub struct ReplaceCellsResponse {
    /// 符合查找条件并替换的单元格信息
    pub replace_result: FindReplaceResult,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}