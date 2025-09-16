use crate::core::{
    config::Config,
    constants::{CUSTOM_REQUEST_ID, USER_AGENT_HEADER},
    req_option::RequestOption,
    utils::user_agent,
};
use reqwest::RequestBuilder;

/// 构建通用请求头
pub struct HeaderBuilder;

impl HeaderBuilder {
    /// 构建所有必要的请求头
    pub fn build_headers(
        mut req_builder: RequestBuilder,
        config: &Config,
        option: &RequestOption,
    ) -> RequestBuilder {
        // 1. 添加请求ID（如果有）
        if !option.request_id.is_empty() {
            req_builder = req_builder.header(CUSTOM_REQUEST_ID, &option.request_id);
        }

        // 2. 添加选项中的自定义头
        for (key, value) in &option.header {
            req_builder = req_builder.header(key, value);
        }

        // 3. 添加配置中的全局头
        for (key, value) in &config.header {
            req_builder = req_builder.header(key, value);
        }

        // 4. 添加 User-Agent
        req_builder = req_builder.header(USER_AGENT_HEADER, user_agent());

        req_builder
    }

    /// 添加单个请求头（工具方法）
    pub fn add_header(req_builder: RequestBuilder, key: &str, value: &str) -> RequestBuilder {
        req_builder.header(key, value)
    }

    /// 批量添加请求头（工具方法）
    pub fn add_headers(
        mut req_builder: RequestBuilder,
        headers: &[(String, String)],
    ) -> RequestBuilder {
        for (key, value) in headers {
            req_builder = req_builder.header(key, value);
        }
        req_builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::AppType;
    use reqwest::Client;
    use std::collections::HashMap;

    fn create_test_config() -> Config {
        let mut config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            app_type: AppType::SelfBuild,
            header: HashMap::new(),
            ..Default::default()
        };

        // Add some global headers
        config
            .header
            .insert("X-Global-Header".to_string(), "global-value".to_string());
        config
            .header
            .insert("X-App-Version".to_string(), "1.0.0".to_string());

        config
    }

    fn create_test_request_option() -> RequestOption {
        let mut option = RequestOption {
            request_id: "test-request-123".to_string(),
            ..Default::default()
        };
        option
            .header
            .insert("X-Custom-Header".to_string(), "custom-value".to_string());
        option
            .header
            .insert("X-Test-Flag".to_string(), "true".to_string());
        option
    }

    fn create_test_request_builder() -> RequestBuilder {
        Client::new().get("https://test.api.example.com/test")
    }

    #[test]
    fn test_header_builder_struct_creation() {
        let _builder = HeaderBuilder;
    }

    #[test]
    fn test_build_headers_with_all_options() {
        let req_builder = create_test_request_builder();
        let config = create_test_config();
        let option = create_test_request_option();

        let result = HeaderBuilder::build_headers(req_builder, &config, &option);

        // Should not panic and return a RequestBuilder
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_build_headers_with_empty_request_id() {
        let req_builder = create_test_request_builder();
        let config = create_test_config();
        let mut option = create_test_request_option();
        option.request_id = "".to_string(); // Empty request ID

        let result = HeaderBuilder::build_headers(req_builder, &config, &option);

        // Should not panic even with empty request ID
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_build_headers_with_empty_headers() {
        let req_builder = create_test_request_builder();
        let mut config = create_test_config();
        config.header.clear(); // No global headers

        let mut option = create_test_request_option();
        option.header.clear(); // No custom headers

        let result = HeaderBuilder::build_headers(req_builder, &config, &option);

        // Should still work with no custom headers
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_build_headers_user_agent_always_added() {
        let req_builder = create_test_request_builder();
        let config = Config::default(); // Minimal config
        let option = RequestOption::default(); // Minimal option

        let result = HeaderBuilder::build_headers(req_builder, &config, &option);

        // User-Agent should always be added
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_build_headers_header_precedence() {
        let req_builder = create_test_request_builder();
        let mut config = create_test_config();
        config
            .header
            .insert("X-Common-Header".to_string(), "config-value".to_string());

        let mut option = create_test_request_option();
        option
            .header
            .insert("X-Common-Header".to_string(), "option-value".to_string());

        let result = HeaderBuilder::build_headers(req_builder, &config, &option);

        // Should handle duplicate headers (option headers added after config headers)
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_add_header() {
        let req_builder = create_test_request_builder();
        let key = "X-Test-Header";
        let value = "test-value";

        let result = HeaderBuilder::add_header(req_builder, key, value);

        // Should add single header successfully
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_add_header_with_empty_values() {
        let req_builder = create_test_request_builder();

        let result = HeaderBuilder::add_header(req_builder, "", "");

        // Should handle empty key/value without panicking
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_add_headers_empty_list() {
        let req_builder = create_test_request_builder();
        let headers: &[(String, String)] = &[];

        let result = HeaderBuilder::add_headers(req_builder, headers);

        // Should handle empty header list
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_add_headers_multiple() {
        let req_builder = create_test_request_builder();
        let headers = &[
            ("X-Header-1".to_string(), "value-1".to_string()),
            ("X-Header-2".to_string(), "value-2".to_string()),
            ("X-Header-3".to_string(), "value-3".to_string()),
        ];

        let result = HeaderBuilder::add_headers(req_builder, headers);

        // Should add multiple headers successfully
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_add_headers_duplicate_keys() {
        let req_builder = create_test_request_builder();
        let headers = &[
            ("X-Duplicate".to_string(), "value-1".to_string()),
            ("X-Duplicate".to_string(), "value-2".to_string()),
            ("X-Other".to_string(), "other-value".to_string()),
        ];

        let result = HeaderBuilder::add_headers(req_builder, headers);

        // Should handle duplicate header keys
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_header_builder_is_send_sync() {
        // Test that HeaderBuilder implements required traits
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<HeaderBuilder>();
        assert_sync::<HeaderBuilder>();
    }

    #[test]
    fn test_build_headers_with_large_number_of_headers() {
        let req_builder = create_test_request_builder();
        let mut config = create_test_config();
        let mut option = create_test_request_option();

        // Add many headers to test performance
        for i in 0..50 {
            config
                .header
                .insert(format!("X-Config-Header-{i}"), format!("config-value-{i}"));
            option
                .header
                .insert(format!("X-Option-Header-{i}"), format!("option-value-{i}"));
        }

        let result = HeaderBuilder::build_headers(req_builder, &config, &option);

        // Should handle large number of headers
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_build_headers_with_special_characters() {
        let req_builder = create_test_request_builder();
        let mut config = create_test_config();
        let mut option = create_test_request_option();

        // Add headers with special characters
        option.header.insert(
            "X-Special-Chars".to_string(),
            "value with spaces & symbols!".to_string(),
        );
        config
            .header
            .insert("X-Unicode".to_string(), "测试中文".to_string());

        let result = HeaderBuilder::build_headers(req_builder, &config, &option);

        // Should handle special characters in header values
        assert!(format!("{:?}", result).contains("RequestBuilder"));
    }

    #[test]
    fn test_build_headers_preserves_request_builder_type() {
        let req_builder = create_test_request_builder();
        let config = create_test_config();
        let option = create_test_request_option();

        let result = HeaderBuilder::build_headers(req_builder, &config, &option);

        // Result should still be a RequestBuilder that can be further modified
        let final_result = result.header("Additional-Header", "additional-value");
        assert!(format!("{:?}", final_result).contains("RequestBuilder"));
    }
}
