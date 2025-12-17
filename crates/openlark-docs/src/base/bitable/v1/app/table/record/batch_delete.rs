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

#[derive(Debug)]
pub struct BatchDeleteRecord {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    req: BatchDeleteRecordRequest,
}

impl BatchDeleteRecord {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            req: BatchDeleteRecordRequest::default(),
        }
    }

    pub fn records(mut self, records: Vec<String>) -> Self {
        self.req.records = records;
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<BatchDeleteRecordResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
