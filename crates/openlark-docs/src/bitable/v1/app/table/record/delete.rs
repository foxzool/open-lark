//! Bitable V1 删除记录API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 删除记录请求
#[derive(Debug, Clone)]
pub struct DeleteRecordRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<DeleteRecordResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 记录的 record_id
    record_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl DeleteRecordRequest {
    /// 创建删除记录请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::delete(""),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteRecordResponse> {
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
        let mut api_request: ApiRequest<DeleteRecordResponse> =
            ApiRequest::delete(&format!("https://open.feishu.cn{}", path));

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 删除记录Builder
pub struct DeleteRecordRequestBuilder {
    request: DeleteRecordRequest,
}

impl DeleteRecordRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteRecordRequest::new(config),
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

    /// 构建请求
    pub fn build(self) -> DeleteRecordRequest {
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
}

/// 删除记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRecordResponse {
    /// 记录信息
    pub data: Record,
}

impl ApiResponseTrait for DeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
