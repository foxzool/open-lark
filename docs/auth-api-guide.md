# OpenLark 认证 API 使用指南

> 本指南提供 OpenLark 认证服务的详细使用说明和最佳实践

## 概述

OpenLark 认证服务基于飞书开放平台 API，提供企业级的安全认证解决方案。本指南将帮助您快速集成和使用认证功能。

## 前置条件

### 环境要求

- Rust 1.75+
- Tokio 1.0+
- 飞书开放平台开发者账号

### 必要信息

- **App ID**: 应用标识符
- **App Secret**: 应用密钥
- **基础 URL**: `https://open.feishu.cn` (生产) 或 `https://open.larksuite.com` (测试)

## 快速开始

### 1. 项目设置

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
openlark-auth = { version = "0.1.0-dev", features = ["token-management", "oauth"] }
openlark-client = { version = "0.15.0-dev", features = ["auth"] }
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
```

### 2. 基础配置

```rust
use openlark_auth::{AuthServices, AuthConfig};

// 创建认证配置
let config = AuthConfig::new(
    env::var("LARK_APP_ID").expect("Missing LARK_APP_ID"),
    env::var("LARK_APP_SECRET").expect("Missing LARK_APP_SECRET"),
).with_base_url("https://open.feishu.cn");

// 创建认证服务
let auth = AuthServices::new(config);
```

### 3. 基础使用

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取自建应用租户访问令牌
    let tenant_token = auth.auth.v3().tenant_access_token()
        .internal()
        .send()
        .await?;

    println!("租户令牌: {}", tenant_token.tenant_access_token);

    // 使用令牌调用其他 API
    let user_info = get_user_info(&auth, &tenant_token.tenant_access_token).await?;

    Ok(())
}
```

## 企业应用认证

### 租户访问令牌

#### 自建应用租户令牌

```rust
async fn get_tenant_token_internal(
    auth: &AuthServices
) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
    let response = auth.auth.v3().tenant_access_token()
        .internal()
        .send()
        .await?;

    println!("租户令牌获取成功");
    println!("过期时间: {} 秒", response.expire);

    Ok(response)
}

// 使用示例
let tenant_token = get_tenant_token_internal(&auth).await?;
```

#### 商店应用租户令牌

```rust
async fn get_tenant_token_store(
    auth: &AuthServices,
    app_access_token: &str,
    tenant_key: &str,
) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
    let response = auth.auth.v3().tenant_access_token()
        .store()
        .app_access_token(app_access_token)
        .tenant_key(tenant_key)
        .send()
        .await?;

    println!("商店应用租户令牌获取成功");

    Ok(response)
}
```

### 应用访问令牌

```rust
async fn get_app_token_internal(
    auth: &AuthServices
) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
    let response = auth.auth.v3().app_access_token()
        .internal()
        .send()
        .await?;

    println!("应用令牌: {}", response.app_access_token);
    Ok(response)
}

async fn get_app_token_store(
    auth: &AuthServices
) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
    let response = auth.auth.v3().app_access_token()
        .store()
        .send()
        .await?;

    println!("商店应用令牌: {}", response.app_access_token);
    Ok(response)
}
```

### 应用票据管理

```rust
async fn resend_app_ticket(
    auth: &AuthServices
) -> Result<(), Box<dyn std::error::Error>> {
    let response = auth.auth.v3().app_ticket()
        .resend()
        .send()
        .await?;

    println!("应用票据重新推送成功");
    Ok(())
}
```

## 用户身份认证

### 用户信息获取

```rust
async fn get_user_info(
    auth: &AuthServices,
    access_token: &str,
) -> Result<openlark_auth::models::UserInfoResponse, Box<dyn std::error::Error>> {
    let response = auth.authen.v1().user_info()
        .get()
        .user_access_token(access_token)
        .user_id_type("open_id")  // 可选: "open_id", "user_id", "union_id"
        .send()
        .await?;

    println!("用户信息获取成功:");
    println!("  姓名: {}", response.name);
    println!("  邮箱: {}", response.email);
    println!("  状态: {:?}", response.status);

    Ok(response)
}
```

### 用户访问令牌

```rust
async fn get_user_access_token(
    auth: &AuthServices,
    code: &str,
) -> Result<openlark_auth::models::UserAccessTokenResponse, Box<dyn std::error::Error>> {
    let response = auth.authen.v1().access_token()
        .create()
        .grant_type("authorization_code")
        .code(code)
        .send()
        .await?;

    println!("用户访问令牌获取成功:");
    println!("  访问令牌: {}...", &response.access_token[..8]);
    println!("  过期时间: {} 秒", response.expires_in);

    Ok(response)
}

// 生成 OAuth 授权 URL
fn generate_oauth_url(
    auth: &AuthService,
    redirect_uri: &str,
    scope: &str,
    state: &str,
) -> String {
    auth.generate_oauth_url(redirect_uri, scope, state)
}
```

### OIDC 认证

```rust
async fn refresh_oidc_token(
    auth: &AuthServices,
    refresh_token: &str,
) -> Result<openlark_auth::models::OidcAccessTokenResponse, Box<dyn std::error::Error>> {
    let response = auth.authen.v1().oidc()
        .create_refresh_access_token()
        .refresh_token(refresh_token)
        .grant_type("refresh_token")
        .send()
        .await?;

    println!("OIDC 令牌刷新成功:");
    println!("  新访问令牌: {}...", &response.access_token[..8]);

    Ok(response)
}
```

## OAuth 授权

### 获取预授权码

```rust
async fn get_pre_auth_code(
    auth: &AuthServices,
    app_id: &str,
    redirect_uri: &str,
    scope: &str,
    state: &str,
) -> Result<impl std::fmt::Debug, Box<dyn std::error::Error>> {
    let response = auth.oauth.old.authorization()
        .get_index()
        .app_id(app_id)
        .redirect_uri(redirect_uri)
        .scope(scope)
        .state(state)
        .send()
        .await?;

    println!("预授权码获取成功");
    println!("  授权码: {}", response.code);

    Ok(response)
}
```

## 令牌管理

### 令牌信息结构

```rust
use openlark_client::services::auth::TokenInfo;

let token_info = TokenInfo {
    access_token: "your_access_token".to_string(),
    token_type: "Bearer".to_string(),
    expires_in: 7200,
    expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
    scope: Some("user:info docs:read".to_string()),
};
```

### 令牌状态检查

```rust
impl TokenInfo {
    /// 检查令牌是否已过期
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() >= self.expires_at
    }

    /// 检查令牌是否需要刷新（提前N分钟）
    pub fn needs_refresh(&self, buffer_minutes: i64) -> bool {
        let buffer = chrono::Duration::minutes(buffer_minutes);
        chrono::Utc::now() + buffer >= self.expires_at
    }

    /// 获取剩余有效时间（秒）
    pub fn remaining_seconds(&self) -> i64 {
        (self.expires_at - chrono::Utc::now()).num_seconds().max(0)
    }
}
```

### 令牌刷新示例

```rust
async fn auto_refresh_token(
    auth: &AuthServices,
    refresh_token: &str,
) -> Result<TokenInfo, Box<dyn std::error::Error>> {
    // 检查当前令牌状态
    let current_token = get_cached_token().await?;

    if current_token.is_expired() || current_token.needs_refresh(30) {
        // 刷新令牌
        let new_token = refresh_access_token(auth, refresh_token).await?;

        // 更新缓存
        update_token_cache(&new_token).await?;

        println!("令牌已自动刷新");
        return Ok(new_token);
    }

    Ok(current_token)
}
```

## 错误处理

### 常见错误类型

```rust
use openlark_auth::AuthError;

match api_result {
    Ok(response) => println!("API调用成功"),
    Err(AuthError::ConfigError(msg)) => {
        eprintln!("配置错误: {}", msg);
    }
    Err(AuthError::NetworkError(e)) => {
        eprintln!("网络错误: {}", e);
        // 实现重试逻辑
    }
    Err(AuthError::APIError { code, message }) => {
        eprintln!("API错误 [{}]: {}", code, message);

        // 根据错误代码进行特定处理
        match code.as_str() {
            "99991663" => {
                eprintln!("应用访问令牌过期，需要刷新");
            }
            "99991666" => {
                eprintln!("权限不足");
            }
            _ => {
                eprintln!("其他API错误: {}", message);
            }
        }
    }
}
```

### 错误重试策略

```rust
use tokio::time::{sleep, Duration};

async fn retry_with_backoff<F, Fut, T, E>(
    operation: F,
    max_attempts: u32,
    base_delay: Duration,
) -> Result<T, E>
where
    F: Fn() -> Fut + Clone,
    Fut: Future<Output = Result<T, E>>,
    T: Clone,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut attempts = 0;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(e);
                }

                let delay = base_delay * 2_u32.pow(attempts - 1);
                sleep(delay).await;

                eprintln!("尝试 {}/{} 失败，{}秒后重试: {}",
                    attempts, max_attempts, delay.as_secs(), e);
            }
        }
    }
}
```

## 最佳实践

### 1. 配置管理

#### 环境变量配置

```bash
# .env 文件
LARK_APP_ID=your_app_id
LARK_APP_SECRET=your_app_secret
LARK_BASE_URL=https://open.feishu.cn
```

```rust
// 从环境变量读取配置
use std::env;

let config = AuthConfig::new(
    env::var("LARK_APP_ID")?,
    env::var("LARK_APP_SECRET")?
).with_base_url(&env::var("LARK_BASE_URL").unwrap_or_else(|_| "https://open.feishu.cn".to_string());
```

#### 配置验证

```rust
fn validate_config(config: &AuthConfig) -> Result<(), AuthError> {
    if config.app_id.is_empty() {
        return Err(AuthError::ConfigError("App ID 不能为空"));
    }

    if config.app_secret.is_empty() {
        return Err(AuthError::ConfigError("App Secret 不能为空"));
    }

    if !config.base_url.starts_with("http") {
        return Err(AuthError::ConfigError("Base URL 必须是有效的 HTTP URL"));
    }

    Ok(())
}
```

### 2. 令牌缓存

#### 内存缓存实现

```rust
use std::collections::HashMap;
use tokio::sync::RwLock;

struct TokenManager {
    cache: RwLock<HashMap<String, TokenInfo>>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_token(&self, key: &str) -> Option<String> {
        self.cache.read().await.get(key)
            .and_then(|token| {
                if !token.is_expired() {
                    Some(token.access_token.clone())
                } else {
                    None
                }
            })
    }

    pub async fn set_token(&self, key: String, token: TokenInfo) {
        self.cache.write().await.insert(key, token);
    }

    pub async fn refresh_token_if_needed(
        &self,
        key: &str,
        auth: &AuthServices,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(token) = self.get_token(key).await {
            if !token.needs_refresh(30) {
                return Ok(token);
            }
        }

        // 这里需要实现实际的令牌刷新逻辑
        let new_token = refresh_token(auth).await?;
        self.set_token(key.to_string(), new_token.clone()).await;

        Ok(new_token.access_token)
    }
}
```

#### Redis 缓存实现

```toml
[dependencies]
redis = { version = "0.23", features = ["tokio-comp"] }
serde_json = "1.0"
```

```rust
use redis::Client as RedisClient;
use serde_json::{json, Value};

struct RedisTokenCache {
    client: RedisClient,
}

impl RedisTokenCache {
    pub fn new() -> Self {
        let client = redis::Client::open("redis://localhost").await
            .expect("Failed to connect to Redis");
        Self { client }
    }

    pub async fn get_token(&self, key: &str) -> Result<Option<TokenInfo>, redis::RedisError> {
        let json_str: String = self.client.get(key).await?;
        if json_str.is_empty() {
            return Ok(None);
        }

        let token_info: TokenInfo = serde_json::from_str(&json_str)?;

        Ok(if token_info.is_expired() { None } else { Some(token_info) })
    }

    pub async fn set_token(&self, key: &str, token: &TokenInfo) -> Result<(), redis::RedisError> {
        let json_str = serde_json::to_string(token)?;
        self.client.set_ex(key, &json_str, Some("EX", 7200)).await?;
        Ok(())
    }
}
```

### 3. 并发安全

#### 共享认证服务

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

struct SharedAuthService {
    auth: Arc<AuthServices>,
    token_cache: Arc<RwLock<HashMap<String, TokenInfo>>>,
    rate_limiter: Arc<RwLock<HashMap<String, u32>>,
}

impl SharedAuthService {
    pub async fn new(config: AuthConfig) -> Self {
        Self {
            auth: Arc::new(AuthServices::new(config)),
            token_cache: Arc::new(RwLock::new(HashMap::new())),
            rate_limiter: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_cached_token(
        &self,
        key: &str,
        auth: &AuthServices,
    ) -> Result<String, AuthError> {
        // 检查缓存
        if let Some(token) = self.token_cache.read().await.get(key) {
            if !token.is_expired() {
                return Ok(token.access_token);
            }
        }

        // 获取新令牌
        let token_info = refresh_token(auth).await?;

        // 更新缓存
        self.token_cache.write().await.insert(key.to_string(), token_info.clone());

        Ok(token_info.access_token)
    }
}
```

### 4. 监控和日志

#### 结构化日志

```rust
use tracing::{info, warn, error, debug};

async fn get_tenant_token_with_logging(
    auth: &AuthServices
) -> Result<TenantAccessTokenResponse, AuthError> {
    info!("开始获取租户访问令牌");

    let start_time = std::time::Instant::now();

    let result = auth.auth.v3().tenant_access_token()
        .internal()
        .send()
        .await;

    let duration = start_time.elapsed();

    match result {
        Ok(response) => {
            info!("租户令牌获取成功，耗时: {:?}", duration);
            info!("令牌过期时间: {} 秒", response.expire);
            Ok(response)
        }
        Err(e) => {
            error!("租户令牌获取失败: {}", e);
            Err(e)
        }
    }
}
```

#### 指标收集

```rust
use std::time::{Instant, Duration};

struct MetricsCollector {
    api_calls: Arc<std::sync::atomic::AtomicU64>,
    total_duration: Arc<std::sync::atomic::AtomicU64>,
    error_count: Arc<std::sync::atomic::AtomicU64>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            api_calls: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            total_duration: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            error_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    pub fn record_api_call(&self, duration: Duration, success: bool) {
        self.api_calls.fetch_add(1);
        self.total_duration.fetch_add(duration.as_millis() as u64);

        if !success {
            self.error_count.fetch_add(1);
        }
    }

    pub fn get_success_rate(&self) -> f64 {
        let total = self.api_calls.load(std::sync::atomic::Ordering::Relaxed);
        let errors = self.error_count.load(std::sync::atomic::Ordering::Relaxed);

        if total == 0 {
            return 1.0;
        }

        (total - errors) as f64 / total
    }
}
```

### 5. 安全最佳实践

#### 敏感信息保护

```rust
// 使用环境变量存储敏感信息
fn load_config() -> AuthConfig {
    AuthConfig::new(
        std::env::var("LARK_APP_ID").expect("Missing LARK_APP_ID"),
        std::env::var("LARK_APP_SECRET").expect("Missing LARK_APP_SECRET"),
    )
}

// 不在日志中记录敏感信息
fn log_token_usage(token: &str) {
    let masked_token = if token.len() > 8 {
        format!("{}****{}", &token[..4], &token[token.len()-4..])
    } else {
        token.to_string()
    };
    info!("使用令牌: {}", masked_token);
}
```

#### HTTPS 强制

```rust
use reqwest::Client;

let client = Client::builder()
    .danger_accept_invalid_certs(false)  // 仅在测试环境
    .use_rustls()
    .https_only(true)
    .timeout(Duration::from_secs(30))
    .build()?;
```

#### 输入验证

```rust
fn validate_authorization_code(code: &str) -> Result<String, String> {
    if code.is_empty() {
        return Err("授权码不能为空".to_string());
    }

    if code.len() > 100 {
        return Err("授权码过长".to_string());
    }

    // 检查授权码格式（通常为字母数字字符串）
    if !code.chars().all(|c| c.is_alphanumeric() || c == '-') || c == '_') {
        return Err("授权码包含无效字符".to_string());
    }

    Ok(code.to_string())
}
```

## 完整示例

### 企业应用集成示例

```rust
use openlark_auth::{AuthServices, AuthConfig};
use openlark_client::Config;
use tokio::time::{sleep, Duration};
use tracing::{info, error, warn};

pub struct LarkIntegration {
    auth: AuthServices,
    token_manager: TokenManager,
    metrics: MetricsCollector,
}

impl LarkIntegration {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // 加载配置
        let config = load_config()?;
        validate_config(&config)?;

        // 创建服务
        let auth = AuthServices::new(config);
        let token_manager = TokenManager::new();
        let metrics = MetricsCollector::new();

        Ok(LarkIntegration {
            auth,
            token_manager,
            metrics,
        })
    }

    /// 获取有效的租户令牌
    pub async fn get_valid_tenant_token(&self) -> Result<String, Box<dyn std::Error::Error>> {
        self.token_manager.get_tenant_token(&self.auth).await
            .map_err(Into::into)
    }

    /// 带用户信息的 API 调用
    pub async fn call_user_api<T, F, Fut>(
        &self,
        api_name: &str,
        operation: F,
    ) -> Result<T, Box<dyn std::Error::Error>>
    where
        F: Fn(&str) -> Fut + Clone,
        Fut: Future<Output = Result<T, Box<dyn std::error::Error>> + Clone,
        T: Clone,
    {
        let start = std::time::Instant::now();

        // 获取有效令牌
        let token = self.get_valid_tenant_token().await?;

        // 执行 API 调用
        let result = operation(&token).await;
        let duration = start.elapsed();

        // 记录指标
        self.metrics.record_api_call(duration, result.is_ok());

        if let Err(e) = &result {
            error!("API 调用失败 [{}]: {} - 耗时: {:?}", api_name, e, duration);
        }

        result
    }
}

// 用户状态管理
struct UserManager {
    auth: AuthServices,
    user_cache: HashMap<String, openlark_auth::models::UserInfoResponse>,
}

impl UserManager {
    pub async fn new(auth: AuthServices) -> Self {
        Self {
            auth,
            user_cache: HashMap::new(),
        }
    }

    pub async fn get_user_info(
        &mut self,
        user_id: &str,
    ) -> Result<&openlark_auth::models::UserInfoResponse, Box<dyn std::error::Error>> {
        // 检查缓存
        if let Some(user_info) = self.user_cache.get(user_id) {
            return Ok(user_info);
        }

        // 调用 API
        let token = self.auth.auth.v3().tenant_access_token()
            .internal()
            .send()
            .await?;

        let user_info = self.auth.authen.v1().user_info()
            .get()
            .user_access_token(&token.tenant_access_token)
            .user_id_type("user_id")
            .send()
            .await?;

        // 更新缓存
        self.user_cache.insert(user_id.to_string(), user_info);
        Ok(user_info)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let lark = LarkIntegration::new().await?;

    // 使用示例：获取用户列表
    let user_ids = vec!["user_001", "user_002", "user_003"];
    let user_infos = stream::iter(user_ids)
        .map(|id| lark.call_user_api("get_user_info", move |id| {
            // 模拟用户 API 调用
            async move {
                sleep(Duration::from_millis(100)).await;
                Ok(("user_003".to_string(), lark.get_valid_tenant_token().await?))
            }
        }))
        .collect::<Vec<Result<_, _>>()
        .await;

    for (i, result) in user_infos.iter().enumerate() {
        match result {
            Ok(user_id) => println!("用户 {}: {}", i + 1, user_id),
            Err(e) => println!("获取用户 {} 失败: {}", i + 1, e),
        }
    }

    Ok(())
}
```

### Web 应用集成示例

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use openlark_auth::{AuthServices, AuthConfig};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    auth: Arc<AuthServices>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let config = AuthConfig::new(
            std::env::var("LARK_APP_ID").unwrap_or_default(),
            std::env::var("LARK_APP_SECRET").unwrap_or_default(),
        );
        let auth = Arc::new(AuthServices::new(config));

        App::new()
            .state(auth)
            .service(
                web::resource("/api/user/{id}")
                .route(web::get().to(get_user_handler))
                .route(web::post().to(create_user_handler))
            )
            .bind(("127.0.0.1:8080"))
    })
    .await?;

    println!("服务器启动在 http://127.0.0.1:8080");

    Ok(())
}

async fn get_user_handler(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let user_id = path.into_inner();

    // 这里应该从令牌中获取用户信息
    // 简化示例，实际应用中需要验证令牌

    HttpResponse::Ok().json(json!({
        "user_id": user_id,
        "name": "用户姓名",
        "message": "用户信息获取成功"
    }))
}

async fn create_user_handler(
    state: web::Data<AppState>,
    form: web::Form<CreateUserRequest>,
) -> HttpResponse {
    // 验证请求权限
    // 实际应用中需要验证令牌和权限

    HttpResponse::Ok().json(json!({
        "message": "用户创建成功",
        "user_id": "new_user_001"
    }))
}

#[derive(serde::Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    // 其他字段...
}
```

### 移动应用集成示例

```rust
use openlark_client::{Config, LarkClient};
use openlark_auth::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 移动应用配置
    let config = Config::builder()
        .app_id("mobile_app_id")
        .app_secret("mobile_app_secret")
        .build()?;

    // 创建客户端
    let client = LarkClient::new(config)?;

    // 认证流程
    let oauth_url = client.auth().generate_oauth_url(
        "myapp://callback",
        "user:info",
        "random_state"
    );

    println!("请在浏览器中访问: {}", oauth_url);
    println!("复制授权码后继续...");

    // 用户输入授权码
    let auth_code = tokio::task::spawn_blocking(|| {
        println!("请输入授权码: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).map(|s| s.trim().to_string());
        input
    }).await?;

    // 获取访问令牌
    let access_token = client.auth()
        .get_user_access_token(&auth_code)
        .await?;

    println!("认证成功！");
    println!("访问令牌: {}...", &access_token[..8]);

    // 使用令牌获取用户信息
    let user_info = client.auth()
        .get_user_info(&access_token)
        .await?;

    println!("用户信息:");
    println!("  姓名: {}", user_info.name);
    println!("  邮箱: {}", user_info.email);

    Ok(())
}
```

## 故障排除

### 常见问题及解决方案

#### 1. 网络连接问题

**问题**: `NetworkError` 或连接超时

**解决方案**:
```rust
use tokio::time::timeout;

// 设置超时
let result = timeout(
    Duration::from_secs(30),
    auth.auth.v3().tenant_access_token().internal().send()
).await;
```

#### 2. 认证失败

**问题**: `APIError` 配码 `99991663`

**解决方案**:
```rust
// 令牌过期，需要刷新
if let AuthError::APIError { code, .. } = error {
    match code.as_str() {
        "99991663" => {
            let new_token = auth.auth.v3().tenant_access_token()
                .internal()
                .send()
                .await?;
            // 使用新令牌重试
        }
        "99991666" => {
            // 权限不足，检查应用权限配置
            return Err("应用权限不足".into());
        }
        _ => return Err(format!("API错误: {}", message));
    }
}
```

#### 3. 配置错误

**问题**: 配置参数无效或缺失

**解决方案**:
```rust
// 验证配置
fn validate_app_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    if config.app_id.is_empty() {
        return Err("应用 ID 不能为空".into());
    }

    if config.app_secret.is_empty() {
        return Err("应用密钥不能为空".into());
    }

    // 验证 URL 格式
    if !config.base_url.starts_with("http") {
        return Err("基础 URL 必须是有效的 HTTP URL".into());
    }

    Ok(())
}
```

#### 4. 编译错误

**问题**: `features` 相关的编译错误

**解决方案**:
```toml
# 确保启用正确的功能标志
cargo run --example auth_service_demo --features auth

# 或在 Cargo.toml 中配置
[features]
auth = ["openlark-auth"]
```

### 调试技巧

#### 启用详细日志

```rust
use tracing::Level;

// 设置日志级别
tracing::subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

// 记录请求详情
info!("发送 API 请求: {}", endpoint);
debug!("请求参数: {:?}", params);
debug!("响应状态: {}", status);
```

#### 使用 Mock 服务测试

```rust
use wiremock::MockServer;

#[tokio::test]
async fn test_token_refresh() {
    let mock_server = MockServer::start().await;

    // Mock 响应
    Mock::given(method("POST"))
        .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "tenant_access_token": "mock_token",
            "expire": 7200
        })))
        .mount(&mock_server)
        .await;

    let config = AuthConfig::new("test", "test")
        .with_base_url(&mock_server.uri());

    let auth = AuthServices::new(config);
    let result = auth.auth.v3().tenant_access_token()
        .internal()
        .send()
        .await;

    assert!(result.is_ok());
}
```

## 性能优化

### 1. 连接池配置

```rust
use reqwest::Client;

let client = Client::builder()
    .pool_max_idle_per_host(10)
    .pool_idle_timeout(Duration::from_secs(30))
    .timeout(Duration::from_secs(30))
    .build()?;
```

### 2. 并发请求

```rust
use tokio::task::JoinSet;
use std::sync::Arc;

let auth = Arc::new(AuthServices::new(config));
let mut join_set = JoinSet::new();

// 并发执行多个 API 调用
for i in 0..10 {
    let auth_clone = auth.clone();
    join_set.spawn(async move {
        auth_clone.auth.v3().tenant_access_token().internal().send().await
    });
}

// 等待所有请求完成
let results: Vec<Result<_, _>> = join_set.join_all().await;
let success_count = results.iter().filter(|r| r.is_ok()).count();
println!("成功率: {}/{}", success_count, results.len());
```

### 3. 缓存策略

```rust
// 设置合适的过期时间
const CACHE_TTL: Duration = Duration::from_secs(3600); // 1小时

// LRU 缓存大小
const MAX_CACHE_SIZE: usize = 1000;

// 异步缓存刷新
async fn refresh_cache_background() {
    let refresh_interval = Duration::from_secs(1800); // 30分钟
    loop {
        sleep(refresh_interval).await;
        // 刷新缓存逻辑
    }
}
```

## 总结

本指南涵盖了 OpenLark 认证服务的主要使用场景和最佳实践。通过遵循这些指南，您可以：

1. **快速集成**: 快速将认证功能集成到您的应用中
2. **安全实践**: 确保认证流程的安全性
3. **性能优化**: 提高 API 调用的性能
4. **错误处理**: 优雅地处理各种异常情况

如需更多帮助，请参考：

- [API 参考文档](https://docs.rs/openlark-auth)
- [项目 GitHub](https://github.com/foxzool/open-lark)
- [飞书开放平台文档](https://open.feishu.cn/document/uAjT4ub/)
- [技术支持](https://github.com/foxzool/open-lark/issues)