/// Bitable 列出数据表API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 列出数据表请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ListTablesRequest {
    api_request: ApiRequest<ListTablesResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListTablesRequest {
    /// 创建列出数据表请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::get("/open-apis/bitable/v1/apps/:app_token/tables"),
            app_token: String::new(),
            page_size: None,
            page_token: None,
            config,
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

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::TableList(self.app_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListTablesResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 构建查询参数
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        if let Some(ref page_token) = self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
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

/// 列出数据表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListTablesData {
    /// 数据表列表
    pub items: Option<Vec<TableInfo>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

/// 列出数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListTablesResponse {
    /// 列出数据表数据
    pub data: ListTablesData,
}

impl ApiResponseTrait for ListTablesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
