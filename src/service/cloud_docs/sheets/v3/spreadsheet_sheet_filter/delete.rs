#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use crate::{,
use open_lark_core::api_req::ApiRequest;    core::{,
        api_resp::{BaseResponse, EmptyResponse}
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option::RequestOption,
        SDKResult,
    }
    service::sheets::v3::SpreadsheetSheetFilterService,
};
/// 删除筛选,
#[derive(Debug, Clone)]
pub struct DeleteSheetFilterRequest {
    api_request: ApiRequest,
    spreadsheet_token: String,
    sheet_id: String}
impl DeleteSheetFilterRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteSheetFilterRequestBuilder {
    request: DeleteSheetFilterRequest}
impl DeleteSheetFilterRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl SpreadsheetSheetFilterService {
    pub fn new(config: Config) -> Self {
        Self { config }
}