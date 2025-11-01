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
use super::create::FilterCondition;
impl SpreadsheetSheetFilterViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 查询筛选条件请求,
#[derive(Debug, Clone)]
pub struct QueryFilterViewConditionsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String}
impl QueryFilterViewConditionsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct QueryFilterViewConditionsRequestBuilder {
    request: QueryFilterViewConditionsRequest}
impl QueryFilterViewConditionsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 筛选条件信息,
#[derive(Debug, Clone)]
pub struct FilterConditionInfo {
    /// 筛选条件 ID
    pub condition_id: String,
    /// 筛选条件详情
    pub condition: FilterCondition,
/// 查询筛选条件响应体最外层,
#[derive(Debug, Clone)]
pub struct QueryFilterViewConditionsResponseData {
    /// 筛选条件列表
    pub items: Vec<FilterConditionInfo>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
