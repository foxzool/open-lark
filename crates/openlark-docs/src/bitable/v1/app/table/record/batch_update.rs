//! Bitable 批量更新记录API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/record/batchUpdate
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量更新记录请求
#[allow(dead_code)]
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

        // 构建API路径
        let path = format!(
            "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update",
            self.app_token, self.table_id
        );

        // 创建API请求
        let mut api_request: ApiRequest<BatchUpdateRecordResponse> =
            ApiRequest::put(&format!("https://open.feishu.cn{}", path));

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // 构建请求体
        let request_body = BatchUpdateRecordRequestBody {
            records: self.records,
        };

        // 设置请求体
        api_request = api_request.body(RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
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

/// 批量更新记录数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRecordData {
    pub records: Vec<Record>,
}

/// 批量更新记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRecordResponse {
    /// 批量更新记录数据
    pub data: BatchUpdateRecordData,
}

impl ApiResponseTrait for BatchUpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
