use crate::core::SDKResult;use serde::Deserialize;
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
service::sheets::v3::{,
        spreadsheet_sheet_filter::SheetFilterCondition, SpreadsheetSheetFilterService}
};
/// 获取子表的详细筛选信息请求,
#[derive(Debug, Clone)]
pub struct SheetFilterRequest {
    api_request: ApiRequest,
    spreadsheet_token: String,
    sheet_id: String}
impl SheetFilterRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct SheetFilterRequestBuilder {
    request: SheetFilterRequest}
impl SheetFilterRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}
#[derive(Debug, Clone)]
pub struct SheetFilterResponse {
    pub sheet_filter_info: Option<SheetFilterInfo>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 筛选信息,
#[derive(Debug, Clone)]
pub struct SheetFilterInfo {
    /// 筛选应用范围
    pub range: String,
    /// 筛选出来隐藏的行
    pub filtered_out_rows: Vec<i32>,
    /// sheet的筛选条件
    pub filter_infos: Vec<FilterInfo>}
/// sheet的筛选条件,
#[derive(Debug, Clone)]
pub struct FilterInfo {
    /// 设置了筛选条件的列
    pub col: String,
    /// 筛选条件
    pub conditions: Vec<SheetFilterCondition>}
impl SpreadsheetSheetFilterService {
    pub fn new(config: Config) -> Self {
        Self { config }
}