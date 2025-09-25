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
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetSheetService,
};

impl SpreadsheetSheetService {
    /// 创建浮动图片
    pub async fn create_float_image(
        &self,
        request: CreateFloatImageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateFloatImageResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FLOAT_IMAGES
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 创建浮动图片请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CreateFloatImageRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 浮动图片信息
    float_image: FloatImageData,
}

impl CreateFloatImageRequest {
    pub fn builder() -> CreateFloatImageRequestBuilder {
        CreateFloatImageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateFloatImageRequestBuilder {
    request: CreateFloatImageRequest,
}

impl CreateFloatImageRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn float_image(mut self, float_image: FloatImageData) -> Self {
        self.request.float_image = float_image;
        self
    }

    pub fn build(mut self) -> CreateFloatImageRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    CreateFloatImageRequestBuilder,
    SpreadsheetSheetService,
    CreateFloatImageRequest,
    BaseResponse<CreateFloatImageResponseData>,
    create_float_image
);

/// 浮动图片数据
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FloatImageData {
    /// 图片token
    pub image_token: String,
    /// 图片位置
    pub position: ImagePosition,
    /// 图片大小
    pub size: ImageSize,
    /// 浮动图片 ID（仅在响应时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_image_id: Option<String>,
    /// 图片名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 图片位置
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImagePosition {
    /// 起始列索引（从0开始）
    pub start_col_index: i32,
    /// 起始行索引（从0开始）
    pub start_row_index: i32,
    /// 在单元格内的水平偏移量（像素）
    #[serde(default)]
    pub offset_x: f64,
    /// 在单元格内的垂直偏移量（像素）
    #[serde(default)]
    pub offset_y: f64,
}

/// 图片大小
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImageSize {
    /// 图片宽度（像素）
    pub width: f64,
    /// 图片高度（像素）
    pub height: f64,
}

impl FloatImageData {
    /// 创建浮动图片
    pub fn new(image_token: impl ToString, position: ImagePosition, size: ImageSize) -> Self {
        Self {
            image_token: image_token.to_string(),
            position,
            size,
            float_image_id: None,
            name: None,
        }
    }

    /// 设置图片名称
    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

impl ImagePosition {
    /// 创建图片位置
    pub fn new(col_index: i32, row_index: i32) -> Self {
        Self {
            start_col_index: col_index,
            start_row_index: row_index,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    /// 设置偏移量
    pub fn with_offset(mut self, offset_x: f64, offset_y: f64) -> Self {
        self.offset_x = offset_x;
        self.offset_y = offset_y;
        self
    }
}

impl ImageSize {
    /// 创建图片大小
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// 创建正方形图片大小
    pub fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/// 创建浮动图片响应体最外层
#[derive(Deserialize, Debug)]
pub struct CreateFloatImageResponseData {
    /// 浮动图片 ID
    pub float_image_id: String,
    /// 浮动图片信息
    #[serde(flatten)]
    pub float_image: FloatImageData,
}

impl ApiResponseTrait for CreateFloatImageResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_float_image_creation() {
        let position = ImagePosition::new(1, 1).with_offset(10.0, 20.0);
        let size = ImageSize::new(200.0, 150.0);
        let float_image =
            FloatImageData::new("img_token_123", position, size).with_name("示例图片");

        assert_eq!(float_image.image_token, "img_token_123");
        assert_eq!(float_image.position.start_col_index, 1);
        assert_eq!(float_image.position.offset_x, 10.0);
        assert_eq!(float_image.size.width, 200.0);
        assert_eq!(float_image.name.as_ref().unwrap(), "示例图片");
    }

    #[test]
    fn test_create_float_image_response() {
        let json = json!({
            "float_image_id": "fimg_001",
            "image_token": "img_token_123",
            "position": {
                "start_col_index": 1,
                "start_row_index": 1,
                "offset_x": 10.0,
                "offset_y": 20.0
            },
            "size": {
                "width": 200.0,
                "height": 150.0
            },
            "name": "示例图片"
        });

        let response: CreateFloatImageResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.float_image_id, "fimg_001");
        assert_eq!(response.float_image.image_token, "img_token_123");
    }
}
