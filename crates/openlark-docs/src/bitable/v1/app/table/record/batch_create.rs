//! 批量创建数据记录模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量创建数据记录请求
pub struct BatchCreateRecordRequest {
    api_request: ApiRequest<BatchCreateRecordResponse>,
    config: Config,
    app_token: String,
    table_id: String,
    records: Vec<Record>,
}

impl Default for BatchCreateRecordRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::post("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create"),
            config: Config::default(),
            app_token: String::new(),
            table_id: String::new(),
            records: Vec::new(),
        }
    }
}

impl BatchCreateRecordRequest {
    pub fn new(config: Config) -> Self {
        let mut request = Self::default();
        request.config = config;
        request
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
        let url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create",
            self.app_token, self.table_id
        );

        // 创建新的API请求，用于Transport
        let api_request = ApiRequest::<()>::post(&url);

        let body = serde_json::json!({
            "records": self.records
        });

        let request_for_transport = api_request.body(body);

        let response = Transport::request(request_for_transport, &self.config, None).await?;
        response.data.ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 数据记录
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Record {
    pub fields: serde_json::Value,
}

/// 批量创建记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateRecordResponse {
    pub records: Vec<BatchCreateRecordResult>,
    pub total: i32,
}

/// 批量创建记录结果
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct BatchCreateRecordRequestBuilder {
    request: BatchCreateRecordRequest,
}

impl Default for BatchCreateRecordRequestBuilder {
    fn default() -> Self {
        Self {
            request: BatchCreateRecordRequest::default(),
        }
    }
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