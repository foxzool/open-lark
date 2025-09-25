use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    service::cloud_docs::sheets::v2::SpreadsheetService,
};

/// 写入图片
#[derive(Serialize, Debug, Default)]
pub struct WriteImageRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 查询范围 range=`<sheetId>!<开始格子>:<结束格子>`
    /// 如：xxxx!A1:D5，详见在线表格开发指南。此处限定为一个格子，如: xxxx!A1:A1
    range: String,
    /// 需要写入的图片二进制流，支持 "PNG", "JPEG", "JPG", "GIF", "BMP", "JFIF", "EXIF", "TIFF",
    /// "BPG", "HEIC" 等图片格式
    image: Vec<u8>,
    /// 写入的图片名字
    name: String,
}

impl WriteImageRequest {
    pub fn builder() -> WriteImageRequestBuilder {
        WriteImageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct WriteImageRequestBuilder {
    request: WriteImageRequest,
}

impl WriteImageRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
        self
    }

    pub fn image(mut self, image: Vec<u8>) -> Self {
        self.request.image = image;
        self
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = name.to_string();
        self
    }

    pub fn build(mut self) -> WriteImageRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 写入图片响应体
#[derive(Debug, Deserialize)]
pub struct WriteImageResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spread_sheet_token: String,
    /// spreadsheet 的版本号
    pub revision: i32,
    /// 写入图片的range
    #[serde(rename = "updateRange")]
    pub update_range: String,
}

impl ApiResponseTrait for WriteImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 该接口用于根据 spreadsheetToken 和 range 向单个格子写入图片。
    pub async fn write_image(
        &self,
        request: WriteImageRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<WriteImageResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_VALUES_IMAGE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{config::Config, constants::AppType},
        service::cloud_docs::sheets::v2::{
            data_operation::{WriteImageRequest, WriteImageResponse},
            SpreadsheetService,
        },
    };

    fn create_service() -> SpreadsheetService {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .build();
        SpreadsheetService { config }
    }

    fn create_test_image_data() -> Vec<u8> {
        // Simple 1x1 PNG image (smallest valid PNG)
        vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
            0x49, 0x48, 0x44, 0x52, // IHDR chunk type
            0x00, 0x00, 0x00, 0x01, // Width: 1
            0x00, 0x00, 0x00, 0x01, // Height: 1
            0x08, 0x06, 0x00, 0x00,
            0x00, // Bit depth, color type, compression, filter, interlace
            0x1F, 0x15, 0xC4, 0x89, // IHDR CRC
            0x00, 0x00, 0x00, 0x0A, // IDAT chunk length
            0x49, 0x44, 0x41, 0x54, // IDAT chunk type
            0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00,
            0x01, // Compressed image data
            0x0D, 0x0A, 0x2D, 0xB4, // IDAT CRC
            0x00, 0x00, 0x00, 0x00, // IEND chunk length
            0x49, 0x45, 0x4E, 0x44, // IEND chunk type
            0xAE, 0x42, 0x60, 0x82, // IEND CRC
        ]
    }

    fn create_jpeg_header() -> Vec<u8> {
        // JPEG file signature
        vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46]
    }

    #[test]
    fn test_write_image_builder_default() {
        let request = WriteImageRequest::builder().build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.range, "");
        assert_eq!(request.image, Vec::<u8>::new());
        assert_eq!(request.name, "");
    }

    #[test]
    fn test_write_image_builder_basic() {
        let image_data = create_test_image_data();
        let request = WriteImageRequest::builder()
            .spreadsheet_token("test_token")
            .range("Sheet1!A1:A1")
            .image(image_data.clone())
            .name("test_image.png")
            .build();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.range, "Sheet1!A1:A1");
        assert_eq!(request.image, image_data);
        assert_eq!(request.name, "test_image.png");
    }

    #[test]
    fn test_write_image_builder_all_options() {
        let image_data = create_test_image_data();
        let request = WriteImageRequest::builder()
            .spreadsheet_token("spreadsheet_abc123")
            .range("Photos!B2:B2")
            .image(image_data.clone())
            .name("company_logo.png")
            .build();

        assert_eq!(request.spreadsheet_token, "spreadsheet_abc123");
        assert_eq!(request.range, "Photos!B2:B2");
        assert_eq!(request.image, image_data);
        assert_eq!(request.name, "company_logo.png");
    }

    #[test]
    fn test_write_image_builder_chaining() {
        let image_data = create_test_image_data();
        let request = WriteImageRequest::builder()
            .spreadsheet_token("chain_test")
            .range("Gallery!C3:C3")
            .name("chained_image.png")
            .image(image_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "chain_test");
        assert_eq!(request.range, "Gallery!C3:C3");
        assert_eq!(request.image, image_data);
        assert_eq!(request.name, "chained_image.png");
    }

    #[test]
    fn test_write_image_with_different_formats() {
        let png_data = create_test_image_data();
        let jpeg_data = create_jpeg_header();

        let png_request = WriteImageRequest::builder()
            .spreadsheet_token("format_test")
            .range("Images!A1:A1")
            .image(png_data.clone())
            .name("image.png")
            .build();

        let jpeg_request = WriteImageRequest::builder()
            .spreadsheet_token("format_test")
            .range("Images!B1:B1")
            .image(jpeg_data.clone())
            .name("image.jpg")
            .build();

        assert_eq!(png_request.image, png_data);
        assert_eq!(jpeg_request.image, jpeg_data);
        assert_eq!(png_request.name, "image.png");
        assert_eq!(jpeg_request.name, "image.jpg");
    }

    #[test]
    fn test_write_image_with_unicode_names() {
        let image_data = create_test_image_data();
        let request = WriteImageRequest::builder()
            .spreadsheet_token("unicode_test")
            .range("图片!A1:A1")
            .image(image_data.clone())
            .name("公司标志.png")
            .build();

        assert_eq!(request.spreadsheet_token, "unicode_test");
        assert_eq!(request.range, "图片!A1:A1");
        assert_eq!(request.image, image_data);
        assert_eq!(request.name, "公司标志.png");
    }

    #[test]
    fn test_write_image_with_special_characters() {
        let image_data = create_test_image_data();
        let request = WriteImageRequest::builder()
            .spreadsheet_token("special_chars_test")
            .range("'Sheet With Spaces'!A1:A1")
            .image(image_data.clone())
            .name("image@2x!#$%^&*().png")
            .build();

        assert_eq!(request.spreadsheet_token, "special_chars_test");
        assert_eq!(request.range, "'Sheet With Spaces'!A1:A1");
        assert_eq!(request.image, image_data);
        assert_eq!(request.name, "image@2x!#$%^&*().png");
    }

    #[test]
    fn test_write_image_empty_image() {
        let request = WriteImageRequest::builder()
            .spreadsheet_token("empty_test")
            .range("Sheet1!A1:A1")
            .image(vec![])
            .name("empty.png")
            .build();

        assert_eq!(request.spreadsheet_token, "empty_test");
        assert_eq!(request.range, "Sheet1!A1:A1");
        assert_eq!(request.image, Vec::<u8>::new());
        assert_eq!(request.name, "empty.png");
    }

    #[test]
    fn test_write_image_large_data() {
        let large_image_data = vec![0u8; 1024 * 1024]; // 1MB of data
        let request = WriteImageRequest::builder()
            .spreadsheet_token("large_image_test")
            .range("BigImages!A1:A1")
            .image(large_image_data.clone())
            .name("large_image.png")
            .build();

        assert_eq!(request.spreadsheet_token, "large_image_test");
        assert_eq!(request.range, "BigImages!A1:A1");
        assert_eq!(request.image.len(), 1024 * 1024);
        assert_eq!(request.name, "large_image.png");
    }

    #[test]
    fn test_write_image_different_ranges() {
        let image_data = create_test_image_data();

        let ranges = [
            "Sheet1!A1:A1",
            "Sheet2!B5:B5",
            "DataSheet!Z100:Z100",
            "第一页!C10:C10",
        ];

        for (i, range) in ranges.iter().enumerate() {
            let request = WriteImageRequest::builder()
                .spreadsheet_token("range_test")
                .range(range)
                .image(image_data.clone())
                .name(format!("image_{}.png", i))
                .build();

            assert_eq!(request.range, *range);
            assert_eq!(request.name, format!("image_{}.png", i));
        }
    }

    #[test]
    fn test_write_image_serialization() {
        let image_data = create_test_image_data();
        let request = WriteImageRequest::builder()
            .spreadsheet_token("serialization_test")
            .range("Sheet1!A1:A1")
            .image(image_data.clone())
            .name("test_image.png")
            .build();

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_value: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        assert_eq!(json_value["range"], "Sheet1!A1:A1");
        assert_eq!(json_value["name"], "test_image.png");
        // Note: image data is binary and serialized as array of numbers
        assert!(json_value["image"].is_array());
    }

    #[test]
    fn test_write_image_response_deserialization() {
        let response_json = serde_json::json!({
            "spreadsheetToken": "test_token_123",
            "revision": 456,
            "updateRange": "Sheet1!A1:A1"
        });

        let response: WriteImageResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.spread_sheet_token, "test_token_123");
        assert_eq!(response.revision, 456);
        assert_eq!(response.update_range, "Sheet1!A1:A1");
    }

    #[test]
    fn test_write_image_various_extensions() {
        let image_data = create_test_image_data();

        let extensions = vec!["png", "jpg", "jpeg", "gif", "bmp", "tiff", "heic", "webp"];

        for ext in extensions {
            let request = WriteImageRequest::builder()
                .spreadsheet_token("extension_test")
                .range("Images!A1:A1")
                .image(image_data.clone())
                .name(format!("test_image.{}", ext))
                .build();

            assert_eq!(request.name, format!("test_image.{}", ext));
            assert_eq!(request.image, image_data);
        }
    }

    #[test]
    fn test_write_image_long_filename() {
        let image_data = create_test_image_data();
        let long_name = "a".repeat(255) + ".png";

        let request = WriteImageRequest::builder()
            .spreadsheet_token("long_name_test")
            .range("Sheet1!A1:A1")
            .image(image_data.clone())
            .name(&long_name)
            .build();

        assert_eq!(request.name, long_name);
        assert_eq!(request.image, image_data);
    }

    #[test]
    fn test_write_image_binary_data_integrity() {
        let original_data = vec![0x89, 0x50, 0x4E, 0x47, 0xFF, 0x00, 0xAA, 0xBB];

        let request = WriteImageRequest::builder()
            .spreadsheet_token("binary_test")
            .range("Sheet1!A1:A1")
            .image(original_data.clone())
            .name("binary_test.png")
            .build();

        assert_eq!(request.image, original_data);
        // Verify each byte is preserved
        for (i, &byte) in original_data.iter().enumerate() {
            assert_eq!(request.image[i], byte);
        }
    }

    #[test]
    fn test_write_image_service_creation() {
        let service = create_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert!(matches!(service.config.app_type, AppType::SelfBuild));
    }

    #[test]
    fn test_write_image_complex_range_references() {
        let image_data = create_test_image_data();

        let complex_ranges = vec![
            "Sheet1!A1:A1",
            "'Sheet with spaces'!B2:B2",
            "工作表1!C3:C3",
            "Sheet-Name_123!D4:D4",
        ];

        for range in complex_ranges {
            let request = WriteImageRequest::builder()
                .spreadsheet_token("complex_range_test")
                .range(range)
                .image(image_data.clone())
                .name("test.png")
                .build();

            assert_eq!(request.range, range);
        }
    }

    #[test]
    fn test_write_image_different_image_sizes() {
        let spreadsheet_token = "size_test";

        // Test different image sizes
        let sizes = vec![1, 10, 100, 1000, 10000];

        for size in sizes {
            let image_data = vec![0xFFu8; size];
            let request = WriteImageRequest::builder()
                .spreadsheet_token(spreadsheet_token)
                .range("Sheet1!A1:A1")
                .image(image_data.clone())
                .name(format!("image_{}bytes.png", size))
                .build();

            assert_eq!(request.image.len(), size);
            assert_eq!(request.name, format!("image_{}bytes.png", size));
        }
    }

    #[test]
    fn test_write_image_metadata_only() {
        // Test building request with metadata but no image data yet
        let request = WriteImageRequest::builder()
            .spreadsheet_token("metadata_test")
            .range("Sheet1!A1:A1")
            .name("placeholder.png")
            .build();

        assert_eq!(request.spreadsheet_token, "metadata_test");
        assert_eq!(request.range, "Sheet1!A1:A1");
        assert_eq!(request.name, "placeholder.png");
        assert_eq!(request.image, Vec::<u8>::new());
    }

    #[test]
    fn test_write_image_very_long_token() {
        let very_long_token = "a".repeat(1000);
        let image_data = create_test_image_data();

        let request = WriteImageRequest::builder()
            .spreadsheet_token(&very_long_token)
            .range("Sheet1!A1:A1")
            .image(image_data.clone())
            .name("test.png")
            .build();

        assert_eq!(request.spreadsheet_token, very_long_token);
        assert_eq!(request.image, image_data);
    }

    #[test]
    fn test_write_image_response_struct_debug() {
        let response = WriteImageResponse {
            spread_sheet_token: "debug_test".to_string(),
            revision: 123,
            update_range: "Sheet1!A1:A1".to_string(),
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("debug_test"));
        assert!(debug_str.contains("123"));
        assert!(debug_str.contains("Sheet1!A1:A1"));
    }

    #[test]
    fn test_write_image_builder_overwrites() {
        let image_data1 = vec![1, 2, 3];
        let image_data2 = vec![4, 5, 6];

        let request = WriteImageRequest::builder()
            .spreadsheet_token("original_token")
            .spreadsheet_token("final_token") // Should overwrite
            .range("Sheet1!A1:A1")
            .range("Sheet2!B2:B2") // Should overwrite
            .image(image_data1)
            .image(image_data2.clone()) // Should overwrite
            .name("original.png")
            .name("final.png") // Should overwrite
            .build();

        assert_eq!(request.spreadsheet_token, "final_token");
        assert_eq!(request.range, "Sheet2!B2:B2");
        assert_eq!(request.image, image_data2);
        assert_eq!(request.name, "final.png");
    }
}
