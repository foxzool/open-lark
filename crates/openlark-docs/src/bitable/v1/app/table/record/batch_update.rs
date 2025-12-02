//! Bitable V1 批量更新记录API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量更新记录请求
#[derive(Debug, Clone)]
pub struct BatchUpdateRecordRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<BatchUpdateRecordResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 要更新的记录列表
    records: Vec<BatchUpdateRecordItem>,
}

/// 批量更新记录项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRecordItem {
    /// 记录ID
    pub record_id: String,
    /// 记录字段
    pub fields: Value,
}

impl BatchUpdateRecordRequest {
    /// 创建批量更新记录请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::put(""),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            records: Vec::new(),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置记录列表
    pub fn records(mut self, records: Vec<BatchUpdateRecordItem>) -> Self {
        self.records = records;
        self
    }

    /// 添加记录
    pub fn add_record(mut self, record_id: String, fields: Value) -> Self {
        self.records
            .push(BatchUpdateRecordItem { record_id, fields });
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchUpdateRecordResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        if self.records.is_empty() {
            return Err(validation_error("records", "记录列表不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update",
            self.config.base_url, self.app_token, self.table_id
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 构建查询参数
        let mut query_params = Vec::new();

        if let Some(ref user_id_type) = self.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }

        // 添加查询参数到URL
        if !query_params.is_empty() {
            api_request.url = format!("{}?{}", api_request.url, query_params.join("&"));
        }

        // 构建请求体
        let request_body = BatchUpdateRecordRequestBody {
            records: self.records,
        };

        // 设置请求体
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::put(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let batch_update_data: BatchUpdateRecordData = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析批量更新记录响应失败", "响应数据格式不正确"))?;

        Ok(BatchUpdateRecordResponse {
            records: batch_update_data.records,
            success: response.raw_response.is_success(),
        })
    }
}

/// 批量更新记录Builder
pub struct BatchUpdateRecordRequestBuilder {
    request: BatchUpdateRecordRequest,
}

impl BatchUpdateRecordRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchUpdateRecordRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置记录列表
    pub fn records(mut self, records: Vec<BatchUpdateRecordItem>) -> Self {
        self.request = self.request.records(records);
        self
    }

    /// 添加记录
    pub fn add_record(mut self, record_id: String, fields: Value) -> Self {
        self.request = self.request.add_record(record_id, fields);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchUpdateRecordRequest {
        self.request
    }
}

/// 记录信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    /// 记录ID
    pub record_id: String,
    /// 字段数据
    pub fields: Value,
    /// 创建时间
    pub created_time: String,
    /// 最后更新时间
    pub last_modified_time: String,
}

/// 批量更新记录请求体（内部使用）
#[derive(Debug, Serialize)]
struct BatchUpdateRecordRequestBody {
    records: Vec<BatchUpdateRecordItem>,
}

/// 批量更新记录数据（内部使用）
#[derive(Debug, Deserialize)]
struct BatchUpdateRecordData {
    records: Vec<Record>,
}

/// 批量更新记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRecordResponse {
    /// 记录信息
    pub records: Vec<Record>,
    /// 操作结果
    pub success: bool,
}
