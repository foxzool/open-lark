#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use reqwest::Method;
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
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetSheetService,
};
use super::create::FloatImageData;
impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新浮动图片请求,
#[derive(Debug, Clone)]
pub struct UpdateFloatImageRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 浮动图片 ID
    float_image_id: String,
    /// 新的浮动图片信息
    float_image: FloatImageData}
impl UpdateFloatImageRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdateFloatImageRequestBuilder {
    request: UpdateFloatImageRequest}
impl UpdateFloatImageRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    UpdateFloatImageRequestBuilder,
    SpreadsheetSheetService,
    UpdateFloatImageRequest,
    BaseResponse<UpdateFloatImageResponseData>,
    update_float_image,
);
/// 更新浮动图片响应体最外层
#[derive(Debug, Clone)]
pub struct UpdateFloatImageResponseData {
    /// 浮动图片 ID
    pub float_image_id: String,
    /// 更新后的浮动图片信息,
#[serde(flatten)]
    pub float_image: FloatImageData,
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
fn test_update_float_image_response() {
        let json = json!({
            "float_image_id": "fimg_001",
            "image_token": "img_token_456",
            "position": {
                "start_col_index": 2,
                "start_row_index": 2,
                "offset_x": 15.0,
                "offset_y": 25.0}
            "size": {
                "width": 300.0,
                "height": 200.0}
            "name": "更新后的图片",
});
let response: UpdateFloatImageResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.float_image_id, "fimg_001");
        assert_eq!(response.float_image.image_token, "img_token_456");
        assert_eq!(response.float_image.size.width, 300.0);
