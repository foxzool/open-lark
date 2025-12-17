//! 新增多个数据表
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchCreateTableRequest {
    pub tables: Vec<TableSpec>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TableSpec {
    pub name: Option<String>,
    pub default_view_name: Option<String>,
    pub fields: Option<Vec<FieldSpec>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FieldSpec {
    pub field_name: String,
    pub field_type: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchCreateTableResponse {
    pub table_ids: Vec<String>,
}

impl ApiResponseTrait for BatchCreateTableResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct BatchCreateTable {
    config: openlark_core::config::Config,
    app_token: String,
    req: BatchCreateTableRequest,
}

impl BatchCreateTable {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            req: BatchCreateTableRequest::default(),
        }
    }

    pub fn tables(mut self, tables: Vec<TableSpec>) -> Self {
        self.req.tables = tables;
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<BatchCreateTableResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/batch_create",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
