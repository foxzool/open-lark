//! OpenLark Client 配置管理
//!
//! 提供灵活的配置系统，支持环境变量、验证和默认值

use crate::Result;
use std::time::Duration;

/// 🔧 OpenLark客户端配置
///
/// 支持从环境变量自动加载配置
///
/// # 环境变量
/// - `OPENLARK_APP_ID`: 应用ID（必需）
/// - `OPENLARK_APP_SECRET`: 应用密钥（必需）
/// - `OPENLARK_BASE_URL`: API基础URL（可选，默认：https://open.feishu.cn）
///
/// # 示例
/// ```rust,no_run
/// use openlark_client::Config;
///
/// // 从环境变量创建配置
/// let config = Config::from_env();
///
/// // 手动构建配置
/// let config = Config::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .base_url("https://open.feishu.cn")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// 🆔 飞书应用ID
    pub app_id: String,
    /// 🔑 飞书应用密钥
    pub app_secret: String,
    /// 🌐 API基础URL
    pub base_url: String,
    /// ⏱️ 请求超时时间
    pub timeout: Duration,
    /// 🔄 默认重试次数
    pub retry_count: u32,
    /// 📝 是否启用日志记录
    pub enable_log: bool,
    /// 📋 自定义HTTP headers
    pub headers: std::collections::HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
        }
    }
}

impl Config {
    /// 🆕 创建新的配置实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 🌍 从环境变量创建配置
    ///
    /// # 环境变量
    /// - `OPENLARK_APP_ID`: 应用ID（必需）
    /// - `OPENLARK_APP_SECRET`: 应用密钥（必需）
    /// - `OPENLARK_BASE_URL`: API基础URL（可选）
    ///
    /// # 返回值
    /// 返回配置实例，环境变量缺失时使用默认值
    pub fn from_env() -> Self {
        let mut config = Self::default();
        config.load_from_env();
        config
    }

    /// 📥 从环境变量加载配置到当前实例
    ///
    /// 只设置存在且非空的环境变量
    pub fn load_from_env(&mut self) {
        if let Ok(app_id) = std::env::var("OPENLARK_APP_ID") {
            if !app_id.is_empty() {
                self.app_id = app_id;
            }
        }

        if let Ok(app_secret) = std::env::var("OPENLARK_APP_SECRET") {
            if !app_secret.is_empty() {
                self.app_secret = app_secret;
            }
        }

        if let Ok(base_url) = std::env::var("OPENLARK_BASE_URL") {
            if !base_url.is_empty() {
                self.base_url = base_url;
            }
        }

        // 可选的环境变量
        if let Ok(timeout_str) = std::env::var("OPENLARK_TIMEOUT") {
            if let Ok(timeout_secs) = timeout_str.parse::<u64>() {
                self.timeout = Duration::from_secs(timeout_secs);
            }
        }

        if let Ok(retry_str) = std::env::var("OPENLARK_RETRY_COUNT") {
            if let Ok(retry_count) = retry_str.parse::<u32>() {
                self.retry_count = retry_count;
            }
        }

        // 日志开关（默认启用，只有设置为"false"时才禁用）
        if let Ok(log_setting) = std::env::var("OPENLARK_ENABLE_LOG") {
            self.enable_log = !log_setting.to_lowercase().starts_with('f');
        }
    }

    /// ✅ 验证配置的有效性
    ///
    /// # 验证规则
    /// - app_id和app_secret不能为空
    /// - base_url必须是有效的HTTP/HTTPS URL
    /// - timeout必须大于0
    /// - retry_count不能超过合理的范围
    ///
    /// # 错误
    /// 返回验证失败的详细错误信息
    pub fn validate(&self) -> Result<()> {
        if self.app_id.is_empty() {
            return Err(crate::error::validation_error("app_id", "app_id不能为空"));
        }

        if self.app_secret.is_empty() {
            return Err(crate::error::validation_error(
                "app_secret",
                "app_secret不能为空",
            ));
        }

        if self.base_url.is_empty() {
            return Err(crate::error::validation_error(
                "base_url",
                "base_url不能为空",
            ));
        }

        // 验证URL格式
        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err(crate::error::validation_error(
                "base_url",
                "base_url必须以http://或https://开头",
            ));
        }

        // 验证超时时间
        if self.timeout.is_zero() {
            return Err(crate::error::validation_error(
                "timeout",
                "timeout必须大于0",
            ));
        }

        // 验证重试次数
        if self.retry_count > 10 {
            return Err(crate::error::validation_error(
                "retry_count",
                "retry_count不能超过10",
            ));
        }

        Ok(())
    }

    /// 🏗️ 创建配置构建器
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// 🔧 添加自定义HTTP header
    pub fn add_header<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
    }

    /// 🧹 清除所有自定义headers
    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    /// 🔍 检查配置是否完整（可用于API调用）
    pub fn is_complete(&self) -> bool {
        !self.app_id.is_empty() && !self.app_secret.is_empty()
    }

    /// 📋 获取配置摘要（不包含敏感信息）
    pub fn summary(&self) -> ConfigSummary {
        ConfigSummary {
            app_id: self.app_id.clone(),
            app_secret_set: !self.app_secret.is_empty(),
            base_url: self.base_url.clone(),
            timeout: self.timeout,
            retry_count: self.retry_count,
            enable_log: self.enable_log,
            header_count: self.headers.len(),
        }
    }

    /// 🔄 更新配置，只更新非空字段
    pub fn update_with(&mut self, other: &Config) {
        if !other.app_id.is_empty() {
            self.app_id = other.app_id.clone();
        }
        if !other.app_secret.is_empty() {
            self.app_secret = other.app_secret.clone();
        }
        if !other.base_url.is_empty() {
            self.base_url = other.base_url.clone();
        }
        if !other.timeout.is_zero() {
            self.timeout = other.timeout;
        }
        if other.retry_count != 3 {
            self.retry_count = other.retry_count;
        }
        if other.enable_log != self.enable_log {
            self.enable_log = other.enable_log;
        }
        // 合并headers
        for (key, value) in &other.headers {
            self.headers.insert(key.clone(), value.clone());
        }
    }
}

/// 🏗️ 配置构建器 - 提供流畅的API
///
/// # 示例
/// ```rust,no_run
/// use openlark_client::Config;
/// use std::time::Duration;
///
/// let config = Config::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .base_url("https://open.feishu.cn")
///     .timeout(Duration::from_secs(60))
///     .retry_count(5)
///     .enable_log(true)
///     .add_header("X-Custom-Header", "value")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// 🆕 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// 🆔 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.config.app_id = app_id.into();
        self
    }

    /// 🔑 设置应用密钥
    pub fn app_secret<S: Into<String>>(mut self, app_secret: S) -> Self {
        self.config.app_secret = app_secret.into();
        self
    }

    /// 🌐 设置API基础URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.config.base_url = base_url.into();
        self
    }

    /// ⏱️ 设置请求超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// 🔄 设置重试次数
    pub fn retry_count(mut self, count: u32) -> Self {
        self.config.retry_count = count;
        self
    }

    /// 📝 设置是否启用日志
    pub fn enable_log(mut self, enable: bool) -> Self {
        self.config.enable_log = enable;
        self
    }

    /// 🔧 添加自定义HTTP header
    pub fn add_header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.config.add_header(key, value);
        self
    }

    /// 🌍 从环境变量加载配置
    pub fn from_env(mut self) -> Self {
        self.config.load_from_env();
        self
    }

    /// 🔨 构建最终的配置实例
    ///
    /// # 错误
    /// 如果配置验证失败，返回相应的错误
    pub fn build(self) -> Result<Config> {
        self.config.validate()?;
        Ok(self.config)
    }

    /// 🔨 构建配置实例（跳过验证）
    ///
    /// 注意：使用此方法时配置可能无效，后续使用时需要验证
    pub fn build_unvalidated(self) -> Config {
        self.config
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 📋 配置摘要（不包含敏感信息）
#[derive(Debug, Clone)]
pub struct ConfigSummary {
    /// 🆔 应用ID
    pub app_id: String,
    /// 🔑 应用密钥是否已设置
    pub app_secret_set: bool,
    /// 🌐 API基础URL
    pub base_url: String,
    /// ⏱️ 请求超时时间
    pub timeout: Duration,
    /// 🔄 重试次数
    pub retry_count: u32,
    /// 📝 是否启用日志
    pub enable_log: bool,
    /// 📋 自定义headers数量
    pub header_count: usize,
}

impl std::fmt::Display for ConfigSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Config {{ app_id: {}, app_secret_set: {}, base_url: {}, timeout: {:?}, retry_count: {}, enable_log: {}, header_count: {} }}",
            self.app_id,
            self.app_secret_set,
            self.base_url,
            self.timeout,
            self.retry_count,
            self.enable_log,
            self.header_count
        )
    }
}

/// 从环境变量创建配置的便利方法
impl From<std::env::Vars> for Config {
    fn from(env_vars: std::env::Vars) -> Self {
        let mut config = Config::default();

        // 解析环境变量
        let env_map: std::collections::HashMap<String, String> = env_vars.collect();

        if let Some(app_id) = env_map.get("OPENLARK_APP_ID") {
            config.app_id = app_id.clone();
        }

        if let Some(app_secret) = env_map.get("OPENLARK_APP_SECRET") {
            config.app_secret = app_secret.clone();
        }

        if let Some(base_url) = env_map.get("OPENLARK_BASE_URL") {
            config.base_url = base_url.clone();
        }

        if let Some(timeout_str) = env_map.get("OPENLARK_TIMEOUT") {
            if let Ok(timeout_secs) = timeout_str.parse::<u64>() {
                config.timeout = Duration::from_secs(timeout_secs);
            }
        }

        if let Some(retry_str) = env_map.get("OPENLARK_RETRY_COUNT") {
            if let Ok(retry_count) = retry_str.parse::<u32>() {
                config.retry_count = retry_count;
            }
        }

        if let Some(log_setting) = env_map.get("OPENLARK_ENABLE_LOG") {
            config.enable_log = !log_setting.to_lowercase().starts_with('f');
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.app_id, "");
        assert_eq!(config.app_secret, "");
        assert_eq!(config.base_url, "https://open.feishu.cn");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.retry_count, 3);
        assert!(config.enable_log);
        assert!(config.headers.is_empty());
    }

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.feishu.cn")
            .timeout(Duration::from_secs(60))
            .retry_count(5)
            .enable_log(false)
            .build();

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.app_id, "test_app_id");
        assert_eq!(config.app_secret, "test_app_secret");
        assert_eq!(config.base_url, "https://test.feishu.cn");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.retry_count, 5);
        assert!(!config.enable_log);
    }

    #[test]
    fn test_config_from_env() {
        // 设置环境变量
        std::env::set_var("OPENLARK_APP_ID", "test_app_id");
        std::env::set_var("OPENLARK_APP_SECRET", "test_app_secret");
        std::env::set_var("OPENLARK_BASE_URL", "https://test.feishu.cn");
        std::env::set_var("OPENLARK_TIMEOUT", "60");
        std::env::set_var("OPENLARK_RETRY_COUNT", "5");
        std::env::set_var("OPENLARK_ENABLE_LOG", "false");

        let config = Config::from_env();
        assert_eq!(config.app_id, "test_app_id");
        assert_eq!(config.app_secret, "test_app_secret");
        assert_eq!(config.base_url, "https://test.feishu.cn");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.retry_count, 5);
        assert!(!config.enable_log);

        // 清理环境变量
        std::env::remove_var("OPENLARK_APP_ID");
        std::env::remove_var("OPENLARK_APP_SECRET");
        std::env::remove_var("OPENLARK_BASE_URL");
        std::env::remove_var("OPENLARK_TIMEOUT");
        std::env::remove_var("OPENLARK_RETRY_COUNT");
        std::env::remove_var("OPENLARK_ENABLE_LOG");
    }

    #[test]
    fn test_config_validation() {
        // 有效配置
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
        };
        assert!(config.validate().is_ok());

        // 无效配置 - 空app_id
        let invalid_config = Config {
            app_id: String::new(),
            ..config.clone()
        };
        assert!(invalid_config.validate().is_err());

        // 无效配置 - 空app_secret
        let invalid_config = Config {
            app_secret: String::new(),
            ..config
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_config_headers() {
        let mut config = Config::default();

        // 添加header
        config.add_header("X-Custom-Header", "custom-value");
        assert_eq!(config.headers.len(), 1);
        assert_eq!(
            config.headers.get("X-Custom-Header"),
            Some(&"custom-value".to_string())
        );

        // 清除headers
        config.clear_headers();
        assert!(config.headers.is_empty());
    }

    #[test]
    fn test_config_update_with() {
        let mut base_config = Config::default();
        let update_config = Config {
            app_id: "updated_app_id".to_string(),
            app_secret: "updated_app_secret".to_string(),
            timeout: Duration::from_secs(60),
            ..Default::default()
        };

        base_config.update_with(&update_config);
        assert_eq!(base_config.app_id, "updated_app_id");
        assert_eq!(base_config.app_secret, "updated_app_secret");
        assert_eq!(base_config.timeout, Duration::from_secs(60));
        // 其他字段保持默认值
        assert_eq!(base_config.base_url, "https://open.feishu.cn");
    }

    #[test]
    fn test_config_summary() {
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
        };

        let summary = config.summary();
        assert_eq!(summary.app_id, "test_app_id");
        assert!(summary.app_secret_set);
        assert_eq!(summary.base_url, "https://open.feishu.cn");
        assert_eq!(summary.timeout, Duration::from_secs(30));
        assert_eq!(summary.retry_count, 3);
        assert!(summary.enable_log);
        assert_eq!(summary.header_count, 0);
    }

    #[test]
    fn test_config_is_complete() {
        let mut config = Config::default();
        assert!(!config.is_complete());

        config.app_id = "test_app_id".to_string();
        assert!(!config.is_complete());

        config.app_secret = "test_app_secret".to_string();
        assert!(config.is_complete());
    }
}
