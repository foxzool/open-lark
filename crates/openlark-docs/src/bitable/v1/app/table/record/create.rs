//! Bitable 创建记录API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/record/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 创建记录请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CreateRecordRequest {
    api_request: ApiRequest<CreateRecordResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 操作的唯一标识，用于幂等的操作
    client_token: Option<String>,
    /// 记录数据
    fields: Value,
    /// 配置信息
    config: Config,
}

impl CreateRecordRequest {
    /// 创建记录请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post(""), // 占位符，将在execute方法中使用enum+builder系统
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            client_token: None,
            fields: Value::Object(Default::default()),
            config,
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

    /// 设置客户端令牌
    pub fn client_token(mut self, client_token: String) -> Self {
        self.client_token = Some(client_token);
        self
    }

    /// 设置记录数据
    pub fn fields(mut self, fields: Value) -> Self {
        self.fields = fields;
        self
    }

    /// 执行请求（集成现代化enum+builder API端点系统）
    pub async fn execute(self) -> SDKResult<CreateRecordResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RecordCreate(self.app_token.clone(), self.table_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CreateRecordResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        if let Some(ref client_token) = self.client_token {
            api_request = api_request.query("client_token", client_token);
        }

        // 构建请求体
        let request_body = CreateRecordRequestBody {
            fields: self.fields,
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

/// 创建记录Builder
pub struct CreateRecordRequestBuilder {
    request: CreateRecordRequest,
}

impl CreateRecordRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateRecordRequest::new(config),
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

    /// 设置客户端令牌
    pub fn client_token(mut self, client_token: String) -> Self {
        self.request = self.request.client_token(client_token);
        self
    }

    /// 设置记录数据
    pub fn fields(mut self, fields: Value) -> Self {
        self.request = self.request.fields(fields);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateRecordRequest {
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

/// 创建记录请求体（内部使用）
#[derive(Serialize)]
struct CreateRecordRequestBody {
    fields: Value,
}

/// 创建记录数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRecordData {
    /// 记录信息
    pub record: Record,
}

/// 创建记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRecordResponse {
    /// 创建记录数据
    pub data: CreateRecordData,
}

impl ApiResponseTrait for CreateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
