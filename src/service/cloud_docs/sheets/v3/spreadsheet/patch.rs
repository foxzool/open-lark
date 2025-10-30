use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::Serialize;
use crate::{,
core::{,
        api_resp::{BaseResponse, RawResponse}
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    }
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetService,
};
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 修改电子表格属性 请求体,
#[derive(Debug, Clone)]
pub struct PatchSpreadSheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 表格的token,
#[serde(skip)]
    spreadsheet_token: String,
    /// 表格标题
    title: String}
impl PatchSpreadSheetRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct PatchSpreadSheetRequestBuilder {
    request: PatchSpreadSheetRequest}
impl PatchSpreadSheetRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// Trait implementation,
impl_executable_builder_owned!(
    PatchSpreadSheetRequestBuilder,
    SpreadsheetService,
    PatchSpreadSheetRequest,
    BaseResponse<RawResponse>,
    patch,
);
