//! 下载图片
//!
//! doc: https://open.feishu.cn/document/server-docs/baike-v1/file/download

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DownloadFileRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DownloadFileResponse {}

impl ApiResponseTrait for DownloadFileResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct DownloadFileBuilder {
    api_req: ApiRequest<DownloadFileRequest>,
    file_token: String,
}

impl DownloadFileBuilder {
    pub fn new(file_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.method = openlark_core::api::HttpMethod::Get;
        builder.file_token = file_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/baike/v1/files/{}/download",
            builder.file_token
        );
        builder.api_req.body = None;
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
