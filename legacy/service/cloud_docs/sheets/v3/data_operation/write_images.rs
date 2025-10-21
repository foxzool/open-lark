use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};

impl DataOperationService {
    /// 写入图片
    pub async fn write_images(
        &self,
        request: WriteImagesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<WriteImagesResponseData> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path =
            SHEETS_V3_SPREADSHEET_VALUES_IMAGE.replace("{}", &request.spreadsheet_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<WriteImagesResponseData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

/// 写入图片请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WriteImagesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 图片范围数据
    #[serde(rename = "valueRange")]
    value_range: ImageValueRange,
}

impl WriteImagesRequest {
    pub fn builder() -> WriteImagesRequestBuilder {
        WriteImagesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct WriteImagesRequestBuilder {
    request: WriteImagesRequest,
}

impl WriteImagesRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.value_range.range = range.to_string();
        self
    }

    pub fn add_image(mut self, image_data: ImageData) -> Self {
        self.request.value_range.values.push(vec![image_data]);
        self
    }

    pub fn images(mut self, images: Vec<Vec<ImageData>>) -> Self {
        self.request.value_range.values = images;
        self
    }

    pub fn build(mut self) -> WriteImagesRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 图片值范围
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImageValueRange {
    /// 范围
    pub range: String,
    /// 图片数据值
    pub values: Vec<Vec<ImageData>>,
}

/// 图片数据
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    /// 图片类型，固定值为 "image"
    #[serde(rename = "type")]
    pub data_type: String,
    /// 图片 token
    pub image_token: String,
    /// 图片宽度（像素）
    pub width: Option<i32>,
    /// 图片高度（像素）
    pub height: Option<i32>,
}

impl ImageData {
    pub fn new(image_token: impl ToString) -> Self {
        Self {
            data_type: "image".to_string(),
            image_token: image_token.to_string(),
            width: None,
            height: None,
        }
    }

    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}

/// 写入图片响应体最外层
#[derive(Deserialize, Debug)]
pub struct WriteImagesResponseData {
    /// 表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 数据更新的位置
    #[serde(rename = "tableRange")]
    pub table_range: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 更新信息
    pub updates: WriteImageUpdatesInfo,
}

impl ApiResponseTrait for WriteImagesResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl_executable_builder_owned!(
    WriteImagesRequestBuilder,
    DataOperationService,
    WriteImagesRequest,
    WriteImagesResponseData,
    write_images
);

/// 更新信息
#[derive(Deserialize, Debug)]
pub struct WriteImageUpdatesInfo {
    /// 受更新影响的表格范围
    #[serde(rename = "updatedRange")]
    pub updated_range: String,
    /// 更新的行数
    #[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updatedCells")]
    pub updated_cells: i32,
    /// 写入的图片数量
    #[serde(rename = "updatedImages")]
    pub updated_images: i32,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::{ImageData, WriteImagesResponseData};

    #[test]
    fn test_write_images_response() {
        let json = json!({
            "spreadsheetToken": "shtcnmBA*****yGehy8",
            "tableRange": "Sheet1!A1:B2",
            "revision": 125,
            "updates": {
                "updatedRange": "Sheet1!A1:B2",
                "updatedRows": 2,
                "updatedColumns": 2,
                "updatedCells": 4,
                "updatedImages": 2
            }
        });

        let response: WriteImagesResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(response.updates.updated_images, 2);
    }

    #[test]
    fn test_image_data_creation() {
        let image =
            ImageData::new("img_v2_041b9112-02e8-4c12-b2f2-**********g").with_size(200, 150);

        assert_eq!(image.data_type, "image");
        assert_eq!(image.width, Some(200));
        assert_eq!(image.height, Some(150));
    }

    #[test]
    fn test_image_data_serialization() {
        let image = ImageData {
            data_type: "image".to_string(),
            image_token: "img_v2_041b9112-02e8-4c12-b2f2-**********g".to_string(),
            width: Some(100),
            height: Some(80),
        };

        let json = serde_json::to_value(&image).unwrap();
        assert_eq!(json["type"], "image");
        assert_eq!(
            json["image_token"],
            "img_v2_041b9112-02e8-4c12-b2f2-**********g"
        );
        assert_eq!(json["width"], 100);
        assert_eq!(json["height"], 80);
    }
}
