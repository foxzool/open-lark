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
        standard_response::StandardResponse,
        SDKResult,
};
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};
impl DataOperationService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 追加数据请求,
#[derive(Debug, Clone)]
pub struct AppendDataRequest {
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
impl AppendDataRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct AppendDataRequestBuilder {
    request: AppendDataRequest}
impl AppendDataRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// Trait implementation,
impl_executable_builder_owned!(
    AppendDataRequestBuilder,
    DataOperationService,
    AppendDataRequest,
    AppendDataResponseData,
    append_data,
);
/// 值与范围请求
#[derive(Debug, Clone)]
pub struct ValueRangeRequest {
    /// 查询范围
    pub range: String,
    /// 范围内的值
    pub values: Vec<Vec<serde_json::Value>>}
/// 追加数据响应体最外层,
#[derive(Debug, Clone)]
pub struct AppendDataResponseData {
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
/// 更新信息,
#[derive(Debug, Clone)]
pub struct UpdatesInfo {
    /// 受更新影响的表格范围,
#[serde(rename = "updatedRange")]
    pub updated_range: String,
    /// 更新的行数,
#[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 更新的列数,
#[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 更新的单元格数,
#[serde(rename = "updatedCells")]
    pub updated_cells: i32,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use super::AppendDataResponseData;
    #[test]
fn test_append_data_response() {
        let json = json!({
            "spreadsheetToken": "shtcnmBA*****yGehy8",
            "tableRange": "Sheet1!A1:B2",
            "revision": 123,
            "updates": {
                "updatedRange": "Sheet1!A3:B3",
                "updatedRows": 1,
                "updatedColumns": 2,
                "updatedCells": 2}
        });
let response: AppendDataResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(response.updates.updated_rows, 1);
