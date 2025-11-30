//! 批量创建数据记录模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量创建数据记录请求
#[derive(Clone)]
pub struct BatchCreateRecordRequest {
    api_request: ApiRequest<BatchCreateRecordResponse>,
    app_token: String,
    table_id: String,
    records: Vec<Record>,
}

impl BatchCreateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Post)
                .api_path("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create".to_string())
                .config(config)
                .build(),
            app_token: String::new(),
            table_id: String::new(),
            records: Vec::new(),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.table_id = table_id.into();
        self
    }

    pub fn records(mut self, records: Vec<Record>) -> Self {
        self.records = records;
        self
    }

    pub async fn execute(mut self) -> SDKResult<BatchCreateRecordResponse> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create",
            self.app_token, self.table_id
        );
        self.api_request = self.api_request.api_path(path);

        let body = serde_json::json!({
            "records": self.records
        });

        self.api_request = self.api_request.body(serde_json::to_vec(&body)?);

        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }
}

/// 数据记录
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub fields: serde_json::Value,
}

/// 批量创建记录响应
#[derive(Clone)]
pub struct BatchCreateRecordResponse {
    pub records: Vec<BatchCreateRecordResult>,
    pub total: i32,
}

/// 批量创建记录结果
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateRecordResult {
    pub record_id: String,
    pub fields: serde_json::Value,
    pub created_time: String,
}

impl ApiResponseTrait for BatchCreateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量创建记录请求构建器
#[derive(Default)]
pub struct BatchCreateRecordRequestBuilder {
    request: BatchCreateRecordRequest,
}

impl BatchCreateRecordRequestBuilder {
    pub fn new(app_token: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.request.app_token = app_token.into();
        builder
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    pub fn records(mut self, records: Vec<Record>) -> Self {
        self.request.records = records;
        self
    }

    pub fn build(self) -> BatchCreateRecordRequest {
        self.request
    }
}