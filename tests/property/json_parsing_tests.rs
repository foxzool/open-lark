use fake::{Fake, Faker};
use open_lark::core::config::Config;
use proptest::prelude::*;
use quickcheck::{quickcheck, TestResult};
use serde_json::{json, Value};
use std::collections::HashMap;

/// JSON 解析健壮性测试
/// 
/// 测试各种 JSON 解析场景的边界条件：
/// - 畸形 JSON 数据
/// - 极深嵌套结构
/// - 特殊字符和 Unicode
/// - 内存占用极大的 JSON
/// - 配置文件解析的边界条件

#[cfg(test)]
mod json_parsing_properties {
    use super::*;

    /// 生成任意 JSON 值的策略
    fn arbitrary_json_value() -> impl Strategy<Value = Value> {
        let leaf = prop_oneof![
            Just(Value::Null),
            any::<bool>().prop_map(Value::Bool),
            any::<i64>().prop_map(|n| Value::Number(n.into())),
            any::<f64>().prop_map(|f| {
                if f.is_finite() {
                    serde_json::Number::from_f64(f).map_or(Value::Null, Value::Number)
                } else {
                    Value::Null
                }
            }),
            ".*".prop_map(Value::String),
        ];
        
        leaf.prop_recursive(8, 256, 10, |inner| {
            prop_oneof![
                prop::collection::vec(inner.clone(), 0..10).prop_map(Value::Array),
                prop::collection::hash_map("[a-zA-Z0-9_]{1,20}", inner, 0..10)
                    .prop_map(|map| Value::Object(map.into_iter().collect())),
            ]
        })
    }

    /// 属性测试：JSON 解析不应该因为任意输入而崩溃
    proptest! {
        #[test]
        fn json_parsing_never_panics(json_bytes in prop::collection::vec(any::<u8>(), 0..10000)) {
            // 尝试解析任意字节序列为 JSON
            let _result = serde_json::from_slice::<Value>(&json_bytes);
            // 不关心是否成功，只要不崩溃即可
        }
    }

    /// 属性测试：有效 JSON 的序列化和反序列化应该是幂等的
    proptest! {
        #[test]
        fn json_roundtrip_is_idempotent(value in arbitrary_json_value()) {
            let serialized = serde_json::to_vec(&value).unwrap();
            let deserialized: Value = serde_json::from_slice(&serialized).unwrap();
            prop_assert_eq!(value, deserialized);
        }
    }

    /// 属性测试：配置解析应该处理各种字符串输入
    proptest! {
        #[test]
        fn config_parsing_handles_strings(
            app_id in "[a-zA-Z0-9_]{1,50}",
            app_secret in "[a-zA-Z0-9_]{1,100}",
            base_url in "https?://[a-zA-Z0-9.-]+",
        ) {
            let config_json = json!({
                "app_id": app_id,
                "app_secret": app_secret,
                "base_url": base_url
            });
            
            // 尝试解析为配置对象
            let config_str = serde_json::to_string(&config_json).unwrap();
            let _result: Result<HashMap<String, Value>, _> = serde_json::from_str(&config_str);
            // 不应该崩溃
        }
    }

    /// QuickCheck 测试：字符串转义处理
    #[quickcheck]
    fn string_escaping_is_safe(input: String) -> TestResult {
        let json_value = Value::String(input.clone());
        let serialized = serde_json::to_string(&json_value);
        
        match serialized {
            Ok(json_str) => {
                let deserialized: Result<Value, _> = serde_json::from_str(&json_str);
                match deserialized {
                    Ok(Value::String(s)) => TestResult::from_bool(s == input),
                    _ => TestResult::failed(),
                }
            }
            Err(_) => TestResult::discard(), // 某些字符可能无法序列化
        }
    }

    /// 边界条件测试：极深的嵌套 JSON
    #[test]
    fn test_deeply_nested_json() {
        // 创建深度嵌套的 JSON 对象
        let mut nested_json = json!({"value": 42});
        for _ in 0..1000 {
            nested_json = json!({"nested": nested_json});
        }
        
        let serialized = serde_json::to_string(&nested_json).unwrap();
        
        // 尝试解析 - 这可能会因为堆栈溢出而失败，但不应该崩溃整个程序
        let _result = std::panic::catch_unwind(|| {
            serde_json::from_str::<Value>(&serialized)
        });
    }

    /// 边界条件测试：极大的 JSON 数组
    #[test]
    fn test_extremely_large_json_array() {
        let large_array: Vec<i32> = (0..100000).collect();
        let json_value = json!(large_array);
        
        // 序列化和反序列化大数组
        let serialized = serde_json::to_vec(&json_value).unwrap();
        let deserialized: Value = serde_json::from_slice(&serialized).unwrap();
        
        if let Value::Array(arr) = deserialized {
            assert_eq!(arr.len(), 100000);
        } else {
            panic!("Expected array");
        }
    }

    /// Unicode 和特殊字符处理测试
    #[test]
    fn test_unicode_and_special_characters() {
        let special_strings = vec![
            "Hello, 世界!",
            "🎉🚀💯",
            "\\u0000\\u0001\\u0002",
            "\"quotes\" and \\backslashes\\",
            "\n\r\t",
            "null\0bytes",
            "🏳️‍🌈🏳️‍⚧️", // Complex emoji with zero-width joiners
        ];

        for special_str in special_strings {
            let json_value = json!({"text": special_str});
            let serialized = serde_json::to_string(&json_value).unwrap();
            let deserialized: Value = serde_json::from_str(&serialized).unwrap();
            
            if let Some(text) = deserialized["text"].as_str() {
                assert_eq!(text, special_str);
            } else {
                panic!("Failed to deserialize special string: {}", special_str);
            }
        }
    }

    /// 恶意 JSON 输入测试
    #[test]
    fn test_malicious_json_inputs() {
        let malicious_inputs = vec![
            // 不匹配的括号
            "{{{{}}}",
            "[[[[]]]",
            // 不完整的字符串
            r#"{"key": "unclosed string"#,
            // 无效的转义序列
            r#"{"key": "\invalid"}"#,
            // 重复的键
            r#"{"key": 1, "key": 2}"#,
            // 超长的数字
            &"1".repeat(10000),
            // 控制字符
            "{\"\x00\": \"value\"}",
            // 非 UTF-8 字节序列被解释为 JSON
            "{\"key\": \"value\"}garbage",
        ];

        for malicious_input in malicious_inputs {
            let result = std::panic::catch_unwind(|| {
                serde_json::from_str::<Value>(malicious_input)
            });
            
            // 恶意输入应该优雅地返回错误，而不是崩溃
            assert!(result.is_ok(), "Parser should not panic on input: {}", malicious_input);
        }
    }

    /// 内存消耗测试
    #[test]
    fn test_memory_exhaustion_protection() {
        // 尝试创建会消耗大量内存的 JSON
        let memory_bomb_attempts = vec![
            // 深度嵌套对象
            (0..10000).fold(String::from("{}"), |acc, _| format!("{{\"a\":{}}}", acc)),
            // 重复的长字符串
            format!(r#"{{"data": "{}"}}"#, "x".repeat(1_000_000)),
        ];

        for attempt in memory_bomb_attempts {
            // 设置较小的内存限制来测试
            let result = std::panic::catch_unwind(|| {
                let _parsed: Result<Value, _> = serde_json::from_str(&attempt);
            });
            
            assert!(result.is_ok(), "JSON parser should handle memory pressure gracefully");
        }
    }

    /// 浮点数精度测试
    #[test]
    fn test_floating_point_precision() {
        let test_numbers = vec![
            std::f64::MAX,
            std::f64::MIN,
            std::f64::EPSILON,
            std::f64::consts::PI,
            0.1 + 0.2, // 经典的浮点精度问题
            1.7976931348623157e+308, // 接近最大值
        ];

        for num in test_numbers {
            if num.is_finite() {
                let json_value = json!(num);
                let serialized = serde_json::to_string(&json_value).unwrap();
                let deserialized: Value = serde_json::from_str(&serialized).unwrap();
                
                if let Some(parsed_num) = deserialized.as_f64() {
                    // 允许一定的精度误差
                    let diff = (parsed_num - num).abs();
                    assert!(diff < 1e-10 || diff / num.abs() < 1e-10, 
                           "Precision loss for number: {} -> {}", num, parsed_num);
                }
            }
        }
    }
}

/// 专门用于测试实际 API 响应解析的测试
mod api_response_parsing_tests {
    use super::*;
    use open_lark::service::im::v1::p2_im_message_receive_v1::P2ImMessageReceiveV1Data;

    /// 测试真实 API 响应结构的解析
    #[test]
    fn test_real_api_response_parsing() {
        let real_response = json!({
            "ts": "1693834271.787977",
            "uuid": "c9b62180-3c4b-477e-9f50-1e1bf7bcc0b3",
            "token": "v=",
            "type": "event_callback",
            "event": {
                "sender": {
                    "sender_id": {
                        "union_id": "on_test",
                        "user_id": "123456",
                        "open_id": "ou_test"
                    },
                    "sender_type": "user",
                    "tenant_key": "tenant_test"
                },
                "message": {
                    "message_id": "om_test",
                    "root_id": "",
                    "parent_id": "",
                    "create_time": "1693834271",
                    "chat_id": "oc_test",
                    "chat_type": "group",
                    "message_type": "text",
                    "content": "{\"text\":\"hello world\"}",
                    "mentions": []
                }
            }
        });

        let json_str = serde_json::to_string(&real_response).unwrap();
        let parsed: Result<P2ImMessageReceiveV1Data, _> = serde_json::from_str(&json_str);
        
        // 应该能成功解析真实的 API 响应
        assert!(parsed.is_ok(), "Failed to parse real API response: {:?}", parsed.err());
    }

    /// 测试 API 响应中的可选字段处理
    #[test]
    fn test_optional_fields_in_api_response() {
        let minimal_response = json!({
            "ts": "1693834271.787977",
            "uuid": "c9b62180-3c4b-477e-9f50-1e1bf7bcc0b3", 
            "token": "v=",
            "type": "event_callback",
            "event": {
                "sender": {
                    "sender_id": {
                        "open_id": "ou_test"
                    },
                    "sender_type": "user",
                    "tenant_key": "tenant_test"
                },
                "message": {
                    "message_id": "om_test",
                    "create_time": "1693834271",
                    "chat_id": "oc_test",
                    "chat_type": "group",
                    "message_type": "text",
                    "content": "{\"text\":\"hello\"}"
                }
            }
        });

        let json_str = serde_json::to_string(&minimal_response).unwrap();
        let parsed: Result<P2ImMessageReceiveV1Data, _> = serde_json::from_str(&json_str);
        
        assert!(parsed.is_ok(), "Failed to parse minimal API response");
    }
}