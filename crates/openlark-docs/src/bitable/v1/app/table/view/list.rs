//! Bitable V1 列出视图API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 列出视图请求
#[derive(Debug, Clone)]
pub struct ListViewsRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<ListViewsResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
}

impl ListViewsRequest {
    /// 创建列出视图请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get(""),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
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

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListViewsResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err(validation_error("page_size", "分页大小必须大于0"));
            }
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/views",
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

        if let Some(ref page_token) = self.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={}", page_size));
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
        let response_data: ListViewsResponseData = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析获取视图列表响应失败", "响应数据格式不正确"))?;

        Ok(ListViewsResponse {
            has_more: response_data.has_more,
            page_token: response_data.page_token,
            items: response_data.items,
            success: response.raw_response.is_success(),
        })
    }
}

/// 列出视图Builder
pub struct ListViewsRequestBuilder {
    request: ListViewsRequest,
}

impl ListViewsRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListViewsRequest::new(config),
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

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListViewsRequest {
        self.request
    }
}

/// 列出视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListViewsResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 视图信息列表
    pub items: Vec<View>,
    /// 操作结果
    pub success: bool,
}

/// 列出视图响应数据（内部使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListViewsResponseData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 视图信息列表
    pub items: Vec<View>,
}
