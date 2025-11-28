//! 测试配置管理
//!
//! 提供测试环境的配置管理和测试参数设置

use std::time::Duration;

/// 测试配置结构体
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// 测试用的 app_token
    pub app_token: String,
    /// 测试用的 table_id
    pub table_id: String,
    /// 测试用的 form_id
    pub form_id: String,
    /// 测试用的 view_id
    pub view_id: String,
    /// 测试用的 role_id
    pub role_id: String,
    /// 测试用的 user_id
    pub user_id: String,
    /// HTTP 请求超时时间
    pub timeout: Duration,
    /// Mock服务器端口
    pub mock_port: u16,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            app_token: "test_app_token_123456789".to_string(),
            table_id: "test_table_id_123456789".to_string(),
            form_id: "test_form_id_123456789".to_string(),
            view_id: "test_view_id_123456789".to_string(),
            role_id: "test_role_id_123456789".to_string(),
            user_id: "test_user_id_123456789".to_string(),
            timeout: Duration::from_secs(30),
            mock_port: 0, // 0 表示随机端口
        }
    }
}

impl TestConfig {
    /// 创建新的测试配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置自定义 app_token
    pub fn with_app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置自定义 table_id
    pub fn with_table_id(mut self, table_id: impl Into<String>) -> Self {
        self.table_id = table_id.into();
        self
    }

    /// 设置自定义 form_id
    pub fn with_form_id(mut self, form_id: impl Into<String>) -> Self {
        self.form_id = form_id.into();
        self
    }

    /// 设置自定义 view_id
    pub fn with_view_id(mut self, view_id: impl Into<String>) -> Self {
        self.view_id = view_id.into();
        self
    }

    /// 设置自定义 role_id
    pub fn with_role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 设置自定义 user_id
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 设置 Mock 服务器端口
    pub fn with_mock_port(mut self, port: u16) -> Self {
        self.mock_port = port;
        self
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> String {
        format!("http://localhost:{}", self.mock_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_config_default() {
        let config = TestConfig::default();
        assert_eq!(config.app_token, "test_app_token_123456789");
        assert_eq!(config.table_id, "test_table_id_123456789");
        assert_eq!(config.form_id, "test_form_id_123456789");
        assert_eq!(config.view_id, "test_view_id_123456789");
        assert_eq!(config.role_id, "test_role_id_123456789");
        assert_eq!(config.user_id, "test_user_id_123456789");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.mock_port, 0);
    }

    #[test]
    fn test_test_config_builder() {
        let config = TestConfig::new()
            .with_app_token("custom_app_token")
            .with_table_id("custom_table_id")
            .with_timeout(Duration::from_secs(60));

        assert_eq!(config.app_token, "custom_app_token");
        assert_eq!(config.table_id, "custom_table_id");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.form_id, "test_form_id_123456789"); // 默认值保持不变
    }

    #[test]
    fn test_base_url() {
        let config = TestConfig::new().with_mock_port(8080);
        assert_eq!(config.base_url(), "http://localhost:8080");
    }
}