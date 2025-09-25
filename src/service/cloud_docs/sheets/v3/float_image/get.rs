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

use super::create::FloatImageData;

impl SpreadsheetSheetService {
    /// 获取浮动图片
    pub async fn get_float_image(
        &self,
        request: GetFloatImageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetFloatImageResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FLOAT_IMAGE_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id)
            .replace("{}", &request.float_image_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 获取浮动图片请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GetFloatImageRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 浮动图片 ID
    float_image_id: String,
}

impl GetFloatImageRequest {
    pub fn builder() -> GetFloatImageRequestBuilder {
        GetFloatImageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetFloatImageRequestBuilder {
    request: GetFloatImageRequest,
}

impl GetFloatImageRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn float_image_id(mut self, float_image_id: impl ToString) -> Self {
        self.request.float_image_id = float_image_id.to_string();
        self
    }

    pub fn build(mut self) -> GetFloatImageRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    GetFloatImageRequestBuilder,
    SpreadsheetSheetService,
    GetFloatImageRequest,
    BaseResponse<GetFloatImageResponseData>,
    get_float_image
);

/// 获取浮动图片响应体最外层
#[derive(Deserialize, Debug)]
pub struct GetFloatImageResponseData {
    /// 浮动图片信息
    #[serde(flatten)]
    pub float_image: FloatImageInfo,
}

/// 浮动图片信息
#[derive(Deserialize, Debug)]
pub struct FloatImageInfo {
    /// 浮动图片 ID
    pub float_image_id: String,
    /// 浮动图片详细信息
    #[serde(flatten)]
    pub float_image: FloatImageData,
}

impl ApiResponseTrait for GetFloatImageResponseData {
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
    fn test_get_float_image_response() {
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

        let response: GetFloatImageResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.float_image.float_image_id, "fimg_001");
        assert_eq!(
            response.float_image.float_image.image_token,
            "img_token_123"
        );
    }
}
