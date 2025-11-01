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
    service::sheets::v3::SheetRowColService,
};
impl SheetRowColService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 增加行列请求,
#[derive(Debug, Clone)]
pub struct AddRowsOrColumnsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 需要增加行列的维度
    dimension: String,
    /// 增加行列的长度
    length: i32}
impl AddRowsOrColumnsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct AddRowsOrColumnsRequestBuilder {
    request: AddRowsOrColumnsRequest}
impl AddRowsOrColumnsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 增加行列响应体最外层,
#[derive(Debug, Clone)]
pub struct AddRowsOrColumnsResponseData {
    /// 增加行列后的信息
    pub add_range: AddRangeInfo,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 增加范围信息,
#[derive(Debug, Clone)]
pub struct AddRangeInfo {
    /// 增加的维度
    pub dimension: String,
    /// 增加的起始位置
    pub start_index: i32,
    /// 增加的结束位置
    pub end_index: i32,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use super::AddRowsOrColumnsResponseData;
    #[test]
fn test_add_rows_or_columns_response() {
        let json = json!({,
"add_range": {,
                "dimension": "ROWS",
                "start_index": 5,
                "end_index": 10}
        });
let response: AddRowsOrColumnsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.add_range.dimension, "ROWS");
        assert_eq!(response.add_range.start_index, 5);
        assert_eq!(response.add_range.end_index, 10);
