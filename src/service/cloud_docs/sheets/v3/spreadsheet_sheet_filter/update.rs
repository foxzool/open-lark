#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use serde::Serialize;
use open_lark_core::core::api_req::ApiRequest;
use crate::{,
core::{,
        api_resp::{BaseResponse, EmptyResponse}
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option::RequestOption,
        SDKResult,
    }
    service::sheets::v3::{
        spreadsheet_sheet_filter::SheetFilterCondition, SpreadsheetSheetFilterService}
};
/// 更新子表筛选范围中的列筛选条件,
#[derive(Debug, Clone)]
pub struct UpdateSheetFilterRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    #[serde(skip)]
    sheet_id: String,
    /// 设置筛选条件的列
    col: String,
    /// 筛选的条件
    condition: SheetFilterCondition}
impl UpdateSheetFilterRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdateSheetFilterRequestBuilder {
    request: UpdateSheetFilterRequest}
impl UpdateSheetFilterRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl SpreadsheetSheetFilterService {
    pub fn new(config: Config) -> Self {
        Self { config }
}