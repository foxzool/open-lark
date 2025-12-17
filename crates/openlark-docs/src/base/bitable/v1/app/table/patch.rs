//! 更新数据表
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/patch

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchTableRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchTableResponse {
    pub name: String,
}

impl ApiResponseTrait for PatchTableResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct PatchTable {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    req: PatchTableRequest,
}

impl PatchTable {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            req: PatchTableRequest::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.req.name = Some(name.into());
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<PatchTableResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::patch(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
