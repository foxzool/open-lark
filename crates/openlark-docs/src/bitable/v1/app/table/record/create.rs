//! Bitable V1 创建记录API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 创建记录请求
#[derive(Debug, Clone)]
pub struct CreateRecordRequest {
    /// 配置信息
    config: Config,
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
}

impl CreateRecordRequest {
    /// 创建记录请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post(""),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            client_token: None,
            fields: Value::Object(Default::default()),
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateRecordResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records",
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

        if let Some(ref client_token) = self.client_token {
            query_params.push(format!("client_token={}", client_token));
        }

        // 添加查询参数到URL
        if !query_params.is_empty() {
            api_request.url = format!("{}?{}", api_request.url, query_params.join("&"));
        }

        // 构建请求体
        let request_body = CreateRecordRequestBody {
            fields: self.fields,
        };

        // 设置请求体
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::post(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let record_data: Record = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析创建记录响应失败", "响应数据格式不正确"))?;

        Ok(CreateRecordResponse {
            record: record_data,
            success: response.raw_response.is_success(),
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

/// 创建记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRecordResponse {
    /// 记录信息
    pub record: Record,
    /// 操作结果
    pub success: bool,
}
