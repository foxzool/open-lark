use serde::Serialize;
use open_lark_core::core::api_req::ApiRequest;
use crate::{,
core::{,
        api_resp::{BaseResponse, EmptyResponse}
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option::RequestOption,
        SDKResult,
    }
    impl_executable_builder_owned,
    service::sheets::v3::{
        spreadsheet_sheet_filter::SheetFilterCondition, SpreadsheetSheetFilterService}
};
/// 在子表内创建筛选,
#[derive(Debug, Clone)]
pub struct CreateSheetFilterRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    #[serde(skip)]
    sheet_id: String,
    /// 筛选应用范围
    range: String,
    /// 设置筛选条件的列
    col: String,
    /// 筛选的条件
    condition: SheetFilterCondition}
impl CreateSheetFilterRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateSheetFilterRequestBuilder {
    request: CreateSheetFilterRequest}
impl CreateSheetFilterRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl SpreadsheetSheetFilterService {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 实现ExecutableBuilder trait,
impl_executable_builder_owned!(
    CreateSheetFilterRequestBuilder,
    SpreadsheetSheetFilterService,
    CreateSheetFilterRequest,
    BaseResponse<EmptyResponse>,
    create,
);
