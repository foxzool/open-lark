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

#[derive(Debug, Default)]
pub struct CreateTableBuilder {
    api_req: ApiRequest<CreateTableRequest>,
    app_token: String,
}

impl CreateTableBuilder {
    pub fn new(app_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_table_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables",
            builder.app_token
        );
        builder.api_req.body = Some(CreateTableRequest::default());
        builder
    }

    pub fn table(mut self, table: TableSpec) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.table = Some(table);
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
