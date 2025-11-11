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
        standard_response::StandardResponse,
        SDKResult,
};
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};

use super::{UpdatesInfo, ValueRangeRequest};
impl DataOperationService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 插入数据请求,
#[derive(Debug, Clone)]
pub struct PrependDataRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 查询范围，包含 sheetId 与单元格范围两部分
    range: String,
    /// 插入数据的方式,
#[serde(rename = "insertDataOption")]
    insert_data_option: Option<String>,
    /// 数据值,
#[serde(rename = "valueRange")]
    value_range: ValueRangeRequest}
impl PrependDataRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct PrependDataRequestBuilder {
    request: PrependDataRequest}
impl PrependDataRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// Trait implementation,
impl_executable_builder_owned!(
    PrependDataRequestBuilder,
    DataOperationService,
    PrependDataRequest,
    PrependDataResponseData,
    prepend_data,
);
/// 插入数据响应体最外层
#[derive(Debug, Clone)]
pub struct PrependDataResponseData {
    /// 表格的 token,
#[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 数据更新的位置,
#[serde(rename = "tableRange")]
    pub table_range: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 更新的行数
    pub updates: UpdatesInfo,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use super::PrependDataResponseData;
    #[test]
fn test_prepend_data_response() {
        let json = json!({
            "spreadsheetToken": "shtcnmBA*****yGehy8",
            "tableRange": "Sheet1!A1:B2",
            "revision": 123,
            "updates": {
                "updatedRange": "Sheet1!A1:B1",
                "updatedRows": 1,
                "updatedColumns": 2,
                "updatedCells": 2}
        });
let response: PrependDataResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(response.updates.updated_rows, 1);
