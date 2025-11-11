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
        standard_response::StandardResponse,
        SDKResult,
};
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};
impl DataOperationService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 写入图片请求,
#[derive(Debug, Clone)]
pub struct WriteImagesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 图片范围数据,
#[serde(rename = "valueRange")]
    value_range: ImageValueRange}
impl WriteImagesRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct WriteImagesRequestBuilder {
    request: WriteImagesRequest}
impl WriteImagesRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 图片值范围,
#[derive(Debug, Clone)]
pub struct ImageValueRange {
    /// 范围
    pub range: String,
    /// 图片数据值
    pub values: Vec<Vec<ImageData>>}
/// 图片数据,
#[derive(Debug, Clone)]
pub struct ImageData {
    /// 图片类型，固定值为 "image",
#[serde(rename = "type")]
    pub data_type: String,
    /// 图片 token
    pub image_token: String,
    /// 图片宽度（像素）
    pub width: Option<i32>,
    /// 图片高度（像素）
    pub height: Option<i32>}
impl ImageData {
    pub fn new(config: Config) -> Self {
        Self { config }
}
    pub fn with_size(mut self, width: i32, height: i32) -> Self {
self.width = Some(width);
        self.height = Some(height);
self}
/// 写入图片响应体最外层,
#[derive(Debug, Clone)]
pub struct WriteImagesResponseData {
    /// 表格的 token,
#[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 数据更新的位置,
#[serde(rename = "tableRange")]
    pub table_range: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 更新信息
    pub updates: WriteImageUpdatesInfo,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl_executable_builder_owned!(,
    WriteImagesRequestBuilder,
    DataOperationService,
    WriteImagesRequest,
    WriteImagesResponseData,
    write_images,
);
/// 更新信息
#[derive(Debug, Clone)]
pub struct WriteImageUpdatesInfo {
    /// 受更新影响的表格范围,
#[serde(rename = "updatedRange")]
    pub updated_range: String,
    /// 更新的行数,
#[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 更新的列数,
#[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 更新的单元格数,
#[serde(rename = "updatedCells")]
    pub updated_cells: i32,
    /// 写入的图片数量,
#[serde(rename = "updatedImages")]
    pub updated_images: i32,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;

    use super::{ImageData, WriteImagesResponseData};
#[test]
    fn test_write_images_response() {
let json = json!({,
            "spreadsheetToken": "shtcnmBA*****yGehy8",
            "tableRange": "Sheet1!A1:B2",
            "revision": 125,
            "updates": {
                "updatedRange": "Sheet1!A1:B2",
                "updatedRows": 2,
                "updatedColumns": 2,
                "updatedCells": 4,
                "updatedImages": 2}
        });
let response: WriteImagesResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(response.updates.updated_images, 2);
#[test]
    fn test_image_data_creation() {
let image =,
            ImageData::new("img_v2_041b9112-02e8-4c12-b2f2-**********g").with_size(200, 150);

        assert_eq!(image.data_type, "image");
        assert_eq!(image.width, Some(200));
        assert_eq!(image.height, Some(150));
#[test]
    fn test_image_data_serialization() {
let image = ImageData {,
            data_type: "image".to_string(),
            image_token: "img_v2_041b9112-02e8-4c12-b2f2-**********g".to_string(),
            width: Some(100),
            height: Some(80)};
let json = serde_json::to_value(&image).unwrap();
        assert_eq!(json["type"] "image");
assert_eq!(,
            json["image_token"]
            "img_v2_041b9112-02e8-4c12-b2f2-**********g",
);
        assert_eq!(json["width"] 100);
        assert_eq!(json["height"] 80);
