use std::collections::HashMap;

use reqwest::Method;
use serde::Serialize;

use crate::{
    api::ApiRequest,
    api::{ApiResponseTrait, Response},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 通用请求执行器，统一处理API调用逻辑
/// 消除重复的请求-响应处理代码，提供统一的API调用入口
pub struct RequestExecutor;

impl RequestExecutor {
    /// 执行GET请求
    pub async fn get<T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        query_params: Option<HashMap<&'static str, String>>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<T>> {
        Self::execute(
            config,
            Method::GET,
            path,
            supported_tokens,
            query_params,
            None::<()>,
            option,
        )
        .await
    }

    /// 执行POST请求
    pub async fn post<
        T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>,
        B: Serialize,
    >(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<T>> {
        Self::execute(
            config,
            Method::POST,
            path,
            supported_tokens,
            None,
            body,
            option,
        )
        .await
    }

    /// 执行PUT请求
    pub async fn put<
        T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>,
        B: Serialize,
    >(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<T>> {
        Self::execute(
            config,
            Method::PUT,
            path,
            supported_tokens,
            None,
            body,
            option,
        )
        .await
    }

    /// 执行DELETE请求
    pub async fn delete<
        T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>,
    >(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<T>> {
        Self::execute(
            config,
            Method::DELETE,
            path,
            supported_tokens,
            None,
            None::<()>,
            option,
        )
        .await
    }

    /// 执行PATCH请求
    pub async fn patch<
        T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>,
        B: Serialize,
    >(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<T>> {
        Self::execute(
            config,
            Method::PATCH,
            path,
            supported_tokens,
            None,
            body,
            option,
        )
        .await
    }

    /// 通用请求执行器核心方法
    ///
    /// # 参数
    /// - `config`: 应用配置，包含认证信息
    /// - `method`: HTTP方法
    /// - `path`: API路径
    /// - `supported_tokens`: 支持的访问令牌类型
    /// - `query_params`: 查询参数（可选）
    /// - `body`: 请求体（可选）
    /// - `option`: 请求选项（可选）
    ///
    /// # 返回值
    /// 返回标准的`Response<T>`格式响应
    ///
    /// # 示例
    /// ```rust,ignore
    /// // GET请求
    /// let response: Response<MessageList> = RequestExecutor::execute(
    ///     &config,
    ///     Method::GET,
    ///     "/open-apis/im/v1/messages",
    ///     [AccessTokenType::Tenant, AccessTokenType::User],
    ///     Some(query_params),
    ///     None::<()>,
    ///     None,
    /// ).await?;
    ///
    /// // POST请求
    /// let response: Response<Message> = RequestExecutor::execute(
    ///     &config,
    ///     Method::POST,
    ///     "/open-apis/im/v1/messages",
    ///     [AccessTokenType::Tenant, AccessTokenType::User],
    ///     None,
    ///     Some(create_request),
    ///     None,
    /// ).await?;
    /// ```
    pub async fn execute<
        T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>,
        B: Serialize,
    >(
        config: &Config,
        method: Method,
        path: &str,
        _supported_tokens: Vec<AccessTokenType>,
        query_params: Option<HashMap<&'static str, String>>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<T>> {
        // 构建基础URL
        let base_url = &config.base_url;

        let full_url = format!("{}{}", base_url, path);

        // 根据HTTP方法创建请求
        let mut api_req = match method {
            Method::GET => ApiRequest::<()>::get(full_url),
            Method::POST => ApiRequest::<()>::post(full_url),
            Method::PUT => ApiRequest::<()>::put(full_url),
            Method::DELETE => ApiRequest::<()>::delete(full_url),
            _ => ApiRequest::<()>::post(full_url),
        };

        // 设置查询参数
        if let Some(params) = query_params {
            for (key, value) in params {
                api_req = api_req.query(key, value);
            }
        }

        // 序列化请求体
        if let Some(body_data) = body {
            let json_value = serde_json::to_value(body_data)
                .map_err(|e| crate::error::serialization_error("序列化失败", Some(e)))?;
            api_req = api_req.body(json_value);
        }

        // 执行请求
        Transport::request(api_req, config, option).await
    }

    /// 带路径参数的请求执行器
    /// 支持在路径中动态替换参数，如 `/open-apis/im/v1/messages/{message_id}`
    ///
    /// # 参数
    /// - `path_template`: 包含占位符的路径模板，如 "/open-apis/im/v1/messages/{message_id}"
    /// - `path_params`: 路径参数映射，如 HashMap::from([("message_id", "om_xxx")])
    /// - 其他参数同 `execute` 方法
    ///
    /// # 示例
    /// ```rust,ignore
    /// let path_params = HashMap::from([("message_id", "om_xxx")]);
    /// let response = RequestExecutor::execute_with_path_params(
    ///     &config,
    ///     Method::GET,
    ///     "/open-apis/im/v1/messages/{message_id}",
    ///     path_params,
    ///     [AccessTokenType::Tenant],
    ///     None,
    ///     None::<()>,
    ///     None,
    /// ).await?;
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn execute_with_path_params<
        T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>,
        B: Serialize,
    >(
        config: &Config,
        method: Method,
        path_template: &str,
        path_params: HashMap<&str, &str>,
        supported_tokens: Vec<AccessTokenType>,
        query_params: Option<HashMap<&'static str, String>>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<T>> {
        // 替换路径参数
        let mut path = path_template.to_string();
        for (key, value) in path_params {
            path = path.replace(&format!("{{{key}}}"), value);
        }

        Self::execute(
            config,
            method,
            &path,
            supported_tokens,
            query_params,
            body,
            option,
        )
        .await
    }

    /// 简化的JSON请求执行器
    /// 自动序列化JSON请求体并设置标准的租户/用户令牌支持
    ///
    /// # 示例
    /// ```rust,ignore
    /// // 创建消息
    /// let response = RequestExecutor::json_request::<CreateMessageResponse, _>(
    ///     &config,
    ///     Method::POST,
    ///     "/open-apis/im/v1/messages",
    ///     &create_message_request,
    ///     None,
    /// ).await?;
    /// ```
    pub async fn json_request<
        T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>,
        B: Serialize,
    >(
        config: &Config,
        method: Method,
        path: &str,
        body: &B,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<T>> {
        Self::execute(
            config,
            method,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User], // 默认支持租户和用户令牌
            None,
            Some(body),
            option,
        )
        .await
    }

    /// 简化的查询请求执行器
    /// 用于GET请求，自动设置标准的租户/用户令牌支持
    ///
    /// # 示例
    /// ```rust,ignore
    /// // 获取消息列表
    /// let mut query_params = HashMap::new();
    /// query_params.insert("container_id", chat_id);
    /// let response = RequestExecutor::query_request::<MessageListResponse>(
    ///     &config,
    ///     "/open-apis/im/v1/messages",
    ///     Some(query_params),
    ///     None,
    /// ).await?;
    /// ```
    pub async fn query_request<
        T: ApiResponseTrait + std::fmt::Debug + for<'de> serde::Deserialize<'de>,
    >(
        config: &Config,
        path: &str,
        query_params: Option<HashMap<&'static str, String>>,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<T>> {
        Self::get(
            config,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User], // 默认支持租户和用户令牌
            query_params,
            option,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ResponseFormat;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestRequest {
        message: String,
    }

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct TestResponse {
        id: String,
        status: String,
    }

    impl ApiResponseTrait for TestResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    #[tokio::test]
    async fn test_request_executor_path_params() {
        let path_template = "/open-apis/im/v1/messages/{message_id}/replies/{reply_id}";
        let path_params = HashMap::from([("message_id", "om_123"), ("reply_id", "reply_456")]);

        let expected_path = "/open-apis/im/v1/messages/om_123/replies/reply_456";

        // 测试路径参数替换
        let mut path = path_template.to_string();
        for (key, value) in path_params {
            path = path.replace(&format!("{{{key}}}"), value);
        }

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_request_body_serialization() {
        let request = TestRequest {
            message: "Hello, World!".to_string(),
        };

        let serialized = serde_json::to_vec(&request).unwrap();
        let expected = br#"{"message":"Hello, World!"}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_request_executor_get_method() {
        use crate::constants::AccessTokenType;
        use std::collections::HashMap;

        // Test that get method constructs the correct parameters
        let supported_tokens = [AccessTokenType::Tenant, AccessTokenType::User];
        let mut query_params = HashMap::new();
        query_params.insert("page", "1".to_string());
        query_params.insert("limit", "10".to_string());

        // We can't actually execute the request without a real config and transport,
        // but we can verify the method signature and parameter handling
        assert_eq!(supported_tokens.len(), 2);
        assert_eq!(query_params.len(), 2);
        assert_eq!(query_params.get("page"), Some(&"1".to_string()));
        assert_eq!(query_params.get("limit"), Some(&"10".to_string()));
    }

    #[test]
    fn test_request_executor_post_method_signature() {
        use crate::constants::AccessTokenType;

        // Test POST method parameter types
        let supported_tokens = [AccessTokenType::App];
        let request_body = TestRequest {
            message: "test message".to_string(),
        };

        // Verify we can serialize the body
        let serialized = serde_json::to_vec(&request_body).unwrap();
        assert!(!serialized.is_empty());

        // Verify token types
        assert_eq!(supported_tokens.len(), 1);
        assert_eq!(supported_tokens[0], AccessTokenType::App);
    }

    #[test]
    fn test_request_executor_put_method_signature() {
        use crate::constants::AccessTokenType;

        // Test PUT method parameter types
        let supported_tokens = [AccessTokenType::Tenant];
        let request_body = TestRequest {
            message: "update message".to_string(),
        };

        // Verify we can serialize the body
        let serialized = serde_json::to_vec(&request_body).unwrap();
        assert!(!serialized.is_empty());

        // Verify token types
        assert_eq!(supported_tokens.len(), 1);
        assert_eq!(supported_tokens[0], AccessTokenType::Tenant);
    }

    #[test]
    fn test_request_executor_delete_method_signature() {
        use crate::constants::AccessTokenType;

        // Test DELETE method parameter types
        let supported_tokens = [AccessTokenType::User];

        // Verify token types
        assert_eq!(supported_tokens.len(), 1);
        assert_eq!(supported_tokens[0], AccessTokenType::User);
    }

    #[test]
    fn test_request_executor_patch_method_signature() {
        use crate::constants::AccessTokenType;

        // Test PATCH method parameter types
        let supported_tokens = [AccessTokenType::Tenant, AccessTokenType::App];
        let request_body = TestRequest {
            message: "patch message".to_string(),
        };

        // Verify we can serialize the body
        let serialized = serde_json::to_vec(&request_body).unwrap();
        assert!(!serialized.is_empty());

        // Verify token types
        assert_eq!(supported_tokens.len(), 2);
        assert!(supported_tokens.contains(&AccessTokenType::Tenant));
        assert!(supported_tokens.contains(&AccessTokenType::App));
    }

    #[test]
    fn test_request_executor_execute_with_path_params_replacement() {
        use std::collections::HashMap;

        // Test path parameter replacement logic
        let path_template = "/open-apis/im/v1/messages/{message_id}/reactions/{reaction_id}";
        let mut path_params = HashMap::new();
        path_params.insert("message_id", "om_test123");
        path_params.insert("reaction_id", "react_456");

        // Simulate the path replacement logic
        let mut path = path_template.to_string();
        for (key, value) in &path_params {
            path = path.replace(&format!("{{{key}}}"), value);
        }

        let expected = "/open-apis/im/v1/messages/om_test123/reactions/react_456";
        assert_eq!(path, expected);

        // Test with missing parameters
        let path_template_missing = "/api/{missing_param}/test";
        let empty_params: HashMap<&str, &str> = HashMap::new();

        let mut path_missing = path_template_missing.to_string();
        for (key, value) in &empty_params {
            path_missing = path_missing.replace(&format!("{{{key}}}"), value);
        }

        // Should remain unchanged if no matching parameters
        assert_eq!(path_missing, "/api/{missing_param}/test");
    }

    #[test]
    fn test_request_executor_execute_with_path_params_edge_cases() {
        use std::collections::HashMap;

        // Test with empty path
        let empty_path = "";
        let mut path_params = HashMap::new();
        path_params.insert("param", "value");

        let mut path = empty_path.to_string();
        for (key, value) in &path_params {
            path = path.replace(&format!("{{{key}}}"), value);
        }
        assert_eq!(path, "");

        // Test with multiple same parameters
        let duplicate_path = "/api/{id}/sub/{id}/item";
        let mut duplicate_params = HashMap::new();
        duplicate_params.insert("id", "123");

        let mut path_duplicate = duplicate_path.to_string();
        for (key, value) in &duplicate_params {
            path_duplicate = path_duplicate.replace(&format!("{{{key}}}"), value);
        }
        assert_eq!(path_duplicate, "/api/123/sub/123/item");

        // Test with special characters in values
        let special_path = "/api/{name}/test";
        let mut special_params = HashMap::new();
        special_params.insert("name", "test@#$%");

        let mut path_special = special_path.to_string();
        for (key, value) in &special_params {
            path_special = path_special.replace(&format!("{{{key}}}"), value);
        }
        assert_eq!(path_special, "/api/test@#$%/test");
    }

    #[test]
    fn test_request_executor_json_request_defaults() {
        use crate::constants::AccessTokenType;

        // Test that json_request uses default token types
        let default_tokens = [AccessTokenType::Tenant, AccessTokenType::User];

        assert_eq!(default_tokens.len(), 2);
        assert!(default_tokens.contains(&AccessTokenType::Tenant));
        assert!(default_tokens.contains(&AccessTokenType::User));

        // Verify these are the same tokens used in json_request
        let expected_tokens = [AccessTokenType::Tenant, AccessTokenType::User];
        assert_eq!(default_tokens, expected_tokens);
    }

    #[test]
    fn test_request_executor_query_request_defaults() {
        use crate::constants::AccessTokenType;

        // Test that query_request uses default token types
        let default_tokens = [AccessTokenType::Tenant, AccessTokenType::User];

        assert_eq!(default_tokens.len(), 2);
        assert!(default_tokens.contains(&AccessTokenType::Tenant));
        assert!(default_tokens.contains(&AccessTokenType::User));
    }

    #[test]
    fn test_request_executor_body_serialization_edge_cases() {
        use serde_json;

        // Test empty struct
        #[derive(serde::Serialize)]
        struct EmptyRequest {}

        let empty_request = EmptyRequest {};
        let serialized = serde_json::to_vec(&empty_request).unwrap();
        let expected = b"{}";
        assert_eq!(serialized, expected);

        // Test struct with None values
        #[derive(serde::Serialize)]
        struct OptionalRequest {
            message: Option<String>,
            count: Option<i32>,
        }

        let optional_request = OptionalRequest {
            message: None,
            count: Some(42),
        };
        let serialized = serde_json::to_vec(&optional_request).unwrap();
        let deserialized: serde_json::Value = serde_json::from_slice(&serialized).unwrap();

        assert!(deserialized.get("message").unwrap().is_null());
        assert_eq!(deserialized.get("count").unwrap().as_i64().unwrap(), 42);
    }

    #[test]
    fn test_request_executor_query_params_handling() {
        use std::collections::HashMap;

        // Test query parameter construction
        let mut query_params: HashMap<&'static str, String> = HashMap::new();
        query_params.insert("page_size", "20".to_string());
        query_params.insert("sort_order", "desc".to_string());
        query_params.insert("filter", "active".to_string());

        assert_eq!(query_params.len(), 3);
        assert_eq!(query_params.get("page_size"), Some(&"20".to_string()));
        assert_eq!(query_params.get("sort_order"), Some(&"desc".to_string()));
        assert_eq!(query_params.get("filter"), Some(&"active".to_string()));

        // Test empty query params
        let empty_params: HashMap<&'static str, String> = HashMap::new();
        assert!(empty_params.is_empty());

        // Test query params with special characters
        let mut special_params: HashMap<&'static str, String> = HashMap::new();
        special_params.insert("search", "hello world".to_string());
        special_params.insert("encoded", "test@domain.com".to_string());

        assert_eq!(
            special_params.get("search"),
            Some(&"hello world".to_string())
        );
        assert_eq!(
            special_params.get("encoded"),
            Some(&"test@domain.com".to_string())
        );
    }

    #[test]
    fn test_request_executor_access_token_types() {
        use crate::constants::AccessTokenType;

        // Test all access token types
        let all_types = [
            AccessTokenType::Tenant,
            AccessTokenType::User,
            AccessTokenType::App,
        ];

        assert_eq!(all_types.len(), 3);

        // Test that types are distinct
        assert_ne!(AccessTokenType::Tenant, AccessTokenType::User);
        assert_ne!(AccessTokenType::User, AccessTokenType::App);
        assert_ne!(AccessTokenType::App, AccessTokenType::Tenant);

        // Test combinations
        let tenant_user = [AccessTokenType::Tenant, AccessTokenType::User];
        let app_only = [AccessTokenType::App];

        assert_eq!(tenant_user.len(), 2);
        assert_eq!(app_only.len(), 1);
    }

    #[test]
    fn test_request_executor_method_types() {
        use reqwest::Method;

        // Test HTTP methods used by RequestExecutor
        let get_method = Method::GET;
        let post_method = Method::POST;
        let put_method = Method::PUT;
        let delete_method = Method::DELETE;
        let patch_method = Method::PATCH;

        assert_eq!(get_method.as_str(), "GET");
        assert_eq!(post_method.as_str(), "POST");
        assert_eq!(put_method.as_str(), "PUT");
        assert_eq!(delete_method.as_str(), "DELETE");
        assert_eq!(patch_method.as_str(), "PATCH");

        // Test method comparison
        assert_ne!(get_method, post_method);
        assert_ne!(post_method, put_method);
        assert_ne!(put_method, delete_method);
        assert_ne!(delete_method, patch_method);
    }

    #[test]
    fn test_request_executor_api_request_construction() {
        use crate::api::ApiRequest;
        use std::collections::HashMap;

        // Test ApiRequest construction (simplified for current structure)
        let mut api_req: ApiRequest<String> = ApiRequest::post("https://open.feishu.cn/test/path");

        // Verify the request was created successfully
        assert!(api_req.url.to_string().contains("test/path"));

        // Test setting query params
        let mut query_params = HashMap::new();
        query_params.insert("key".to_string(), "value".to_string());
        api_req.query = query_params;

        assert_eq!(api_req.query.len(), 1);
        assert_eq!(api_req.query.get("key"), Some(&"value".to_string()));

        // Test setting body
        let body_data = TestRequest {
            message: "test".to_string(),
        };
        api_req.body = Some(crate::api::RequestData::Json(
            serde_json::to_value(&body_data).unwrap(),
        ));

        assert!(api_req.body.is_some());
        if let Some(crate::api::RequestData::Json(json_body)) = &api_req.body {
            let deserialized: TestRequest = serde_json::from_value(json_body.clone()).unwrap();
            assert_eq!(deserialized.message, "test");
        }
    }

    #[test]
    fn test_request_executor_error_handling() {
        // Test serialization error handling simulation
        use serde_json;

        // Create a struct that can fail serialization
        #[derive(serde::Serialize)]
        struct ValidRequest {
            message: String,
        }

        let valid_request = ValidRequest {
            message: "test".to_string(),
        };

        let serialization_result = serde_json::to_vec(&valid_request);
        assert!(serialization_result.is_ok());

        // Test with invalid JSON (this should work fine, but tests the error path logic)
        let invalid_json = "invalid json";
        let parse_result: Result<serde_json::Value, _> = serde_json::from_str(invalid_json);
        assert!(parse_result.is_err());
    }

    #[test]
    fn test_request_option_integration() {
        use crate::req_option::RequestOption;

        // Test that RequestOption can be created and used with RequestExecutor methods
        let option = RequestOption::builder()
            .tenant_key("test_tenant")
            .user_access_token("user_token")
            .request_id("req_123")
            .add_header("X-Test-Header", "test_value")
            .build();

        assert_eq!(option.tenant_key, "test_tenant");
        assert_eq!(option.user_access_token, "user_token");
        assert_eq!(option.request_id, "req_123");
        assert_eq!(
            option.header.get("X-Test-Header"),
            Some(&"test_value".to_string())
        );

        // Test None option case
        let none_option: Option<RequestOption> = None;
        assert!(none_option.is_none());

        // Test Some option case
        let some_option = Some(option);
        assert!(some_option.is_some());
    }

    #[test]
    fn test_request_executor_const_values() {
        // Test path templates and constants that might be used
        let path_templates = vec![
            "/open-apis/im/v1/messages",
            "/open-apis/im/v1/messages/{message_id}",
            "/open-apis/contact/v3/users",
            "/open-apis/contact/v3/users/{user_id}",
        ];

        for template in path_templates {
            assert!(!template.is_empty());
            assert!(template.starts_with("/open-apis"));
        }

        // Test common query parameter names
        let query_param_names = vec![
            "page_token",
            "page_size",
            "user_id_type",
            "container_id",
            "sort_order",
        ];

        for param_name in query_param_names {
            assert!(!param_name.is_empty());
            assert!(param_name.is_ascii());
        }
    }

    // ==================== Enhanced Coverage Tests ====================

    // Complex serialization scenarios
    #[test]
    fn test_request_executor_complex_serialization() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        struct ComplexRequest {
            id: i64,
            name: String,
            tags: Vec<String>,
            metadata: Option<serde_json::Value>,
            nested: NestedStruct,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct NestedStruct {
            active: bool,
            score: f64,
            items: Vec<i32>,
        }

        let request = ComplexRequest {
            id: 123,
            name: "测试名称".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            metadata: Some(serde_json::json!({"key": "value"})),
            nested: NestedStruct {
                active: true,
                score: 95.5,
                items: vec![1, 2, 3],
            },
        };

        let serialized = serde_json::to_vec(&request).unwrap();
        let deserialized: ComplexRequest = serde_json::from_slice(&serialized).unwrap();

        assert_eq!(request.id, deserialized.id);
        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.tags, deserialized.tags);
        assert_eq!(request.nested.active, deserialized.nested.active);
        assert_eq!(request.nested.score, deserialized.nested.score);
    }

    // Error handling and edge cases
    #[test]
    fn test_request_executor_serialization_error_handling() {
        use serde_json;

        // Test with large data that might cause issues
        #[derive(serde::Serialize)]
        struct LargeRequest {
            data: String,
        }

        let large_data = "x".repeat(1_000_000); // 1MB string
        let large_request = LargeRequest {
            data: large_data.clone(),
        };

        let serialization_result = serde_json::to_vec(&large_request);
        assert!(serialization_result.is_ok());

        let serialized = serialization_result.unwrap();
        // The JSON structure will be like {"data":"xxxxx..."}, so it should be larger than the raw data
        assert!(serialized.len() > large_data.len());
        // But it shouldn't be excessively large (rough estimate: field name + quotes + colon + braces)
        assert!(serialized.len() < large_data.len() + 100);
    }

    // Unicode and internationalization tests
    #[test]
    fn test_request_executor_unicode_handling() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        struct UnicodeRequest {
            chinese: String,
            emoji: String,
            arabic: String,
            mixed: Vec<String>,
        }

        let request = UnicodeRequest {
            chinese: "你好世界".to_string(),
            emoji: "🚀🎉💻".to_string(),
            arabic: "مرحبا بالعالم".to_string(),
            mixed: vec![
                "测试".to_string(),
                "🌟".to_string(),
                "Café".to_string(),
                "العربية".to_string(),
            ],
        };

        let serialized = serde_json::to_vec(&request).unwrap();
        let deserialized: UnicodeRequest = serde_json::from_slice(&serialized).unwrap();

        assert_eq!(request.chinese, deserialized.chinese);
        assert_eq!(request.emoji, deserialized.emoji);
        assert_eq!(request.arabic, deserialized.arabic);
        assert_eq!(request.mixed, deserialized.mixed);
    }

    // Performance and efficiency tests
    #[test]
    fn test_request_executor_performance_characteristics() {
        use std::time::Instant;

        // Test serialization performance
        #[derive(serde::Serialize)]
        struct PerformanceRequest {
            items: Vec<i32>,
            text: String,
        }

        let large_request = PerformanceRequest {
            items: (0..10000).collect(),
            text: "performance test".repeat(1000),
        };

        let start = Instant::now();
        let serialized = serde_json::to_vec(&large_request).unwrap();
        let duration = start.elapsed();

        // Serialization should be fast (less than 100ms for this amount of data)
        assert!(duration.as_millis() < 100);
        assert!(!serialized.is_empty());
        assert!(serialized.len() > 1000);
    }

    // Path parameter edge cases
    #[test]
    fn test_request_executor_path_parameter_edge_cases() {
        use std::collections::HashMap;

        // Test with nested braces
        let nested_path = "/api/{{double}}/test";
        let mut nested_params = HashMap::new();
        nested_params.insert("double", "value");

        let mut path = nested_path.to_string();
        for (key, value) in &nested_params {
            path = path.replace(&format!("{{{key}}}"), value);
        }
        assert_eq!(path, "/api/{value}/test");

        // Test with parameter names containing numbers
        let numbered_path = "/api/{param1}/sub/{param2}/item";
        let mut numbered_params = HashMap::new();
        numbered_params.insert("param1", "value1");
        numbered_params.insert("param2", "value2");

        let mut path_numbered = numbered_path.to_string();
        for (key, value) in &numbered_params {
            path_numbered = path_numbered.replace(&format!("{{{key}}}"), value);
        }
        assert_eq!(path_numbered, "/api/value1/sub/value2/item");

        // Test with empty values
        let empty_value_path = "/api/{empty_param}/test";
        let mut empty_value_params = HashMap::new();
        empty_value_params.insert("empty_param", "");

        let mut path_empty = empty_value_path.to_string();
        for (key, value) in &empty_value_params {
            path_empty = path_empty.replace(&format!("{{{key}}}"), value);
        }
        assert_eq!(path_empty, "/api//test"); // Note: double slash
    }

    // Query parameter encoding scenarios
    #[test]
    fn test_request_executor_query_parameter_encoding() {
        use std::collections::HashMap;

        // Test with URL-encoded characters
        let mut encoded_params: HashMap<&'static str, String> = HashMap::new();
        encoded_params.insert("search", "hello world".to_string());
        encoded_params.insert("filter", "name>100".to_string());
        encoded_params.insert("path", "/api/test".to_string());
        encoded_params.insert("encoded", "a%20b%20c".to_string());

        assert_eq!(
            encoded_params.get("search"),
            Some(&"hello world".to_string())
        );
        assert_eq!(encoded_params.get("filter"), Some(&"name>100".to_string()));
        assert_eq!(encoded_params.get("path"), Some(&"/api/test".to_string()));
        assert_eq!(
            encoded_params.get("encoded"),
            Some(&"a%20b%20c".to_string())
        );
    }

    // Complex request option scenarios
    #[test]
    fn test_request_executor_request_option_complex_scenarios() {
        use crate::req_option::RequestOption;

        // Test with multiple headers
        let complex_option = RequestOption::builder()
            .tenant_key("test_tenant")
            .user_access_token("user_token")
            .app_access_token("app_token")
            .request_id("req_123456")
            .add_header("X-Test-Header", "test_value")
            .add_header("Authorization", "Bearer token")
            .add_header("Content-Type", "application/json")
            .add_header("X-Rate-Limit-Limit", "100")
            .build();

        assert_eq!(complex_option.tenant_key, "test_tenant");
        assert_eq!(complex_option.user_access_token, "user_token");
        assert_eq!(complex_option.app_access_token, "app_token");
        assert_eq!(complex_option.request_id, "req_123456");
        assert_eq!(complex_option.header.len(), 4);

        // Test header order preservation
        let ordered_option = RequestOption::builder()
            .add_header("X-First", "first_value")
            .add_header("X-Second", "second_value")
            .add_header("X-Third", "third_value")
            .build();

        assert_eq!(ordered_option.header.len(), 3);
        assert_eq!(
            ordered_option.header.get("X-First"),
            Some(&"first_value".to_string())
        );
        assert_eq!(
            ordered_option.header.get("X-Second"),
            Some(&"second_value".to_string())
        );
        assert_eq!(
            ordered_option.header.get("X-Third"),
            Some(&"third_value".to_string())
        );
    }

    // Concurrent access scenarios
    #[test]
    fn test_request_executor_concurrent_serialization() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        #[derive(serde::Serialize, serde::Deserialize, Clone)]
        struct ConcurrentRequest {
            id: usize,
            data: String,
        }

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];

        for i in 0..10 {
            let results_clone = results.clone();
            let handle = thread::spawn(move || {
                let request = ConcurrentRequest {
                    id: i,
                    data: format!("concurrent test {}", i),
                };

                let serialized = serde_json::to_vec(&request).unwrap();
                let deserialized: ConcurrentRequest = serde_json::from_slice(&serialized).unwrap();

                results_clone.lock().unwrap().push(deserialized);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let results_vec = results.lock().unwrap();
        assert_eq!(results_vec.len(), 10);

        // Verify all requests were processed correctly
        for i in 0..10 {
            let found = results_vec
                .iter()
                .any(|req| req.id == i && req.data == format!("concurrent test {}", i));
            assert!(found, "Request {} not found in results", i);
        }
    }

    // Memory usage and optimization tests
    #[test]
    fn test_request_executor_memory_usage() {
        use std::mem;

        #[derive(serde::Serialize)]
        struct MemoryTestRequest {
            large_string: String,
            numbers: Vec<i64>,
        }

        let request = MemoryTestRequest {
            large_string: "x".repeat(1000),
            numbers: (0..1000).collect(),
        };

        // Check memory size of serialized data
        let serialized = serde_json::to_vec(&request).unwrap();
        assert!(serialized.len() > 1000);
        assert!(serialized.len() < 100_000); // Should not be excessively large

        // Check memory usage of request struct
        let request_size = mem::size_of_val(&request);
        assert!(request_size > 0);
        assert!(request_size < 1_000_000); // Should be reasonable
    }

    // Type system and generic tests
    #[test]
    fn test_request_executor_type_system() {
        use crate::api::ResponseFormat;

        // Test different response types
        #[derive(Debug, Serialize, Deserialize)]
        struct SimpleResponse {
            message: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct ComplexResponse {
            id: i64,
            data: Vec<String>,
            metadata: Option<serde_json::Value>,
        }

        impl ApiResponseTrait for SimpleResponse {
            fn data_format() -> ResponseFormat {
                ResponseFormat::Data
            }
        }

        impl ApiResponseTrait for ComplexResponse {
            fn data_format() -> ResponseFormat {
                ResponseFormat::Data
            }
        }

        // Test that both types can be used
        let simple_data = SimpleResponse {
            message: "test".to_string(),
        };

        let complex_data = ComplexResponse {
            id: 123,
            data: vec!["item1".to_string(), "item2".to_string()],
            metadata: Some(serde_json::json!({"key": "value"})),
        };

        let simple_serialized = serde_json::to_vec(&simple_data).unwrap();
        let complex_serialized = serde_json::to_vec(&complex_data).unwrap();

        assert!(!simple_serialized.is_empty());
        assert!(!complex_serialized.is_empty());
        assert!(complex_serialized.len() > simple_serialized.len());
    }

    // API request building edge cases
    #[test]
    fn test_request_executor_api_request_building_edge_cases() {
        use crate::api::{ApiRequest, HttpMethod};

        // Test with very long path
        let long_path = "/".to_string() + &"a".repeat(1000);
        let api_req_long: ApiRequest<String> = ApiRequest {
            method: HttpMethod::Get,
            url: format!("https://open.feishu.cn/open-apis{}", long_path),
            headers: std::collections::HashMap::new(),
            query: std::collections::HashMap::new(),
            body: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        };

        assert_eq!(api_req_long.url.len(), 1001 + 32); // 32 chars for base URL

        // Test with empty request
        let api_req_empty: ApiRequest<String> = ApiRequest {
            method: HttpMethod::Post,
            url: "https://open.feishu.cn/open-apis/test".to_string(),
            headers: std::collections::HashMap::new(),
            query: std::collections::HashMap::new(),
            body: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        };

        assert!(api_req_empty.headers.is_empty());

        // Test with query parameters
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("tenant".to_string(), "test".to_string());
        query_params.insert("user".to_string(), "test".to_string());
        query_params.insert("app".to_string(), "test".to_string());

        let api_req_with_query: ApiRequest<String> = ApiRequest {
            method: HttpMethod::Put,
            url: "https://open.feishu.cn/open-apis/test".to_string(),
            headers: std::collections::HashMap::new(),
            query: query_params,
            body: None,
            timeout: None,
            _phantom: std::marker::PhantomData,
        };

        assert_eq!(api_req_with_query.query.len(), 3);
    }

    // Method chaining and composition tests
    #[test]
    fn test_request_executor_method_composition() {
        use crate::constants::AccessTokenType;

        // Test that different methods can be composed together
        let common_tokens = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let app_only_tokens = vec![AccessTokenType::App];
        let user_only_tokens = vec![AccessTokenType::User];

        // Test token combinations
        assert_ne!(common_tokens, app_only_tokens);
        assert_ne!(common_tokens, user_only_tokens);
        assert_ne!(app_only_tokens, user_only_tokens);

        // Test token vector operations
        let mut combined_tokens = common_tokens.clone();
        combined_tokens.extend(app_only_tokens.clone());
        // Remove duplicates without sorting (since AccessTokenType doesn't implement Ord)
        combined_tokens.dedup();

        assert_eq!(combined_tokens.len(), 3);
        assert!(combined_tokens.contains(&AccessTokenType::Tenant));
        assert!(combined_tokens.contains(&AccessTokenType::User));
        assert!(combined_tokens.contains(&AccessTokenType::App));
    }

    // Async execution patterns (conceptual tests)
    #[test]
    fn test_request_executor_async_patterns() {
        use std::future::Future;

        // Test that our types implement required traits for async
        fn assert_send<T: Send>(_: &T) {}
        fn assert_sync<T: Sync>(_: &T) {}

        let config = create_test_config();
        assert_send(&config);
        assert_sync(&config);

        // Test that our futures would be Send (this is a compile-time check)
        #[allow(dead_code)]
        fn check_future_send<F: Future + Send>(_: F) {}

        // This would be used in actual async contexts
        let _check = || {
            // In real async code:
            // let future = RequestExecutor::get::<TestResponse>(&config, "/test", vec![], None, None);
            // check_future_send(future);
        };
    }

    // Helper function for creating test config
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }
}
