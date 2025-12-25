/// Bitable 删除多条记录
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_delete
/// doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量删除记录请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BatchDeleteRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_ids: Vec<String>,
}

impl BatchDeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_ids: Vec::new(),
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

    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.record_ids = record_ids;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchDeleteRecordResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "table_id 不能为空"));
        }
        if self.record_ids.is_empty() {
            return Err(validation_error("record_ids", "record_ids 不能为空"));
        }
        if self.record_ids.len() > 500 {
            return Err(validation_error("record_ids", "单次最多删除 500 条记录"));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RecordBatchDelete(self.app_token.clone(), self.table_id.clone());

        let api_request: ApiRequest<BatchDeleteRecordResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(
                &BatchDeleteRecordRequestBody {
                    record_ids: self.record_ids,
                },
            )?);

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 批量删除记录 Builder
pub struct BatchDeleteRecordRequestBuilder {
    request: BatchDeleteRecordRequest,
}

impl BatchDeleteRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchDeleteRecordRequest::new(config),
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

    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.request = self.request.record_ids(record_ids);
        self
    }

    pub fn build(self) -> BatchDeleteRecordRequest {
        self.request
    }
}

#[derive(Serialize)]
struct BatchDeleteRecordRequestBody {
    record_ids: Vec<String>,
}

/// 删除结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeletedRecord {
    pub deleted: bool,
    pub record_id: String,
}

/// 批量删除记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRecordResponse {
    pub records: Vec<DeletedRecord>,
}

impl ApiResponseTrait for BatchDeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
