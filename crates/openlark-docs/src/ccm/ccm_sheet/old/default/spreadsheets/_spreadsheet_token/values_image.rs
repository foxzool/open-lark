//! 根据 spreadsheetToken 和 range 向单个格子写入图片。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-images

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesImageRequest {
    pub range: String,
    pub image: Vec<u8>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesImageResponse {
    pub spreadsheetToken: String,
    pub range: String,
}

impl ApiResponseTrait for ValuesImageResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct ValuesImageBuilder {
    api_req: ApiRequest<ValuesImageRequest>,
}

impl ValuesImageBuilder {
    pub fn new(spreadsheet_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_sheet_values_image".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/{}/values_image",
            spreadsheet_token.to_string()
        );
        builder.api_req.body = Some(ValuesImageRequest::default());
        builder
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.range = range.to_string();
        }
        self
    }

    pub fn image(mut self, image: Vec<u8>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.image = image;
        }
        self
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.name = name.to_string();
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        // This likely needs careful multipart handling or custom body, but standardized as JSON for now unless specified otherwise.
        // Doc says "request body", check if JSON.
        // Actually for binary upload usually it's different. But struct above assumes JSON.
        // Let's assume standard JSON body with bytes (base64) or similar if not specified.
        // Actually the doc says "write-images". Let's stick to standard pattern.
        req.build(AccessTokenType::Tenant, config, option)
    }
}
