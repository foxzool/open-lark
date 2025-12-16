/// Bitable 批量创建数据记录API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/record/batchCreate
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
};
use serde::{Deserialize, Serialize};


/// 批量创建数据记录请求
#[allow(dead_code)]
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

    pub async fn execute(self) -> SDKResult<BatchCreateRecordResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "app_token",
                "应用token不能为空",
            ));
        }

        if self.table_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "table_id",
                "数据表ID不能为空",
            ));
        }

        if self.records.is_empty() {
            return Err(openlark_core::error::validation_error(
                "records",
                "记录列表不能为空",
            ));
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RecordBatchCreate(self.app_token.clone(), self.table_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<BatchCreateRecordResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 构建请求体
        let body = serde_json::json!({
            "records": self.records
        });

        // 设置请求体
        let api_request = api_request.body(RequestData::Binary(serde_json::to_vec(&body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 数据记录
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Record {
    pub fields: serde_json::Value,
}

/// 批量创建记录数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRecordData {
    /// 创建的记录列表
    pub records: Vec<BatchCreateRecordResult>,
    /// 记录总数
    pub total: i32,
}

/// 批量创建记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRecordResponse {
    /// 批量创建记录数据
    pub data: BatchCreateRecordData,
}

/// 批量创建记录结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
