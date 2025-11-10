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
use super::query::FilterConditionInfo;
impl SpreadsheetSheetFilterViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取筛选条件请求,
#[derive(Debug, Clone)]
pub struct GetFilterViewConditionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String,
    /// 筛选条件 ID
    condition_id: String}
impl GetFilterViewConditionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct GetFilterViewConditionRequestBuilder {
    request: GetFilterViewConditionRequest}
impl GetFilterViewConditionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取筛选条件响应体最外层,
#[derive(Debug, Clone)]
pub struct GetFilterViewConditionResponseData {
    /// 筛选条件信息,
#[serde(flatten)]
    pub condition_info: FilterConditionInfo,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
