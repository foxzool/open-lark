/// Bitable 新增多条记录
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_create
/// doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::Record;

/// 批量新增记录请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BatchCreateRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    user_id_type: Option<String>,
    client_token: Option<String>,
    ignore_consistency_check: Option<bool>,
    records: Vec<CreateRecordItem>,
}

impl BatchCreateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            client_token: None,
            ignore_consistency_check: None,
            records: Vec::new(),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub fn client_token(mut self, client_token: String) -> Self {
        self.client_token = Some(client_token);
        self
    }

    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.ignore_consistency_check = Some(ignore_consistency_check);
        self
    }

    pub fn records(mut self, records: Vec<CreateRecordItem>) -> Self {
        self.records = records;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreateRecordResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "table_id 不能为空"));
        }
        if self.records.is_empty() {
            return Err(validation_error("records", "records 不能为空"));
        }
        if self.records.len() > 500 {
            return Err(validation_error("records", "单次最多新增 500 条记录"));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RecordBatchCreate(self.app_token.clone(), self.table_id.clone());

        let mut api_request: ApiRequest<BatchCreateRecordResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&BatchCreateRecordRequestBody {
            records: self.records,
        })?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt("client_token", self.client_token);
        api_request = api_request.query_opt(
            "ignore_consistency_check",
            self.ignore_consistency_check.map(|v| v.to_string()),
        );

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 批量新增记录 Builder
pub struct BatchCreateRecordRequestBuilder {
    request: BatchCreateRecordRequest,
}

impl BatchCreateRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchCreateRecordRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    pub fn client_token(mut self, client_token: String) -> Self {
        self.request = self.request.client_token(client_token);
        self
    }

    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.request = self
            .request
            .ignore_consistency_check(ignore_consistency_check);
        self
    }

    pub fn records(mut self, records: Vec<CreateRecordItem>) -> Self {
        self.request = self.request.records(records);
        self
    }

    pub fn build(self) -> BatchCreateRecordRequest {
        self.request
    }
}

/// 新增记录条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRecordItem {
    /// 记录字段
    pub fields: Value,
}

#[derive(Serialize)]
struct BatchCreateRecordRequestBody {
    records: Vec<CreateRecordItem>,
}

/// 批量新增记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRecordResponse {
    /// 创建的记录列表
    pub records: Vec<Record>,
}

impl ApiResponseTrait for BatchCreateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
