#!/bin/bash

# 重建 builders.rs 文件的测试模块
file="crates/open-lark-communication/src/im/v1/message/builders.rs"

# 删除从 "#[cfg(test)]" 到文件末尾的所有内容
sed -i '' '/#\[cfg(test)\]/,$d' "$file"

# 添加正确的测试模块
cat >> "$file" << 'EOF'

    // Test CreateMessageRequestBuilder
    #[test]
    fn test_create_message_request_builder_default() {
        let builder = CreateMessageRequestBuilder::default();
        let request = builder.build();
        assert!(request.api_req.query_params.is_empty());
        assert!(request.api_req.body.is_empty());
    }

    #[test]
    fn test_create_message_request_builder_receive_id() {
        let builder = CreateMessageRequestBuilder::default();
        let request = builder.receive_id("test_receive_id").build();
        assert_eq!(
            request.api_req.query_params.get("receive_id"),
            Some(&"test_receive_id".to_string())
        );
    }

    #[test]
    fn test_create_message_request_builder_msg_type() {
        let builder = CreateMessageRequestBuilder::default();
        let request = builder.msg_type("text").build();
        assert_eq!(
            request.api_req.query_params.get("msg_type"),
            Some(&"text".to_string())
        );
    }

    #[test]
    fn test_create_message_request_builder_content() {
        let builder = CreateMessageRequestBuilder::default();
        let request = builder.content("test content").build();
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"test content".to_string())
        );
    }

    #[test]
    fn test_create_message_request_builder_receive_id_type() {
        let builder = CreateMessageRequestBuilder::default();
        let request = builder.receive_id_type("open_id").build();
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string())
        );
    }

    #[test]
    fn test_create_message_request_builder_all_params() {
        let builder = CreateMessageRequestBuilder::default();
        let request = builder
            .receive_id("user_123")
            .msg_type("text")
            .content("Hello, world!")
            .receive_id_type("open_id")
            .build();

        assert_eq!(
            request.api_req.query_params.get("receive_id"),
            Some(&"user_123".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("msg_type"),
            Some(&"text".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"Hello, world!".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string())
        );
    }

    #[test]
    fn test_create_message_request_builder_request_body() {
        let body = CreateMessageRequestBody {
            receive_id: "user_123".to_string(),
            msg_type: "text".to_string(),
            content: "{\"text\":\"Hello\"}".to_string(),
            uuid: Some("uuid-123".to_string()),
        };

        let builder = CreateMessageRequestBodyBuilder::default();
        let request = builder.request_body(body).build();
        assert!(!request.api_req.body.is_empty());

        // Verify that body was serialized correctly
        let deserialized: CreateMessageRequestBody =
            serde_json::from_slice(&request.api_req.body).unwrap();
        assert_eq!(deserialized.receive_id, "user_123");
        assert_eq!(deserialized.msg_type, "text");
        assert_eq!(deserialized.content, "{\"text\":\"Hello\"}");
        assert_eq!(deserialized.uuid, Some("uuid-123".to_string()));
    }

    #[test]
    fn test_create_message_request_builder_default() {
        let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.build();
        assert!(body.receive_id.is_empty());
        assert!(body.msg_type.is_empty());
        assert!(body.content.is_empty());
        assert!(body.uuid.is_none());
    }

    #[test]
    fn test_create_message_request_body_builder_receive_id() {
        let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.receive_id("test_user").build();
        assert_eq!(body.receive_id, "test_user");
    }

    #[test]
    fn test_create_message_request_body_builder_msg_type() {
        let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.msg_type("image").build();
        assert_eq!(body.msg_type, "image");
    }

    #[test]
    fn test_create_message_request_body_builder_content() {
        let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.content("{\"image\":{\"key\":\"img_123\"}}").build();
        assert_eq!(body.content, "{\"image\":{\"key\":\"img_123\"}}");
    }

    #[test]
    fn test_create_message_request_body_builder_uuid() {
        let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.uuid("test-uuid-123").build();
        assert_eq!(body.uuid, Some("test-uuid-123".to_string()));
    }

    #[test]
    fn test_create_message_request_body_builder_all_fields() {
        let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder
            .receive_id("user_456")
            .msg_type("post")
            .content("{\"post\":{\"zh_cn\":{\"title\":\"Test\"}}}")
            .uuid("unique-uuid-789")
            .build();

        assert_eq!(body.receive_id, "user_456");
        assert_eq!(body.msg_type, "post");
        assert_eq!(body.content, "{\"post\":{\"zh_cn\":{\"title\":\"Test\"}}}");
        assert_eq!(body.uuid, Some("unique-uuid-789".to_string()));
    }

    #[test]
    fn test_create_message_request_debug_trait() {
        let request = CreateMessageRequest::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CreateMessageRequest"));
    }

    #[test]
    fn test_create_message_request_clone_trait() {
        let request = CreateMessageRequest::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();
        let cloned_request = request.clone();
        assert_eq!(
            request.api_req.query_params,
            cloned_request.api_req.query_params
        );
        assert_eq!(request.api_req.body, cloned_request.api_req.body);
    }

    #[test]
    fn test_create_message_request_body_debug_trait() {
        let body = CreateMessageRequestBodyBuilder::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();

        let debug_str = format!("{:?}", body);
        assert!(debug_str.contains("CreateMessageRequestBody"));
    }

    #[test]
    fn test_create_message_request_body_clone_trait() {
        let body = CreateMessageRequestBodyBuilder::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();
        let cloned_body = body.clone();
        assert_eq!(body.receive_id, cloned_body.receive_id);
        assert_eq!(body.msg_type, cloned_body.msg_type);
        assert_eq!(body.content, cloned_body.content);
        assert_eq!(body.uuid, cloned_body.uuid);
    }

    // Test UpdateMessageRequestBuilder
    #[test]
    fn test_update_message_request_builder_default() {
        let builder = UpdateMessageRequestBuilder::default();
        let request = builder.build();
        assert!(request.api_req.query_params.is_empty());
        assert!(request.api_req.body.is_empty());
    }

    #[test]
    fn test_update_message_builder_content() {
        let builder = UpdateMessageRequestBuilder::default();
        let request = builder.content("updated content").build();
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"updated content".to_string())
        );
    }

    // Test CreateMessageRequest convenience methods
    #[test]
    fn test_create_message_request_with_msg() {
        // Mock SendMessageTrait for testing
        pub struct MockMessage {
            msg_type_value: String,
            content_value: String,
        }

        impl SendMessageTrait for MockMessage {
            fn msg_type(&self) -> String {
                self.msg_type_value.clone()
            }

            fn content(&self) -> String {
                self.content_value.clone()
            }
        }

        impl MockMessage {
            pub fn new(msg_type: impl ToString, content: impl ToString) -> Self {
                Self {
                    msg_type_value: msg_type.to_string(),
                    content_value: content.to_string(),
                }
            }
        }

        let mock_msg = MockMessage::new("text", "{\"text\":\"Hello from mock\"}");
        let request = CreateMessageRequest::with_msg("user_789", mock_msg, "open_id");

        assert_eq!(
            request.api_req.query_params.get("receive_id"),
            Some(&"user_789".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("msg_type"),
            Some(&"text".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"{\"text\":\"Hello from mock\"}".to_string())
        );
    }

    #[test]
    fn test_create_message_request_with_msg_different_types() {
        let text_msg = MockMessage::new("text", "simple text");
        let image_msg = MockMessage::new("image", "{\"image\":{\"key\":\"img_456\"}}");
        let card_msg = MockMessage::new("interactive", "{\"card\":{\"elements\":[]}}");

        let text_request = CreateMessageRequest::with_msg("user_1", text_msg, "user_id");
        let image_request = CreateMessageRequest::with_msg("chat_2", image_msg, "chat_id");
        let card_request = CreateMessageRequest::with_msg("user_3", card_msg, "union_id");

        assert_eq!(
            text_request.api_req.query_params.get("msg_type"),
            Some(&"text".to_string())
        );
        assert_eq!(
            image_request.api_req.query_params.get("msg_type"),
            Some(&"image".to_string())
        );
        assert_eq!(
            card_request.api_req.query_params.get("msg_type"),
            Some(&"interactive".to_string())
        );

        assert_eq!(
            text_request.api_req.query_params.get("receive_id_type"),
            Some(&"user_id".to_string())
        );
        assert_eq!(
            image_request.api_req.query_params.get("receive_id_type"),
            Some(&"chat_id".to_string())
        );
        assert_eq!(
            card_request.api_req.query_params.get("receive_id_type"),
            Some(&"union_id".to_string())
        );
    }

    #[test]
    fn test_create_message_request_body_serialization() {
        let body = CreateMessageRequestBody {
            receive_id: "user_123".to_string(),
            msg_type: "text".to_string(),
            content: "{\"text\":\"Hello World\"}".to_string(),
            uuid: Some("uuid-abc123".to_string()),
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: CreateMessageRequestBody = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.receive_id, body.receive_id);
        assert_eq!(deserialized.msg_type, body.msg_type);
        assert_eq!(deserialized.content, body.content);
        assert_eq!(deserialized.uuid, body.uuid);
    }

    #[test]
    fn test_create_message_request_body_serialization_without_uuid() {
        let body = CreateMessageRequestBody {
            receive_id: "user_456".to_string(),
            msg_type: "image".to_string(),
            content: "{\"image\":{\"key\":\"img_xyz\"}}".to_string(),
            uuid: None,
        };

        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: CreateMessageRequestBody = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.receive_id, body.receive_id);
        assert_eq!(deserialized.msg_type, body.msg_type);
        assert_eq!(deserialized.content, body.content);
        assert_eq!(deserialized.uuid, None);
    }

    #[test]
    fn test_create_message_request_builder_chaining() {
        let builder1 = CreateMessageRequestBuilder::default();
        let builder2 = builder1.receive_id("user_123");
        let request2 = builder2.build();

        // Modified builder should have the receive_id
        assert_eq!(
            request2.api_req.query_params.get("receive_id"),
            Some(&"user_123".to_string())
        );
    }

    #[test]
    fn test_create_message_request_builder_chaining() {
        let builder1 = CreateMessageRequestBuilder::default();
        let builder2 = builder1.receive_id("user_123");
        let request2 = builder2.build();

        // Modified builder should have the receive_id
        assert_eq!(
            request2.api_req.query_params.get("receive_id"),
            Some(&"user_123".to_string())
        );
    }

    #[test]
    fn test_create_message_request_debug_trait() {
        let request = CreateMessageRequest::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CreateMessageRequest"));
    }

    #[test]
    fn test_create_message_request_clone_trait() {
        let request = CreateMessageRequest::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();
        let cloned_request = request.clone();
        assert_eq!(
            request.api_req.query_params,
            cloned_request.api_req.query_params
        );
        assert_eq!(request.api_req.body, cloned_request.api_req.body);
    }

    #[test]
    fn test_create_message_request_body_debug_trait() {
        let body = CreateMessageRequestBodyBuilder::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();

        let debug_str = format!("{:?}", body);
        assert!(debug_str.contains("CreateMessageRequestBody"));
    }

    #[test]
    fn test_create_message_request_body_clone_trait() {
        let body = CreateMessageRequestBodyBuilder::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();
        let cloned_body = body.clone();
        assert_eq!(body.receive_id, cloned_body.receive_id);
        assert_eq!(body.msg_type, cloned_body.msg_type);
        assert_eq!(body.content, cloned_body.content);
        assert_eq!(body.uuid, cloned_body.uuid);
    }

    // Test edge cases and error handling
    #[test]
    fn test_create_message_request_builder_with_unicode_content() {
        let builder = CreateMessageRequestBuilder::default();
        let request = builder
            .receive_id("用户_123")
            .content("你好，世界！")
            .build();

        assert_eq!(
            request.api_req.query_params.get("receive_id"),
            Some(&"用户_123".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"你好，世界！".to_string())
        );
    }

    #[test]
    fn test_create_message_request_builder_with_empty_strings() {
        let builder = CreateMessageRequestBuilder::default();
        let request = builder
            .receive_id("")
            .msg_type("")
            .content("")
            .receive_id_type("")
            .build();

        assert_eq!(
            request.api_req.query_params.get("receive_id"),
            Some(&"".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("msg_type"),
            Some(&"".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"".to_string())
        );
    }

    #[test]
    fn test_create_message_request_builder_chaining() {
        let builder1 = CreateMessageRequestBuilder::default();
        let builder2 = builder1.receive_id("user_123");
        let request2 = builder2.build();

        // Modified builder should have the receive_id
        assert_eq!(
            request2.api_req.query_params.get("receive_id"),
            Some(&"user_123".to_string())
        );
    }

    #[test]
    fn test_create_message_request_debug_trait() {
        let request = CreateMessageRequest::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CreateMessageRequest"));
    }

    #[test]
    fn test_create_message_request_clone_trait() {
        let request = CreateMessageRequest::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();
        let cloned_request = request.clone();
        assert_eq!(
            request.api_req.query_params,
            cloned_request.api_req.query_params
        );
        assert_eq!(request.api_req.body, cloned_request.api_req.body);
    }

    #[test]
    fn test_create_message_request_body_debug_trait() {
        let body = CreateMessageRequestBodyBuilder::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();

        let debug_str = format!("{:?}", body);
        assert!(debug_str.contains("CreateMessageRequestBody"));
    }

    #[test]
    fn test_create_message_request_body_clone_trait() {
        let body = CreateMessageRequestBodyBuilder::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();
        let cloned_body = body.clone();
        assert_eq!(body.receive_id, cloned_body.receive_id);
        assert_eq!(body.msg_type, cloned_body.msg_type);
        assert_eq!(body.content, cloned_body.content);
        assert_eq!(body.uuid, cloned_body.uuid);
    }

    // Performance and memory tests
    #[test]
    fn test_builder_memory_efficiency() {
        // Create many builders to test memory usage
        let mut builders = Vec::new();
        for i in 0..100 {
            let builder = CreateMessageRequestBuilder::default()
                .receive_id(format!("user_{}", i))
                .msg_type("text")
                .content(format!("Message {}", i))
                .receive_id_type("open_id");
            builders.push(builder);
        }

        // Build all requests
        let requests: Vec<CreateMessageRequest> = builders.into_iter().map(|b| b.build()).collect();
        assert_eq!(requests.len(), 100);

        // Verify a few random requests
        assert_eq!(
            requests[0].api_req.query_params.get("receive_id"),
            Some(&"user_0".to_string())
        );
        assert_eq!(
            requests[99].api_req.query_params.get("content"),
            Some(&"Message 99".to_string())
        );
    }

    #[test]
    fn test_create_message_request_builder_chaining() {
        let builder1 = CreateMessageRequestBuilder::default();
        let builder2 = builder1.receive_id("user_123");
        let request2 = builder2.build();

        // Modified builder should have the receive_id
        assert_eq!(
            request2.api_req.query_params.get("receive_id"),
            Some(&"user_123".to_string())
        );
    }

    #[test]
    fn test_create_message_request_debug_trait() {
        let request = CreateMessageRequest::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CreateMessageRequest"));
    }

    #[test]
    fn test_create_message_request_clone_trait() {
        let request = CreateMessageRequest::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();
        let cloned_request = request.clone();
        assert_eq!(
            request.api_req.query_params,
            cloned_request.api_req.query_params
        );
        assert_eq!(request.api_req.body, cloned_request.api_req.body);
    }

    #[test]
    fn test_create_message_request_body_debug_trait() {
        let body = CreateMessageRequestBodyBuilder::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();

        let debug_str = format!("{:?}", body);
        assert!(debug_str.contains("CreateMessageRequestBody"));
    }

    #[test]
    fn test_create_message_request_body_clone_trait() {
        let body = CreateMessageRequestBodyBuilder::default()
            .receive_id("user_123")
            .msg_type("text")
            .content("test content")
            .build();
        let cloned_body = body.clone();
        assert_eq!(body.receive_id, cloned_body.receive_id);
        assert_eq!(body.msg_type, cloned_body.msg_type);
        assert_eq!(body.content, cloned_body.content);
        assert_eq!(body.uuid, cloned_body.uuid);
    }

    // Test complex scenarios
    #[test]
    fn test_complex_message_content() {
        let complex_content = r#"{
            "post": {
                "zh_cn": {
                    "title": "复杂消息",
                    "content": [
                        [
                            {
                                "tag": "text",
                                "text": "这是一条复杂消息"
                            },
                            {
                                "tag": "a",
                                "text": "链接文本",
                                "href": "https://example.com"
                            }
                        ]
                    ]
                }
            }
        }"#;

        let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder
            .receive_id("user_complex")
            .msg_type("post")
            .content(complex_content.to_string())
            .uuid("complex-uuid")
            .build();

        assert_eq!(body.content, complex_content);
    }
}
EOF