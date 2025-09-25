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
    /// 删除浮动图片
    pub async fn delete_float_image(
        &self,
        request: DeleteFloatImageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteFloatImageResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::DELETE;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FLOAT_IMAGE_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id)
            .replace("{}", &request.float_image_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 删除浮动图片请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteFloatImageRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 浮动图片 ID
    float_image_id: String,
}

impl DeleteFloatImageRequest {
    pub fn builder() -> DeleteFloatImageRequestBuilder {
        DeleteFloatImageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteFloatImageRequestBuilder {
    request: DeleteFloatImageRequest,
}

impl DeleteFloatImageRequestBuilder {
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

    pub fn build(mut self) -> DeleteFloatImageRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    DeleteFloatImageRequestBuilder,
    SpreadsheetSheetService,
    DeleteFloatImageRequest,
    BaseResponse<DeleteFloatImageResponseData>,
    delete_float_image
);

/// 删除浮动图片响应体最外层
#[derive(Deserialize, Debug)]
pub struct DeleteFloatImageResponseData {
    /// 删除操作是否成功
    #[serde(default)]
    pub success: bool,
    /// 删除的浮动图片 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_image_id: Option<String>,
}

impl ApiResponseTrait for DeleteFloatImageResponseData {
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
    fn test_delete_float_image_response() {
        let json = json!({
            "success": true,
            "float_image_id": "fimg_001"
        });

        let response: DeleteFloatImageResponseData = serde_json::from_value(json).unwrap();
        assert!(response.success);
        assert_eq!(response.float_image_id, Some("fimg_001".to_string()));
    }
}
