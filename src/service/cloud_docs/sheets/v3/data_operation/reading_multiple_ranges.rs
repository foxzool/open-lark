use crate::core::SDKResult;use reqwest::Method;
use crate::core::error::LarkAPIError;
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
use super::ValueRange;
impl DataOperationService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 读取多个范围请求,
#[derive(Debug, Clone)]
pub struct ReadingMultipleRangesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 查询范围列表
    ranges: Vec<String>,
    /// 指定单元格数据的格式,
#[serde(rename = "valueRenderOption")]
    value_render_option: Option<String>,
    /// 指定数据类型为日期、时间、或时间日期的单元格数据的格式,
#[serde(rename = "dateTimeRenderOption")]
    date_time_render_option: Option<String>,
    /// 指定返回的用户 ID 类型
    user_id_type: Option<String>}
impl ReadingMultipleRangesRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}if self.ranges.is_empty() {,
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "ranges cannot be empty".to_string(),
            ));
// 验证范围数量限制,
        if self.ranges.len() > 500 {,
return Err(crate::core::error::LarkAPIError::illegal_param(,
                "Too many ranges. Maximum 500 ranges allowed".to_string(),
            ));
// 验证每个单元格范围格式,
        for (i, range) in self.ranges.iter().enumerate() {,
if let ValidationResult::Invalid(msg) = validation::validate_cell_range(range) {,
                return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                    "Invalid cell range at index {}: '{}': {}",
                    i, range, msg,
)));
            }
// 验证值渲染选项,
        if let ValidationResult::Invalid(msg) =,
validation::validate_value_render_option(&self.value_render_option),
        {,
return Err(crate::core::error::LarkAPIError::illegal_param(format!(,
                "Invalid valueRenderOption: {}",
                msg,
)));
        }
// 验证日期时间渲染选项,
        if let ValidationResult::Invalid(msg) =,
validation::validate_date_time_render_option(&self.date_time_render_option),
        {,
return Err(crate::core::error::LarkAPIError::illegal_param(format!(,
                "Invalid dateTimeRenderOption: {}",
                msg,
)));
        }
Ok(()),
    }
#[derive(Debug, Clone)]
pub struct ReadingMultipleRangesRequestBuilder {
    request: ReadingMultipleRangesRequest}
impl ReadingMultipleRangesRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// Trait implementation,
impl_executable_builder_owned!(
    ReadingMultipleRangesRequestBuilder,
    DataOperationService,
    ReadingMultipleRangesRequest,
    ReadingMultipleRangesResponseData,
    reading_multiple_ranges,
);
/// 读取多个范围响应体最外层
#[derive(Debug, Clone)]
pub struct ReadingMultipleRangesResponseData {
    /// 值与范围列表,
#[serde(rename = "valueRanges")]
    pub value_ranges: Vec<ValueRange>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
