//! 上传图片
//!
//! doc: https://open.feishu.cn/document/lingo-v1/file/upload

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UploadFileRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UploadFileResponse {
    pub file_token: String,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct UploadFileBuilder {
    api_req: ApiRequest<UploadFileRequest>,
}

impl UploadFileBuilder {
    pub fn new(name: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "lingo_file_upload".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/lingo/v1/files/upload".to_string();
        builder.api_req.body = Some(UploadFileRequest {
            name: name.to_string(),
        });
        builder
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
