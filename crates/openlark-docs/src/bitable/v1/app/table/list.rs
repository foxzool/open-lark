//! 列出数据表模块

use openlark_core::{
    api::{
        ApiRequest, ApiResponseTrait, BaseResponse, ResponseFormat, HttpMethod,
    },
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use crate::endpoints::BITABLE_V1_TABLES;

/// 列出数据表请求
pub struct ListTablesRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl Default for ListTablesRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::get(BITABLE_V1_TABLES),
            app_token: String::new(),
            page_size: None,
            page_token: None,
        }
    }
}

impl ListTablesRequest {
    pub fn new(config: Config) -> Self {
        Self::default()
    }

    pub fn builder() -> ListTablesRequestBuilder {
        ListTablesRequestBuilder::default()
    }
}

pub struct ListTablesRequestBuilder {
    request: ListTablesRequest,
}

impl Default for ListTablesRequestBuilder {
    fn default() -> Self {
        Self {
            request: ListTablesRequest::default(),
        }
    }
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
#[derive(Debug, Clone)]
pub struct TableInfo {
    /// 数据表的ID
    pub table_id: String,
    /// 数据表的版本号
    pub revision: i32,
    /// 数据表的名称
    pub name: String,
}

/// 列出数据表响应
#[derive(Debug, Clone)]
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

