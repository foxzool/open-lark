#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
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
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};
impl SpreadsheetSheetFilterViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 查询筛选视图请求,
#[derive(Debug, Clone)]
pub struct QueryFilterViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String}
impl QueryFilterViewRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct QueryFilterViewRequestBuilder {
    request: QueryFilterViewRequest}
impl QueryFilterViewRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 查询筛选视图响应体最外层,
#[derive(Debug, Clone)]
pub struct QueryFilterViewResponseData {
    /// 筛选视图列表
    pub items: Vec<FilterViewInfo>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 筛选视图信息,
#[derive(Debug, Clone)]
pub struct FilterViewInfo {
    /// 筛选视图 ID
    pub filter_view_id: String,
    /// 筛选视图名称
    pub filter_view_name: String,
    /// 筛选范围
    pub range: String,
