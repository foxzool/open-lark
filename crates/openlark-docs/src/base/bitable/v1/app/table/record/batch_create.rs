//! 该接口用于在数据表中新增多条记录，单次调用最多新增 500 条记录。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchCreateRecordRequest {
    pub records: Vec<RecordSpec>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecordSpec {
    pub fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchCreateRecordResponse {
    pub records: Vec<AppTableRecord>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppTableRecord {
    pub record_id: String,
    pub created_by: Option<User>,
    pub created_time: Option<i64>,
    pub last_modified_by: Option<User>,
    pub last_modified_time: Option<i64>,
    pub fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
    pub en_name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

impl ApiResponseTrait for BatchCreateRecordResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchCreateRecordBuilder {
    api_req: ApiRequest<BatchCreateRecordRequest>,
    app_token: String,
    table_id: String,
}

impl BatchCreateRecordBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_record_batch_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create",
            builder.app_token, builder.table_id
        );
        builder.api_req.body = Some(BatchCreateRecordRequest::default());
        builder
    }

    pub fn records(mut self, records: Vec<RecordSpec>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.records = records;
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
