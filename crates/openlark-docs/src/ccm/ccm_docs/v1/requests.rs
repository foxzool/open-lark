/// CCM Docs V1 请求类型定义
use serde::{Deserialize, Serialize};

/// 搜索云文档请求
#[derive(Debug, Clone)]
pub struct SearchObjectRequest {
    /// 搜索关键字
    pub query: String,
    /// 搜索类型：doc, sheet, bitable, mindnote, file, folder
    pub doc_type: Option<String>,
    /// 搜索范围：all, owned, shared
    pub scope: Option<String>,
    /// 每页数量，默认20，最大100
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
    /// 排序字段：create_time, update_time, title
    pub sort_field: Option<String>,
    /// 排序方向：asc, desc
    pub sort_direction: Option<String>,
}

impl SearchObjectRequest {
    /// 创建新的搜索请求
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            doc_type: None,
            scope: None,
            page_size: None,
            page_token: None,
            sort_field: None,
            sort_direction: None,
        }
    }

    /// 设置文档类型
    pub fn doc_type(mut self, doc_type: &str) -> Self {
        self.doc_type = Some(doc_type.to_string());
        self
    }

    /// 设置搜索范围
    pub fn scope(mut self, scope: &str) -> Self {
        self.scope = Some(scope.to_string());
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    /// 设置排序字段
    pub fn sort_field(mut self, sort_field: &str) -> Self {
        self.sort_field = Some(sort_field.to_string());
        self
    }

    /// 设置排序方向
    pub fn sort_direction(mut self, sort_direction: &str) -> Self {
        self.sort_direction = Some(sort_direction.to_string());
        self
    }
}

/// 获取元数据请求
#[derive(Debug, Clone)]
pub struct MetaDataRequest {
    /// 文档token列表
    pub obj_tokens: Vec<String>,
    /// 返回类型
    pub return_type: Option<String>,
}

impl MetaDataRequest {
    /// 创建新的获取元数据请求
    pub fn new(obj_tokens: Vec<&str>) -> Self {
        Self {
            obj_tokens: obj_tokens.into_iter().map(|s| s.to_string()).collect(),
            return_type: None,
        }
    }

    /// 设置返回类型
    pub fn return_type(mut self, return_type: &str) -> Self {
        self.return_type = Some(return_type.to_string());
        self
    }
}