//! 列出数据表模块

use openlark_core::{
    core::{
        BaseResponse,
        ResponseFormat,
        api::ApiResponseTrait,
    },
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 列出数据表请求
#[derive(Clone)]
pub struct ListTablesRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListTablesRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::GET,
                LIST_TABLES.to_string(),
            ),
            app_token: String::new(),
            page_size: None,
            page_token: None,
        }
    }

    pub fn builder() -> ListTablesRequestBuilder {
        ListTablesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListTablesRequestBuilder {
    request: ListTablesRequest,
}

impl ListTablesRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn build(self) -> ListTablesRequest {
        self.request
    }
}

/// 数据表信息
#[derive(Clone, Serialize)]
pub struct TableInfo {
    /// 数据表的ID
    pub table_id: String,
    /// 数据表的版本号
    pub revision: i32,
    /// 数据表的名称
    pub name: String,
}

/// 列出数据表响应
#[derive(Clone)]
pub struct ListTablesResponse {
    /// 数据表列表
    pub items: Option<Vec<TableInfo>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListTablesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tables_request() {
        let request = ListTablesRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .page_size(50)
            .page_token("page_token")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token".to_string()));
    }

    #[test]
    fn test_page_size_limit() {
        let request = ListTablesRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .page_size(200) // 超过100的限制
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }
}