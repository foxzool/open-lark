//! Bitable V1 批量删除记录API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
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

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete", self.app_token, self.table_id);

        // 创建API请求
        let mut api_request: ApiRequest<BatchDeleteRecordResponse> =
            ApiRequest::delete(&format!("https://open.feishu.cn{}", path));

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // 构建请求体
        let request_body = BatchDeleteRecordRequestBody {
            record_ids: self.record_ids,
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

/// 批量删除记录数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRecordData {
    /// 被删除的记录信息
    pub records: Vec<DeletedRecord>,
}

/// 批量删除记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRecordResponse {
    /// 批量删除记录数据
    pub data: BatchDeleteRecordData,
}

impl ApiResponseTrait for BatchDeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
