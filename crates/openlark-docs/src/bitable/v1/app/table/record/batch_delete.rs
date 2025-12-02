//! Bitable V1 批量删除记录API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量删除记录请求
#[derive(Debug, Clone)]
pub struct BatchDeleteRecordRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<BatchDeleteRecordResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 记录 ID 列表
    record_ids: Vec<String>,
}

impl BatchDeleteRecordRequest {
    /// 创建批量删除记录请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::delete(""),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            record_ids: Vec::new(),
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

    /// 设置记录ID列表
    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.record_ids = record_ids;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchDeleteRecordResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        if self.record_ids.is_empty() {
            return Err(validation_error("record_ids", "记录ID列表不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete",
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
        let request_body = BatchDeleteRecordRequestBody {
            record_ids: self.record_ids,
        };

        // 设置请求体
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::delete(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let batch_delete_data: BatchDeleteRecordData = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析批量删除记录响应失败", "响应数据格式不正确"))?;

        Ok(BatchDeleteRecordResponse {
            records: batch_delete_data.records,
            success: response.raw_response.is_success(),
        })
    }
}

/// 批量删除记录Builder
pub struct BatchDeleteRecordRequestBuilder {
    request: BatchDeleteRecordRequest,
}

impl BatchDeleteRecordRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchDeleteRecordRequest::new(config),
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

    /// 设置记录ID列表
    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.request = self.request.record_ids(record_ids);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchDeleteRecordRequest {
        self.request
    }
}

/// 被删除的记录信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeletedRecord {
    /// 记录 ID
    pub record_id: String,
    /// 是否删除成功
    pub deleted: bool,
}

/// 批量删除记录请求体（内部使用）
#[derive(Debug, Serialize)]
struct BatchDeleteRecordRequestBody {
    record_ids: Vec<String>,
}

/// 批量删除记录数据（内部使用）
#[derive(Debug, Deserialize)]
struct BatchDeleteRecordData {
    records: Vec<DeletedRecord>,
}

/// 批量删除记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRecordResponse {
    /// 被删除的记录信息
    pub records: Vec<DeletedRecord>,
    /// 操作结果
    pub success: bool,
}
