//! 该接口用于删除数据表中现有的多条记录，单次调用中最多删除 500 条记录。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteRecordRequest {
    pub records: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteRecordResponse {
    pub records: Vec<DeleteRecordResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteRecordResult {
    pub deleted: bool,
    pub record_id: String,
}

impl ApiResponseTrait for BatchDeleteRecordResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchDeleteRecordBuilder {
    api_req: ApiRequest<BatchDeleteRecordRequest>,
    app_token: String,
    table_id: String,
}

impl BatchDeleteRecordBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_record_batch_delete".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete",
            builder.app_token, builder.table_id
        );
        builder.api_req.body = Some(BatchDeleteRecordRequest::default());
        builder
    }

    pub fn records(mut self, records: Vec<String>) -> Self {
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
