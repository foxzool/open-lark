//! 删除一个数据表
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteTableRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteTableResponse {}

impl ApiResponseTrait for DeleteTableResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct DeleteTable {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
}

impl DeleteTable {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
        }
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<DeleteTableResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::delete(&url);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
