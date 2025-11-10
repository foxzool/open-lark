#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use crate::,
{,
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option::RequestOption,
        SDKResult,
};
    service::sheets::v3::{spreadsheet_sheet::Sheet, SpreadsheetSheetService}
};
/// 获取工作表请求,
#[derive(Debug, Clone)]
pub struct QuerySpreadsheetSheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String}
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl QuerySpreadsheetSheetRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct QuerySpreadsheetSheetRequestBuilder {
    request: QuerySpreadsheetSheetRequest}
impl QuerySpreadsheetSheetRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}
#[derive(Debug, Clone)]
pub struct QuerySpreadsheetSheetResponse {
    pub sheets: Vec<Sheet>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
