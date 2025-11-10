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
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};
impl SpreadsheetSheetFilterViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建筛选视图请求,
#[derive(Debug, Clone)]
pub struct CreateFilterViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图的名称
    filter_view_name: String,
    /// 筛选视图的范围
    range: String}
impl CreateFilterViewRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateFilterViewRequestBuilder {
    request: CreateFilterViewRequest}
impl CreateFilterViewRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建筛选视图响应体最外层,
#[derive(Debug, Clone)]
pub struct CreateFilterViewResponseData {
    /// 筛选视图 ID
    pub filter_view_id: String,
    /// 筛选视图名称
    pub filter_view_name: String,
    /// 筛选范围
    pub range: String,
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
use super::CreateFilterViewResponseData;
    #[test]
fn test_create_filter_view_response() {
        let json = json!({
            "filter_view_id": "fltr_vw_001",
            "filter_view_name": "销售数据筛选",
            "range": "A1:E100"});
let response: CreateFilterViewResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.filter_view_id, "fltr_vw_001");
        assert_eq!(response.filter_view_name, "销售数据筛选");
        assert_eq!(response.range, "A1:E100");
// 实现ExecutableBuilder trait,
impl_executable_builder_owned!(
    CreateFilterViewRequestBuilder,
    SpreadsheetSheetFilterViewService,
    CreateFilterViewRequest,
    BaseResponse<CreateFilterViewResponseData>,
    create,
);
