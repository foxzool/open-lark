# Open-Lark 增强错误处理设计

## 概述

提供更加详细、类型安全、用户友好的错误处理机制，提升调试体验和错误诊断能力。

## 错误类型设计

### 核心错误枚举

```rust
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum SDKError {
    /// 网络相关错误
    #[error("Network error: {source}")]
    Network {
        #[from]
        source: reqwest::Error,
        url: Option<String>,
        method: Option<String>,
        retry_count: u32,
    },

    /// API调用错误
    #[error("API error (code: {code}): {message}")]
    API {
        code: i32,
        message: String,
        request_id: Option<String>,
        error_detail: Option<Value>,
        help_suggestion: Option<String>,
        is_retryable: bool,
    },

    /// 认证错误
    #[error("Authentication failed: {reason}")]
    Authentication {
        reason: String,
        token_type: Option<String>,
        expires_at: Option<i64>,
        refresh_url: Option<String>,
    },

    /// 权限错误
    #[error("Permission denied: {permission}")]
    Permission {
        permission: String,
        resource: String,
        required_scope: Option<String>,
    },

    /// 限流错误
    #[error("Rate limited: retry after {retry_after}s, limit: {limit}")]
    RateLimited {
        retry_after: u64,
        limit: Option<u32>,
        reset_time: Option<u64>,
        bucket_type: Option<String>,
    },

    /// 参数验证错误
    #[error("Validation error: {field} - {message}")]
    Validation {
        field: String,
        message: String,
        value: Option<String>,
        constraint: Option<String>,
    },

    /// 配置错误
    #[error("Configuration error: {field} - {message}")]
    Configuration {
        field: String,
        message: String,
        current_value: Option<String>,
        expected_type: Option<String>,
    },

    /// 序列化错误
    #[error("Serialization error: {context}")]
    Serialization {
        #[from]
        source: serde_json::Error,
        context: String,
        field_path: Option<String>,
    },

    /// 业务逻辑错误
    #[error("Business logic error: {operation}")]
    Business {
        operation: String,
        reason: String,
        context: HashMap<String, Value>,
    },

    /// 内部错误
    #[error("Internal error: {component} - {message}")]
    Internal {
        component: String,
        message: String,
        error_id: String,
        stack_trace: Option<String>,
    },

    /// WebSocket错误
    #[error("WebSocket error: {reason}")]
    WebSocket {
        reason: String,
        code: Option<u16>,
        reconnect_suggested: bool,
    },

    /// 超时错误
    #[error("Timeout error: {operation} after {duration_ms}ms")]
    Timeout {
        operation: String,
        duration_ms: u64,
        timeout_type: TimeoutType,
    },
}

#[derive(Debug, Clone)]
pub enum TimeoutType {
    Request,
    Connection,
    Read,
    Write,
}
```

### 错误上下文和诊断

```rust
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub request_id: String,
    pub timestamp: i64,
    pub operation: String,
    pub user_id: Option<String>,
    pub app_id: Option<String>,
    pub api_version: String,
    pub request_metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ErrorDiagnostic {
    pub error: SDKError,
    pub context: ErrorContext,
    pub suggestions: Vec<String>,
    pub related_docs: Vec<String>,
    pub support_links: Vec<String>,
}

impl SDKError {
    /// 检查错误是否可重试
    pub fn is_retryable(&self) -> bool {
        match self {
            SDKError::Network { retry_count, .. } => *retry_count < 3,
            SDKError::API { is_retryable, .. } => *is_retryable,
            SDKError::RateLimited { .. } => true,
            SDKError::Timeout { .. } => true,
            _ => false,
        }
    }

    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            SDKError::Network { method, url, .. } => {
                format!("网络连接失败，请检查网络连接后重试。请求: {} {}",
                       method.as_deref().unwrap_or("UNKNOWN"),
                       url.as_deref().unwrap_or("UNKNOWN"))
            },
            SDKError::API { code, message, .. } => {
                format!("API调用失败 (错误码: {}): {}", code, message)
            },
            SDKError::Authentication { reason, refresh_url, .. } => {
                if let Some(url) = refresh_url {
                    format!("认证失败: {}。请访问 {} 重新认证。", reason, url)
                } else {
                    format!("认证失败: {}", reason)
                }
            },
            SDKError::RateLimited { retry_after, .. } => {
                format!("请求频率过高，请等待 {} 秒后重试。", retry_after)
            },
            SDKError::Validation { field, message, .. } => {
                format!("参数验证失败 - {}: {}", field, message)
            },
            SDKError::Timeout { operation, duration_ms, .. } => {
                format!("操作超时: {} 在 {}ms 后未完成。", operation, duration_ms)
            },
            _ => self.to_string(),
        }
    }

    /// 获取错误代码
    pub fn error_code(&self) -> &'static str {
        match self {
            SDKError::Network { .. } => "NETWORK_ERROR",
            SDKError::API { .. } => "API_ERROR",
            SDKError::Authentication { .. } => "AUTH_ERROR",
            SDKError::Permission { .. } => "PERMISSION_ERROR",
            SDKError::RateLimited { .. } => "RATE_LIMITED",
            SDKError::Validation { .. } => "VALIDATION_ERROR",
            SDKError::Configuration { .. } => "CONFIG_ERROR",
            SDKError::Serialization { .. } => "SERIALIZATION_ERROR",
            SDKError::Business { .. } => "BUSINESS_ERROR",
            SDKError::Internal { .. } => "INTERNAL_ERROR",
            SDKError::WebSocket { .. } => "WEBSOCKET_ERROR",
            SDKError::Timeout { .. } => "TIMEOUT_ERROR",
        }
    }

    /// 获取建议的解决方案
    pub fn suggestions(&self) -> Vec<String> {
        match self {
            SDKError::Network { .. } => vec![
                "检查网络连接".to_string(),
                "确认防火墙设置".to_string(),
                "稍后重试".to_string(),
            ],
            SDKError::API { code, help_suggestion, .. } => {
                let mut suggestions = vec![
                    "检查API参数格式".to_string(),
                    "确认权限范围".to_string(),
                ];
                if let Some(suggestion) = help_suggestion {
                    suggestions.insert(0, suggestion.clone());
                }
                suggestions
            },
            SDKError::Authentication { refresh_url, .. } => {
                if refresh_url.is_some() {
                    vec!["重新获取访问令牌".to_string()]
                } else {
                    vec!["检查应用凭证".to_string()]
                }
            },
            SDKError::RateLimited { .. } => vec![
                "降低请求频率".to_string(),
                "使用批量API".to_string(),
                "实现指数退避重试".to_string(),
            ],
            SDKError::Validation { field, value, constraint, .. } => {
                let mut suggestions = vec![format!("检查 {} 参数", field)];
                if let Some(constraint) = constraint {
                    suggestions.push(format!("约束条件: {}", constraint));
                }
                if let Some(value) = value {
                    suggestions.push(format!("当前值: {}", value));
                }
                suggestions
            },
            _ => vec![
                "查看详细错误信息".to_string(),
                "联系技术支持".to_string(),
            ],
        }
    }
}
```

## 重试机制

### 智能重试策略

```rust
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(1000),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

pub async fn retry_with_backoff<F, T, E>(
    config: &RetryConfig,
    operation: F,
) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
    E: std::fmt::Debug + Clone,
{
    let mut attempt = 0;
    let mut delay = config.base_delay;

    loop {
        attempt += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt >= config.max_attempts {
                    return Err(error);
                }

                // 计算延迟时间
                let jitter_amount = if config.jitter {
                    fastrand::u64(0..=delay.as_millis() as u64)
                } else {
                    0
                };

                let actual_delay = std::cmp::min(
                    Duration::from_millis(delay.as_millis() as u64 + jitter_amount),
                    config.max_delay,
                );

                tracing::warn!(
                    "Attempt {}/{} failed, retrying after {:?}: {:?}",
                    attempt,
                    config.max_attempts,
                    actual_delay,
                    error
                );

                sleep(actual_delay).await;
                delay = Duration::from_millis((delay.as_millis() as f64 * config.backoff_multiplier) as u64);
            }
        }
    }
}

// 使用示例
impl HttpClient {
    pub async fn execute_with_retry<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::Request,
    ) -> SDKResult<T> {
        let config = RetryConfig::default();

        retry_with_backoff(&config, || {
            let request_clone = request.try_clone().unwrap();
            async move {
                let response = self.client.execute(request_clone).await?;
                let result: T = response.json().await?;
                Ok(result)
            }
        }).await.map_err(|e| {
            if let Some(reqwest_error) = e.downcast_ref::<reqwest::Error>() {
                SDKError::Network {
                    source: reqwest_error.clone(),
                    url: request.url().map(|u| u.to_string()),
                    method: Some(request.method().to_string()),
                    retry_count: config.max_attempts,
                }
            } else {
                e
            }
        })
    }
}
```

## 错误监控和分析

### 错误收集器

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ErrorMetrics {
    pub total_errors: u64,
    pub errors_by_type: HashMap<String, u64>,
    pub errors_by_operation: HashMap<String, u64>,
    pub retry_rate: f64,
    pub avg_retry_count: f64,
}

pub struct ErrorCollector {
    metrics: Arc<RwLock<ErrorMetrics>>,
    recent_errors: Arc<Mutex<Vec<ErrorDiagnostic>>>,
    max_recent_errors: usize,
}

impl ErrorCollector {
    pub fn new(max_recent_errors: usize) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(ErrorMetrics {
                total_errors: 0,
                errors_by_type: HashMap::new(),
                errors_by_operation: HashMap::new(),
                retry_rate: 0.0,
                avg_retry_count: 0.0,
            })),
            recent_errors: Arc::new(Mutex::new(Vec::new())),
            max_recent_errors,
        }
    }

    pub async fn record_error(&self, error: &SDKError, context: &ErrorContext) {
        // 更新指标
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_errors += 1;

            let error_type = error.error_code().to_string();
            *metrics.errors_by_type.entry(error_type).or_insert(0) += 1;

            *metrics.errors_by_operation
                .entry(context.operation.clone())
                .or_insert(0) += 1;
        }

        // 创建诊断信息
        let diagnostic = ErrorDiagnostic {
            error: error.clone(),
            context: context.clone(),
            suggestions: error.suggestions(),
            related_docs: self.get_related_docs(error),
            support_links: self.get_support_links(error),
        };

        // 保存最近的错误
        {
            let mut recent_errors = self.recent_errors.lock().unwrap();
            recent_errors.push(diagnostic);

            // 保持最大数量限制
            if recent_errors.len() > self.max_recent_errors {
                recent_errors.remove(0);
            }
        }

        // 记录日志
        tracing::error!(
            error_code = error.error_code(),
            operation = context.operation,
            request_id = context.request_id,
            "Error occurred: {}", error
        );
    }

    pub async fn get_metrics(&self) -> ErrorMetrics {
        self.metrics.read().await.clone()
    }

    pub async fn get_recent_errors(&self, limit: Option<usize>) -> Vec<ErrorDiagnostic> {
        let recent_errors = self.recent_errors.lock().unwrap();
        let actual_limit = limit.unwrap_or(self.max_recent_errors);
        recent_errors.iter()
            .rev()
            .take(actual_limit)
            .cloned()
            .collect()
    }

    fn get_related_docs(&self, error: &SDKError) -> Vec<String> {
        match error {
            SDKError::Authentication { .. } => vec![
                "https://open.feishu.cn/document/ukTMukTMukTM/uAjLw4CM".to_string(),
            ],
            SDKError::Permission { .. } => vec![
                "https://open.feishu.cn/document/ukTMukTMukTM/uAjLw4CM".to_string(),
            ],
            SDKError::RateLimited { .. } => vec![
                "https://open.feishu.cn/document/ukTMukTMukTM/uAjLw4CM".to_string(),
            ],
            _ => vec![],
        }
    }

    fn get_support_links(&self, error: &SDKError) -> Vec<String> {
        vec![
            "https://github.com/open-lark/open-lark/issues".to_string(),
            "https://community.feishu.cn".to_string(),
        ]
    }
}
```

## 使用示例

### 集成到服务中

```rust
use openlark_core::Config;

pub struct HRService {
    config: Config,
    error_collector: Arc<ErrorCollector>,
}

impl HRService {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            error_collector: Arc::new(ErrorCollector::new(100)),
        }
    }

    pub async fn create_user(&self, request: UserCreateRequestV3) -> SDKResult<UserCreateResponseV3> {
        let context = ErrorContext {
            request_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            operation: "user_create".to_string(),
            user_id: None,
            app_id: Some(self.config.app_id.clone()),
            api_version: "v3".to_string(),
            request_metadata: HashMap::new(),
        };

        let result = self.execute_user_create(&request).await;

        if let Err(ref error) = result {
            self.error_collector.record_error(error, &context).await;
        }

        result
    }

    async fn execute_user_create(&self, request: &UserCreateRequestV3) -> SDKResult<UserCreateResponseV3> {
        // 参数验证
        validate_user_request(request)?;

        // API调用
        let url = format!("{}/contact/v3/users", self.config.api_base_url);
        let client = HttpClient::new(&self.config)?;

        let http_request = client
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.get_access_token()?))
            .json(request)
            .build()?;

        let response: UserCreateResponseV3 = client
            .execute_with_retry(http_request)
            .await
            .map_err(|e| {
                if let SDKError::API { code, .. } = e {
                    SDKError::API {
                        code,
                        message: e.to_string(),
                        request_id: None,
                        error_detail: None,
                        help_suggestion: Some("检查用户创建权限和参数格式".to_string()),
                        is_retryable: false,
                    }
                } else {
                    e
                }
            })?;

        Ok(response)
    }
}

fn validate_user_request(request: &UserCreateRequestV3) -> SDKResult<()> {
    validate_required!(name, &request.name, "用户姓名不能为空");

    if request.name.len() > 50 {
        return Err(SDKError::Validation {
            field: "name".to_string(),
            message: "用户姓名长度不能超过50个字符".to_string(),
            value: Some(request.name.clone()),
            constraint: Some("max_length=50".to_string()),
        });
    }

    validate_required!(department_id, &request.department_id, "部门ID不能为空");

    if let Some(email) = &request.email {
        if !email.contains('@') {
            return Err(SDKError::Validation {
                field: "email".to_string(),
                message: "邮箱格式不正确".to_string(),
                value: Some(email.clone()),
                constraint: Some("format=email".to_string()),
            });
        }
    }

    Ok(())
}

// 宏定义
#[macro_export]
macro_rules! validate_required {
    ($field:ident, $value:expr, $error_msg:expr) => {
        if $value.is_empty() {
            return Err(SDKError::Validation {
                field: stringify!($field).to_string(),
                message: $error_msg.to_string(),
                value: Some($value.to_string()),
                constraint: None,
            });
        }
    };
}
```

## 客户端错误处理

### 统一错误处理中间件

```rust
use tower::{Layer, Service};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ErrorHandlingLayer {
    collector: Arc<ErrorCollector>,
}

impl ErrorHandlingLayer {
    pub fn new(collector: Arc<ErrorCollector>) -> Self {
        Self { collector }
    }
}

impl<S> Layer<S> for ErrorHandlingLayer {
    type Service = ErrorHandlingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ErrorHandlingService {
            inner,
            collector: self.collector.clone(),
        }
    }
}

pub struct ErrorHandlingService<S> {
    inner: S,
    collector: Arc<ErrorCollector>,
}

impl<S> Service<Request> for ErrorHandlingService<S>
where
    S: Service<Request, Response = Response, Error = SDKError> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let collector = self.collector.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let context = ErrorContext {
                request_id: request.headers()
                    .get("x-request-id")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or(&uuid::Uuid::new_v4().to_string())
                    .to_string(),
                timestamp: chrono::Utc::now().timestamp(),
                operation: request.operation.clone(),
                user_id: request.user_id.clone(),
                app_id: request.app_id.clone(),
                api_version: request.api_version.clone(),
                request_metadata: request.metadata.clone(),
            };

            let result = inner.call(request).await;

            if let Err(ref error) = result {
                collector.record_error(error, &context).await;
            }

            result
        })
    }
}
```

这个增强的错误处理设计提供了：

1. **详细的错误分类和上下文**
2. **用户友好的错误消息**
3. **智能重试机制**
4. **错误监控和分析**
5. **性能优化建议**
6. **完整的调试支持**

通过这些改进，开发者可以更容易地诊断和解决问题，提升开发效率。