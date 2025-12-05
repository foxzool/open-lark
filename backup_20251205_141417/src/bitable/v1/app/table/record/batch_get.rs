//! Bitable V1 批量获取记录API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
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

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get", self.app_token, self.table_id);

        // 创建API请求
        let mut api_request: ApiRequest<BatchGetRecordResponse> =
            ApiRequest::post(&format!("https://open.feishu.cn{}", path));

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // 构建请求体
        let request_body = BatchGetRecordRequestBody {
            record_ids: self.record_ids,
            automatic: self.automatic,
            with_shared_url: self.with_shared_url,
        };

        // 设置请求体
        api_request = api_request.body(RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
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

/// 批量获取记录数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetRecordData {
    /// 记录列表
    pub items: Vec<Record>,
}

/// 批量获取记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetRecordResponse {
    /// 批量获取记录数据
    pub data: BatchGetRecordData,
}

impl ApiResponseTrait for BatchGetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
