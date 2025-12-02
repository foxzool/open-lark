//! Bitable V1 批量获取记录API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量获取记录请求
#[derive(Debug, Clone)]
pub struct BatchGetRecordRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<BatchGetRecordResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 记录ID列表
    record_ids: Vec<String>,
    /// 是否返回自动计算字段
    automatic: Option<bool>,
    /// 是否返回记录权限
    with_shared_url: Option<bool>,
}

impl BatchGetRecordRequest {
    /// 创建批量获取记录请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post(""),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            record_ids: Vec::new(),
            automatic: None,
            with_shared_url: None,
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

    /// 设置是否返回自动计算字段
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.automatic = Some(automatic);
        self
    }

    /// 设置是否返回记录权限
    pub fn with_shared_url(mut self, with_shared_url: bool) -> Self {
        self.with_shared_url = Some(with_shared_url);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetRecordResponse> {
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

        if self.record_ids.len() > 500 {
            return Err(validation_error("record_ids", "记录ID数量不能超过500个"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get",
            self.config.base_url, self.app_token, self.table_id
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request.url = format!("{}?user_id_type={}", api_request.url, user_id_type);
        }

        // 构建请求体
        let request_body = BatchGetRecordRequestBody {
            record_ids: self.record_ids,
            automatic: self.automatic,
            with_shared_url: self.with_shared_url,
        };

        // 设置请求体
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::post(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let response_data: BatchGetRecordResponseData = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析批量获取记录响应失败", "响应数据格式不正确"))?;

        Ok(BatchGetRecordResponse {
            items: response_data.items,
            success: response.raw_response.is_success(),
        })
    }
}

/// 批量获取记录Builder
pub struct BatchGetRecordRequestBuilder {
    request: BatchGetRecordRequest,
}

impl BatchGetRecordRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchGetRecordRequest::new(config),
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

    /// 设置是否返回自动计算字段
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request = self.request.automatic(automatic);
        self
    }

    /// 设置是否返回记录权限
    pub fn with_shared_url(mut self, with_shared_url: bool) -> Self {
        self.request = self.request.with_shared_url(with_shared_url);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchGetRecordRequest {
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

/// 批量获取记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetRecordResponse {
    /// 记录列表
    pub items: Vec<Record>,
    /// 操作结果
    pub success: bool,
}

/// 批量获取记录响应数据（内部使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetRecordResponseData {
    /// 记录列表
    pub items: Vec<Record>,
}

/// 批量获取记录请求体（内部使用）
#[derive(Serialize)]
struct BatchGetRecordRequestBody {
    record_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_shared_url: Option<bool>,
}
