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
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};
impl SpreadsheetSheetFilterViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新筛选视图请求,
#[derive(Debug, Clone)]
pub struct PatchFilterViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String,
    /// 筛选视图的新名称
    filter_view_name: Option<String>,
    /// 筛选视图的新范围
    range: Option<String>}
impl PatchFilterViewRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct PatchFilterViewRequestBuilder {
    request: PatchFilterViewRequest}
impl PatchFilterViewRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新筛选视图响应体最外层,
#[derive(Debug, Clone)]
pub struct PatchFilterViewResponseData {
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
