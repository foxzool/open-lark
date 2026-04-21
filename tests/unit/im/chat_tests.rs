//! 聊天服务单元测试
//!
//! 测试聊天列表获取、聊天信息管理等功能

use rstest::*;
use serde_json::json;
use wiremock::{
    matchers::{header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};
use open_lark::service::im::v1::chats::*;
use open_lark::core::config::Config;

/// 创建测试用的配置
fn create_test_config(base_url: &str) -> Config {
    Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(), 
        base_url: base_url.to_string(),
        enable_token_cache: false,
        ..Default::default()
    }
}

/// 创建测试用的聊天服务
fn create_test_chats_service(config: Config) -> ChatsService {
    ChatsService { config }
}

/// 列表聊天请求测试
#[cfg(test)]
mod list_chat_request_tests {
    use super::*;

    #[test]
    fn test_list_chat_request_builder_default() {
        let request = ListChatRequest::builder().build();
        assert!(request.api_req.query_params.is_empty());
    }

    #[test]
    fn test_list_chat_request_builder_with_user_id_type() {
        let request = ListChatRequest::builder()
            .user_id_type("open_id")
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("user_id_type"),
            Some(&"open_id".to_string())
        );
    }

    #[test]
    fn test_list_chat_request_builder_with_sort_type() {
        let request = ListChatRequest::builder()
            .sort_type("ByCreateTimeAsc")
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("sort_type"),
            Some(&"ByCreateTimeAsc".to_string())
        );
    }

    #[test]
    fn test_list_chat_request_builder_with_pagination() {
        let request = ListChatRequest::builder()
            .page_token("token_12345")
            .page_size(50)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("page_token"),
            Some(&"token_12345".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("page_size"),
            Some(&"50".to_string())
        );
    }

    #[test]
    fn test_list_chat_request_builder_complete() {
        let request = ListChatRequest::builder()
            .user_id_type("open_id")
            .sort_type("ByCreateTimeDesc")
            .page_token("dmJCRHhpd3JRbGV1VEVNRFFyTitRWDY5ZFkybmYrMEUwMUFYT0VMMWdENEtuYUhsNUxGMDIwemtvdE5ORjBNQQ==")
            .page_size(100)
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("user_id_type"), Some(&"open_id".to_string()));
        assert_eq!(params.get("sort_type"), Some(&"ByCreateTimeDesc".to_string()));
        assert_eq!(params.get("page_token"), Some(&"dmJCRHhpd3JRbGV1VEVNRFFyTitRWDY5ZFkybmYrMEUwMUFYT0VMMWdENEtuYUhsNUxGMDIwemtvdE5ORjBNQQ==".to_string()));
        assert_eq!(params.get("page_size"), Some(&"100".to_string()));
    }

    #[rstest]
    #[case("user_id")]
    #[case("union_id")]
    #[case("open_id")]
    fn test_list_chat_request_different_user_id_types(#[case] user_id_type: &str) {
        let request = ListChatRequest::builder()
            .user_id_type(user_id_type)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("user_id_type"),
            Some(&user_id_type.to_string())
        );
    }

    #[rstest]
    #[case("ByCreateTimeAsc")]
    #[case("ByCreateTimeDesc")]
    #[case("ByActiveTime")]
    fn test_list_chat_request_different_sort_types(#[case] sort_type: &str) {
        let request = ListChatRequest::builder()
            .sort_type(sort_type)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("sort_type"),
            Some(&sort_type.to_string())
        );
    }

    #[rstest]
    #[case(1)]
    #[case(20)]
    #[case(50)]
    #[case(100)]
    #[case(200)]
    fn test_list_chat_request_different_page_sizes(#[case] page_size: i32) {
        let request = ListChatRequest::builder()
            .page_size(page_size)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("page_size"),
            Some(&page_size.to_string())
        );
    }

    #[test]
    fn test_list_chat_request_builder_chaining() {
        let request = ListChatRequest::builder()
            .user_id_type("open_id")
            .sort_type("ByCreateTimeAsc")
            .page_size(25)
            .page_token("chaining_test_token")
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.len(), 4);
        assert!(params.contains_key("user_id_type"));
        assert!(params.contains_key("sort_type"));
        assert!(params.contains_key("page_size"));
        assert!(params.contains_key("page_token"));
    }

    #[test]
    fn test_list_chat_request_builder_overwrite_values() {
        let request = ListChatRequest::builder()
            .user_id_type("user_id")
            .user_id_type("open_id")  // 覆盖前一个值
            .page_size(50)
            .page_size(100)  // 覆盖前一个值
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("user_id_type"), Some(&"open_id".to_string()));
        assert_eq!(params.get("page_size"), Some(&"100".to_string()));
    }

    #[test]
    fn test_list_chat_request_empty_values() {
        let request = ListChatRequest::builder()
            .user_id_type("")
            .sort_type("")
            .page_token("")
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("user_id_type"), Some(&"".to_string()));
        assert_eq!(params.get("sort_type"), Some(&"".to_string()));
        assert_eq!(params.get("page_token"), Some(&"".to_string()));
    }

    #[test]
    fn test_list_chat_request_zero_and_negative_page_size() {
        let request_zero = ListChatRequest::builder()
            .page_size(0)
            .build();
            
        let request_negative = ListChatRequest::builder()
            .page_size(-10)
            .build();
            
        assert_eq!(
            request_zero.api_req.query_params.get("page_size"),
            Some(&"0".to_string())
        );
        assert_eq!(
            request_negative.api_req.query_params.get("page_size"),
            Some(&"-10".to_string())
        );
    }

    #[test]
    fn test_list_chat_request_extreme_page_size() {
        let request = ListChatRequest::builder()
            .page_size(i32::MAX)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("page_size"),
            Some(&i32::MAX.to_string())
        );
    }

    #[test]
    fn test_list_chat_request_special_characters() {
        let special_token = "token_with_特殊字符_😀_and_symbols!@#$%";
        let request = ListChatRequest::builder()
            .page_token(special_token)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("page_token"),
            Some(&special_token.to_string())
        );
    }

    #[test]
    fn test_list_chat_request_very_long_token() {
        let long_token = "token_".to_owned() + &"a".repeat(10000);
        let request = ListChatRequest::builder()
            .page_token(&long_token)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("page_token"),
            Some(&long_token)
        );
    }
}

/// 聊天列表响应数据测试
#[cfg(test)]
mod list_chat_response_tests {
    use super::*;

    #[test]
    fn test_list_chat_resp_data_serialization() {
        let chat = ListChat {
            chat_id: "oc_test_chat_123".to_string(),
            avatar: "https://example.com/avatar.jpg".to_string(),
            name: "测试群组".to_string(),
            description: "这是一个测试群组".to_string(),
            owner_id: "ou_owner_123".to_string(),
            owner_id_type: "open_id".to_string(),
            external: false,
            tenant_key: "tenant_123".to_string(),
            chat_status: "active".to_string(),
        };

        let resp_data = ListChatRespData {
            items: vec![chat],
            page_token: "next_page_token_123".to_string(),
            has_more: true,
        };

        let serialized = serde_json::to_string(&resp_data).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["has_more"], true);
        assert_eq!(parsed["page_token"], "next_page_token_123");
        assert_eq!(parsed["items"].as_array().unwrap().len(), 1);
        assert_eq!(parsed["items"][0]["chat_id"], "oc_test_chat_123");
        assert_eq!(parsed["items"][0]["name"], "测试群组");
        assert_eq!(parsed["items"][0]["external"], false);
    }

    #[test]
    fn test_list_chat_resp_data_deserialization() {
        let json_data = json!({
            "items": [{
                "chat_id": "oc_deser_test",
                "avatar": "https://example.com/deser_avatar.jpg",
                "name": "反序列化测试群",
                "description": "用于测试反序列化",
                "owner_id": "ou_deser_owner",
                "owner_id_type": "open_id",
                "external": true,
                "tenant_key": "tenant_deser",
                "chat_status": "active"
            }],
            "page_token": "deser_token",
            "has_more": false
        });

        let resp_data: ListChatRespData = serde_json::from_value(json_data).unwrap();
        
        assert_eq!(resp_data.has_more, false);
        assert_eq!(resp_data.page_token, "deser_token");
        assert_eq!(resp_data.items.len(), 1);
        
        let chat = &resp_data.items[0];
        assert_eq!(chat.chat_id, "oc_deser_test");
        assert_eq!(chat.name, "反序列化测试群");
        assert_eq!(chat.external, true);
        assert_eq!(chat.chat_status, "active");
    }

    #[test]
    fn test_list_chat_resp_data_empty_items() {
        let resp_data = ListChatRespData {
            items: vec![],
            page_token: "".to_string(),
            has_more: false,
        };

        let serialized = serde_json::to_string(&resp_data).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["items"].as_array().unwrap().len(), 0);
        assert_eq!(parsed["has_more"], false);
        assert_eq!(parsed["page_token"], "");
    }

    #[test]
    fn test_list_chat_resp_data_multiple_items() {
        let chats = vec![
            ListChat {
                chat_id: "oc_multi_1".to_string(),
                avatar: "avatar1.jpg".to_string(),
                name: "群组1".to_string(),
                description: "第一个群组".to_string(),
                owner_id: "ou_owner_1".to_string(),
                owner_id_type: "open_id".to_string(),
                external: false,
                tenant_key: "tenant_1".to_string(),
                chat_status: "active".to_string(),
            },
            ListChat {
                chat_id: "oc_multi_2".to_string(),
                avatar: "avatar2.jpg".to_string(),
                name: "群组2".to_string(),
                description: "第二个群组".to_string(),
                owner_id: "ou_owner_2".to_string(),
                owner_id_type: "open_id".to_string(),
                external: true,
                tenant_key: "tenant_2".to_string(),
                chat_status: "archived".to_string(),
            },
        ];

        let resp_data = ListChatRespData {
            items: chats,
            page_token: "multi_token".to_string(),
            has_more: true,
        };

        let serialized = serde_json::to_string(&resp_data).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["items"].as_array().unwrap().len(), 2);
        assert_eq!(parsed["items"][0]["name"], "群组1");
        assert_eq!(parsed["items"][1]["name"], "群组2");
        assert_eq!(parsed["items"][0]["external"], false);
        assert_eq!(parsed["items"][1]["external"], true);
    }
}

/// 聊天信息测试
#[cfg(test)]
mod list_chat_tests {
    use super::*;

    #[test]
    fn test_list_chat_serialization() {
        let chat = ListChat {
            chat_id: "oc_serialization_test".to_string(),
            avatar: "https://avatar.example.com/test.png".to_string(),
            name: "序列化测试群".to_string(),
            description: "用于测试序列化功能的群组".to_string(),
            owner_id: "ou_serialization_owner".to_string(),
            owner_id_type: "open_id".to_string(),
            external: false,
            tenant_key: "tenant_serialization".to_string(),
            chat_status: "active".to_string(),
        };

        let serialized = serde_json::to_string(&chat).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["chat_id"], "oc_serialization_test");
        assert_eq!(parsed["avatar"], "https://avatar.example.com/test.png");
        assert_eq!(parsed["name"], "序列化测试群");
        assert_eq!(parsed["description"], "用于测试序列化功能的群组");
        assert_eq!(parsed["owner_id"], "ou_serialization_owner");
        assert_eq!(parsed["owner_id_type"], "open_id");
        assert_eq!(parsed["external"], false);
        assert_eq!(parsed["tenant_key"], "tenant_serialization");
        assert_eq!(parsed["chat_status"], "active");
    }

    #[test]
    fn test_list_chat_deserialization() {
        let json_data = json!({
            "chat_id": "oc_deserialization_test",
            "avatar": "https://avatar.example.com/deser.png",
            "name": "反序列化测试群",
            "description": "用于测试反序列化功能的群组",
            "owner_id": "ou_deserialization_owner",
            "owner_id_type": "union_id",
            "external": true,
            "tenant_key": "tenant_deserialization",
            "chat_status": "archived"
        });

        let chat: ListChat = serde_json::from_value(json_data).unwrap();
        
        assert_eq!(chat.chat_id, "oc_deserialization_test");
        assert_eq!(chat.avatar, "https://avatar.example.com/deser.png");
        assert_eq!(chat.name, "反序列化测试群");
        assert_eq!(chat.description, "用于测试反序列化功能的群组");
        assert_eq!(chat.owner_id, "ou_deserialization_owner");
        assert_eq!(chat.owner_id_type, "union_id");
        assert_eq!(chat.external, true);
        assert_eq!(chat.tenant_key, "tenant_deserialization");
        assert_eq!(chat.chat_status, "archived");
    }

    #[test]
    fn test_list_chat_external_true() {
        let chat = ListChat {
            chat_id: "oc_external_true".to_string(),
            avatar: "".to_string(),
            name: "外部群组".to_string(),
            description: "这是一个外部群组".to_string(),
            owner_id: "ou_external_owner".to_string(),
            owner_id_type: "user_id".to_string(),
            external: true,
            tenant_key: "external_tenant".to_string(),
            chat_status: "active".to_string(),
        };

        let serialized = serde_json::to_string(&chat).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["external"], true);
        assert_eq!(parsed["name"], "外部群组");
    }

    #[test]
    fn test_list_chat_empty_fields() {
        let chat = ListChat {
            chat_id: "oc_empty_test".to_string(),
            avatar: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            owner_id: "".to_string(),
            owner_id_type: "".to_string(),
            external: false,
            tenant_key: "".to_string(),
            chat_status: "".to_string(),
        };

        let serialized = serde_json::to_string(&chat).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["avatar"], "");
        assert_eq!(parsed["name"], "");
        assert_eq!(parsed["description"], "");
        assert_eq!(parsed["owner_id"], "");
        assert_eq!(parsed["owner_id_type"], "");
        assert_eq!(parsed["tenant_key"], "");
        assert_eq!(parsed["chat_status"], "");
    }

    #[rstest]
    #[case("active")]
    #[case("archived")]
    #[case("disbanded")]
    #[case("unknown")]
    fn test_list_chat_different_statuses(#[case] status: &str) {
        let chat = ListChat {
            chat_id: format!("oc_status_{}", status),
            avatar: "avatar.jpg".to_string(),
            name: format!("{}状态群组", status),
            description: format!("状态为{}的群组", status),
            owner_id: "ou_status_owner".to_string(),
            owner_id_type: "open_id".to_string(),
            external: false,
            tenant_key: "tenant_status".to_string(),
            chat_status: status.to_string(),
        };

        let serialized = serde_json::to_string(&chat).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["chat_status"], status);
        assert_eq!(parsed["chat_id"], format!("oc_status_{}", status));
    }

    #[rstest]
    #[case("open_id")]
    #[case("user_id")]
    #[case("union_id")]
    fn test_list_chat_different_owner_id_types(#[case] id_type: &str) {
        let chat = ListChat {
            chat_id: format!("oc_id_type_{}", id_type),
            avatar: "avatar.jpg".to_string(),
            name: format!("群主ID类型{}", id_type),
            description: "测试不同群主ID类型".to_string(),
            owner_id: format!("owner_{}", id_type),
            owner_id_type: id_type.to_string(),
            external: false,
            tenant_key: "tenant_id_type".to_string(),
            chat_status: "active".to_string(),
        };

        let serialized = serde_json::to_string(&chat).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["owner_id_type"], id_type);
        assert_eq!(parsed["owner_id"], format!("owner_{}", id_type));
    }

    #[test]
    fn test_list_chat_special_characters() {
        let chat = ListChat {
            chat_id: "oc_special_chars".to_string(),
            avatar: "https://example.com/特殊字符头像😀.jpg".to_string(),
            name: "特殊字符群组 😀🎉 \"引号\" '单引号'".to_string(),
            description: "包含特殊字符的群组描述\n换行符\t制表符".to_string(),
            owner_id: "ou_特殊_owner".to_string(),
            owner_id_type: "open_id".to_string(),
            external: false,
            tenant_key: "tenant_特殊_key".to_string(),
            chat_status: "active".to_string(),
        };

        let serialized = serde_json::to_string(&chat).unwrap();
        let deserialized: ListChat = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(deserialized.name, "特殊字符群组 😀🎉 \"引号\" '单引号'");
        assert_eq!(deserialized.description, "包含特殊字符的群组描述\n换行符\t制表符");
        assert_eq!(deserialized.avatar, "https://example.com/特殊字符头像😀.jpg");
    }

    #[test]
    fn test_list_chat_very_long_fields() {
        let long_name = "群组名称_".to_owned() + &"长".repeat(1000);
        let long_description = "群组描述_".to_owned() + &"描".repeat(5000);
        
        let chat = ListChat {
            chat_id: "oc_long_fields".to_string(),
            avatar: "avatar.jpg".to_string(),
            name: long_name.clone(),
            description: long_description.clone(),
            owner_id: "ou_long_owner".to_string(),
            owner_id_type: "open_id".to_string(),
            external: false,
            tenant_key: "tenant_long".to_string(),
            chat_status: "active".to_string(),
        };

        let serialized = serde_json::to_string(&chat).unwrap();
        let deserialized: ListChat = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(deserialized.name, long_name);
        assert_eq!(deserialized.description, long_description);
        assert!(deserialized.name.len() > 1000);
        assert!(deserialized.description.len() > 5000);
    }
}

/// 聊天迭代器测试
#[cfg(test)]
mod list_chat_iterator_tests {
    use super::*;

    #[test]
    fn test_list_chat_iterator_creation() {
        let config = create_test_config("https://test.example.com");
        let service = create_test_chats_service(config);
        
        let request = ListChatRequest::builder()
            .user_id_type("open_id")
            .page_size(50)
            .build();

        let mut iterator = service.list_iter(request, None);
        
        // 验证迭代器初始状态
        assert!(iterator.has_more);
    }

    #[test]
    fn test_list_chat_iterator_initial_state() {
        let config = create_test_config("https://iterator.test.com");
        let service = create_test_chats_service(config);
        
        let request = ListChatRequest::builder().build();
        let iterator = service.list_iter(request, None);
        
        assert!(iterator.has_more);
        // 验证请求参数仍然存在
        assert!(iterator.request.api_req.query_params.is_empty());
    }

    // 注意：next() 方法需要实际的HTTP请求，所以我们主要测试迭代器的创建和初始状态
    // 实际的网络请求测试在集成测试中进行
}

/// Mock HTTP 服务器测试
#[cfg(test)]
mod http_mock_tests {
    use super::*;

    #[tokio::test]
    async fn test_list_chats_success_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/chats"))
            .and(query_param("user_id_type", "open_id"))
            .and(query_param("page_size", "50"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "chat_id": "oc_mock_test_123",
                        "avatar": "https://mock.example.com/avatar.jpg",
                        "name": "Mock测试群组",
                        "description": "这是一个Mock测试群组",
                        "owner_id": "ou_mock_owner",
                        "owner_id_type": "open_id",
                        "external": false,
                        "tenant_key": "tenant_mock",
                        "chat_status": "active"
                    }],
                    "page_token": "mock_page_token",
                    "has_more": true
                }
            })))
            .mount(&mock_server)
            .await;

        let request = ListChatRequest::builder()
            .user_id_type("open_id")
            .page_size(50)
            .build();

        // 验证请求参数
        let params = &request.api_req.query_params;
        assert_eq!(params.get("user_id_type"), Some(&"open_id".to_string()));
        assert_eq!(params.get("page_size"), Some(&"50".to_string()));
    }

    #[tokio::test]
    async fn test_list_chats_empty_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/chats"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [],
                    "page_token": "",
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        // 使用reqwest客户端直接测试Mock服务器
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/open-apis/im/v1/chats", mock_server.uri()))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body: serde_json::Value = response.json().await.unwrap();
        assert_eq!(body["code"], 0);
        assert_eq!(body["data"]["items"].as_array().unwrap().len(), 0);
        assert_eq!(body["data"]["has_more"], false);
    }

    #[tokio::test]
    async fn test_list_chats_pagination_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/chats"))
            .and(query_param("page_token", "page_2_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "chat_id": "oc_page_2_chat",
                        "avatar": "https://page2.example.com/avatar.jpg",
                        "name": "第二页群组",
                        "description": "分页测试群组",
                        "owner_id": "ou_page_2_owner",
                        "owner_id_type": "open_id",
                        "external": true,
                        "tenant_key": "tenant_page_2",
                        "chat_status": "active"
                    }],
                    "page_token": "",
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/open-apis/im/v1/chats", mock_server.uri()))
            .query(&[("page_token", "page_2_token")])
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body: serde_json::Value = response.json().await.unwrap();
        assert_eq!(body["data"]["items"][0]["name"], "第二页群组");
        assert_eq!(body["data"]["has_more"], false);
    }

    #[tokio::test]
    async fn test_list_chats_error_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/chats"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({
                "code": 99991663,
                "msg": "permission denied",
                "data": {}
            })))
            .mount(&mock_server)
            .await;

        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/open-apis/im/v1/chats", mock_server.uri()))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 403);
        let body: serde_json::Value = response.json().await.unwrap();
        assert_eq!(body["code"], 99991663);
        assert_eq!(body["msg"], "permission denied");
    }
}

/// 边界条件和错误处理测试
#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_list_chat_request_extreme_page_sizes() {
        let max_request = ListChatRequest::builder()
            .page_size(i32::MAX)
            .build();
            
        let min_request = ListChatRequest::builder()
            .page_size(i32::MIN)
            .build();
            
        assert_eq!(
            max_request.api_req.query_params.get("page_size"),
            Some(&i32::MAX.to_string())
        );
        assert_eq!(
            min_request.api_req.query_params.get("page_size"),
            Some(&i32::MIN.to_string())
        );
    }

    #[test]
    fn test_list_chat_invalid_json_deserialization() {
        // 测试缺少必需字段的JSON
        let incomplete_json = json!({
            "chat_id": "oc_incomplete"
            // 缺少其他必需字段
        });

        let result: Result<ListChat, _> = serde_json::from_value(incomplete_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_chat_resp_data_invalid_json() {
        let invalid_json = json!({
            "items": "not_an_array",  // 应该是数组但给了字符串
            "page_token": "valid_token",
            "has_more": true
        });

        let result: Result<ListChatRespData, _> = serde_json::from_value(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_chat_null_values_in_json() {
        let json_with_nulls = json!({
            "chat_id": null,
            "avatar": null,
            "name": null,
            "description": null,
            "owner_id": null,
            "owner_id_type": null,
            "external": null,
            "tenant_key": null,
            "chat_status": null
        });

        // 由于字段不是Option类型，null值会导致反序列化失败
        let result: Result<ListChat, _> = serde_json::from_value(json_with_nulls);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_chat_request_unicode_values() {
        let unicode_token = "page_token_with_unicode_字符_😀_end";
        let request = ListChatRequest::builder()
            .page_token(unicode_token)
            .sort_type("ByCreateTimeAsc")
            .build();

        let params = &request.api_req.query_params;
        assert_eq!(params.get("page_token"), Some(&unicode_token.to_string()));
        
        // 验证可以正确处理Unicode字符
        assert!(unicode_token.chars().any(|c| c as u32 > 127));
    }

    #[test]
    fn test_list_chat_resp_data_max_items() {
        // 测试大量聊天项目
        let mut chats = Vec::new();
        for i in 0..1000 {
            chats.push(ListChat {
                chat_id: format!("oc_max_test_{}", i),
                avatar: format!("avatar_{}.jpg", i),
                name: format!("群组_{}", i),
                description: format!("描述_{}", i),
                owner_id: format!("ou_owner_{}", i),
                owner_id_type: "open_id".to_string(),
                external: i % 2 == 0,
                tenant_key: format!("tenant_{}", i),
                chat_status: if i % 3 == 0 { "active" } else { "archived" }.to_string(),
            });
        }

        let resp_data = ListChatRespData {
            items: chats,
            page_token: "max_items_token".to_string(),
            has_more: false,
        };

        // 验证可以正确序列化和反序列化大量数据
        let serialized = serde_json::to_string(&resp_data).unwrap();
        let deserialized: ListChatRespData = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(deserialized.items.len(), 1000);
        assert_eq!(deserialized.items[0].chat_id, "oc_max_test_0");
        assert_eq!(deserialized.items[999].chat_id, "oc_max_test_999");
    }
}

/// 性能测试
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_list_chat_request_build_performance() {
        let start = Instant::now();
        
        for i in 0..1000 {
            let _request = ListChatRequest::builder()
                .user_id_type("open_id")
                .sort_type("ByCreateTimeAsc")
                .page_size(50 + i % 100)
                .page_token(&format!("token_{}", i))
                .build();
        }
        
        let duration = start.elapsed();
        // 1000次构建应该在50ms内完成
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_list_chat_serialization_performance() {
        let chat = ListChat {
            chat_id: "oc_performance_test".to_string(),
            avatar: "https://performance.example.com/avatar.jpg".to_string(),
            name: "性能测试群组".to_string(),
            description: "用于测试序列化性能的群组".to_string(),
            owner_id: "ou_performance_owner".to_string(),
            owner_id_type: "open_id".to_string(),
            external: false,
            tenant_key: "tenant_performance".to_string(),
            chat_status: "active".to_string(),
        };

        let start = Instant::now();
        
        for _ in 0..1000 {
            let _serialized = serde_json::to_string(&chat).unwrap();
        }
        
        let duration = start.elapsed();
        // 1000次序列化应该在30ms内完成
        assert!(duration.as_millis() < 30);
    }

    #[test]
    fn test_list_chat_resp_data_large_dataset_performance() {
        let mut chats = Vec::new();
        for i in 0..100 {
            chats.push(ListChat {
                chat_id: format!("oc_perf_{}", i),
                avatar: format!("https://example.com/avatar_{}.jpg", i),
                name: format!("性能测试群组_{}", i),
                description: format!("用于性能测试的第{}个群组", i),
                owner_id: format!("ou_owner_{}", i),
                owner_id_type: "open_id".to_string(),
                external: i % 2 == 0,
                tenant_key: format!("tenant_{}", i),
                chat_status: "active".to_string(),
            });
        }

        let resp_data = ListChatRespData {
            items: chats,
            page_token: "perf_token".to_string(),
            has_more: true,
        };

        let start = Instant::now();
        
        for _ in 0..100 {
            let serialized = serde_json::to_string(&resp_data).unwrap();
            let _deserialized: ListChatRespData = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        }
        
        let duration = start.elapsed();
        // 100次大数据集的序列化/反序列化应该在500ms内完成
        assert!(duration.as_millis() < 500);
    }
}

/// 并发安全测试
#[cfg(test)]
mod concurrency_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_request_building() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    for j in 0..50 {
                        let _request = ListChatRequest::builder()
                            .user_id_type("open_id")
                            .sort_type("ByCreateTimeAsc")
                            .page_size(10 + j)
                            .page_token(&format!("thread_{}_token_{}", i, j))
                            .build();
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_shared_config_thread_safety() {
        let config = Arc::new(create_test_config("https://concurrent.test.com"));
        let handles: Vec<_> = (0..5)
            .map(|_| {
                let config_clone = Arc::clone(&config);
                thread::spawn(move || {
                    let _service = create_test_chats_service((*config_clone).clone());
                    // 在多个线程中使用相同的配置应该是安全的
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_concurrent_serialization() {
        let chat = Arc::new(ListChat {
            chat_id: "oc_concurrent_test".to_string(),
            avatar: "avatar.jpg".to_string(),
            name: "并发测试群组".to_string(),
            description: "用于并发测试".to_string(),
            owner_id: "ou_concurrent_owner".to_string(),
            owner_id_type: "open_id".to_string(),
            external: false,
            tenant_key: "tenant_concurrent".to_string(),
            chat_status: "active".to_string(),
        });

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let chat_clone = Arc::clone(&chat);
                thread::spawn(move || {
                    for _ in 0..100 {
                        let _serialized = serde_json::to_string(&*chat_clone).unwrap();
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

/// 服务创建测试
#[cfg(test)]
mod service_tests {
    use super::*;

    #[test]
    fn test_chats_service_creation() {
        let config = create_test_config("https://service.test.com");
        let service = create_test_chats_service(config.clone());
        
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert_eq!(service.config.base_url, "https://service.test.com");
        assert_eq!(service.config.enable_token_cache, false);
    }

    #[test]
    fn test_chats_service_with_different_configs() {
        let configs = vec![
            create_test_config("https://config1.test.com"),
            create_test_config("https://config2.test.com"),
            create_test_config("https://config3.test.com"),
        ];

        for (i, config) in configs.into_iter().enumerate() {
            let service = create_test_chats_service(config);
            assert_eq!(service.config.base_url, format!("https://config{}.test.com", i + 1));
        }
    }
}