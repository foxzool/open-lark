//! 统一测试夹具系统
//!
//! 提供测试配置的快速构建，避免在每个测试中重复设置。

use crate::config::Config;

/// 测试配置构建器
///
/// 提供合理的默认值，快速创建测试用的配置。
///
/// # 示例
///
/// ```rust,ignore
/// let config = TestConfigBuilder::new()
///     .app_id("test_app")
///     .app_secret("test_secret")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct TestConfigBuilder {
    app_id: String,
    app_secret: String,
    base_url: String,
}

impl TestConfigBuilder {
    /// 创建新的测试配置构建器（使用默认值）
    pub fn new() -> Self {
        Self {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
        }
    }

    /// 设置应用 ID
    pub fn app_id(mut self, id: impl Into<String>) -> Self {
        self.app_id = id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, secret: impl Into<String>) -> Self {
        self.app_secret = secret.into();
        self
    }

    /// 设置基础 URL
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// 构建配置实例
    pub fn build(self) -> Config {
        Config::builder()
            .app_id(self.app_id)
            .app_secret(self.app_secret)
            .base_url(self.base_url)
            .build()
    }
}

impl Default for TestConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 快捷方法：创建默认测试配置
///
/// # 示例
///
/// ```rust,ignore
/// let config = test_config();
/// ```
pub fn test_config() -> Config {
    TestConfigBuilder::new().build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_config_builder() {
        let config = TestConfigBuilder::new()
            .app_id("my_app")
            .app_secret("my_secret")
            .build();

        assert_eq!(config.app_id(), "my_app");
        assert_eq!(config.app_secret(), "my_secret");
    }

    #[test]
    fn test_test_config_default() {
        let config = test_config();
        assert_eq!(config.app_id(), "test_app_id");
        assert_eq!(config.app_secret(), "test_app_secret");
    }
}
