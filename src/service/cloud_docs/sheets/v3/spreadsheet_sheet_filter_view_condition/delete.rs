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
impl SpreadsheetSheetFilterViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除筛选条件请求,
#[derive(Debug, Clone)]
pub struct DeleteFilterViewConditionRequest {
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
impl DeleteFilterViewConditionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteFilterViewConditionRequestBuilder {
    request: DeleteFilterViewConditionRequest}
impl DeleteFilterViewConditionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除筛选条件响应体最外层,
#[derive(Debug, Clone)]
pub struct DeleteFilterViewConditionResponseData {
    /// 操作结果
    pub success: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
