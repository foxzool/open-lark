//! 该接口用于更新数据表中的多条记录，单次调用最多更新 500 条记录。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_update

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateRecordRequest {
    pub records: Vec<RecordSpec>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RecordSpec {
    pub record_id: String,
    pub fields: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateRecordResponse {
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

impl ApiResponseTrait for BatchUpdateRecordResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct BatchUpdateRecord {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    req: BatchUpdateRecordRequest,
}

impl BatchUpdateRecord {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            req: BatchUpdateRecordRequest::default(),
        }
    }

    pub fn records(mut self, records: Vec<RecordSpec>) -> Self {
        self.req.records = records;
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<BatchUpdateRecordResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
