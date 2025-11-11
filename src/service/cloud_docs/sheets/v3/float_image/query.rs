#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
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
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetSheetService,
};
use super::get::FloatImageInfo;
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 查询浮动图片请求,
#[derive(Debug, Clone)]
pub struct QueryFloatImagesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String}
impl QueryFloatImagesRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct QueryFloatImagesRequestBuilder {
    request: QueryFloatImagesRequest}
impl QueryFloatImagesRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    QueryFloatImagesRequestBuilder,
    SpreadsheetSheetService,
    QueryFloatImagesRequest,
    BaseResponse<QueryFloatImagesResponseData>,
    query_float_images,
);
/// 查询浮动图片响应体最外层
#[derive(Debug, Clone)]
pub struct QueryFloatImagesResponseData {
    /// 浮动图片列表
    pub items: Vec<FloatImageInfo>,
    /// 是否还有更多数据,
#[serde(default)]
    pub has_more: bool,
    /// 下次请求的页面标记,
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use super::*;
use serde_json::json;
    #[test]
fn test_query_float_images_response() {
        let json = json!({,
"items": [,
                {
                    "float_image_id": "fimg_001",
                    "image_token": "img_token_123",
                    "position": {
                        "start_col_index": 1,
                        "start_row_index": 1,
                        "offset_x": 10.0,
                        "offset_y": 20.0}
                    "size": {
                        "width": 200.0,
                        "height": 150.0}
                    "name": "图片1",
                {
                    "float_image_id": "fimg_002",
                    "image_token": "img_token_456",
                    "position": {
                        "start_col_index": 3,
                        "start_row_index": 3,
                        "offset_x": 0.0,
                        "offset_y": 0.0}
                    "size": {
                        "width": 150.0,
                        "height": 100.0}
                    "name": "图片2",
            ],
            "has_more": false,
});
let response: QueryFloatImagesResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].float_image_id, "fimg_001");
        assert_eq!(response.items[1].float_image.image_token, "img_token_456");
assert!(!response.has_more);
    }
