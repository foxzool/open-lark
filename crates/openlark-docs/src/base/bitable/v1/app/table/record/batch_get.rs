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

#[derive(Debug)]
pub struct BatchGetRecord {
    config: openlark_core::config::Config,
    app_token: String,
    table_id: String,
    req: BatchGetRecordRequest,
}

impl BatchGetRecord {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            table_id: table_id.into(),
            req: BatchGetRecordRequest::default(),
        }
    }

    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.req.record_ids = record_ids;
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.req.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<BatchGetRecordResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get",
            self.config.base_url, self.app_token, self.table_id
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
