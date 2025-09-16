use std::collections::HashMap;

use reqwest::Method;

use crate::core::constants::AccessTokenType;

/// API请求的核心数据结构 - 命令模式的体现
///
/// `ApiRequest` 是整个SDK架构的核心，采用命令模式（Command Pattern）设计。
/// 它封装了发起一次飞书API调用所需的所有信息，充当服务层（Service）与传输层（Transport）之间的桥梁。
///
/// # 设计理念
///
/// - **解耦性**：服务层只负责构建请求，不关心HTTP细节
/// - **统一性**：所有API调用都通过这个统一的结构体表示
/// - **灵活性**：支持普通请求和multipart/form-data请求
///
/// # 使用流程
///
/// 1. 服务层方法创建并配置 `ApiRequest` 实例
/// 2. 设置HTTP方法、路径、认证需求等基本信息
/// 3. 根据请求类型填充 `body` 和/或 `file` 字段
/// 4. 将配置好的请求传递给 `Transport::request` 进行处理
///
/// # 示例
///
/// ```rust,ignore
/// // 普通JSON请求
/// let mut api_req = ApiRequest {
///     http_method: Method::POST,
///     api_path: "/open-apis/drive/v1/files".to_string(),
///     body: serde_json::to_vec(&request_data).unwrap(),
///     ..Default::default()
/// };
///
/// // 文件上传请求（multipart）
/// let mut api_req = ApiRequest {
///     http_method: Method::POST,
///     api_path: "/open-apis/drive/v1/files/upload".to_string(),
///     body: serde_json::to_vec(&metadata).unwrap(),  // JSON元数据
///     file: file_bytes,  // 文件内容
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct ApiRequest {
    /// HTTP请求方法（GET、POST、PUT、DELETE等）
    ///
    /// 由服务层根据具体API的要求设置。
    /// 使用 `pub(crate)` 限制只能在crate内部修改，保证一致性。
    pub(crate) http_method: Method,

    /// API的相对路径
    ///
    /// 例如：`/open-apis/drive/v1/files/{file_id}`
    ///
    /// 路径中的动态参数（如 `{file_id}`）通常通过 `format!` 宏直接嵌入，
    /// 而不是使用 `path_params` 字段。
    pub api_path: String,

    /// 请求体数据（序列化后的字节数组）
    ///
    /// # 在不同请求类型中的用途：
    ///
    /// - **普通请求**：包含完整的请求体，通常是JSON序列化后的数据
    /// - **文件上传（multipart）**：仅包含JSON元数据部分，文件内容存储在 `file` 字段
    /// - **无请求体的请求**：保持为空 `Vec`
    ///
    /// # 注意事项
    ///
    /// 服务层通常使用 `serde_json::to_vec()` 将请求结构体序列化到这个字段。
    pub body: Vec<u8>,

    /// URL查询参数
    ///
    /// 存储将被附加到URL末尾的查询参数。
    /// 例如：`?page_size=10&page_token=xxx`
    ///
    /// # 性能优化
    ///
    /// 键使用 `&'static str` 避免堆分配，配合 `QueryParams` 常量使用：
    ///
    /// ```rust,ignore
    /// api_req.query_params.insert(QueryParams::PAGE_SIZE, "10".to_string());
    /// api_req.query_params.insert(QueryParams::PAGE_TOKEN, token);
    /// ```
    ///
    /// 这种设计减少了每次API调用约8-16字节的内存分配。
    pub query_params: HashMap<&'static str, String>,

    /// URL路径参数（保留字段）
    ///
    /// 该字段为未来的路径模板功能保留。目前在现有架构中：
    ///
    /// - **当前做法**: 路径参数通过 `format!` 宏直接嵌入 `api_path`
    /// - **替代方案**: 可使用 `RequestExecutor::execute_with_path_params()` 进行路径参数替换
    /// - **设计考虑**: 保留该字段可为未来的模板系统升级提供支持
    ///
    /// 关于路径参数处理，参考 `crate::service::endpoints::EndpointHelper::replace_path_params`
    /// 和 `crate::core::request_executor::RequestExecutor::execute_with_path_params`。
    pub path_params: HashMap<String, Vec<String>>,

    /// 支持的访问令牌类型
    ///
    /// 指定此API端点接受哪些类型的访问令牌：
    /// - `User`：用户访问令牌
    /// - `Tenant`：租户访问令牌  
    /// - `App`：应用访问令牌
    ///
    /// Transport层会根据这个列表和当前配置选择合适的令牌类型。
    /// 使用 `pub(crate)` 确保只能由服务层设置。
    pub(crate) supported_access_token_types: Vec<AccessTokenType>,

    /// 文件内容（用于multipart/form-data请求）
    ///
    /// # 在不同请求类型中的用途：
    ///
    /// - **普通请求**：保持为空 `Vec`
    /// - **文件上传（multipart）**：包含要上传的文件的二进制内容
    ///
    /// # 工作原理
    ///
    /// 当 `file` 字段非空时，Transport层会自动识别这是一个multipart请求：
    /// 1. `body` 字段的内容作为multipart的JSON元数据部分
    /// 2. `file` 字段的内容作为文件部分
    /// 3. Content-Type自动设置为 `multipart/form-data`
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// // 文件上传请求
    /// api_req.body = serde_json::to_vec(&FileMetadata {
    ///     name: "document.pdf",
    ///     parent_id: "folder123",
    /// }).unwrap();
    /// api_req.file = std::fs::read("path/to/document.pdf").unwrap();
    /// ```
    pub file: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::AccessTokenType;
    use reqwest::Method;

    #[test]
    fn test_api_request_creation() {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/test/v1/endpoint".to_string(),
            body: b"test body".to_vec(),
            query_params: HashMap::new(),
            path_params: HashMap::new(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            file: Vec::new(),
        };

        assert_eq!(api_req.http_method, Method::POST);
        assert_eq!(api_req.api_path, "/open-apis/test/v1/endpoint");
        assert_eq!(api_req.body, b"test body".to_vec());
        assert!(api_req.query_params.is_empty());
        assert!(api_req.path_params.is_empty());
        assert_eq!(
            api_req.supported_access_token_types,
            vec![AccessTokenType::Tenant]
        );
        assert!(api_req.file.is_empty());
    }

    #[test]
    fn test_api_request_default() {
        let api_req = ApiRequest::default();

        assert_eq!(api_req.http_method, Method::GET);
        assert!(api_req.api_path.is_empty());
        assert!(api_req.body.is_empty());
        assert!(api_req.query_params.is_empty());
        assert!(api_req.path_params.is_empty());
        assert!(api_req.supported_access_token_types.is_empty());
        assert!(api_req.file.is_empty());
    }

    #[test]
    fn test_api_request_clone() {
        let original = ApiRequest {
            http_method: Method::PUT,
            api_path: "/open-apis/clone/test".to_string(),
            body: b"original body".to_vec(),
            query_params: {
                let mut params = HashMap::new();
                params.insert("page_size", "10".to_string());
                params
            },
            path_params: {
                let mut params = HashMap::new();
                params.insert("file_id".to_string(), vec!["123".to_string()]);
                params
            },
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            file: b"file content".to_vec(),
        };

        let cloned = original.clone();

        assert_eq!(original.http_method, cloned.http_method);
        assert_eq!(original.api_path, cloned.api_path);
        assert_eq!(original.body, cloned.body);
        assert_eq!(original.query_params, cloned.query_params);
        assert_eq!(original.path_params, cloned.path_params);
        assert_eq!(
            original.supported_access_token_types,
            cloned.supported_access_token_types
        );
        assert_eq!(original.file, cloned.file);
    }

    #[test]
    fn test_api_request_debug() {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: "/debug/test".to_string(),
            body: b"debug body".to_vec(),
            ..Default::default()
        };

        let debug_str = format!("{:?}", api_req);

        assert!(debug_str.contains("ApiRequest"));
        assert!(debug_str.contains("DELETE"));
        assert!(debug_str.contains("/debug/test"));
    }

    #[test]
    fn test_api_request_with_different_http_methods() {
        let methods = vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::HEAD,
            Method::OPTIONS,
        ];

        for method in methods {
            let api_req = ApiRequest {
                http_method: method.clone(),
                ..Default::default()
            };
            assert_eq!(api_req.http_method, method);
        }
    }

    #[test]
    fn test_api_request_with_query_params() {
        let mut api_req = ApiRequest::default();

        // Add query parameters
        api_req.query_params.insert("page_size", "20".to_string());
        api_req
            .query_params
            .insert("page_token", "token123".to_string());
        api_req
            .query_params
            .insert("filter", "status=active".to_string());

        assert_eq!(api_req.query_params.len(), 3);
        assert_eq!(
            api_req.query_params.get("page_size"),
            Some(&"20".to_string())
        );
        assert_eq!(
            api_req.query_params.get("page_token"),
            Some(&"token123".to_string())
        );
        assert_eq!(
            api_req.query_params.get("filter"),
            Some(&"status=active".to_string())
        );
    }

    #[test]
    fn test_api_request_with_path_params() {
        let mut api_req = ApiRequest::default();

        // Add path parameters
        api_req
            .path_params
            .insert("user_id".to_string(), vec!["user123".to_string()]);
        api_req
            .path_params
            .insert("file_id".to_string(), vec!["file456".to_string()]);
        api_req.path_params.insert(
            "multiple".to_string(),
            vec!["val1".to_string(), "val2".to_string()],
        );

        assert_eq!(api_req.path_params.len(), 3);
        assert_eq!(
            api_req.path_params.get("user_id"),
            Some(&vec!["user123".to_string()])
        );
        assert_eq!(
            api_req.path_params.get("file_id"),
            Some(&vec!["file456".to_string()])
        );
        assert_eq!(
            api_req.path_params.get("multiple"),
            Some(&vec!["val1".to_string(), "val2".to_string()])
        );
    }

    #[test]
    fn test_api_request_with_different_access_token_types() {
        let token_types = vec![
            vec![AccessTokenType::User],
            vec![AccessTokenType::Tenant],
            vec![AccessTokenType::App],
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            vec![
                AccessTokenType::User,
                AccessTokenType::Tenant,
                AccessTokenType::App,
            ],
        ];

        for token_type_vec in token_types {
            let api_req = ApiRequest {
                supported_access_token_types: token_type_vec.clone(),
                ..Default::default()
            };
            assert_eq!(api_req.supported_access_token_types, token_type_vec);
        }
    }

    #[test]
    fn test_api_request_with_body_serialization() {
        // Test with JSON serialization
        let json_data = serde_json::json!({
            "name": "test file",
            "parent_id": "folder123"
        });
        let json_bytes = serde_json::to_vec(&json_data).unwrap();

        let api_req = ApiRequest {
            body: json_bytes.clone(),
            ..Default::default()
        };

        assert_eq!(api_req.body, json_bytes);

        // Verify it can be deserialized back
        let deserialized: serde_json::Value = serde_json::from_slice(&api_req.body).unwrap();
        assert_eq!(deserialized, json_data);
    }

    #[test]
    fn test_api_request_with_empty_body() {
        let api_req = ApiRequest {
            body: Vec::new(),
            ..Default::default()
        };

        assert!(api_req.body.is_empty());
    }

    #[test]
    fn test_api_request_with_large_body() {
        let large_body = vec![0u8; 1024 * 1024]; // 1MB
        let api_req = ApiRequest {
            body: large_body.clone(),
            ..Default::default()
        };

        assert_eq!(api_req.body.len(), 1024 * 1024);
        assert_eq!(api_req.body, large_body);
    }

    #[test]
    fn test_api_request_with_file_upload() {
        let file_content = b"binary file content";
        let metadata = serde_json::json!({
            "filename": "test.txt",
            "size": file_content.len()
        });

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/upload".to_string(),
            body: serde_json::to_vec(&metadata).unwrap(),
            file: file_content.to_vec(),
            ..Default::default()
        };

        assert_eq!(api_req.http_method, Method::POST);
        assert_eq!(api_req.api_path, "/upload");
        assert!(!api_req.body.is_empty());
        assert_eq!(api_req.file, file_content.to_vec());
    }

    #[test]
    fn test_api_request_with_empty_file() {
        let api_req = ApiRequest {
            file: Vec::new(),
            ..Default::default()
        };

        assert!(api_req.file.is_empty());
    }

    #[test]
    fn test_api_request_with_large_file() {
        let large_file = vec![1u8; 10 * 1024 * 1024]; // 10MB
        let api_req = ApiRequest {
            file: large_file.clone(),
            ..Default::default()
        };

        assert_eq!(api_req.file.len(), 10 * 1024 * 1024);
        assert_eq!(api_req.file, large_file);
    }

    #[test]
    fn test_api_request_multipart_structure() {
        let metadata = serde_json::json!({
            "name": "document.pdf",
            "parent_id": "folder123"
        });
        let file_content = b"PDF file binary content";

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/upload/multipart".to_string(),
            body: serde_json::to_vec(&metadata).unwrap(),
            file: file_content.to_vec(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            ..Default::default()
        };

        // Verify multipart request structure
        assert_eq!(api_req.http_method, Method::POST);
        assert!(!api_req.body.is_empty()); // Has metadata
        assert!(!api_req.file.is_empty()); // Has file content
        assert_eq!(
            api_req.supported_access_token_types,
            vec![AccessTokenType::Tenant]
        );
    }

    #[test]
    fn test_api_request_path_variations() {
        let paths = vec![
            "/open-apis/drive/v1/files",
            "/open-apis/drive/v1/files/{file_id}",
            "/open-apis/contact/v3/users/{user_id}/update",
            "",
            "/",
            "/simple",
            "/very/deep/nested/path/structure/endpoint",
        ];

        for path in paths {
            let api_req = ApiRequest {
                api_path: path.to_string(),
                ..Default::default()
            };
            assert_eq!(api_req.api_path, path);
        }
    }

    #[test]
    fn test_api_request_special_characters_in_paths() {
        let special_paths = vec![
            "/path/with spaces",
            "/path/with-dashes",
            "/path/with_underscores",
            "/path/with.dots",
            "/path/with@symbols",
            "/path/with中文字符",
            "/path/with🚀emoji",
        ];

        for path in special_paths {
            let api_req = ApiRequest {
                api_path: path.to_string(),
                ..Default::default()
            };
            assert_eq!(api_req.api_path, path);
        }
    }

    #[test]
    fn test_api_request_query_params_special_values() {
        let mut api_req = ApiRequest::default();

        // Test with special characters and edge cases
        api_req.query_params.insert("empty", "".to_string());
        api_req
            .query_params
            .insert("space", "value with space".to_string());
        api_req
            .query_params
            .insert("special", "value@#$%^&*()".to_string());
        api_req
            .query_params
            .insert("unicode", "中文值🚀".to_string());
        api_req
            .query_params
            .insert("url_encoded", "value%20with%20encoding".to_string());

        assert_eq!(api_req.query_params.len(), 5);
        assert_eq!(api_req.query_params.get("empty"), Some(&"".to_string()));
        assert_eq!(
            api_req.query_params.get("space"),
            Some(&"value with space".to_string())
        );
        assert_eq!(
            api_req.query_params.get("special"),
            Some(&"value@#$%^&*()".to_string())
        );
        assert_eq!(
            api_req.query_params.get("unicode"),
            Some(&"中文值🚀".to_string())
        );
        assert_eq!(
            api_req.query_params.get("url_encoded"),
            Some(&"value%20with%20encoding".to_string())
        );
    }

    #[test]
    fn test_api_request_path_params_complex() {
        let mut api_req = ApiRequest::default();

        // Test with complex path parameter structures
        api_req
            .path_params
            .insert("single".to_string(), vec!["one".to_string()]);
        api_req.path_params.insert(
            "multiple".to_string(),
            vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string(),
            ],
        );
        api_req.path_params.insert("empty".to_string(), vec![]);
        api_req.path_params.insert(
            "special".to_string(),
            vec![
                "value@#$".to_string(),
                "中文".to_string(),
                "🚀emoji".to_string(),
            ],
        );

        assert_eq!(api_req.path_params.len(), 4);
        assert_eq!(
            api_req.path_params.get("single"),
            Some(&vec!["one".to_string()])
        );
        assert_eq!(
            api_req.path_params.get("multiple"),
            Some(&vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string()
            ])
        );
        assert_eq!(api_req.path_params.get("empty"), Some(&vec![]));
        assert_eq!(
            api_req.path_params.get("special"),
            Some(&vec![
                "value@#$".to_string(),
                "中文".to_string(),
                "🚀emoji".to_string()
            ])
        );
    }

    #[test]
    fn test_api_request_binary_data_handling() {
        let binary_data = vec![0, 1, 2, 3, 4, 255, 254, 253];
        let api_req = ApiRequest {
            body: binary_data.clone(),
            file: binary_data.clone(),
            ..Default::default()
        };

        assert_eq!(api_req.body, binary_data);
        assert_eq!(api_req.file, binary_data);
    }

    #[test]
    fn test_api_request_memory_efficiency() {
        // Test creating many ApiRequest instances
        let requests: Vec<ApiRequest> = (0..100)
            .map(|i| ApiRequest {
                api_path: format!("/api/path/{}", i),
                body: format!("body_{}", i).into_bytes(),
                ..Default::default()
            })
            .collect();

        assert_eq!(requests.len(), 100);

        for (i, req) in requests.iter().enumerate() {
            assert_eq!(req.api_path, format!("/api/path/{}", i));
            assert_eq!(req.body, format!("body_{}", i).into_bytes());
        }
    }

    #[test]
    fn test_api_request_field_independence() {
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/test".to_string(),
            body: b"test body".to_vec(),
            ..Default::default()
        };
        api_req.query_params.insert("test", "value".to_string());
        api_req
            .path_params
            .insert("id".to_string(), vec!["123".to_string()]);
        api_req
            .supported_access_token_types
            .push(AccessTokenType::User);
        api_req.file = b"file content".to_vec();

        // Verify all fields are set correctly
        assert_eq!(api_req.http_method, Method::POST);
        assert_eq!(api_req.api_path, "/test");
        assert_eq!(api_req.body, b"test body");
        assert_eq!(api_req.query_params.len(), 1);
        assert_eq!(api_req.path_params.len(), 1);
        assert_eq!(api_req.supported_access_token_types.len(), 1);
        assert_eq!(api_req.file, b"file content");
    }
}
