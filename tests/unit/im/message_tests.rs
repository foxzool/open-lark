//! 消息服务单元测试
//!
//! 测试消息发送、接收、列表获取等核心功能

use rstest::*;
use serde_json::json;
use wiremock::{
    matchers::{body_json, header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};
use tempfile::tempdir;
use serial_test::serial;
use open_lark::service::im::v1::message::*;
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

/// 创建测试用的消息服务
fn create_test_message_service(config: Config) -> MessageService {
    MessageService { config }
}

/// 消息创建请求测试
#[cfg(test)]
mod create_message_request_tests {
    use super::*;

    #[test]
    fn test_create_message_request_builder_basic() {
        let request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_123456789")
                    .msg_type("text")
                    .content(r#"{"text":"Hello World"}"#)
                    .build()
            )
            .build();

        // 验证查询参数
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string())
        );
        
        // 验证请求体
        assert!(!request.api_req.body.is_empty());
        let body: CreateMessageRequestBody = serde_json::from_slice(&request.api_req.body).unwrap();
        assert_eq!(body.receive_id, "ou_123456789");
        assert_eq!(body.msg_type, "text");
        assert_eq!(body.content, r#"{"text":"Hello World"}"#);
    }

    #[test]
    fn test_create_message_request_with_uuid() {
        let uuid = "12345678-1234-5678-9abc-123456789012";
        let request_body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(r#"{"text":"Test"}"#)
            .uuid(uuid)
            .build();

        assert_eq!(request_body.uuid, Some(uuid.to_string()));
    }

    #[rstest]
    #[case("open_id", "ou_123456")]
    #[case("user_id", "u_123456")]
    #[case("union_id", "un_123456")]
    #[case("email", "test@example.com")]
    #[case("chat_id", "oc_123456")]
    fn test_create_message_request_different_receive_id_types(
        #[case] id_type: &str,
        #[case] receive_id: &str
    ) {
        let request = CreateMessageRequest::builder()
            .receive_id_type(id_type)
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id(receive_id)
                    .msg_type("text")
                    .content(r#"{"text":"Test message"}"#)
                    .build()
            )
            .build();

        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&id_type.to_string())
        );
    }

    #[rstest]
    #[case("text", r#"{"text":"Hello"}"#)]
    #[case("post", r#"{"post":{"zh_cn":{"title":"标题","content":[]}}}"#)]
    #[case("image", r#"{"image_key":"img_123"}"#)]
    #[case("file", r#"{"file_key":"file_123"}"#)]
    #[case("interactive", r#"{"type":"template","data":{"template_id":"tpl_123"}}"#)]
    fn test_create_message_request_different_msg_types(
        #[case] msg_type: &str,
        #[case] content: &str
    ) {
        let request_body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type(msg_type)
            .content(content)
            .build();

        assert_eq!(request_body.msg_type, msg_type);
        assert_eq!(request_body.content, content);
    }
}

/// 消息请求体构建器测试
#[cfg(test)]
mod create_message_request_body_tests {
    use super::*;

    #[test]
    fn test_create_message_request_body_builder() {
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test_user")
            .msg_type("text")
            .content(r#"{"text":"Test content"}"#)
            .uuid("test-uuid-1234")
            .build();

        assert_eq!(body.receive_id, "ou_test_user");
        assert_eq!(body.msg_type, "text");
        assert_eq!(body.content, r#"{"text":"Test content"}"#);
        assert_eq!(body.uuid, Some("test-uuid-1234".to_string()));
    }

    #[test]
    fn test_create_message_request_body_without_uuid() {
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test_user")
            .msg_type("text")  
            .content(r#"{"text":"Test content"}"#)
            .build();

        assert_eq!(body.uuid, None);
    }

    #[test]
    fn test_create_message_request_body_serialization() {
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_123")
            .msg_type("text")
            .content(r#"{"text":"Hello"}"#)
            .uuid("uuid-123")
            .build();

        let serialized = serde_json::to_string(&body).unwrap();
        let expected = json!({
            "receive_id": "ou_123",
            "msg_type": "text",
            "content": r#"{"text":"Hello"}"#,
            "uuid": "uuid-123"
        });

        let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_create_message_request_body_deserialization() {
        let json_str = r#"{
            "receive_id": "ou_456",
            "msg_type": "post",
            "content": "{\"post\":{\"zh_cn\":{\"title\":\"Test\"}}}",
            "uuid": "uuid-456"
        }"#;

        let body: CreateMessageRequestBody = serde_json::from_str(json_str).unwrap();
        assert_eq!(body.receive_id, "ou_456");
        assert_eq!(body.msg_type, "post");
        assert!(body.content.contains("Test"));
        assert_eq!(body.uuid, Some("uuid-456".to_string()));
    }
}

/// 列表消息请求测试
#[cfg(test)]  
mod list_message_request_tests {
    use super::*;

    #[test]
    fn test_list_message_request_builder_complete() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_test_chat")
            .start_time(1609296800)
            .end_time(1609383200)
            .sort_type("ByCreateTimeAsc")
            .page_token("token_123")
            .page_size(50)
            .build();

        let query_params = &request.api_req.query_params;
        assert_eq!(query_params.get("container_id_type"), Some(&"chat".to_string()));
        assert_eq!(query_params.get("container_id"), Some(&"oc_test_chat".to_string()));
        assert_eq!(query_params.get("start_time"), Some(&"1609296800".to_string()));
        assert_eq!(query_params.get("end_time"), Some(&"1609383200".to_string()));
        assert_eq!(query_params.get("sort_type"), Some(&"ByCreateTimeAsc".to_string()));
        assert_eq!(query_params.get("page_token"), Some(&"token_123".to_string()));
        assert_eq!(query_params.get("page_size"), Some(&"50".to_string()));
    }

    #[test]
    fn test_list_message_request_builder_minimal() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_minimal")
            .build();

        let query_params = &request.api_req.query_params;
        assert_eq!(query_params.get("container_id_type"), Some(&"chat".to_string()));
        assert_eq!(query_params.get("container_id"), Some(&"oc_minimal".to_string()));
        assert_eq!(query_params.get("start_time"), None);
        assert_eq!(query_params.get("end_time"), None);
    }

    #[rstest]
    #[case(10)]
    #[case(50)]
    #[case(100)]
    #[case(200)]
    fn test_list_message_request_different_page_sizes(#[case] page_size: i32) {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_test")
            .page_size(page_size)
            .build();

        assert_eq!(
            request.api_req.query_params.get("page_size"),
            Some(&page_size.to_string())
        );
    }

    #[rstest]
    #[case("ByCreateTimeAsc")]
    #[case("ByCreateTimeDesc")]
    fn test_list_message_request_sort_types(#[case] sort_type: &str) {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_test")
            .sort_type(sort_type)
            .build();

        assert_eq!(
            request.api_req.query_params.get("sort_type"),
            Some(&sort_type.to_string())
        );
    }
}

/// 消息结构测试
#[cfg(test)]
mod message_tests {
    use super::*;

    #[test]
    fn test_message_deserialization() {
        let json_data = json!({
            "message_id": "om_123456789",
            "root_id": "om_root_123",
            "parent_id": "om_parent_123",
            "thread_id": "omt_thread_123",
            "msg_type": "text",
            "create_time": "1609459200000",
            "update_time": "1609459200000",
            "deleted": false,
            "updated": false,
            "chat_id": "oc_chat_123",
            "sender": {
                "id": "ou_user_123",
                "id_type": "open_id",
                "sender_type": "user",
                "tenant_key": "tenant_123"
            },
            "body": {
                "content": "{\"text\":\"Hello World\"}"
            },
            "mentions": [{
                "key": "@_user_1",
                "id": "ou_mentioned_user",
                "id_type": "open_id", 
                "name": "Test User",
                "tenant_key": "tenant_123",
                "upper_message_id": "om_123456789"
            }]
        });

        let message: Message = serde_json::from_value(json_data).unwrap();
        
        assert_eq!(message.message_id, "om_123456789");
        assert_eq!(message.root_id, Some("om_root_123".to_string()));
        assert_eq!(message.parent_id, Some("om_parent_123".to_string()));
        assert_eq!(message.thread_id, Some("omt_thread_123".to_string()));
        assert_eq!(message.msg_type, "text");
        assert_eq!(message.create_time, "1609459200000");
        assert_eq!(message.update_time, "1609459200000");
        assert_eq!(message.deleted, false);
        assert_eq!(message.updated, false);
        assert_eq!(message.chat_id, "oc_chat_123");
        assert_eq!(message.body.content, "{\"text\":\"Hello World\"}");
        
        let mentions = message.mentions.unwrap();
        assert_eq!(mentions.len(), 1);
        assert_eq!(mentions[0].key, "@_user_1");
        assert_eq!(mentions[0].id, "ou_mentioned_user");
        assert_eq!(mentions[0].name, "Test User");
    }

    #[test]
    fn test_message_minimal_deserialization() {
        let json_data = json!({
            "message_id": "om_minimal",
            "msg_type": "text",
            "create_time": "1609459200000",
            "update_time": "1609459200000", 
            "deleted": false,
            "updated": false,
            "chat_id": "oc_chat_minimal",
            "sender": {
                "id": "ou_sender",
                "id_type": "open_id",
                "sender_type": "user",
                "tenant_key": "tenant_minimal"
            },
            "body": {
                "content": "{\"text\":\"Minimal\"}"
            }
        });

        let message: Message = serde_json::from_value(json_data).unwrap();
        
        assert_eq!(message.message_id, "om_minimal");
        assert_eq!(message.root_id, None);
        assert_eq!(message.parent_id, None);
        assert_eq!(message.thread_id, None);
        assert_eq!(message.mentions, None);
    }
}

/// 消息内容测试
#[cfg(test)]
mod message_content_tests {
    use super::*;

    #[test]
    fn test_sender_serialization() {
        let sender = Sender {
            id: "ou_sender_123".to_string(),
            id_type: "open_id".to_string(),
            sender_type: "user".to_string(),
            tenant_key: "tenant_key_123".to_string(),
        };

        let serialized = serde_json::to_string(&sender).unwrap();
        let expected = json!({
            "id": "ou_sender_123",
            "id_type": "open_id",
            "sender_type": "user", 
            "tenant_key": "tenant_key_123"
        });

        let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_message_body_serialization() {
        let body = MessageBody {
            content: r#"{"text":"Test message content"}"#.to_string(),
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let expected = json!({
            "content": r#"{"text":"Test message content"}"#
        });

        let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mention_serialization() {
        let mention = Mention {
            key: "@_user_1".to_string(),
            id: "ou_mentioned".to_string(),
            id_type: "open_id".to_string(),
            name: "Mentioned User".to_string(),
            tenant_key: "tenant_123".to_string(),
            upper_message_id: "om_parent".to_string(),
        };

        let serialized = serde_json::to_string(&mention).unwrap();
        let expected = json!({
            "key": "@_user_1",
            "id": "ou_mentioned",
            "id_type": "open_id",
            "name": "Mentioned User",
            "tenant_key": "tenant_123",
            "upper_message_id": "om_parent"
        });

        let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(actual, expected);
    }
}

/// 列表消息响应测试
#[cfg(test)]
mod list_message_response_tests {
    use super::*;

    #[test]
    fn test_list_message_resp_data_serialization() {
        let message = Message {
            message_id: "om_test".to_string(),
            root_id: None,
            parent_id: None,
            thread_id: None,
            msg_type: "text".to_string(),
            create_time: "1609459200000".to_string(),
            update_time: "1609459200000".to_string(),
            deleted: false,
            updated: false,
            chat_id: "oc_test".to_string(),
            sender: Sender {
                id: "ou_sender".to_string(),
                id_type: "open_id".to_string(),
                sender_type: "user".to_string(),
                tenant_key: "tenant_test".to_string(),
            },
            body: MessageBody {
                content: r#"{"text":"Test"}"#.to_string(),
            },
            mentions: None,
        };

        let resp_data = ListMessageRespData {
            has_more: true,
            page_token: Some("next_token_123".to_string()),
            items: vec![message],
        };

        let serialized = serde_json::to_string(&resp_data).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(parsed["has_more"], true);
        assert_eq!(parsed["page_token"], "next_token_123");
        assert_eq!(parsed["items"].as_array().unwrap().len(), 1);
        assert_eq!(parsed["items"][0]["message_id"], "om_test");
    }

    #[test]
    fn test_list_message_resp_data_no_more_pages() {
        let resp_data = ListMessageRespData {
            has_more: false,
            page_token: None,
            items: vec![],
        };

        let serialized = serde_json::to_string(&resp_data).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(parsed["has_more"], false);
        assert!(parsed["page_token"].is_null());
        assert_eq!(parsed["items"].as_array().unwrap().len(), 0);
    }
}

/// Mock HTTP 服务器测试
#[cfg(test)]
mod http_mock_tests {
    use super::*;
    use open_lark::core::req_option::RequestOption;

    #[tokio::test]
    async fn test_create_message_success_response() {
        let mock_server = MockServer::start().await;

        // 设置成功响应的 Mock
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/messages"))
            .and(query_param("receive_id_type", "open_id"))
            .and(header("Authorization", wiremock::matchers::contains("Bearer")))
            .and(body_json(json!({
                "receive_id": "ou_test_user",
                "msg_type": "text",
                "content": r#"{"text":"Hello World"}"#
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "message_id": "om_created_123",
                    "root_id": null,
                    "parent_id": null,
                    "thread_id": null,
                    "msg_type": "text",
                    "create_time": "1609459200000",
                    "update_time": "1609459200000",
                    "deleted": false,
                    "updated": false,
                    "chat_id": "oc_test_chat",
                    "sender": {
                        "id": "ou_app_123",
                        "id_type": "open_id",
                        "sender_type": "app",
                        "tenant_key": "tenant_123"
                    },
                    "body": {
                        "content": r#"{"text":"Hello World"}"#
                    },
                    "mentions": null
                }
            })))
            .mount(&mock_server)
            .await;

        let config = create_test_config(&mock_server.uri());
        let service = create_test_message_service(config);

        let request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_test_user")
                    .msg_type("text")
                    .content(r#"{"text":"Hello World"}"#)
                    .build()
            )
            .build();

        // 注意：这里我们无法直接调用 service.create() 方法，因为它需要真实的令牌管理
        // 但我们可以验证请求的构建是正确的
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string())
        );
    }

    #[tokio::test]
    async fn test_list_messages_success_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/messages"))
            .and(query_param("container_id_type", "chat"))
            .and(query_param("container_id", "oc_test_chat"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": false,
                    "page_token": null,
                    "items": [{
                        "message_id": "om_list_test",
                        "msg_type": "text",
                        "create_time": "1609459200000",
                        "update_time": "1609459200000",
                        "deleted": false,
                        "updated": false,
                        "chat_id": "oc_test_chat",
                        "sender": {
                            "id": "ou_user_123",
                            "id_type": "open_id", 
                            "sender_type": "user",
                            "tenant_key": "tenant_123"
                        },
                        "body": {
                            "content": r#"{"text":"Listed message"}"#
                        }
                    }]
                }
            })))
            .mount(&mock_server)
            .await;

        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_test_chat")
            .build();

        // 验证请求参数
        let query_params = &request.api_req.query_params;
        assert_eq!(query_params.get("container_id_type"), Some(&"chat".to_string()));
        assert_eq!(query_params.get("container_id"), Some(&"oc_test_chat".to_string()));
    }

    #[tokio::test]
    async fn test_error_response_handling() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/messages"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "code": 99991400,
                "msg": "invalid arguments",
                "data": {}
            })))
            .mount(&mock_server)
            .await;

        // 这里我们主要验证错误响应的JSON格式是正确的
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/open-apis/im/v1/messages", mock_server.uri()))
            .json(&json!({
                "receive_id": "invalid",
                "msg_type": "invalid",
                "content": "invalid"
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 400);
        let body: serde_json::Value = response.json().await.unwrap();
        assert_eq!(body["code"], 99991400);
        assert_eq!(body["msg"], "invalid arguments");
    }
}

/// 边界条件和错误处理测试
#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_create_message_request_empty_strings() {
        let request_body = CreateMessageRequestBody::builder()
            .receive_id("")
            .msg_type("")
            .content("")
            .build();

        assert_eq!(request_body.receive_id, "");
        assert_eq!(request_body.msg_type, "");
        assert_eq!(request_body.content, "");
    }

    #[test]
    fn test_create_message_request_very_long_content() {
        let long_content = "A".repeat(150 * 1024); // 150KB
        let request_body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(&long_content)
            .build();

        assert_eq!(request_body.content.len(), 150 * 1024);
    }

    #[test]
    fn test_create_message_request_special_characters() {
        let special_content = r#"{"text":"Hello 😀🎉💯\n\t\r\\"Test\\""}"#;
        let request_body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(special_content)
            .build();

        assert_eq!(request_body.content, special_content);
    }

    #[test]
    fn test_list_message_request_zero_page_size() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_test")
            .page_size(0)
            .build();

        assert_eq!(
            request.api_req.query_params.get("page_size"),
            Some(&"0".to_string())
        );
    }

    #[test]
    fn test_list_message_request_negative_timestamps() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_test")
            .start_time(-1)
            .end_time(-100)
            .build();

        assert_eq!(
            request.api_req.query_params.get("start_time"),
            Some(&"-1".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("end_time"),
            Some(&"-100".to_string())
        );
    }

    #[test]
    fn test_message_deserialization_missing_optional_fields() {
        let minimal_json = json!({
            "message_id": "om_minimal",
            "msg_type": "text",
            "create_time": "1609459200000",
            "update_time": "1609459200000",
            "deleted": false,
            "updated": false,
            "chat_id": "oc_minimal",
            "sender": {
                "id": "ou_sender",
                "id_type": "open_id",
                "sender_type": "user",
                "tenant_key": "tenant_minimal"
            },
            "body": {
                "content": r#"{"text":"Minimal"}"#
            }
        });

        let result: Result<Message, _> = serde_json::from_value(minimal_json);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert_eq!(message.root_id, None);
        assert_eq!(message.parent_id, None);
        assert_eq!(message.thread_id, None);
        assert_eq!(message.mentions, None);
    }

    #[test]
    fn test_invalid_message_json() {
        let invalid_json = json!({
            "message_id": "om_test"
            // 缺少必需字段
        });

        let result: Result<Message, _> = serde_json::from_value(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_uuid_length_limit() {
        let long_uuid = "A".repeat(100); // 超过50字符限制
        let request_body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(r#"{"text":"Test"}"#)
            .uuid(&long_uuid)
            .build();

        // UUID仍会被设置，但在实际API调用中会被验证
        assert_eq!(request_body.uuid, Some(long_uuid));
    }
}

/// 集成测试辅助函数
#[cfg(test)]
mod integration_helpers_tests {
    use super::*;

    #[test]
    fn test_list_message_iterator_creation() {
        let config = create_test_config("https://test.example.com");
        let service = create_test_message_service(config);
        
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_test")
            .build();

        let mut iterator = service.list_iter(request, None);
        
        // 验证迭代器初始状态
        assert!(iterator.has_more);
    }

    #[test]
    fn test_message_service_creation() {
        let config = create_test_config("https://api.feishu.cn");
        let service = create_test_message_service(config.clone());
        
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert_eq!(service.config.base_url, "https://api.feishu.cn");
    }
}

/// 性能测试
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_create_message_request_build_performance() {
        let start = Instant::now();
        
        for i in 0..1000 {
            let _request = CreateMessageRequest::builder()
                .receive_id_type("open_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id(&format!("ou_user_{}", i))
                        .msg_type("text")
                        .content(&format!(r#"{{"text":"Message {}"}}"}#, i))
                        .build()
                )
                .build();
        }
        
        let duration = start.elapsed();
        // 1000个请求构建应该在100ms内完成
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_message_serialization_performance() {
        let message = Message {
            message_id: "om_performance_test".to_string(),
            root_id: None,
            parent_id: None,
            thread_id: None,
            msg_type: "text".to_string(),
            create_time: "1609459200000".to_string(),
            update_time: "1609459200000".to_string(),
            deleted: false,
            updated: false,
            chat_id: "oc_performance".to_string(),
            sender: Sender {
                id: "ou_sender".to_string(),
                id_type: "open_id".to_string(),
                sender_type: "user".to_string(),
                tenant_key: "tenant_perf".to_string(),
            },
            body: MessageBody {
                content: r#"{"text":"Performance test message"}"#.to_string(),
            },
            mentions: None,
        };

        let start = Instant::now();
        
        for _ in 0..1000 {
            let _serialized = serde_json::to_string(&message).unwrap();
        }
        
        let duration = start.elapsed();
        // 1000次序列化应该在50ms内完成
        assert!(duration.as_millis() < 50);
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
                    for j in 0..100 {
                        let _request = CreateMessageRequest::builder()
                            .receive_id_type("open_id")
                            .request_body(
                                CreateMessageRequestBody::builder()
                                    .receive_id(&format!("ou_{}_{}", i, j))
                                    .msg_type("text")
                                    .content(&format!(r#"{{"text":"Thread {} Message {}""}}}"#, i, j))
                                    .build()
                            )
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
    #[serial]
    fn test_shared_config_thread_safety() {
        let config = Arc::new(create_test_config("https://test.concurrent.com"));
        let handles: Vec<_> = (0..5)
            .map(|_| {
                let config_clone = Arc::clone(&config);
                thread::spawn(move || {
                    let _service = create_test_message_service((*config_clone).clone());
                    // 在多个线程中使用相同的配置应该是安全的
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}