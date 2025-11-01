use crate::core::SDKResult;use reqwest::Method;
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
    service::sheets::v3::SheetRowColService,
};
use super::insert_rows_or_columns::DimensionRange;
impl SheetRowColService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新行列请求,
#[derive(Debug, Clone)]
pub struct UpdateRowsOrColumnsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 更新位置的维度信息
    dimension_range: DimensionRange,
    /// 维度的属性
    dimension_properties: DimensionProperties}
impl UpdateRowsOrColumnsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdateRowsOrColumnsRequestBuilder {
    request: UpdateRowsOrColumnsRequest}
impl UpdateRowsOrColumnsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 维度属性,
#[derive(Debug, Clone)]
pub struct DimensionProperties {
    /// 是否可见
    pub visible: Option<bool>,
    /// 行高或列宽（像素）
    pub pixel_size: Option<i32>}
/// 更新行列响应体最外层,
#[derive(Debug, Clone)]
pub struct UpdateRowsOrColumnsResponseData {
    /// 更新行列后的信息
    pub update_range: UpdateRangeInfo,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 更新范围信息,
#[derive(Debug, Clone)]
pub struct UpdateRangeInfo {
    /// 更新的维度
    pub dimension: String,
    /// 更新的起始位置
    pub start_index: i32,
    /// 更新的结束位置
    pub end_index: i32,
    /// 更新后的属性
    pub properties: DimensionProperties,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use super::UpdateRowsOrColumnsResponseData;
    #[test]
fn test_update_rows_or_columns_response() {
        let json = json!({,
"update_range": {,
                "dimension": "ROWS",
                "start_index": 1,
                "end_index": 3,
                "properties": {
                    "visible": true,
                    "pixel_size": 50}
});
let response: UpdateRowsOrColumnsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.update_range.dimension, "ROWS");
        assert_eq!(response.update_range.start_index, 1);
        assert_eq!(response.update_range.end_index, 3);
        assert_eq!(response.update_range.properties.visible, Some(true));
        assert_eq!(response.update_range.properties.pixel_size, Some(50));
