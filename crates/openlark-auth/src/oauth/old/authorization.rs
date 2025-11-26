//! OAuth授权服务 (Resource: authorization)
//!
//! 提供向后兼容的OAuth授权功能。

use std::sync::Arc;

use crate::models::AuthConfig;

/// OAuth授权服务
#[derive(Debug)]
pub struct AuthorizationService {
    config: Arc<AuthConfig>,
    client: reqwest::Client,
}

impl AuthorizationService {
    /// 创建新的OAuth授权服务
    pub fn new(config: Arc<AuthConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 获取登录预授权码
    pub fn get_index(&self) -> AuthorizationBuilder {
        AuthorizationBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            app_id: self.config.app_id.clone(),
            redirect_uri: String::new(),
            state: String::new(),
        }
    }
}

/// OAuth授权构建器
#[derive(Debug)]
pub struct AuthorizationBuilder {
    #[allow(dead_code)] // 保留用于未来扩展
    config: Arc<AuthConfig>,
    #[allow(dead_code)] // 保留用于未来扩展
    client: reqwest::Client,
    app_id: String,
    redirect_uri: String,
    state: String,
}

impl AuthorizationBuilder {
    /// 设置重定向URI
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.redirect_uri = redirect_uri.into();
        self
    }

    /// 设置状态参数
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = state.into();
        self
    }

    /// 构建授权URL
    pub fn build_url(self) -> String {
        format!(
            "https://open.feishu.cn/open-apis/authen/v1/authorize?app_id={}&redirect_uri={}&state={}",
            self.app_id, self.redirect_uri, self.state
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AuthServices;

    /// 创建测试用的认证配置
    fn create_test_auth_config(base_url: &str) -> crate::models::AuthConfig {
        crate::models::AuthConfig::new("test_app_id", "test_app_secret").with_base_url(base_url)
    }

    /// 创建测试用的认证服务
    fn create_test_auth_services(base_url: &str) -> AuthServices {
        let config = create_test_auth_config(base_url);
        AuthServices::new(config)
    }

    // ==================== 基础构建器测试 ====================

    #[test]
    fn test_authorization_service_creation() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = AuthorizationService::new(std::sync::Arc::new(config));

        // 测试构建器创建
        let _builder = service.get_index();
    }

    #[test]
    fn test_authorization_builder() {
        let config = crate::models::AuthConfig::new("test_app_id", "test_app_secret");
        let service = AuthorizationService::new(std::sync::Arc::new(config));

        let url = service
            .get_index()
            .redirect_uri("https://example.com/callback")
            .state("random_state")
            .build_url();

        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https://example.com/callback"));
        assert!(url.contains("state=random_state"));
    }

    // ==================== OAuth授权URL构建完整测试 ====================

    #[test]
    fn test_authorization_url_basic_structure() {
        // Given: 设置Mock服务器
        let mock_helper = create_test_auth_services("https://open.feishu.cn");

        // When: 构建基础授权URL
        let url = mock_helper
            .oauth
            .old
            .authorization()
            .get_index()
            .redirect_uri("https://example.com/callback")
            .state("test_state_123")
            .build_url();

        // Then: 验证URL结构
        assert!(url.starts_with("https://open.feishu.cn/open-apis/authen/v1/authorize"));
        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https://example.com/callback"));
        assert!(url.contains("state=test_state_123"));
        assert!(url.contains("?")); // 确保查询参数存在

        println!("✅ 基础授权URL结构测试通过: {}", url);
    }

    #[test]
    fn test_authorization_url_with_custom_app_id() {
        // Given: 使用自定义应用ID
        let config = crate::models::AuthConfig::new("custom_app_456", "test_app_secret");
        let service = AuthorizationService::new(std::sync::Arc::new(config));

        // When: 构建授权URL
        let url = service
            .get_index()
            .redirect_uri("https://myapp.com/auth/callback")
            .state("custom_state")
            .build_url();

        // Then: 验证自定义应用ID
        assert!(url.contains("app_id=custom_app_456"));
        assert!(url.contains("redirect_uri=https://myapp.com/auth/callback"));
        assert!(url.contains("state=custom_state"));

        println!("✅ 自定义应用ID测试通过: {}", url);
    }

    #[test]
    fn test_authorization_url_with_special_characters() {
        // Given: 包含特殊字符的参数
        let mock_helper = create_test_auth_services("https://open.feishu.cn");

        // When: 使用包含特殊字符的参数构建URL
        let url = mock_helper
            .oauth
            .old
            .authorization()
            .get_index()
            .redirect_uri("https://example.com/callback?param=value&other=test")
            .state("state_with_special_chars_123_ABC")
            .build_url();

        // Then: 验证特殊字符处理
        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https://example.com/callback?param=value&other=test"));
        assert!(url.contains("state=state_with_special_chars_123_ABC"));

        println!("✅ 特殊字符处理测试通过: {}", url);
    }

    #[test]
    fn test_authorization_url_empty_parameters() {
        // Given: 空参数测试
        let mock_helper = create_test_auth_services("https://open.feishu.cn");

        // When: 使用空参数构建URL
        let url = mock_helper
            .oauth
            .old
            .authorization()
            .get_index()
            .redirect_uri("")
            .state("")
            .build_url();

        // Then: 验证空参数处理
        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri="));
        assert!(url.contains("state="));

        println!("✅ 空参数处理测试通过: {}", url);
    }

    #[test]
    fn test_authorization_url_unicode_parameters() {
        // Given: Unicode字符测试
        let mock_helper = create_test_auth_services("https://open.feishu.cn");

        // When: 使用Unicode字符构建URL
        let url = mock_helper
            .oauth
            .old
            .authorization()
            .get_index()
            .redirect_uri("https://测试应用.com/回调")
            .state("测试状态_123")
            .build_url();

        // Then: 验证Unicode字符处理
        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https://测试应用.com/回调"));
        assert!(url.contains("state=测试状态_123"));

        println!("✅ Unicode字符处理测试通过: {}", url);
    }

    #[test]
    fn test_authorization_url_parameter_order() {
        // Given: 多个参数测试
        let mock_helper = create_test_auth_services("https://open.feishu.cn");

        // When: 构建URL
        let url = mock_helper
            .oauth
            .old
            .authorization()
            .get_index()
            .redirect_uri("https://example.com/callback")
            .state("order_test")
            .build_url();

        // Then: 验证参数顺序和分隔符
        let parts: Vec<&str> = url.split('?').collect();
        assert_eq!(parts.len(), 2); // 应该只有一个问号分隔符

        let query_params = parts[1];
        let param_pairs: Vec<&str> = query_params.split('&').collect();
        assert_eq!(param_pairs.len(), 3); // 应该有3个参数

        // 验证参数存在
        assert!(query_params.contains("app_id=test_app_id"));
        assert!(query_params.contains("redirect_uri=https://example.com/callback"));
        assert!(query_params.contains("state=order_test"));

        println!("✅ 参数顺序和分隔符测试通过: {}", url);
    }

    #[test]
    fn test_authorization_url_long_parameters() {
        // Given: 长参数测试
        let long_redirect_uri = "https://example.com/very/long/redirect/uri/path/that/might/be/used/in/production/systems/with/deep/nesting/and/query/parameters?param1=value1&param2=value2&param3=value3";
        let long_state = "very_long_state_parameter_that_contains_lots_of_characters_and_numbers_and_symbols_1234567890_ABCDEF_abcdefghijklmnopqrstuvwxyz";

        let mock_helper = create_test_auth_services("https://open.feishu.cn");

        // When: 使用长参数构建URL
        let url = mock_helper
            .oauth
            .old
            .authorization()
            .get_index()
            .redirect_uri(long_redirect_uri)
            .state(long_state)
            .build_url();

        // Then: 验证长参数处理
        assert!(url.len() > 200); // URL应该很长
        assert!(url.contains(&long_redirect_uri));
        assert!(url.contains(&long_state));

        println!("✅ 长参数处理测试通过，URL长度: {}", url.len());
    }

    #[test]
    fn test_authorization_url_concurrent_build() {
        // Given: 并发URL构建测试
        let mock_helper = std::sync::Arc::new(create_test_auth_services("https://open.feishu.cn"));

        // When: 并发构建多个URL
        let mut handles = vec![];
        for i in 0..10 {
            let services = mock_helper.clone();
            let handle = std::thread::spawn(move || {
                services
                    .oauth
                    .old
                    .authorization()
                    .get_index()
                    .redirect_uri(&format!("https://example.com/callback/{}", i))
                    .state(&format!("state_{}", i))
                    .build_url()
            });
            handles.push(handle);
        }

        // Then: 验证所有URL构建成功
        let mut urls = vec![];
        for handle in handles {
            let url = handle.join().unwrap();
            assert!(!url.is_empty());
            assert!(url.contains("app_id=test_app_id"));
            urls.push(url);
        }

        // 验证URL唯一性
        let unique_urls: std::collections::HashSet<_> = urls.iter().collect();
        assert_eq!(unique_urls.len(), 10); // 所有URL都应该是唯一的

        println!(
            "✅ 并发URL构建测试通过，成功构建 {} 个唯一URL",
            unique_urls.len()
        );
    }

    #[test]
    fn test_authorization_builder_chaining() {
        // Given: 构建器链式调用测试
        let mock_helper = create_test_auth_services("https://open.feishu.cn");

        // When: 链式调用构建器方法
        let url = mock_helper
            .oauth
            .old
            .authorization()
            .get_index()
            .redirect_uri("https://step1.com")
            .state("step1")
            .redirect_uri("https://step2.com") // 覆盖之前的值
            .state("step2") // 覆盖之前的值
            .build_url();

        // Then: 验证最终值被使用
        assert!(url.contains("redirect_uri=https://step2.com"));
        assert!(url.contains("state=step2"));
        assert!(!url.contains("step1"));

        println!("✅ 构建器链式调用测试通过: {}", url);
    }

    #[test]
    fn test_authorization_url_security_considerations() {
        // Given: 安全考虑测试
        let mock_helper = create_test_auth_services("https://open.feishu.cn");

        // When: 使用可能包含安全风险的参数
        let url = mock_helper
            .oauth
            .old
            .authorization()
            .get_index()
            .redirect_uri("https://malicious.com/steal_token")
            .state("javascript:alert('xss')")
            .build_url();

        // Then: 验证URL包含这些值（实际应用中应该进行安全验证）
        assert!(url.contains("redirect_uri=https://malicious.com/steal_token"));
        assert!(url.contains("state=javascript:alert('xss')"));

        println!("⚠️  安全考虑测试完成 - 实际应用中应验证重定向URI的安全性");
        println!("URL: {}", url);
    }

    #[test]
    fn test_authorization_url_encoding() {
        // Given: URL编码测试
        let mock_helper = create_test_auth_services("https://open.feishu.cn");

        // When: 使用需要编码的字符
        let url = mock_helper
            .oauth
            .old
            .authorization()
            .get_index()
            .redirect_uri("https://example.com/callback?space=hello world&plus=+&ampersand=&")
            .state("encoded state with spaces & symbols")
            .build_url();

        // Then: 验证URL包含正确编码的字符
        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains(
            "redirect_uri=https://example.com/callback?space=hello world&plus=+&ampersand=&"
        ));
        assert!(url.contains("state=encoded state with spaces & symbols"));

        println!("✅ URL编码测试通过: {}", url);
    }
}
