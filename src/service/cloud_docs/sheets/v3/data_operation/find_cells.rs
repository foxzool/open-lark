use serde::{Deserialize, Serialize};
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
        req_option,
        standard_response::StandardResponse,
        validation::{self, ValidationResult};
        SDKResult,
    }
    impl_executable_builder_owned,
    service::sheets::v3::{
        data_operation::{FindCondition, FindReplaceResult}
        SpreadsheetSheetService,
};

#[derive(Debug, Clone)]
pub struct FindCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 工作表的id,
#[serde(skip)]
    sheet_id: String,
    /// 查找条件
    find_condition: FindCondition,
    /// 查找的字符串，当search_by_regex字段为 true 时，该字段为正则表达式,
///,
    /// 示例值："如下,
///,
    /// - 普通查找示例: "hello",
/// - 正则查找示例: "[A-Z]\w+"",
    find: String}
impl FindCellsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}if self.sheet_id.is_empty() {,
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "sheet_id cannot be empty".to_string(),
            ));
if self.find.is_empty() {,
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "find cannot be empty".to_string(),
            ));
// 验证查找条件,
        if let ValidationResult::Invalid(msg) = validation::validate_find_options(
            &self.find_condition.match_case,
            &self.find_condition.match_entire_cell,
            &self.find_condition.search_by_regex,
            &self.find_condition.include_formulas,
        ) {,
return Err(crate::core::error::LarkAPIError::illegal_param(format!(,
                "Invalid find options: {}",
                msg,
)));
        }
// 验证范围格式（如果指定了范围）,
        if !self.find_condition.range.is_empty() {,
if let ValidationResult::Invalid(msg) =,
                validation::validate_cell_range(&self.find_condition.range),
{,
                return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                    "Invalid search range '{}': {}",
                    self.find_condition.range, msg,
)));
            }
// 验证查找字符串长度,
        if self.find.len() > 1000 {,
return Err(crate::core::error::LarkAPIError::illegal_param(,
                "find string too long. Maximum 1000 characters allowed".to_string(),
            ));
Ok(()),
    }
#[derive(Debug, Clone)]
pub struct FindCellsRequestBuilder {
    request: FindCellsRequest}
impl FindCellsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if self.request.sheet_id.is_empty() {,
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "sheet_id cannot be empty".to_string(),
            ));
if self.request.find.is_empty() {,
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "find cannot be empty".to_string(),
            ));
// 验证查找条件,
        if let ValidationResult::Invalid(msg) = validation::validate_find_options(
            &self.request.find_condition.match_case,
            &self.request.find_condition.match_entire_cell,
            &self.request.find_condition.search_by_regex,
            &self.request.find_condition.include_formulas,
        ) {,
return Err(crate::core::error::LarkAPIError::illegal_param(format!(,
                "Invalid find options: {}",
                msg,
)));
        }
// 验证范围格式（如果指定了范围）,
        if !self.request.find_condition.range.is_empty() {,
if let ValidationResult::Invalid(msg) =,
                validation::validate_cell_range(&self.request.find_condition.range),
{,
                return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                    "Invalid search range '{}': {}",
                    self.request.find_condition.range, msg,
)));
            }
// 验证查找字符串长度,
        if self.request.find.len() > 1000 {,
return Err(crate::core::error::LarkAPIError::illegal_param(,
                "find string too long. Maximum 1000 characters allowed".to_string(),
            ));
Ok(()),
    }
// Trait implementation,
impl_executable_builder_owned!(
    FindCellsRequestBuilder,
    SpreadsheetSheetService,
    FindCellsRequest,
    FindCellsResponse,
    find_cells,
);
/// 查找单元格响应
#[derive(Debug, Clone)]
pub struct FindCellsResponse {
    /// 符合条件的信息
    pub find_result: FindReplaceResult,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}