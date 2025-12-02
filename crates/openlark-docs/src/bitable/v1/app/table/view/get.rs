//! Bitable V1 获取视图API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 获取视图请求
#[derive(Debug, Clone)]
pub struct GetViewRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<GetViewResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl GetViewRequest {
    /// 创建获取视图请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get(""),
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
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

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.view_id = view_id;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetViewResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        if self.view_id.trim().is_empty() {
            return Err(validation_error("view_id", "视图ID不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
            self.config.base_url, self.app_token, self.table_id, self.view_id
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request.url = format!("{}?user_id_type={}", api_request.url, user_id_type);
        }

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> =
            ApiRequest::get(api_request.url.clone()).body(RequestData::Empty);

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let view_data: View = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析获取视图响应失败", "响应数据格式不正确"))?;

        Ok(GetViewResponse {
            view: view_data,
            success: response.raw_response.is_success(),
        })
    }
}

/// 获取视图Builder
pub struct GetViewRequestBuilder {
    request: GetViewRequest,
}

impl GetViewRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetViewRequest::new(config),
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

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.request = self.request.view_id(view_id);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetViewRequest {
        self.request
    }
}

/// 获取视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetViewResponse {
    /// 视图信息
    pub view: View,
    /// 操作结果
    pub success: bool,
}
