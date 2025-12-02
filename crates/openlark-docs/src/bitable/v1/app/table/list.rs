//! 列出数据表模块

//! Bitable V1 列出数据表API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 列出数据表请求
#[derive(Debug, Clone)]
pub struct ListTablesRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<ListTablesResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl ListTablesRequest {
    /// 创建列出数据表请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get(""),
            app_token: String::new(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListTablesResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err(validation_error("page_size", "分页大小必须大于0"));
            }
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables",
            self.config.base_url, self.app_token
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 构建查询参数
        let mut query_params = Vec::new();

        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={}", page_size));
        }

        if let Some(ref page_token) = self.page_token {
            query_params.push(format!("page_token={}", page_token));
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
        let response_data: ListTablesResponseData = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析获取数据表列表响应失败", "响应数据格式不正确"))?;

        Ok(ListTablesResponse {
            items: response_data.items,
            page_token: response_data.page_token,
            has_more: response_data.has_more,
            success: response.raw_response.is_success(),
        })
    }
}

/// 列出数据表Builder
pub struct ListTablesRequestBuilder {
    request: ListTablesRequest,
}

impl ListTablesRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListTablesRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListTablesRequest {
        self.request
    }
}

/// 数据表信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableInfo {
    /// 数据表的ID
    pub table_id: String,
    /// 数据表的版本号
    pub revision: i32,
    /// 数据表的名称
    pub name: String,
}

/// 列出数据表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTablesResponse {
    /// 数据表列表
    pub items: Option<Vec<TableInfo>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 操作结果
    pub success: bool,
}

/// 列出数据表响应数据（内部使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTablesResponseData {
    /// 数据表列表
    pub items: Option<Vec<TableInfo>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}
