//! 该接口用于在数据表中删除一个字段
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteFieldRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteFieldResponse {
    pub field_id: String,
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteFieldResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct DeleteField {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    field_id: String,
}

impl DeleteField {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>, field_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            field_id: field_id.into(),
        }
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<DeleteFieldResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}",
            self.config.base_url, self.app_token, self.table_id, self.field_id
        );
        let request = ApiRequest::delete(&url);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
