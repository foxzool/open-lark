//! 批量获取记录
//!
//! doc: https://open.feishu.cn/document/docs/bitable-v1/app-table-record/batch_get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetRecordRequest {
    pub record_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetRecordResponse {
    pub records: Vec<AppTableRecord>,
    pub forbidden_record_ids: Option<Vec<String>>,
    pub absent_record_ids: Option<Vec<String>>,
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

impl ApiResponseTrait for BatchGetRecordResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchGetRecordBuilder {
    api_req: ApiRequest<BatchGetRecordRequest>,
    app_token: String,
    table_id: String,
}

impl BatchGetRecordBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_record_batch_get".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get",
            builder.app_token, builder.table_id
        );
        builder.api_req.body = Some(BatchGetRecordRequest::default());
        builder
    }

    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.record_ids = record_ids;
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
