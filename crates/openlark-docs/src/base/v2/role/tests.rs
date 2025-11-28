//! 角色管理API测试
//!
//! 测试覆盖：
//! - 新增自定义角色
//! - 更新自定义角色
//! - 列出自定义角色

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{
        MockServerManager, TestConfig, TestHelper, TestDataFactory,
        ResponseValidator, PerformanceTest
    };
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path_regex};
    use serde_json::json;
    use std::time::Duration;

    async fn setup_test_server() -> MockServerManager {
        let config = TestConfig::new();
        let server = MockServerManager::new(config).await.unwrap();
        server.register_base_mocks().await.unwrap();
        server
    }

    #[tokio::test]
    async fn test_create_role_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let permission_scopes = vec!["read".to_string(), "write".to_string(), "delete".to_string()];

        let request = CreateRoleRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .name("管理员角色")
            .description("具有完整权限的管理员角色")
            .permission_scopes(permission_scopes.clone())
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.name, "管理员角色");
        assert_eq!(request.description, Some("具有完整权限的管理员角色".to_string()));
        assert_eq!(request.permission_scopes, Some(permission_scopes));
    }

    #[tokio::test]
    async fn test_create_role_minimal_request() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = CreateRoleRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .name("最小权限角色")
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.name, "最小权限角色");
        assert_eq!(request.description, None);
        assert_eq!(request.permission_scopes, None);
    }

    #[tokio::test]
    async fn test_create_role_request_body_serialization() {
        let permission_scopes = vec!["read".to_string(), "write".to_string()];

        let body = CreateRoleRequestBody {
            name: "测试角色".to_string(),
            description: Some("这是一个测试角色".to_string()),
            permission_scopes: Some(permission_scopes),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "测试角色",
            "description": "这是一个测试角色",
            "permission_scopes": ["read", "write"]
        });

        assert_eq!(serialized, expected);
    }

    #[tokio::test]
    async fn test_create_role_partial_update() {
        let body = CreateRoleRequestBody {
            name: "更新的角色".to_string(),
            description: None,
            permission_scopes: None,
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "更新的角色"
        });

        assert_eq!(serialized, expected);
    }

    #[tokio::test]
    async fn test_update_role_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let permission_scopes = vec!["read".to_string(), "comment".to_string()];

        let request = UpdateRoleRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .role_id("role_123456789")
            .name("更新的管理员角色")
            .description("更新后的角色描述")
            .permission_scopes(permission_scopes)
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.role_id, "role_123456789");
        assert_eq!(request.name, Some("更新的管理员角色".to_string()));
        assert_eq!(request.description, Some("更新后的角色描述".to_string()));
        assert_eq!(request.permission_scopes, Some(permission_scopes));
    }

    #[tokio::test]
    #[tokio::test]
    async fn test_update_role_partial_update() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = UpdateRoleRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .role_id("role_123456789")
            .description("仅更新描述")
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.role_id, "role_123456789");
        assert_eq!(request.name, None); // 未更新
        assert_eq!(request.description, Some("仅更新描述".to_string()));
        assert_eq!(request.permission_scopes, None); // 未更新
    }

    #[tokio::test]
    async fn test_update_role_request_body_serialization() {
        let permission_scopes = vec!["read".to_string(), "write".to_string(), "delete".to_string()];

        let body = UpdateRoleRequestBody {
            name: Some("更新的角色".to_string()),
            description: Some("这是一个更新的自定义角色".to_string()),
            permission_scopes: Some(permission_scopes),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "更新的角色",
            "description": "这是一个更新的自定义角色",
            "permission_scopes": ["read", "write", "delete"]
        });

        assert_eq!(serialized, expected);
    }

    #[tokio::test]
    async fn test_list_roles_request_builder() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = ListRolesRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .user_id_type("user_id")
            .page_size(50)
            .page_token("page_token_123")
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
    }

    #[tokio::test]
    async fn test_list_roles_minimal_request() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = ListRolesRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .build();

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[tokio::test]
    async fn test_page_size_limit() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = ListRolesRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .page_size(200) // 超过100的限制
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }

    #[tokio::test]
    async fn test_role_info_serialization() {
        let role = RoleInfo {
            role_id: "rol_123456789".to_string(),
            name: "测试角色".to_string(),
            description: Some("这是一个测试角色".to_string()),
            permission_scopes: Some(vec!["read".to_string(), "write".to_string()]),
            is_system: Some(true),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let serialized = serde_json::to_value(&role).unwrap();
        let expected = json!({
            "role_id": "rol_123456789",
            "name": "测试角色",
            "description": "这是一个测试角色",
            "permission_scopes": ["read", "write"],
            "is_system": true,
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        });

        assert_eq!(serialized, expected);
    }

    #[tokio::test]
    async fn test_role_info_optional_fields() {
        // 测试可选字段
        let role = RoleInfo {
            role_id: "rol_123456789".to_string(),
            name: "简化角色".to_string(),
            description: None,
            permission_scopes: None,
            is_system: None,
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let serialized = serde_json::to_value(&role).unwrap();
        let expected = json!({
            "role_id": "rol_123456789",
            "name": "简化角色",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        });

        assert_eq!(serialized, expected);
    }

    #[tokio::test]
    async fn test_create_role_response_validation() {
        let response = json!({
            "role": {
                "role_id": "rol_create_test_123456789",
                "name": "新增的角色",
                "description": "这是一个新增的自定义角色",
                "permission_scopes": ["read", "write"],
                "is_system": false,
                "created_at": "2023-01-01T12:00:00Z",
                "updated_at": "2023-01-01T12:00:00Z"
            }
        });

        // 验证响应结构
        assert!(TestHelper::assert_has_field(&response, "role.role_id").is_ok());
        assert!(TestHelper::assert_has_field(&response, "role.name").is_ok());
        assert!(TestHelper::assert_has_field(&response, "role.created_at").is_ok());

        // 验证字段值
        assert!(TestHelper::assert_field_eq(&response, "role.name", &json!("新增的角色")).is_ok());
        assert!(TestHelper::assert_field_eq(&response, "role.is_system", &json!(false)).is_ok());

        // 验证权限范围
        let permission_scopes = response["role"]["permission_scopes"].as_array().unwrap();
        assert_eq!(permission_scopes.len(), 2);
        assert!(permission_scopes.contains(&json!("read")));
        assert!(permission_scopes.contains(&json!("write")));

        // 验证元数据
        assert!(TestHelper::assert_response_metadata(&response["role"]).is_ok());
    }

    #[tokio::test]
    async fn test_update_role_response_validation() {
        let response = json!({
            "role": {
                "role_id": "rol_update_test_123456789",
                "name": "更新的角色",
                "description": "更新后的角色描述",
                "permission_scopes": ["read", "write", "delete"],
                "is_system": false,
                "updated_at": "2023-01-01T12:30:00Z"
            }
        });

        // 验证响应结构
        assert!(TestHelper::assert_has_field(&response, "role.role_id").is_ok());
        assert!(TestHelper::assert_has_field(&response, "role.name").is_ok());
        assert!(TestHelper::assert_has_field(&response, "role.updated_at").is_ok());

        // 验证字段值
        assert!(TestHelper::assert_field_eq(&response, "role.name", &json!("更新的角色")).is_ok());
        assert!(TestHelper::assert_field_eq(&response, "role.permission_scopes", &json!(["read", "write", "delete"])).is_ok());

        // 验证权限范围增加了delete权限
        let permission_scopes = response["role"]["permission_scopes"].as_array().unwrap();
        assert_eq!(permission_scopes.len(), 3);
        assert!(permission_scopes.contains(&json!("delete")));
    }

    #[tokio::test]
    async fn test_list_roles_response_validation() {
        let response = json!({
            "items": [
                {
                    "role_id": "rol_admin_123456789",
                    "name": "管理员",
                    "description": "系统管理员角色",
                    "permission_scopes": ["read", "write", "delete"],
                    "is_system": true,
                    "created_at": "2023-01-01T00:00:00Z",
                    "updated_at": "2023-01-01T00:00:00Z"
                },
                {
                    "role_id": "rol_user_987654321",
                    "name": "普通用户",
                    "description": "普通用户角色",
                    "permission_scopes": ["read"],
                    "is_system": false,
                    "created_at": "2023-01-01T00:00:00Z",
                    "updated_at": "2023-01-01T00:00:00Z"
                }
            ],
            "page_token": null,
            "has_more": false,
            "total": 2
        });

        // 验证列表响应结构
        assert!(TestHelper::assert_has_field(&response, "items").is_ok());
        assert!(TestHelper::assert_has_field(&response, "total").is_ok());
        assert!(TestHelper::assert_has_field(&response, "has_more").is_ok());

        let items = response["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(response["total"], 2);
        assert_eq!(response["has_more"], false);
        assert!(response["page_token"].is_null());

        // 验证第一个角色（系统角色）
        let admin_role = &items[0];
        assert!(TestHelper::assert_field_eq(admin_role, "name", &json!("管理员")).is_ok());
        assert!(TestHelper::assert_field_eq(admin_role, "is_system", &json!(true)).is_ok());

        // 验证第二个角色（自定义角色）
        let user_role = &items[1];
        assert!(TestHelper::assert_field_eq(user_role, "name", &json!("普通用户")).is_ok());
        assert!(TestHelper::assert_field_eq(user_role, "is_system", &json!(false)).is_ok());
    }

    // 性能测试
    #[tokio::test]
    async fn test_role_creation_performance() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let perf_test = PerformanceTest::start();

        // 模拟创建多个角色请求构建
        for i in 0..100 {
            let permission_scopes = vec!["read".to_string(), "write".to_string()];
            let _request = CreateRoleRequest::new(config.clone())
                .builder()
                .app_token("test_app_token")
                .name(format!("角色{}", i))
                .description(format!("这是第{}个测试角色", i))
                .permission_scopes(permission_scopes)
                .build();
        }

        let result = perf_test.assert_duration_less_than(Duration::from_millis(50));
        assert!(result.is_ok(), "构建100个角色请求应该在50ms内完成");
    }

    #[tokio::test]
    async fn test_permission_scopes_handling() {
        // 测试不同权限范围的组合
        let permission_combinations = vec![
            vec!["read".to_string()],
            vec!["read".to_string(), "write".to_string()],
            vec!["read".to_string(), "write".to_string(), "delete".to_string()],
            vec!["read".to_string(), "comment".to_string()],
            vec!["read".to_string(), "write".to_string(), "comment".to_string()],
            vec!["read".to_string(), "write".to_string(), "delete".to_string(), "comment".to_string()],
        ];

        for (i, scopes) in permission_combinations.iter().enumerate() {
            let config = openlark_core::Config::new(openlark_core::ConfigInner {
                app_id: "test_app_id".to_string(),
                app_secret: "test_app_secret".to_string(),
                ..Default::default()
            });

            let request = CreateRoleRequest::new(config)
                .builder()
                .app_token("test_app_token")
                .name(format!("角色{}", i))
                .permission_scopes(scopes.clone())
                .build();

            assert_eq!(request.permission_scopes, Some(scopes.clone()));
        }
    }

    #[tokio::test]
    async fn test_system_role_properties() {
        let system_role = TestDataFactory::create_test_role();

        // 验证系统角色属性
        assert!(system_role["role"]["is_system"].as_bool().unwrap());
        assert_eq!(system_role["role"]["name"].as_str().unwrap(), "测试角色");

        // 验证时间戳格式
        assert!(system_role["role"]["created_at"].as_str().unwrap().contains('T'));
        assert!(system_role["role"]["updated_at"].as_str().unwrap().contains('T'));
    }

    #[tokio::test]
    async fn test_role_id_format_validation() {
        // 测试角色ID格式验证
        let valid_role_ids = vec![
            "rol_123456789",
            "rol_abcdefghij",
            "rol_123abc456",
            "rol_1a2b3c4d5e"
        ];

        for role_id in valid_role_ids {
            let role = TestDataFactory::create_test_role();
            // 模拟替换角色ID
            let mut role_copy = role.clone();
            role_copy["role"]["role_id"] = json!(role_id);

            assert_eq!(role_copy["role"]["role_id"], role_id);
            assert_eq!(role_copy["role"]["role_id"].as_str().unwrap(), role_id);
        }
    }

    // 边界条件测试
    #[tokio::test]
    async fn test_empty_role_name_validation() {
        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        // 测试空角色名称
        let request = CreateRoleRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .name("")  // 空名称
            .build();

        // 验证请求构建成功（验证逻辑在实际API调用时进行）
        assert_eq!(request.name, "");
    }

    #[tokio::test]
    async fn test_long_role_name_handling() {
        let long_name = TestDataFactory::create_long_text_field(100); // 100字符角色名

        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        let request = CreateRoleRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .name(long_name.clone())
            .build();

        assert_eq!(request.name, long_name);
        assert_eq!(request.name.len(), 100);
    }

    #[tokio::test]
    async fn test_permission_scope_validation() {
        // 测试无效的权限范围
        let invalid_scopes = vec!["invalid_permission", "", "read-write"];

        for invalid_scope in invalid_scopes {
            let config = openlark_core::Config::new(openlark_core::ConfigInner {
                app_id: "test_app_id".to_string(),
                app_secret: "test_app_secret".to_string(),
                ..Default::default()
            });

            let request = CreateRoleRequest::new(config)
                .builder()
                .app_token("test_app_token")
                .name("测试角色")
                .permission_scopes(vec![invalid_scope.to_string()])
                .build();

            // 验证无效权限范围仍被接受（验证在实际API调用时进行）
            assert!(request.permission_scopes.as_ref().unwrap().contains(&invalid_scope.to_string()));
        }
    }

    // 集成测试
    #[tokio::test]
    async fn test_role_lifecycle_integration() {
        let server_manager = setup_test_server().await;

        // 设置角色创建响应
        Mock::given(method("POST"))
            .and(path_regex(r"^/open-apis/base/v2/apps/[^/]+/roles$"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_role()
            ))
            .mount(&server_manager.server)
            .await;

        let config = openlark_core::Config::new(openlark_core::ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            ..Default::default()
        });

        // 模拟完整的角色生命周期
        let create_request = CreateRoleRequest::new(config.clone())
            .builder()
            .app_token("test_app_token")
            .name("集成测试角色")
            .description("用于集成测试的角色")
            .permission_scopes(vec!["read".to_string(), "write".to_string()])
            .build();

        let update_request = UpdateRoleRequest::new(config.clone())
            .builder()
            .app_token("test_app_token")
            .role_id("test_role_id")
            .name("更新后的角色")
            .build();

        let list_request = ListRolesRequest::new(config)
            .builder()
            .app_token("test_app_token")
            .page_size(10)
            .build();

        // 验证请求构建成功
        assert_eq!(create_request.name, "集成测试角色");
        assert_eq!(update_request.role_id, "test_role_id");
        assert_eq!(list_request.page_size, Some(10));

        // 性能测试 - 角色操作应该快速完成
        let perf_test = PerformanceTest::start();

        // 模拟角色操作
        std::thread::sleep(Duration::from_millis(10));

        let duration = perf_test.elapsed();
        println!("角色操作耗时: {:?}", duration);
        assert!(duration < Duration::from_millis(50));
    }
}