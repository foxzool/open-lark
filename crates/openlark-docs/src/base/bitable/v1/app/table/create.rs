//! 新增一个数据表
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateTableRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableSpec>,
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
pub struct CreateTableResponse {
    pub table_id: String,
    pub default_view_id: String,
    pub field_id_list: Vec<String>,
}

impl ApiResponseTrait for CreateTableResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct CreateTable {
    config: openlark_core::config::Config,
    app_token: String,
    req: CreateTableRequest,
}

impl CreateTable {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            req: CreateTableRequest::default(),
        }
    }

    pub fn table(mut self, table: TableSpec) -> Self {
        self.req.table = Some(table);
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<CreateTableResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables",
            self.config.base_url, self.app_token
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
