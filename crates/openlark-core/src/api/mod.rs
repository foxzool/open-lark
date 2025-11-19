//! API处理模块 - 独立版本
//!
//! 现代化、类型安全的API请求和响应处理系统。
//! 完全独立，不依赖已弃用的api_req/api_resp模块。

// ============================================================================
// 核心类型定义
// ============================================================================

pub use responses::RawResponse;
use std::{collections::HashMap, time::Duration};

/// HTTP方法枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl HttpMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 请求数据枚举
#[derive(Debug, Clone)]
pub enum RequestData {
    Json(serde_json::Value),
    Text(String),
    Binary(Vec<u8>),
    Form(std::collections::HashMap<String, String>),
    Empty,
}

impl From<serde_json::Value> for RequestData {
    fn from(value: serde_json::Value) -> Self {
        RequestData::Json(value)
    }
}

impl From<String> for RequestData {
    fn from(value: String) -> Self {
        RequestData::Text(value)
    }
}

impl From<&str> for RequestData {
    fn from(value: &str) -> Self {
        RequestData::Text(value.to_string())
    }
}

// 重新导出响应类型
pub use responses::{ApiResponseTrait, BaseResponse, ErrorInfo, Response, ResponseFormat};

/// 简化的API请求结构
#[derive(Debug, Clone)]
pub struct ApiRequest<R> {
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub body: Option<RequestData>,
    pub timeout: Option<Duration>,
    pub _phantom: std::marker::PhantomData<R>,
}

impl<R> ApiRequest<R> {
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Get,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Post,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn put(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Put,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn delete(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Delete,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn query<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.query.insert(key.into(), value.into());
        self
    }

    pub fn body(mut self, body: impl Into<RequestData>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    pub fn build_url(&self) -> String {
        if self.query.is_empty() {
            self.url.clone()
        } else {
            let query_string = self
                .query
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}?{}", self.url, query_string)
        }
    }

    // 兼容性字段和方法，用于与现有http.rs代码兼容
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn api_path(&self) -> &str {
        // 从URL中提取路径部分
        if let Some(start) = self.url.find("/open-apis/") {
            &self.url[start..]
        } else {
            &self.url
        }
    }

    pub fn supported_access_token_types(&self) -> Vec<crate::constants::AccessTokenType> {
        // 默认返回用户和租户令牌类型
        vec![
            crate::constants::AccessTokenType::User,
            crate::constants::AccessTokenType::Tenant,
        ]
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match &self.body {
            Some(RequestData::Json(data)) => serde_json::to_vec(data).unwrap_or_default(),
            Some(RequestData::Binary(data)) => data.clone(),
            Some(RequestData::Form(data)) => data
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&")
                .into_bytes(),
            Some(RequestData::Text(text)) => text.clone().into_bytes(),
            Some(RequestData::Empty) => vec![],
            None => vec![],
        }
    }

    pub fn file(&self) -> Vec<u8> {
        // 默认返回空，因为新结构不直接支持文件上传
        vec![]
    }
}

// 类型别名，保持兼容性
pub type ApiResponse<R> = Response<R>;

// ============================================================================
// 子模块
// ============================================================================

pub mod prelude;
pub mod responses;
pub mod traits;

// ============================================================================
// 重新导出
// ============================================================================

pub use traits::{AsyncApiClient, SyncApiClient};
