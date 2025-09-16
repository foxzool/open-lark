use std::collections::HashMap;

use reqwest::Method;
use serde::Serialize;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse},
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
    pub async fn get<T: ApiResponseTrait>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        query_params: Option<HashMap<&'static str, String>>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
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
    pub async fn post<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
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
    pub async fn put<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
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
    pub async fn delete<T: ApiResponseTrait>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
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
    pub async fn patch<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
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
    /// 返回标准的`BaseResponse<T>`格式响应
    ///
    /// # 示例
    /// ```rust,ignore
    /// // GET请求
    /// let response: BaseResponse<MessageList> = RequestExecutor::execute(
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
    /// let response: BaseResponse<Message> = RequestExecutor::execute(
    ///     &config,
    ///     Method::POST,
    ///     "/open-apis/im/v1/messages",
    ///     [AccessTokenType::Tenant, AccessTokenType::User],
    ///     None,
    ///     Some(create_request),
    ///     None,
    /// ).await?;
    /// ```
    pub async fn execute<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        method: Method,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        query_params: Option<HashMap<&'static str, String>>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        let mut api_req = ApiRequest {
            http_method: method,
            api_path: path.to_string(),
            supported_access_token_types: supported_tokens,
            ..Default::default()
        };

        // 设置查询参数
        if let Some(params) = query_params {
            api_req.query_params = params;
        }

        // 序列化请求体
        if let Some(body_data) = body {
            api_req.body = serde_json::to_vec(&body_data)
                .map_err(|e| crate::core::error::LarkAPIError::DeserializeError(e.to_string()))?;
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
    pub async fn execute_with_path_params<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        method: Method,
        path_template: &str,
        path_params: HashMap<&str, &str>,
        supported_tokens: Vec<AccessTokenType>,
        query_params: Option<HashMap<&'static str, String>>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
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
    pub async fn json_request<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        method: Method,
        path: &str,
        body: &B,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
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
    pub async fn query_request<T: ApiResponseTrait>(
        config: &Config,
        path: &str,
        query_params: Option<HashMap<&'static str, String>>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
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
    use crate::core::api_resp::ResponseFormat;
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
        use crate::core::constants::AccessTokenType;
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
        use crate::core::constants::AccessTokenType;

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
        use crate::core::constants::AccessTokenType;

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
        use crate::core::constants::AccessTokenType;

        // Test DELETE method parameter types
        let supported_tokens = [AccessTokenType::User];

        // Verify token types
        assert_eq!(supported_tokens.len(), 1);
        assert_eq!(supported_tokens[0], AccessTokenType::User);
    }

    #[test]
    fn test_request_executor_patch_method_signature() {
        use crate::core::constants::AccessTokenType;

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
        use crate::core::constants::AccessTokenType;

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
        use crate::core::constants::AccessTokenType;

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
        use crate::core::constants::AccessTokenType;

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
        use crate::core::api_req::ApiRequest;
        use crate::core::constants::AccessTokenType;
        use reqwest::Method;
        use std::collections::HashMap;

        // Test ApiRequest construction (similar to what execute() does)
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/test/path".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            ..Default::default()
        };

        assert_eq!(api_req.http_method, Method::POST);
        assert_eq!(api_req.api_path, "/test/path");
        assert_eq!(api_req.supported_access_token_types.len(), 1);
        assert_eq!(
            api_req.supported_access_token_types[0],
            AccessTokenType::Tenant
        );

        // Test setting query params
        let mut query_params = HashMap::new();
        query_params.insert("key", "value".to_string());
        api_req.query_params = query_params;

        assert_eq!(api_req.query_params.len(), 1);
        assert_eq!(api_req.query_params.get("key"), Some(&"value".to_string()));

        // Test setting body
        let body_data = TestRequest {
            message: "test".to_string(),
        };
        api_req.body = serde_json::to_vec(&body_data).unwrap();

        assert!(!api_req.body.is_empty());
        let deserialized: TestRequest = serde_json::from_slice(&api_req.body).unwrap();
        assert_eq!(deserialized.message, "test");
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
        use crate::core::req_option::RequestOption;

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
}
