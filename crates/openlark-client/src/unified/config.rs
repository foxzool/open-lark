//! OpenLark 统一配置管理系统
//!
//! 提供类型安全的配置管理，支持环境变量、配置文件和程序化配置。

use std::collections::HashMap;
use std::time::Duration;

use openlark_core::{SDKResult, config::Config as CoreConfig};

use super::{error::{UnifiedError, UnifiedResult}, traits::FromEnvConfig};

/// OpenLark 统一配置
///
/// 包含客户端运行所需的所有配置信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConfig {
    /// 核心配置
    pub core: Arc<CoreConfig>,
    /// 服务配置
    pub services: ServiceConfigs,
    /// 功能标志
    pub features: FeatureFlags,
    /// 性能配置
    pub performance: PerformanceConfig,
    /// 安全配置
    pub security: SecurityConfig,
    /// 监控配置
    pub monitoring: MonitoringConfig,
    /// 自定义元数据
    pub metadata: HashMap<String, String>,
}

/// 服务配置集合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfigs {
    /// 通信服务配置
    pub communication: Option<CommunicationConfig>,
    /// HR服务配置
    pub hr: Option<HRConfig>,
    /// 文档服务配置
    pub docs: Option<DocsConfig>,
    /// AI服务配置
    pub ai: Option<AIConfig>,
    /// 认证服务配置
    pub auth: Option<AuthConfig>,
    /// 其他自定义服务配置
    pub custom: HashMap<String, serde_json::Value>,
}

/// 通信服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationConfig {
    /// 是否启用
    pub enabled: bool,
    /// API端点URL
    pub api_url: String,
    /// 默认超时时间
    pub timeout: Duration,
    /// 重试配置
    pub retry: RetryConfig,
    /// 消息配置
    pub message: MessageConfig,
}

/// HR服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HRConfig {
    /// 是否启用
    pub enabled: bool,
    /// API端点URL
    pub api_url: String,
    /// 默认超时时间
    pub timeout: Duration,
    /// 数据访问配置
    pub data_access: DataAccessConfig,
}

/// 文档服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsConfig {
    /// 是否启用
    pub enabled: bool,
    /// API端点URL
    pub api_url: String,
    /// 文件上传配置
    pub upload: UploadConfig,
    /// 缓存配置
    pub cache: CacheConfig,
}

/// AI服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// 是否启用
    pub enabled: bool,
    /// API端点URL
    pub api_url: String,
    /// 模型配置
    pub models: ModelConfig,
    /// 速率限制
    pub rate_limit: RateLimitConfig,
}

/// 认证服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// 是否启用
    pub enabled: bool,
    /// 令牌配置
    pub token: TokenConfig,
    /// OAuth配置
    pub oauth: Option<OAuthConfig>,
}

/// 消息配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageConfig {
    /// 最大消息长度
    pub max_length: usize,
    /// 支持的消息类型
    pub supported_types: Vec<String>,
    /// 默认消息格式
    pub default_format: String,
}

/// 数据访问配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessConfig {
    /// 批量操作大小
    pub batch_size: usize,
    /// 数据格式
    pub data_format: String,
    /// 字段映射
    pub field_mapping: HashMap<String, String>,
}

/// 文件上传配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadConfig {
    /// 最大文件大小（字节）
    pub max_file_size: u64,
    /// 支持的文件类型
    pub allowed_types: Vec<String>,
    /// 分片上传配置
    pub chunk_upload: ChunkUploadConfig,
}

/// 分片上传配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkUploadConfig {
    /// 是否启用
    pub enabled: bool,
    /// 分片大小（字节）
    pub chunk_size: usize,
    /// 并发分片数
    pub concurrent_chunks: usize,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 是否启用
    pub enabled: bool,
    /// TTL（秒）
    pub ttl: u64,
    /// 最大缓存条目数
    pub max_entries: usize,
}

/// 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// 默认模型
    pub default_model: String,
    /// 可用模型列表
    pub available_models: Vec<String>,
    /// 模型参数
    pub parameters: HashMap<String, serde_json::Value>,
}

/// 速率限制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// 每分钟请求限制
    pub requests_per_minute: u32,
    /// 每小时请求限制
    pub requests_per_hour: u32,
    /// 每日请求限制
    pub requests_per_day: u32,
}

/// 令牌配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    /// 访问令牌TTL（秒）
    pub access_token_ttl: u64,
    /// 刷新令牌TTL（秒）
    pub refresh_token_ttl: u64,
    /// 缓存配置
    pub cache: CacheConfig,
}

/// OAuth配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    /// 授权端点
    pub auth_url: String,
    /// 令牌端点
    pub token_url: String,
    /// 回调URL
    pub redirect_url: String,
    /// 作用域
    pub scopes: Vec<String>,
}

/// 重试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_attempts: u32,
    /// 基础退避时间（毫秒）
    pub base_delay: u64,
    /// 最大退避时间（毫秒）
    pub max_delay: u64,
    /// 退避倍数
    pub backoff_multiplier: f64,
    /// 抖动因子
    pub jitter_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: 1000,
            max_delay: 30000,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
        }
    }
}

/// 功能标志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// 启用的功能列表
    pub enabled: Vec<String>,
    /// 禁用的功能列表
    pub disabled: Vec<String>,
    /// 功能配置映射
    pub config: HashMap<String, serde_json::Value>,
}

/// 性能配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// 连接池配置
    pub connection_pool: ConnectionPoolConfig,
    /// 并发控制配置
    pub concurrency: ConcurrencyConfig,
    /// 缓存配置
    pub cache: CacheConfig,
}

/// 连接池配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    /// 最大连接数
    pub max_connections: usize,
    /// 最小连接数
    pub min_connections: usize,
    /// 连接TTL（秒）
    pub connection_ttl: u64,
    /// 空闲超时（秒）
    pub idle_timeout: u64,
}

/// 并发控制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// 最大并发请求数
    pub max_concurrent_requests: usize,
    /// 请求队列大小
    pub request_queue_size: usize,
    /// 超时配置
    pub timeout: Duration,
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// TLS配置
    pub tls: TLSConfig,
    /// 加密配置
    pub encryption: EncryptionConfig,
}

/// TLS配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSConfig {
    /// 是否验证证书
    pub verify_certificates: bool,
    /// CA证书路径
    pub ca_cert_path: Option<String>,
    /// 客户端证书路径
    pub client_cert_path: Option<String>,
    /// 客户端私钥路径
    pub client_key_path: Option<String>,
}

/// 加密配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// 加密算法
    pub algorithm: String,
    /// 密钥长度
    pub key_length: usize,
    /// 密钥派生配置
    pub key_derivation: KeyDerivationConfig,
}

/// 密钥派生配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationConfig {
    /// 算法
    pub algorithm: String,
    /// 迭代次数
    pub iterations: u32,
    /// 盐值长度
    pub salt_length: usize,
}

/// 监控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// 指标配置
    pub metrics: MetricsConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 追踪配置
    pub tracing: TracingConfig,
}

/// 指标配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// 是否启用
    pub enabled: bool,
    /// 收集间隔（秒）
    pub collect_interval: u64,
    /// 导出器配置
    pub exporter: ExporterConfig,
}

/// 导出器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExporterConfig {
    /// 导出器类型
    pub r#type: String, // "prometheus", "statsd", "opentelemetry"
    /// 端点URL
    pub endpoint: Option<String>,
    /// 标签
    pub tags: HashMap<String, String>,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// 日志级别
    pub level: String,
    /// 日志格式
    pub format: String,
    /// 输出目标
    pub output: String,
    /// 文件路径
    pub file_path: Option<String>,
}

/// 追踪配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// 是否启用
    pub enabled: bool,
    /// 采样率
    pub sampling_rate: f64,
    /// 导出器配置
    pub exporter: ExporterConfig,
}

impl UnifiedConfig {
    /// 从核心配置创建统一配置
    pub fn from_core(core: CoreConfig) -> Self {
        Self {
            core,
            services: ServiceConfigs::default(),
            features: FeatureFlags::default(),
            performance: PerformanceConfig::default(),
            security: SecurityConfig::default(),
            monitoring: MonitoringConfig::default(),
            metadata: HashMap::new(),
        }
    }

    /// 创建默认配置
    pub fn default() -> Self {
        Self::from_core(CoreConfig::default())
    }

    /// 设置应用信息
    pub fn with_app_info(mut self, app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        let core_config = CoreConfigBuilder::new()
            .app_id(app_id)
            .app_secret(app_secret)
            .build()
            .unwrap_or_else(|_| CoreConfig::default());

        self.core = Arc::new(core_config);
        self
    }

    /// 设置基础URL
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        let core_config = CoreConfigBuilder::new()
            .base_url(base_url)
            .build()
            .unwrap_or_else(|_| CoreConfig::default());

        self.core = Arc::new(core_config);
        self
    }

    /// 启用功能
    pub fn enable_feature(mut self, feature: impl Into<String>) -> Self {
        let feature = feature.into();
        self.features.disabled.retain(|f| f != &feature);
        if !self.features.enabled.contains(&feature) {
            self.features.enabled.push(feature);
        }
        self
    }

    /// 禁用功能
    pub fn disable_feature(mut self, feature: impl Into<String>) -> Self {
        let feature = feature.into();
        self.features.enabled.retain(|f| f != &feature);
        if !self.features.disabled.contains(&feature) {
            self.features.disabled.push(feature);
        }
        self
    }

    /// 检查功能是否启用
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.features.enabled.contains(&feature) && !self.features.disabled.contains(&feature)
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// 验证配置
    pub fn validate(&self) -> UnifiedResult<()> {
        // 验证基础配置
        if self.core.app_id.is_empty() {
            return Err(UnifiedError::ConfigurationError("app_id不能为空".to_string()));
        }

        if self.core.app_secret.is_empty() {
            return Err(UnifiedError::ConfigurationError("app_secret不能为空".to_string()));
        }

        if self.core.base_url.is_empty() {
            return Err(UnifiedError::ConfigurationError("base_url不能为空".to_string()));
        }

        Ok(())
    }
}

impl Default for ServiceConfigs {
    fn default() -> Self {
        Self {
            communication: Some(CommunicationConfig::default()),
            hr: Some(HRConfig::default()),
            docs: Some(DocsConfig::default()),
            ai: Some(AIConfig::default()),
            auth: Some(AuthConfig::default()),
            custom: HashMap::new(),
        }
    }
}

impl Default for CommunicationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_url: "https://open.feishu.cn".to_string(),
            timeout: Duration::from_secs(30),
            retry: RetryConfig::default(),
            message: MessageConfig::default(),
        }
    }
}

impl Default for MessageConfig {
    fn default() -> Self {
        Self {
            max_length: 4096,
            supported_types: vec!["text".to_string(), "image".to_string(), "file".to_string()],
            default_format: "text".to_string(),
        }
    }
}

impl Default for HRConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_url: "https://open.feishu.cn".to_string(),
            timeout: Duration::from_secs(60),
            data_access: DataAccessConfig::default(),
        }
    }
}

impl Default for DataAccessConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            data_format: "json".to_string(),
            field_mapping: HashMap::new(),
        }
    }
}

impl Default for DocsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_url: "https://open.feishu.cn".to_string(),
            upload: UploadConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

impl Default for UploadConfig {
    fn default() -> Self {
        Self {
            max_file_size: 100 * 1024 * 1024, // 100MB
            allowed_types: vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "application/pdf".to_string(),
                "text/plain".to_string(),
            ],
            chunk_upload: ChunkUploadConfig::default(),
        }
    }
}

impl Default for ChunkUploadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            chunk_size: 1024 * 1024, // 1MB
            concurrent_chunks: 3,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ttl: 3600, // 1小时
            max_entries: 1000,
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_url: "https://open.feishu.cn".to_string(),
            models: ModelConfig::default(),
            rate_limit: RateLimitConfig::default(),
        }
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            default_model: "gpt-3.5-turbo".to_string(),
            available_models: vec!["gpt-3.5-turbo".to_string(), "gpt-4".to_string()],
            parameters: HashMap::new(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            token: TokenConfig::default(),
            oauth: None,
        }
    }
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            access_token_ttl: 7200, // 2小时
            refresh_token_ttl: 604800, // 7天
            cache: CacheConfig::default(),
        }
    }
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            enabled: vec![
                "communication".to_string(),
                "auth".to_string(),
            ],
            disabled: vec![],
            config: HashMap::new(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            connection_pool: ConnectionPoolConfig::default(),
            concurrency: ConcurrencyConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            min_connections: 5,
            connection_ttl: 300, // 5分钟
            idle_timeout: 60,     // 1分钟
        }
    }
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 50,
            request_queue_size: 1000,
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            tls: TLSConfig::default(),
            encryption: EncryptionConfig::default(),
        }
    }
}

impl Default for TLSConfig {
    fn default() -> Self {
        Self {
            verify_certificates: true,
            ca_cert_path: None,
            client_cert_path: None,
            client_key_path: None,
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: "AES-256-GCM".to_string(),
            key_length: 256,
            key_derivation: KeyDerivationConfig::default(),
        }
    }
}

impl Default for KeyDerivationConfig {
    fn default() -> Self {
        Self {
            algorithm: "PBKDF2".to_string(),
            iterations: 100000,
            salt_length: 32,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics: MetricsConfig::default(),
            logging: LoggingConfig::default(),
            tracing: TracingConfig::default(),
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collect_interval: 60,
            exporter: ExporterConfig::default(),
        }
    }
}

impl Default for ExporterConfig {
    fn default() -> Self {
        Self {
            r#type: "prometheus".to_string(),
            endpoint: None,
            tags: HashMap::new(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            output: "stdout".to_string(),
            file_path: None,
        }
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sampling_rate: 0.1,
            exporter: ExporterConfig::default(),
        }
    }
}

/// 配置构建器
pub struct ConfigBuilder {
    config: UnifiedConfig,
}

impl ConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self {
            config: UnifiedConfig::default(),
        }
    }

    /// 设置应用信息
    pub fn app_info(mut self, app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        self.config = self.config.with_app_info(app_id, app_secret);
        self
    }

    /// 设置基础URL
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.config = self.config.with_base_url(base_url);
        self
    }

    /// 启用服务
    pub fn enable_service(mut self, service: &str) -> Self {
        self.config = self.config.enable_feature(service);
        self
    }

    /// 禁用服务
    pub fn disable_service(mut self, service: &str) -> Self {
        self.config = self.config.disable_feature(service);
        self
    }

    /// 设置超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.core.timeout = timeout;
        self
    }

    /// 设置连接池配置
    pub fn connection_pool(mut self, config: ConnectionPoolConfig) -> Self {
        self.config.performance.connection_pool = config;
        self
    }

    /// 构建配置
    pub fn build(self) -> UnifiedResult<UnifiedConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FromEnvConfig for UnifiedConfig {
    fn from_env() -> UnifiedResult<Self> {
        use std::env;

        let mut builder = ConfigBuilder::new();

        // 从环境变量读取基础配置
        if let Ok(app_id) = env::var("OPENLARK_APP_ID") {
            builder = builder.with_app_info(app_id, env::var("OPENLARK_APP_SECRET").unwrap_or_default());
        }

        if let Ok(base_url) = env::var("OPENLARK_BASE_URL") {
            builder = builder.with_base_url(base_url);
        }

        let mut config = builder.build()?;

        // 从环境变量读取功能标志
        if let Ok(features) = env::var("OPENLARK_FEATURES") {
            config.features.enabled = features.split(',').map(|s| s.trim().to_string()).collect();
        }

        if let Ok(disabled_features) = env::var("OPENLARK_DISABLED_FEATURES") {
            config.features.disabled = disabled_features
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .app_info("test_app_id", "test_app_secret")
            .base_url("https://test.feishu.cn")
            .enable_service("communication")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        assert_eq!(config.core.app_id, "test_app_id");
        assert_eq!(config.core.app_secret, "test_app_secret");
        assert_eq!(config.core.base_url, "https://test.feishu.cn");
        assert_eq!(config.core.timeout, Duration::from_secs(60));
        assert!(config.is_feature_enabled("communication"));
    }

    #[test]
    fn test_feature_flags() {
        let mut config = UnifiedConfig::default();

        config = config.enable_feature("test_feature");
        assert!(config.is_feature_enabled("test_feature"));

        config = config.disable_feature("test_feature");
        assert!(!config.is_feature_enabled("test_feature"));
    }

    #[test]
    fn test_config_validation() {
        let mut config = UnifiedConfig::default();
        config.core.app_id = "".to_string();

        assert!(config.validate().is_err());
    }
}