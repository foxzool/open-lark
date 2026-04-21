//! 构建器模式单元测试
//!
//! 测试各种构建器的功能，包括参数验证、链式调用、默认值等

use rstest::*;
use serde_json::json;
use open_lark::service::im::v1::message::*;
use std::collections::HashMap;

/// 创建消息请求构建器测试
#[cfg(test)]
mod create_message_request_builder_tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let builder = CreateMessageRequestBuilder::default();
        let request = builder.build();
        
        // 默认状态应该有空的查询参数和空的请求体
        assert!(request.api_req.query_params.is_empty());
        assert!(request.api_req.body.is_empty());
    }

    #[test]
    fn test_builder_receive_id_type_setting() {
        let request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string())
        );
    }

    #[test]
    fn test_builder_multiple_receive_id_types() {
        let mut builder = CreateMessageRequest::builder();
        
        // 测试覆盖设置
        builder = builder.receive_id_type("user_id");
        builder = builder.receive_id_type("open_id");
        
        let request = builder.build();
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string())
        );
    }

    #[test]
    fn test_builder_with_request_body() {
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(r#"{"text":"Test"}"#)
            .build();
            
        let request = CreateMessageRequest::builder()
            .request_body(body)
            .build();
            
        assert!(!request.api_req.body.is_empty());
        
        let parsed_body: CreateMessageRequestBody = 
            serde_json::from_slice(&request.api_req.body).unwrap();
        assert_eq!(parsed_body.receive_id, "ou_test");
    }

    #[test]
    fn test_builder_chaining() {
        let request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_chaining_test")
                    .msg_type("text")
                    .content(r#"{"text":"Chaining test"}"#)
                    .uuid("chain-uuid")
                    .build()
            )
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string())
        );
        
        let parsed_body: CreateMessageRequestBody = 
            serde_json::from_slice(&request.api_req.body).unwrap();
        assert_eq!(parsed_body.receive_id, "ou_chaining_test");
        assert_eq!(parsed_body.uuid, Some("chain-uuid".to_string()));
    }

    #[rstest]
    #[case("open_id")]
    #[case("user_id")]
    #[case("union_id")]
    #[case("email")]
    #[case("chat_id")]
    fn test_builder_all_receive_id_types(#[case] id_type: &str) {
        let request = CreateMessageRequest::builder()
            .receive_id_type(id_type)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&id_type.to_string())
        );
    }

    #[test]
    fn test_builder_empty_receive_id_type() {
        let request = CreateMessageRequest::builder()
            .receive_id_type("")
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"".to_string())
        );
    }

    #[test]
    fn test_builder_special_characters_in_receive_id_type() {
        let special_type = "special-id_type!@#$%";
        let request = CreateMessageRequest::builder()
            .receive_id_type(special_type)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&special_type.to_string())
        );
    }
}

/// 创建消息请求体构建器测试
#[cfg(test)]
mod create_message_request_body_builder_tests {
    use super::*;

    #[test]
    fn test_body_builder_default_state() {
        let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.build();
        
        // 默认状态应该都是空字符串和None
        assert_eq!(body.receive_id, "");
        assert_eq!(body.msg_type, "");
        assert_eq!(body.content, "");
        assert_eq!(body.uuid, None);
    }

    #[test]
    fn test_body_builder_all_fields() {
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_full_test")
            .msg_type("text")
            .content(r#"{"text":"Full test"}"#)
            .uuid("full-test-uuid")
            .build();
            
        assert_eq!(body.receive_id, "ou_full_test");
        assert_eq!(body.msg_type, "text");
        assert_eq!(body.content, r#"{"text":"Full test"}"#);
        assert_eq!(body.uuid, Some("full-test-uuid".to_string()));
    }

    #[test]
    fn test_body_builder_without_uuid() {
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_no_uuid")
            .msg_type("post")
            .content(r#"{"post":{"zh_cn":{"title":"No UUID"}}}"#)
            .build();
            
        assert_eq!(body.receive_id, "ou_no_uuid");
        assert_eq!(body.msg_type, "post");
        assert_eq!(body.uuid, None);
    }

    #[test]
    fn test_body_builder_overwrite_values() {
        let body = CreateMessageRequestBody::builder()
            .receive_id("first_id")
            .receive_id("second_id")
            .msg_type("text")
            .msg_type("image")
            .content("first_content")
            .content("second_content")
            .uuid("first_uuid")
            .uuid("second_uuid")
            .build();
            
        assert_eq!(body.receive_id, "second_id");
        assert_eq!(body.msg_type, "image");
        assert_eq!(body.content, "second_content");
        assert_eq!(body.uuid, Some("second_uuid".to_string()));
    }

    #[rstest]
    #[case("text", r#"{"text":"Hello"}"#)]
    #[case("post", r#"{"post":{"zh_cn":{"title":"Post"}}}"#)]
    #[case("image", r#"{"image_key":"img_123"}"#)]
    #[case("file", r#"{"file_key":"file_123"}"#)]
    #[case("interactive", r#"{"type":"template","data":{"template_id":"tpl_123"}}"#)]
    fn test_body_builder_different_message_types(#[case] msg_type: &str, #[case] content: &str) {
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_type_test")
            .msg_type(msg_type)
            .content(content)
            .build();
            
        assert_eq!(body.msg_type, msg_type);
        assert_eq!(body.content, content);
    }

    #[test]
    fn test_body_builder_empty_values() {
        let body = CreateMessageRequestBody::builder()
            .receive_id("")
            .msg_type("")
            .content("")
            .uuid("")
            .build();
            
        assert_eq!(body.receive_id, "");
        assert_eq!(body.msg_type, "");
        assert_eq!(body.content, "");
        assert_eq!(body.uuid, Some("".to_string()));
    }

    #[test]
    fn test_body_builder_very_long_values() {
        let long_id = "ou_".to_owned() + &"a".repeat(1000);
        let long_content = format!(r#"{{"text":"{}"}}"#, "Long text ".repeat(1000));
        let long_uuid = "uuid-".to_owned() + &"b".repeat(100);
        
        let body = CreateMessageRequestBody::builder()
            .receive_id(&long_id)
            .msg_type("text")
            .content(&long_content)
            .uuid(&long_uuid)
            .build();
            
        assert_eq!(body.receive_id, long_id);
        assert!(body.content.len() > 10000);
        assert_eq!(body.uuid, Some(long_uuid));
    }

    #[test]
    fn test_body_builder_special_characters() {
        let special_id = "ou_特殊字符_😀_id";
        let special_content = r#"{"text":"特殊字符 😀🎉 \"引号\" '\'' \n\t"}"#;
        let special_uuid = "uuid-特殊-😀-uuid";
        
        let body = CreateMessageRequestBody::builder()
            .receive_id(special_id)
            .msg_type("text")
            .content(special_content)
            .uuid(special_uuid)
            .build();
            
        assert_eq!(body.receive_id, special_id);
        assert_eq!(body.content, special_content);
        assert_eq!(body.uuid, Some(special_uuid.to_string()));
    }

    #[test]
    fn test_body_builder_json_content_validation() {
        // 虽然构建器不验证JSON格式，但我们可以测试它是否保持原始字符串
        let invalid_json = r#"{"text":"missing quote and brace"#;
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(invalid_json)
            .build();
            
        assert_eq!(body.content, invalid_json);
    }
}

/// 列表消息请求构建器测试
#[cfg(test)]
mod list_message_request_builder_tests {
    use super::*;

    #[test]
    fn test_list_builder_default_state() {
        let builder = ListMessageRequestBuilder::default();
        let request = builder.build();
        
        assert!(request.api_req.query_params.is_empty());
    }

    #[test]
    fn test_list_builder_required_fields() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_test_chat")
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("container_id_type"),
            Some(&"chat".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("container_id"),
            Some(&"oc_test_chat".to_string())
        );
    }

    #[test]
    fn test_list_builder_all_fields() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_full_test")
            .start_time(1609459200)
            .end_time(1609545600)
            .sort_type("ByCreateTimeAsc")
            .page_token("token_123")
            .page_size(100)
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("container_id_type"), Some(&"chat".to_string()));
        assert_eq!(params.get("container_id"), Some(&"oc_full_test".to_string()));
        assert_eq!(params.get("start_time"), Some(&"1609459200".to_string()));
        assert_eq!(params.get("end_time"), Some(&"1609545600".to_string()));
        assert_eq!(params.get("sort_type"), Some(&"ByCreateTimeAsc".to_string()));
        assert_eq!(params.get("page_token"), Some(&"token_123".to_string()));
        assert_eq!(params.get("page_size"), Some(&"100".to_string()));
    }

    #[test]
    fn test_list_builder_time_range() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_time_test")
            .start_time(1609459200)  // 2021-01-01 00:00:00
            .end_time(1640995200)    // 2022-01-01 00:00:00
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("start_time"), Some(&"1609459200".to_string()));
        assert_eq!(params.get("end_time"), Some(&"1640995200".to_string()));
    }

    #[rstest]
    #[case("ByCreateTimeAsc")]
    #[case("ByCreateTimeDesc")]
    fn test_list_builder_sort_types(#[case] sort_type: &str) {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_sort_test")
            .sort_type(sort_type)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("sort_type"),
            Some(&sort_type.to_string())
        );
    }

    #[rstest]
    #[case(10)]
    #[case(50)]
    #[case(100)]
    #[case(200)]
    fn test_list_builder_page_sizes(#[case] page_size: i32) {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_size_test")
            .page_size(page_size)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("page_size"),
            Some(&page_size.to_string())
        );
    }

    #[test]
    fn test_list_builder_pagination_tokens() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_pagination_test")
            .page_token("eyJ0aW1lc3RhbXAiOjE2MDk0NTkyMDB9")
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("page_token"),
            Some(&"eyJ0aW1lc3RhbXAiOjE2MDk0NTkyMDB9".to_string())
        );
    }

    #[test]
    fn test_list_builder_overwrite_values() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("first_chat")
            .container_id("second_chat")
            .page_size(50)
            .page_size(100)
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("container_id"), Some(&"second_chat".to_string()));
        assert_eq!(params.get("page_size"), Some(&"100".to_string()));
    }

    #[test]
    fn test_list_builder_negative_timestamps() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_negative_test")
            .start_time(-1)
            .end_time(-100)
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("start_time"), Some(&"-1".to_string()));
        assert_eq!(params.get("end_time"), Some(&"-100".to_string()));
    }

    #[test]
    fn test_list_builder_zero_values() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_zero_test")
            .start_time(0)
            .end_time(0)
            .page_size(0)
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("start_time"), Some(&"0".to_string()));
        assert_eq!(params.get("end_time"), Some(&"0".to_string()));
        assert_eq!(params.get("page_size"), Some(&"0".to_string()));
    }

    #[test]
    fn test_list_builder_empty_strings() {
        let request = ListMessageRequest::builder()
            .container_id_type("")
            .container_id("")
            .sort_type("")
            .page_token("")
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("container_id_type"), Some(&"".to_string()));
        assert_eq!(params.get("container_id"), Some(&"".to_string()));
        assert_eq!(params.get("sort_type"), Some(&"".to_string()));
        assert_eq!(params.get("page_token"), Some(&"".to_string()));
    }

    #[test]
    fn test_list_builder_extreme_values() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_extreme_test")
            .start_time(i64::MIN)
            .end_time(i64::MAX)
            .page_size(i32::MAX)
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("start_time"), Some(&i64::MIN.to_string()));
        assert_eq!(params.get("end_time"), Some(&i64::MAX.to_string()));
        assert_eq!(params.get("page_size"), Some(&i32::MAX.to_string()));
    }
}

/// 构建器线程安全测试
#[cfg(test)]
mod builder_thread_safety_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_builders_are_not_shared_between_threads() {
        // 每个线程应该有自己的构建器实例
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let request = CreateMessageRequest::builder()
                        .receive_id_type("open_id")
                        .request_body(
                            CreateMessageRequestBody::builder()
                                .receive_id(&format!("ou_thread_{}", i))
                                .msg_type("text")
                                .content(&format!(r#"{{"text":"Thread {} message"}}"#, i))
                                .build()
                        )
                        .build();
                    
                    let body: CreateMessageRequestBody = 
                        serde_json::from_slice(&request.api_req.body).unwrap();
                    assert_eq!(body.receive_id, format!("ou_thread_{}", i));
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_concurrent_list_builder_creation() {
        let handles: Vec<_> = (0..20)
            .map(|i| {
                thread::spawn(move || {
                    let request = ListMessageRequest::builder()
                        .container_id_type("chat")
                        .container_id(&format!("oc_concurrent_{}", i))
                        .page_size(i * 10)
                        .build();
                    
                    let params = &request.api_req.query_params;
                    assert_eq!(
                        params.get("container_id"),
                        Some(&format!("oc_concurrent_{}", i))
                    );
                    assert_eq!(
                        params.get("page_size"),
                        Some(&(i * 10).to_string())
                    );
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

/// 构建器性能测试
#[cfg(test)]
mod builder_performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_create_message_builder_performance() {
        let start = Instant::now();
        
        for i in 0..1000 {
            let _request = CreateMessageRequest::builder()
                .receive_id_type("open_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id(&format!("ou_perf_{}", i))
                        .msg_type("text")
                        .content(&format!(r#"{{"text":"Performance test {}"}}"#, i))
                        .uuid(&format!("uuid-perf-{}", i))
                        .build()
                )
                .build();
        }
        
        let duration = start.elapsed();
        // 1000次构建应该在100ms内完成
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_list_message_builder_performance() {
        let start = Instant::now();
        
        for i in 0..1000 {
            let _request = ListMessageRequest::builder()
                .container_id_type("chat")
                .container_id(&format!("oc_perf_{}", i))
                .start_time(1609459200 + i as i64)
                .end_time(1609459200 + i as i64 + 3600)
                .sort_type("ByCreateTimeAsc")
                .page_size(50)
                .build();
        }
        
        let duration = start.elapsed();
        // 1000次构建应该在50ms内完成
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_builder_memory_usage() {
        // 测试构建器不会导致内存泄漏
        let mut requests = Vec::new();
        
        for i in 0..10000 {
            let request = CreateMessageRequest::builder()
                .receive_id_type("open_id")
                .request_body(
                    CreateMessageRequestBody::builder()
                        .receive_id(&format!("ou_mem_{}", i))
                        .msg_type("text")
                        .content("Memory test")
                        .build()
                )
                .build();
            requests.push(request);
        }
        
        // 确保所有请求都被正确创建
        assert_eq!(requests.len(), 10000);
        
        // 释放内存
        requests.clear();
        assert_eq!(requests.len(), 0);
    }
}

/// 构建器边界条件测试
#[cfg(test)]
mod builder_edge_cases_tests {
    use super::*;

    #[test]
    fn test_builder_with_malformed_json_content() {
        // 构建器应该接受任何字符串作为内容，即使不是有效的JSON
        let malformed_json = r#"{"text":"unclosed string"#;
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(malformed_json)
            .build();
            
        assert_eq!(body.content, malformed_json);
    }

    #[test]
    fn test_builder_with_null_bytes() {
        let content_with_null = "Hello\0World\0Test";
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(content_with_null)
            .build();
            
        assert_eq!(body.content, content_with_null);
    }

    #[test]
    fn test_builder_with_very_long_uuid() {
        let long_uuid = "a".repeat(1000);
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(r#"{"text":"test"}"#)
            .uuid(&long_uuid)
            .build();
            
        assert_eq!(body.uuid, Some(long_uuid));
    }

    #[test]
    fn test_list_builder_timestamp_edge_cases() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_edge_test")
            .start_time(i64::MIN)
            .end_time(i64::MAX)
            .build();
            
        let params = &request.api_req.query_params;
        assert_eq!(params.get("start_time"), Some(&i64::MIN.to_string()));
        assert_eq!(params.get("end_time"), Some(&i64::MAX.to_string()));
    }

    #[test]
    fn test_builder_unicode_handling() {
        let unicode_content = "🚀 Hello 世界 مرحبا שלום こんにちは";
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_unicode")
            .msg_type("text")
            .content(unicode_content)
            .build();
            
        assert_eq!(body.content, unicode_content);
        
        // 确保可以正确序列化
        let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: CreateMessageRequestBody = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(deserialized.content, unicode_content);
    }

    #[test]
    fn test_builder_query_params_encoding() {
        // 测试特殊字符在查询参数中的处理
        let special_chat_id = "oc_special_!@#$%^&*()_+{}[]|\\:;\"'<>?,./ ";
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id(special_chat_id)
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("container_id"),
            Some(&special_chat_id.to_string())
        );
    }

    #[test]
    fn test_builder_empty_but_present_fields() {
        let request = CreateMessageRequest::builder()
            .receive_id_type("")  // 空但存在
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("")
                    .msg_type("")
                    .content("")
                    .uuid("")
                    .build()
            )
            .build();
            
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"".to_string())
        );
        
        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&request.api_req.body).unwrap();
        assert_eq!(body.receive_id, "");
        assert_eq!(body.msg_type, "");
        assert_eq!(body.content, "");
        assert_eq!(body.uuid, Some("".to_string()));
    }
}

/// 集成测试 - 构建器与其他组件的交互
#[cfg(test)]
mod builder_integration_tests {
    use super::*;

    #[test]
    fn test_builder_creates_valid_api_request() {
        let request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_integration_test")
                    .msg_type("text")
                    .content(r#"{"text":"Integration test message"}"#)
                    .uuid("integration-test-uuid")
                    .build()
            )
            .build();
            
        // 验证API请求结构
        assert!(!request.api_req.query_params.is_empty());
        assert!(!request.api_req.body.is_empty());
        
        // 验证查询参数格式
        for (key, value) in &request.api_req.query_params {
            assert!(!key.is_empty());
            assert!(!value.is_empty());
        }
        
        // 验证请求体可以被正确解析
        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&request.api_req.body).unwrap();
        assert!(!body.receive_id.is_empty());
        assert!(!body.msg_type.is_empty());
        
        // 验证JSON内容格式
        let content_json: serde_json::Value = serde_json::from_str(&body.content).expect("JSON 反序列化失败");
        assert!(content_json.is_object());
    }

    #[test]
    fn test_list_builder_creates_valid_pagination_request() {
        let request = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_pagination_integration")
            .start_time(1609459200)
            .end_time(1609545600)
            .sort_type("ByCreateTimeAsc")
            .page_size(50)
            .page_token("next_page_token")
            .build();
            
        let params = &request.api_req.query_params;
        
        // 验证所有分页相关参数都存在
        assert!(params.contains_key("container_id_type"));
        assert!(params.contains_key("container_id"));
        assert!(params.contains_key("start_time"));
        assert!(params.contains_key("end_time"));
        assert!(params.contains_key("sort_type"));
        assert!(params.contains_key("page_size"));
        assert!(params.contains_key("page_token"));
        
        // 验证数值类型的正确性
        let page_size: i32 = params.get("page_size").unwrap().parse().unwrap();
        assert_eq!(page_size, 50);
        
        let start_time: i64 = params.get("start_time").unwrap().parse().unwrap();
        assert_eq!(start_time, 1609459200);
    }
}

/// 构建器文档示例测试
#[cfg(test)]
mod builder_documentation_examples_tests {
    use super::*;

    #[test]
    fn test_readme_example_create_message() {
        // 模拟README中的示例代码
        let request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_xxx")
                    .msg_type("text")
                    .content(r#"{"text":"Hello!"}"#)
                    .build()
            )
            .build();
            
        // 验证示例是否按预期工作
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string())
        );
        
        let body: CreateMessageRequestBody = 
            serde_json::from_slice(&request.api_req.body).unwrap();
        assert_eq!(body.receive_id, "ou_xxx");
        assert_eq!(body.msg_type, "text");
        assert_eq!(body.content, r#"{"text":"Hello!"}"#);
    }

    #[test]
    fn test_common_usage_patterns() {
        // 测试常见的使用模式
        
        // 模式1：简单文本消息
        let text_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_user123")
                    .msg_type("text")
                    .content(r#"{"text":"简单文本消息"}"#)
                    .build()
            )
            .build();
            
        // 模式2：富文本消息
        let post_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_user456")
                    .msg_type("post")
                    .content(r#"{"post":{"zh_cn":{"title":"富文本标题","content":[]}}}"#)
                    .build()
            )
            .build();
            
        // 模式3：图片消息
        let image_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id("ou_user789")
                    .msg_type("image")
                    .content(r#"{"image_key":"img_v2_123456"}"#)
                    .build()
            )
            .build();
            
        // 验证所有模式都能正确构建
        assert!(!text_request.api_req.body.is_empty());
        assert!(!post_request.api_req.body.is_empty());
        assert!(!image_request.api_req.body.is_empty());
    }
}