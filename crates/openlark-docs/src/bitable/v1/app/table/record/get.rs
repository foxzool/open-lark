//! Bitable V1 获取记录API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取记录请求
#[derive(Debug, Clone)]
pub struct GetRecordRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<GetRecordResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 记录的 record_id
    record_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 视图的唯一标识符
    view_id: Option<String>,
    /// 字段名称，用于指定本次查询返回记录中包含的字段
    field_names: Option<Vec<String>>,
    /// 控制是否返回自动计算的字段
    automatic: Option<bool>,
}

impl GetRecordRequest {
    /// 创建获取记录请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get(""),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            view_id: None,
            field_names: None,
            automatic: None,
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

    /// 设置记录ID
    pub fn record_id(mut self, record_id: String) -> Self {
        self.record_id = record_id;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.view_id = Some(view_id);
        self
    }

    /// 设置字段名称
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.field_names = Some(field_names);
        self
    }

    /// 设置是否返回自动计算字段
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.automatic = Some(automatic);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetRecordResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        if self.record_id.trim().is_empty() {
            return Err(validation_error("record_id", "记录ID不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
            self.config.base_url, self.app_token, self.table_id, self.record_id
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 构建查询参数
        let mut query_params = Vec::new();

        if let Some(ref user_id_type) = self.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }

        if let Some(ref view_id) = self.view_id {
            query_params.push(format!("view_id={}", view_id));
        }

        if let Some(ref field_names) = self.field_names {
            query_params.push(format!("field_names={}", field_names.join(",")));
        }

        if let Some(automatic) = self.automatic {
            query_params.push(format!("automatic={}", automatic));
        }

        // 添加查询参数到URL
        if !query_params.is_empty() {
            api_request.url = format!("{}?{}", api_request.url, query_params.join("&"));
        }

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> =
            ApiRequest::get(api_request.url.clone()).body(RequestData::Empty);

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let record_data: Record = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析获取记录响应失败", "响应数据格式不正确"))?;

        Ok(GetRecordResponse {
            record: record_data,
            success: response.raw_response.is_success(),
        })
    }
}

/// 获取记录Builder
pub struct GetRecordRequestBuilder {
    request: GetRecordRequest,
}

impl GetRecordRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetRecordRequest::new(config),
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

    /// 设置记录ID
    pub fn record_id(mut self, record_id: String) -> Self {
        self.request = self.request.record_id(record_id);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.request = self.request.view_id(view_id);
        self
    }

    /// 设置字段名称
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.request = self.request.field_names(field_names);
        self
    }

    /// 设置是否返回自动计算字段
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request = self.request.automatic(automatic);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetRecordRequest {
        self.request
    }
}

/// 记录信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    /// 记录ID
    pub record_id: String,
    /// 字段数据
    pub fields: serde_json::Value,
    /// 创建时间
    pub created_time: String,
    /// 最后更新时间
    pub last_modified_time: String,
    /// 记录的创建者信息
    pub created_by: Option<CreatorInfo>,
    /// 记录的最后修改者信息
    pub last_modified_by: Option<UpdaterInfo>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 更新者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdaterInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 获取记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRecordResponse {
    /// 记录信息
    pub record: Record,
    /// 操作结果
    pub success: bool,
}
