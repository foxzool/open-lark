//! 根据 folderToken 在该 folder 下创建文件夹。
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/create-a-new-folder

pub mod children;
pub mod meta;
pub use children::*;
pub use meta::*;

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateFolderRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateFolderResponse {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct CreateFolderBuilder {
    api_req: ApiRequest<CreateFolderRequest>,
    folder_token: String,
}

impl CreateFolderBuilder {
    pub fn new(folder_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_drive_explorer_folder_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.folder_token = folder_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/drive/explorer/v2/folder/{}",
            builder.folder_token
        );
        builder.api_req.body = Some(CreateFolderRequest::default());
        builder
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
        req.build(AccessTokenType::Tenant, config, option)
    }
}
