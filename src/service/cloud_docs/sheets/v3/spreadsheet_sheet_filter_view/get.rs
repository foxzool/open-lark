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
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};
use super::query::FilterViewInfo;
impl SpreadsheetSheetFilterViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取筛选视图请求,
#[derive(Debug, Clone)]
pub struct GetFilterViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String}
impl GetFilterViewRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct GetFilterViewRequestBuilder {
    request: GetFilterViewRequest}
impl GetFilterViewRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取筛选视图响应体最外层,
#[derive(Debug, Clone)]
pub struct GetFilterViewResponseData {
    /// 筛选视图信息,
#[serde(flatten)]
    pub filter_view: FilterViewInfo,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
