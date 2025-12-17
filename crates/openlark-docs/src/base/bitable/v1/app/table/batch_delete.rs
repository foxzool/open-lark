//! 删除多个数据表
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteTableRequest {
    pub table_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteTableResponse {}

impl ApiResponseTrait for BatchDeleteTableResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct BatchDeleteTable {
    config: openlark_core::config::Config,
    app_token: String,
    req: BatchDeleteTableRequest,
}

impl BatchDeleteTable {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            req: BatchDeleteTableRequest::default(),
        }
    }

    pub fn table_ids(mut self, table_ids: Vec<String>) -> Self {
        self.req.table_ids = table_ids;
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<BatchDeleteTableResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/batch_delete",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
