//! Bitable V1 获取记录API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
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

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/{}", self.app_token, self.table_id, self.record_id);

        // 创建API请求
        let mut api_request: ApiRequest<GetRecordResponse> =
            ApiRequest::get(&format!("https://open.feishu.cn{}", path));

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        if let Some(ref view_id) = self.view_id {
            api_request = api_request.query("view_id", view_id);
        }

        if let Some(ref field_names) = self.field_names {
            api_request = api_request.query("field_names", &field_names.join(","));
        }

        if let Some(automatic) = self.automatic {
            api_request = api_request.query("automatic", &automatic.to_string());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
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

/// 获取记录数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRecordData {
    /// 记录信息
    pub record: Record,
}

/// 获取记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRecordResponse {
    /// 获取记录数据
    pub data: GetRecordData,
}

impl ApiResponseTrait for GetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
