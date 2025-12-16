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

#[derive(Debug, Default)]
pub struct BatchCreateTableBuilder {
    api_req: ApiRequest<BatchCreateTableRequest>,
    app_token: String,
}

impl BatchCreateTableBuilder {
    pub fn new(app_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_table_batch_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/batch_create",
            builder.app_token
        );
        builder.api_req.body = Some(BatchCreateTableRequest::default());
        builder
    }

    pub fn tables(mut self, tables: Vec<TableSpec>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.tables = tables;
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
