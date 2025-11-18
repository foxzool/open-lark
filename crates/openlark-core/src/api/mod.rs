//! API处理模块 - 简化版本
//!
//! 现代化、类型安全的API请求和响应处理系统。
//! 这个版本专注于核心功能，避免复杂的依赖关系。

// ============================================================================
// 核心类型定义
// ============================================================================

use crate::api_resp::RawResponse;
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

/// 请求数据枚举
#[derive(Debug, Clone)]
pub enum RequestData {
    Json(serde_json::Value),
    Text(String),
    Binary(Vec<u8>),
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

/// API响应特征
pub trait ApiResponseTrait: Send + Sync + 'static {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 响应格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseFormat {
    Data,
    Flatten,
    Binary,
    Text,
    Custom,
}

// 为常见类型实现ApiResponseTrait
impl ApiResponseTrait for serde_json::Value {}
impl ApiResponseTrait for String {}
impl ApiResponseTrait for Vec<u8> {}
impl ApiResponseTrait for () {}

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
}

/// API响应结构
#[derive(Debug, Clone)]
pub struct ApiResponse<R> {
    pub data: Option<R>,
    pub raw_response: RawResponse,
}

impl<R> ApiResponse<R> {
    pub fn new(data: Option<R>, raw_response: RawResponse) -> Self {
        Self { data, raw_response }
    }

    pub fn is_success(&self) -> bool {
        self.raw_response.code == 0
    }

    pub fn code(&self) -> i32 {
        self.raw_response.code
    }

    pub fn message(&self) -> &str {
        &self.raw_response.msg
    }

    pub fn data(&self) -> Option<&R> {
        self.data.as_ref()
    }
}

// ============================================================================
// 子模块
// ============================================================================

pub mod prelude;
pub mod traits;

// ============================================================================
// 重新导出
// ============================================================================

pub use traits::{AsyncApiClient, SyncApiClient};
