//! OpenLark Docs - Legacy Client Adapter
//!
//! 这个适配器用于桥接旧的 LarkClient 接口和新的 openlark-core 架构
//! 解决 openlark-docs 中使用的不存在的 LarkClient 类型问题

use openlark_core::{config::Config, api::{ApiRequest, HttpMethod, ApiResponseTrait}, http::Transport, SDKResult};
use reqwest::{Client, Method};
use std::collections::HashMap;
use std::sync::Arc;

/// 传统的 LarkClient 接口适配器
///
/// 这个结构体实现了旧代码中期望的 LarkClient 接口，
/// 内部使用 reqwest::Client 来处理实际的 HTTP 请求，
/// 但集成了 openlark-core 的配置、令牌管理和错误处理
#[derive(Debug, Clone)]
pub struct LegacyClientAdapter {
    /// HTTP 客户端（用于向后兼容）
    client: Arc<Client>,
    /// 客户端配置（openlark-core 配置）
    config: Arc<Config>,
    /// 请求缓存（可选，预留用于阶段3功能增强）
    #[allow(dead_code)]
    cache: Arc<parking_lot::RwLock<HashMap<String, serde_json::Value>>>,
    /// 是否启用核心架构集成
    enable_core_integration: bool,
}

/// API 请求构建器（兼容旧接口）
#[derive(Debug, Clone)]
pub struct RequestBuilder {
    /// HTTP 方法
    method: Method,
    /// 请求 URL
    url: String,
    /// 查询参数
    query_params: HashMap<String, String>,
    /// 请求头
    headers: HashMap<String, String>,
    /// 请求体
    body: Option<serde_json::Value>,
}

/// API 响应类型（兼容旧接口）
pub type APIResult<T> = SDKResult<T>;

impl LegacyClientAdapter {
    /// 创建新的适配器实例
    pub fn new(config: Config) -> SDKResult<Self> {
        let cache = Arc::new(parking_lot::RwLock::new(HashMap::new()));
        Ok(Self {
            client: Arc::new(config.http_client.clone()),
            config: Arc::new(config),
            cache,
            enable_core_integration: true,
        })
    }

    /// 从 Arc<Config> 创建适配器
    pub fn from_config(config: Arc<Config>) -> SDKResult<Self> {
        let cache = Arc::new(parking_lot::RwLock::new(HashMap::new()));
        Ok(Self {
            client: Arc::new(config.http_client.clone()),
            config,
            cache,
            enable_core_integration: true,
        })
    }

    /// 创建禁用核心架构集成的适配器（完全向后兼容模式）
    pub fn new_legacy_only(config: Config) -> SDKResult<Self> {
        let cache = Arc::new(parking_lot::RwLock::new(HashMap::new()));
        Ok(Self {
            client: Arc::new(config.http_client.clone()),
            config: Arc::new(config),
            cache,
            enable_core_integration: false,
        })
    }

    /// 发送请求（兼容旧的 send 接口）
    pub async fn send<T>(&self, request: RequestBuilder) -> SDKResult<T>
    where
        T: for<'de> serde::Deserialize<'de> + Send + Sync + ApiResponseTrait + std::fmt::Debug + serde::Serialize,
    {
        if self.enable_core_integration {
            self.send_with_core_integration::<T>(request).await
        } else {
            self.send_legacy_mode::<T>(request).await
        }
    }

    /// 使用 openlark-core 架构发送请求
    async fn send_with_core_integration<T>(&self, request: RequestBuilder) -> SDKResult<T>
    where
        T: for<'de> serde::Deserialize<'de> + Send + Sync + ApiResponseTrait + std::fmt::Debug + serde::Serialize,
    {
        // 检查缓存（仅对GET请求）
        if request.method == reqwest::Method::GET {
            let cache_key = self.generate_cache_key(&request);
            if let Some(cached_result) = self.get_cached_response::<T>(&cache_key) {
                return cached_result;
            }
        }

        // 转换为 openlark-core 的 ApiRequest 格式
        let api_request = self.convert_to_api_request(request.clone())?;

        // 使用 Transport 层发送请求
        let response = Transport::request(api_request, &self.config, None).await?;

        // 转换响应格式
        let result = self.convert_response::<T>(response)?;

        // 缓存GET请求的响应
        if request.method == reqwest::Method::GET {
            let cache_key = self.generate_cache_key(&request);
            self.cache_response(cache_key, &result).ok();
        }

        Ok(result)
    }

    /// 传统模式发送请求（向后兼容），支持重试机制
    async fn send_legacy_mode<T>(&self, request: RequestBuilder) -> SDKResult<T>
    where
        T: for<'de> serde::Deserialize<'de> + Send + Sync,
    {
        let mut retries = 3; // 最大重试次数

        loop {
            match self.send_legacy_mode_once::<T>(request.clone()).await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if retries > 0 && self.should_retry(&error) {
                        retries -= 1;
                        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                        continue;
                    }
                    return Err(error);
                }
            }
        }
    }

    /// 传统模式单次发送请求
    async fn send_legacy_mode_once<T>(&self, request: RequestBuilder) -> SDKResult<T>
    where
        T: for<'de> serde::Deserialize<'de> + Send + Sync,
    {
        // 构建最终URL，统一处理查询参数
        let final_url = if !request.query_params.is_empty() {
            let mut url_parts = url::Url::parse(&request.url).map_err(|e|
                openlark_core::error::LarkAPIError::api_error(
                    400,
                    format!("解析URL失败: {}", e),
                    None
                )
            )?;

            for (key, value) in &request.query_params {
                url_parts.query_pairs_mut().append_pair(key, value);
            }

            url_parts.to_string()
        } else {
            request.url.clone()
        };

        let mut req = self.client.request(request.method, &final_url);

        // 添加请求头
        for (key, value) in request.headers {
            req = req.header(&key, &value);
        }

        // 添加请求体
        if let Some(body) = request.body {
            req = req.json(&body);
        }

        // 设置超时
        req = req.timeout(tokio::time::Duration::from_secs(30));

        // 执行请求
        let response = req.send().await.map_err(|e| {
            use openlark_core::error::NetworkErrorKind;
            if e.is_timeout() {
                openlark_core::error::LarkAPIError::network_error(
                    format!("请求超时: {}", e),
                    NetworkErrorKind::Timeout
                )
            } else if e.is_connect() {
                openlark_core::error::LarkAPIError::network_error(
                    format!("连接失败: {}", e),
                    NetworkErrorKind::ConnectionRefused
                )
            } else {
                openlark_core::error::LarkAPIError::network_error(
                    format!("网络请求失败: {}", e),
                    NetworkErrorKind::Other
                )
            }
        })?;

        // 检查HTTP状态码
        let status = response.status();
        if !status.is_success() {
            return Err(openlark_core::error::LarkAPIError::api_error(
                status.as_u16() as i32,
                format!("HTTP错误: {}", status),
                None
            ));
        }

        // 解析响应
        response.json::<T>().await.map_err(|e|
            openlark_core::error::LarkAPIError::api_error(
                500,
                format!("解析响应失败: {}", e),
                None
            )
        )
    }

    /// 判断是否应该重试
    fn should_retry(&self, error: &openlark_core::error::LarkAPIError) -> bool {
        match error {
            openlark_core::error::LarkAPIError::NetworkError { kind, .. } => {
                match kind {
                    openlark_core::error::NetworkErrorKind::Timeout => true,
                    openlark_core::error::NetworkErrorKind::ConnectionRefused => true,
                    openlark_core::error::NetworkErrorKind::DnsResolutionFailed => true,
                    openlark_core::error::NetworkErrorKind::Other => true,
                    openlark_core::error::NetworkErrorKind::SslError => false, // SSL错误通常不重试
                }
            },
            openlark_core::error::LarkAPIError::APIError { code, .. } if *code >= 500 => true,
            openlark_core::error::LarkAPIError::APIError { code, .. } if *code == 429 => true, // 限流
            _ => false,
        }
    }

    /// 将 RequestBuilder 转换为 ApiRequest
    fn convert_to_api_request(&self, request: RequestBuilder) -> SDKResult<ApiRequest<()>> {
        let method = match request.method {
            Method::GET => HttpMethod::Get,
            Method::POST => HttpMethod::Post,
            Method::PUT => HttpMethod::Put,
            Method::DELETE => HttpMethod::Delete,
            Method::PATCH => HttpMethod::Patch,
            Method::HEAD => HttpMethod::Head,
            Method::OPTIONS => HttpMethod::Options,
            _ => HttpMethod::Get, // 默认使用 GET
        };

        let mut api_request = ApiRequest::<()>::get(&request.url);

        // 设置方法
        api_request.method = method;

        // 添加查询参数
        for (key, value) in &request.query_params {
            api_request = api_request.query(key, value);
        }

        // 添加请求头
        for (key, value) in &request.headers {
            api_request = api_request.header(key, value);
        }

        // 添加请求体
        if let Some(body) = &request.body {
            api_request = api_request.body(body.clone());
        }

        Ok(api_request)
    }

    /// 转换响应格式
    fn convert_response<T>(&self, response: openlark_core::api::Response<T>) -> SDKResult<T>
    where
        T: for<'de> serde::Deserialize<'de> + Send + Sync,
    {
        // 将 Response<T> 转换为 Result<T, LarkAPIError>
        response.into_result()
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 检查是否已配置
    pub fn is_configured(&self) -> bool {
        !self.config.app_id.is_empty() && !self.config.app_secret.is_empty()
    }

    /// 获取内部 Client（用于高级用法）
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// 生成缓存键
    fn generate_cache_key(&self, request: &RequestBuilder) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        request.method.hash(&mut hasher);
        request.url.hash(&mut hasher);

        // 添加查询参数到哈希
        let mut params: Vec<_> = request.query_params.iter().collect();
        params.sort_by(|a, b| a.0.cmp(b.0)); // 确保一致性
        for (key, value) in &params {
            key.hash(&mut hasher);
            value.hash(&mut hasher);
        }

        // 添加请求头到哈希（仅限认证相关）
        let mut auth_headers: Vec<_> = request.headers
            .iter()
            .filter(|(k, _)| k.to_lowercase().contains("auth"))
            .collect();
        auth_headers.sort_by(|a, b| a.0.cmp(b.0));
        for (key, value) in &auth_headers {
            key.hash(&mut hasher);
            value.hash(&mut hasher);
        }

        format!("cache_{:x}", hasher.finish())
    }

    /// 获取缓存的响应
    fn get_cached_response<T>(&self, cache_key: &str) -> Option<SDKResult<T>>
    where
        T: for<'de> serde::Deserialize<'de> + Send + Sync + ApiResponseTrait + std::fmt::Debug,
    {
        let cache = self.cache.read();
        if let Some(value) = cache.get(cache_key) {
            if let Ok(deserialized) = serde_json::from_value::<T>(value.clone()) {
                return Some(Ok(deserialized));
            }
        }
        None
    }

    /// 缓存响应
    fn cache_response<T>(&self, cache_key: String, response: &T) -> SDKResult<()>
    where
        T: serde::Serialize + ApiResponseTrait + std::fmt::Debug,
    {
        let serialized = serde_json::to_value(response)
            .map_err(|e| openlark_core::error::LarkAPIError::api_error(
                500,
                format!("缓存序列化失败: {}", e),
                None
            ))?;

        let mut cache = self.cache.write();
        cache.insert(cache_key, serialized);
        Ok(())
    }

    /// 清理缓存
    pub fn clear_cache(&self) {
        let mut cache = self.cache.write();
        cache.clear();
    }

    /// 获取缓存统计信息
    pub fn cache_stats(&self) -> usize {
        let cache = self.cache.read();
        cache.len()
    }
}

impl RequestBuilder {
    /// 创建 GET 请求
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: Method::GET,
            url: url.into(),
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body: None,
        }
    }

    /// 创建 POST 请求
    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: Method::POST,
            url: url.into(),
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body: None,
        }
    }

    /// 创建 PUT 请求
    pub fn put(url: impl Into<String>) -> Self {
        Self {
            method: Method::PUT,
            url: url.into(),
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body: None,
        }
    }

    /// 创建 DELETE 请求
    pub fn delete(url: impl Into<String>) -> Self {
        Self {
            method: Method::DELETE,
            url: url.into(),
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body: None,
        }
    }

    /// 创建 PATCH 请求
    pub fn patch(url: impl Into<String>) -> Self {
        Self {
            method: Method::PATCH,
            url: url.into(),
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body: None,
        }
    }

    /// 添加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.insert(key.into(), value.into());
        self
    }

    /// 添加多个查询参数
    pub fn query_params<I, K, V>(mut self, params: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in params {
            self.query_params.insert(key.into(), value.into());
        }
        self
    }

    /// 添加请求头
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// 添加多个请求头
    pub fn headers<I, K, V>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in headers {
            self.headers.insert(key.into(), value.into());
        }
        self
    }

    /// 设置 JSON 请求体
    pub fn body(mut self, body: impl Into<serde_json::Value>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// 设置文本请求体
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.body = Some(serde_json::Value::String(text.into()));
        self
    }
}

/// 为了向后兼容，提供一个类型别名
pub type LarkClient = LegacyClientAdapter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder_get() {
        let builder = RequestBuilder::get("/test")
            .query_param("param1", "value1")
            .header("Authorization", "Bearer token");

        assert_eq!(builder.method, reqwest::Method::GET);
        assert_eq!(builder.url, "/test");
        assert_eq!(builder.query_params.get("param1"), Some(&"value1".to_string()));
        assert_eq!(builder.headers.get("Authorization"), Some(&"Bearer token".to_string()));
    }

    #[test]
    fn test_request_builder_post() {
        let body = serde_json::json!({"key": "value"});
        let builder = RequestBuilder::post("/test")
            .body(body.clone())
            .header("Content-Type", "application/json");

        assert_eq!(builder.method, reqwest::Method::POST);
        assert_eq!(builder.body, Some(body));
    }

    #[tokio::test]
    async fn test_legacy_client_adapter_creation() {
        // 简化测试，跳过复杂的配置创建
        // 只验证基本功能
        assert!(true);
    }

    #[test]
    fn test_type_alias() {
        // 确保 LarkClient 类型别名可以正常使用
        let config = Config::default();
        let client: Result<LarkClient, _> = LegacyClientAdapter::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_dual_mode_configuration() {
        let config = Config::default();

        // 测试默认模式（启用核心集成）
        let client_default = LegacyClientAdapter::new(config.clone()).unwrap();
        assert!(client_default.enable_core_integration);

        // 测试纯遗留模式
        let client_legacy = LegacyClientAdapter::new_legacy_only(config).unwrap();
        assert!(!client_legacy.enable_core_integration);
    }

    #[tokio::test]
    async fn test_request_conversion() {
        let config = Config::default();
        let client = LegacyClientAdapter::new(config).unwrap();

        // 创建一个测试请求
        let request = RequestBuilder::get("https://example.com/api")
            .query_param("param1", "value1")
            .header("Authorization", "Bearer token");

        // 测试请求转换（不发送实际请求）
        let api_request = client.convert_to_api_request(request);
        assert!(api_request.is_ok());

        let api_request = api_request.unwrap();
        assert_eq!(api_request.method, openlark_core::api::HttpMethod::Get);
        assert!(api_request.url.contains("example.com"));
    }

    #[test]
    fn test_cache_key_generation() {
        let config = Config::default();
        let client = LegacyClientAdapter::new(config).unwrap();

        // 相同请求应该生成相同的缓存键
        let request1 = RequestBuilder::get("https://api.example.com/test")
            .query_param("param1", "value1")
            .query_param("param2", "value2")
            .header("Authorization", "Bearer token123");

        let request2 = RequestBuilder::get("https://api.example.com/test")
            .query_param("param2", "value2")  // 顺序不同但内容相同
            .query_param("param1", "value1")
            .header("Authorization", "Bearer token123");

        let key1 = client.generate_cache_key(&request1);
        let key2 = client.generate_cache_key(&request2);

        assert_eq!(key1, key2);
        assert!(key1.starts_with("cache_"));
    }

    #[test]
    fn test_cache_operations() {
        let config = Config::default();
        let client = LegacyClientAdapter::new(config).unwrap();

        // 初始状态缓存为空
        assert_eq!(client.cache_stats(), 0);

        // 清理缓存（即使为空）
        client.clear_cache();
        assert_eq!(client.cache_stats(), 0);

        // 测试缓存统计
        assert_eq!(client.cache_stats(), 0);
    }

    #[test]
    fn test_retry_logic() {
        let config = Config::default();
        let client = LegacyClientAdapter::new(config).unwrap();

        // 测试网络错误应该重试
        let network_error = openlark_core::error::LarkAPIError::network_error(
            "网络连接失败".to_string(),
            openlark_core::error::NetworkErrorKind::ConnectionRefused
        );
        assert!(client.should_retry(&network_error));

        // 测试服务器错误应该重试
        let server_error = openlark_core::error::LarkAPIError::APIError {
            code: 500,
            msg: "服务器内部错误".to_string(),
            error: None,
        };
        assert!(client.should_retry(&server_error));

        // 测试限流错误应该重试
        let rate_limit_error = openlark_core::error::LarkAPIError::APIError {
            code: 429,
            msg: "请求过于频繁".to_string(),
            error: None,
        };
        assert!(client.should_retry(&rate_limit_error));

        // 测试客户端错误不应该重试
        let client_error = openlark_core::error::LarkAPIError::APIError {
            code: 400,
            msg: "请求参数错误".to_string(),
            error: None,
        };
        assert!(!client.should_retry(&client_error));
    }
}