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

use super::get::FloatImageInfo;

impl SpreadsheetSheetService {
    /// 查询浮动图片
    pub async fn query_float_images(
        &self,
        request: QueryFloatImagesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryFloatImagesResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FLOAT_IMAGES
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 查询浮动图片请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct QueryFloatImagesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
}

impl QueryFloatImagesRequest {
    pub fn builder() -> QueryFloatImagesRequestBuilder {
        QueryFloatImagesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct QueryFloatImagesRequestBuilder {
    request: QueryFloatImagesRequest,
}

impl QueryFloatImagesRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn build(mut self) -> QueryFloatImagesRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    QueryFloatImagesRequestBuilder,
    SpreadsheetSheetService,
    QueryFloatImagesRequest,
    BaseResponse<QueryFloatImagesResponseData>,
    query_float_images
);

/// 查询浮动图片响应体最外层
#[derive(Deserialize, Debug)]
pub struct QueryFloatImagesResponseData {
    /// 浮动图片列表
    pub items: Vec<FloatImageInfo>,
    /// 是否还有更多数据
    #[serde(default)]
    pub has_more: bool,
    /// 下次请求的页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for QueryFloatImagesResponseData {
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
    fn test_query_float_images_response() {
        let json = json!({
            "items": [
                {
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
                    "name": "图片1"
                },
                {
                    "float_image_id": "fimg_002",
                    "image_token": "img_token_456",
                    "position": {
                        "start_col_index": 3,
                        "start_row_index": 3,
                        "offset_x": 0.0,
                        "offset_y": 0.0
                    },
                    "size": {
                        "width": 150.0,
                        "height": 100.0
                    },
                    "name": "图片2"
                }
            ],
            "has_more": false
        });

        let response: QueryFloatImagesResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].float_image_id, "fimg_001");
        assert_eq!(response.items[1].float_image.image_token, "img_token_456");
        assert!(!response.has_more);
    }
}
