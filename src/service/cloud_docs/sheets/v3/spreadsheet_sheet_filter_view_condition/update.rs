use reqwest::Method;
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
}/// 更新筛选条件请求,
#[derive(Debug, Clone)]
pub struct UpdateFilterViewConditionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String,
    /// 筛选条件 ID
    condition_id: String,
    /// 新的筛选条件
    condition: FilterCondition}
impl UpdateFilterViewConditionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdateFilterViewConditionRequestBuilder {
    request: UpdateFilterViewConditionRequest}
impl UpdateFilterViewConditionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新筛选条件响应体最外层,
#[derive(Debug, Clone)]
pub struct UpdateFilterViewConditionResponseData {
    /// 筛选条件 ID
    pub condition_id: String,
    /// 更新后的筛选条件
    pub condition: FilterCondition,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
