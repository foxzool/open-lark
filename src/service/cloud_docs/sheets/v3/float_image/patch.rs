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
    /// 更新浮动图片
    pub async fn update_float_image(
        &self,
        request: UpdateFloatImageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateFloatImageResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PATCH;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FLOAT_IMAGE_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id)
            .replace("{}", &request.float_image_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 更新浮动图片请求
#[derive(Default, Debug, Serialize, Deserialize)]
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
    float_image: FloatImageData,
}

impl UpdateFloatImageRequest {
    pub fn builder() -> UpdateFloatImageRequestBuilder {
        UpdateFloatImageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateFloatImageRequestBuilder {
    request: UpdateFloatImageRequest,
}

impl UpdateFloatImageRequestBuilder {
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

    pub fn float_image(mut self, float_image: FloatImageData) -> Self {
        self.request.float_image = float_image;
        self
    }

    pub fn build(mut self) -> UpdateFloatImageRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    UpdateFloatImageRequestBuilder,
    SpreadsheetSheetService,
    UpdateFloatImageRequest,
    BaseResponse<UpdateFloatImageResponseData>,
    update_float_image
);

/// 更新浮动图片响应体最外层
#[derive(Deserialize, Debug)]
pub struct UpdateFloatImageResponseData {
    /// 浮动图片 ID
    pub float_image_id: String,
    /// 更新后的浮动图片信息
    #[serde(flatten)]
    pub float_image: FloatImageData,
}

impl ApiResponseTrait for UpdateFloatImageResponseData {
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
    fn test_update_float_image_response() {
        let json = json!({
            "float_image_id": "fimg_001",
            "image_token": "img_token_456",
            "position": {
                "start_col_index": 2,
                "start_row_index": 2,
                "offset_x": 15.0,
                "offset_y": 25.0
            },
            "size": {
                "width": 300.0,
                "height": 200.0
            },
            "name": "更新后的图片"
        });

        let response: UpdateFloatImageResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.float_image_id, "fimg_001");
        assert_eq!(response.float_image.image_token, "img_token_456");
        assert_eq!(response.float_image.size.width, 300.0);
    }
}
