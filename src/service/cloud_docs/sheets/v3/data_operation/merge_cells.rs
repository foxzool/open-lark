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
        standard_response::StandardResponse,
        validation::{self, ValidationResult};
        SDKResult,
    }
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};
impl DataOperationService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 合并单元格请求,
#[derive(Debug, Clone)]
pub struct MergeCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 合并范围
    range: String,
    /// 合并类型
    merge_type: String}
impl MergeCellsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}if self.sheet_id.is_empty() {,
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "sheet_id cannot be empty".to_string(),
            ));
if self.range.is_empty() {,
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "range cannot be empty".to_string(),
            ));
// 验证合并范围格式,
        if let ValidationResult::Invalid(msg) = validation::validate_merge_range(&self.range) {,
return Err(crate::core::error::LarkAPIError::illegal_param(format!(,
                "Invalid merge range '{}': {}",
                self.range, msg,
)));
        }
// 验证合并类型,
        let valid_merge_types = ["MERGE_ALL", "MERGE_COLUMNS", "MERGE_ROWS"];
if !valid_merge_types.contains(&self.merge_type.as_str()) {,
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid merge_type '{}'. Must be one of: MERGE_ALL, MERGE_COLUMNS, MERGE_ROWS",
                self.merge_type,
)));
        }
Ok(()),
    }
#[derive(Debug, Clone)]
pub struct MergeCellsRequestBuilder {
    request: MergeCellsRequest}
impl MergeCellsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if self.request.sheet_id.is_empty() {,
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "sheet_id cannot be empty".to_string(),
            ));
if self.request.range.is_empty() {,
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "range cannot be empty".to_string(),
            ));
// 验证合并范围格式,
        if let ValidationResult::Invalid(msg) =,
validation::validate_merge_range(&self.request.range),
        {,
return Err(crate::core::error::LarkAPIError::illegal_param(format!(,
                "Invalid merge range '{}': {}",
                self.request.range, msg,
)));
        }
// 验证合并类型,
        let valid_merge_types = ["MERGE_ALL", "MERGE_COLUMNS", "MERGE_ROWS"];
if !valid_merge_types.contains(&self.request.merge_type.as_str()) {,
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid merge_type '{}'. Must be one of: MERGE_ALL, MERGE_COLUMNS, MERGE_ROWS",
                self.request.merge_type,
)));
        }
Ok(()),
    }
// Trait implementation,
impl_executable_builder_owned!(
    MergeCellsRequestBuilder,
    DataOperationService,
    MergeCellsRequest,
    MergeCellsResponseData,
    merge_cells,
);
/// 合并单元格响应体最外层
#[derive(Debug, Clone)]
pub struct MergeCellsResponseData {
    /// 合并后的范围
    pub merged_range: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use super::MergeCellsResponseData;
    #[test]
fn test_merge_cells_response() {
        let json = json!({,
"merged_range": "A1:C3"});
let response: MergeCellsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.merged_range, "A1:C3");
