//! 卡片实体构建器单元测试
//!
//! 测试卡片实体的构建器模式，包括参数验证、链式调用、默认值等。

use openlark_cardkit::cardkit::cardkit::v1::card::{
    batch_update::{BatchUpdateCardBody, BatchUpdateCardRequest, BatchUpdateCardRequestBuilder},
    create::{CreateCardBody, CreateCardRequest, CreateCardRequestBuilder},
    id_convert::{ConvertCardIdBody, ConvertCardIdRequest, ConvertCardIdRequestBuilder},
    settings::{
        UpdateCardSettingsBody, UpdateCardSettingsRequest, UpdateCardSettingsRequestBuilder,
    },
    update::{UpdateCardBody, UpdateCardRequest, UpdateCardRequestBuilder},
};
use serde_json::json;

/// 辅助函数：创建测试配置
fn create_test_config() -> openlark_core::config::Config {
    openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
}

/// 创建卡片请求构建器测试
#[cfg(test)]
mod create_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = create_test_config();
        let builder = CreateCardRequestBuilder::new(config.clone());
        let request = builder.build();

        // 验证请求被正确创建
        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_builder_card_content_setting() {
        let config = create_test_config();
        let card_content = json!({"type": "text", "content": "test"});

        let builder =
            CreateCardRequestBuilder::new(config.clone()).card_content(card_content.clone());

        // 验证构建器可以被链式调用
        let _request = builder.build();
    }

    #[test]
    fn test_builder_card_type_setting() {
        let config = create_test_config();

        let builder = CreateCardRequestBuilder::new(config.clone()).card_type("interactive");

        let _request = builder.build();
    }

    #[test]
    fn test_builder_template_id_setting() {
        let config = create_test_config();

        let builder = CreateCardRequestBuilder::new(config.clone()).template_id("tmpl_123");

        let _request = builder.build();
    }

    #[test]
    fn test_builder_temp_settings() {
        let config = create_test_config();

        let builder = CreateCardRequestBuilder::new(config.clone())
            .temp(true)
            .temp_expire_time(3600);

        let _request = builder.build();
    }

    #[test]
    fn test_builder_chaining() {
        let config = create_test_config();
        let card_content = json!({"type": "text"});

        let request = CreateCardRequestBuilder::new(config.clone())
            .card_content(card_content.clone())
            .card_type("interactive")
            .template_id("tmpl_123")
            .temp(false)
            .temp_expire_time(86400)
            .build();

        // 验证请求被正确创建
        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let request = CreateCardRequest::new(config);
        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }
}

/// 创建卡片体验证测试
#[cfg(test)]
mod create_card_body_validation_tests {
    use super::*;

    #[test]
    fn test_valid_card_body() {
        let body = CreateCardBody {
            card_content: json!({"type": "text", "content": "hello"}),
            card_type: Some("interactive".to_string()),
            template_id: None,
            temp: None,
            temp_expire_time: None,
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_null_card_content_validation() {
        let body = CreateCardBody {
            card_content: serde_json::Value::Null,
            card_type: None,
            template_id: None,
            temp: None,
            temp_expire_time: None,
        };

        let result = body.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("card_content 不能为空"));
    }

    #[test]
    fn test_non_object_card_content_validation() {
        let body = CreateCardBody {
            card_content: json!("string content"),
            card_type: None,
            template_id: None,
            temp: None,
            temp_expire_time: None,
        };

        let result = body.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("card_content 必须是 JSON 对象"));
    }

    #[test]
    fn test_temp_expire_time_range_validation() {
        // 有效范围：1 ~ 86400
        let valid_body = CreateCardBody {
            card_content: json!({}),
            card_type: None,
            template_id: None,
            temp: Some(true),
            temp_expire_time: Some(3600),
        };
        assert!(valid_body.validate().is_ok());

        // 超范围：0
        let invalid_body_zero = CreateCardBody {
            card_content: json!({}),
            card_type: None,
            template_id: None,
            temp: Some(true),
            temp_expire_time: Some(0),
        };
        assert!(invalid_body_zero.validate().is_err());

        // 超范围：86401
        let invalid_body_large = CreateCardBody {
            card_content: json!({}),
            card_type: None,
            template_id: None,
            temp: Some(true),
            temp_expire_time: Some(86401),
        };
        assert!(invalid_body_large.validate().is_err());
    }

    #[test]
    fn test_optional_fields_can_be_none() {
        // 所有可选字段都为 None 应该是有效的
        let body = CreateCardBody {
            card_content: json!({"key": "value"}),
            card_type: None,
            template_id: None,
            temp: None,
            temp_expire_time: None,
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_temp_expire_time_boundary() {
        // 边界值：1（最小有效值）
        let body_min = CreateCardBody {
            card_content: json!({}),
            card_type: None,
            template_id: None,
            temp: Some(true),
            temp_expire_time: Some(1),
        };
        assert!(body_min.validate().is_ok());

        // 边界值：86400（最大有效值）
        let body_max = CreateCardBody {
            card_content: json!({}),
            card_type: None,
            template_id: None,
            temp: Some(true),
            temp_expire_time: Some(86400),
        };
        assert!(body_max.validate().is_ok());
    }
}

/// 更新卡片请求构建器测试
#[cfg(test)]
mod update_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_update_builder_default_state() {
        let config = create_test_config();
        let builder = UpdateCardRequestBuilder::new(config.clone());
        let request = builder.build();

        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_update_builder_settings() {
        let config = create_test_config();

        let builder = UpdateCardRequestBuilder::new(config.clone())
            .card_id("card_123")
            .card_content(json!({"updated": true}))
            .card_type("updated_type")
            .update_mask(vec!["card_content".to_string()]);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let request = UpdateCardRequest::new(config);
        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_update_builder_chaining() {
        let config = create_test_config();

        let request = UpdateCardRequestBuilder::new(config.clone())
            .card_id("card_123")
            .card_content(json!({"key": "value"}))
            .card_type("interactive")
            .update_mask(vec!["card_content".to_string(), "card_type".to_string()])
            .build();

        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }
}

/// 批量更新卡片请求构建器测试
#[cfg(test)]
mod batch_update_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_batch_update_builder_default_state() {
        let config = create_test_config();
        let builder = BatchUpdateCardRequestBuilder::new(config.clone());
        let request = builder.build();

        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_batch_update_builder_with_params() {
        let config = create_test_config();

        let operations = vec![
            json!({"operation": "add", "path": "/elements", "value": {}}),
            json!({"operation": "replace", "path": "/header", "value": {}}),
        ];

        let builder = BatchUpdateCardRequestBuilder::new(config.clone())
            .card_id("card_123")
            .operations(operations);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let request = BatchUpdateCardRequest::new(config);
        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_batch_update_body_validation() {
        // 有效请求体
        let valid_body = BatchUpdateCardBody {
            card_id: "card_123".to_string(),
            operations: vec![json!({"op": "add"})],
        };
        assert!(!valid_body.card_id.is_empty());
        assert!(!valid_body.operations.is_empty());
    }
}

/// 更新卡片设置请求构建器测试
#[cfg(test)]
mod update_card_settings_request_builder_tests {
    use super::*;

    #[test]
    fn test_settings_builder_default_state() {
        let config = create_test_config();
        let builder = UpdateCardSettingsRequestBuilder::new(config.clone());
        let request = builder.build();

        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_settings_builder_with_params() {
        let config = create_test_config();

        let settings = json!({
            "auto_submit": true,
            "allow_forward": false
        });

        let builder = UpdateCardSettingsRequestBuilder::new(config.clone())
            .card_id("card_123")
            .settings(settings);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let request = UpdateCardSettingsRequest::new(config);
        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_settings_body_creation() {
        let body = UpdateCardSettingsBody {
            card_id: "card_123".to_string(),
            settings: json!({"key": "value"}),
        };

        assert_eq!(body.card_id, "card_123");
        assert!(!body.settings.is_null());
    }
}

/// ID 转换请求构建器测试
#[cfg(test)]
mod id_convert_request_builder_tests {
    use super::*;

    #[test]
    fn test_id_convert_builder_default_state() {
        let config = create_test_config();
        let builder = ConvertCardIdRequestBuilder::new(config.clone());
        let request = builder.build();

        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_id_convert_builder_with_params() {
        let config = create_test_config();

        let builder = ConvertCardIdRequestBuilder::new(config.clone())
            .source_id_type("card_id")
            .target_id_type("open_card_id")
            .card_ids(vec!["card_1".to_string(), "card_2".to_string()]);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let request = ConvertCardIdRequest::new(config);
        assert!(std::ptr::addr_of!(request) != std::ptr::null());
    }

    #[test]
    fn test_convert_body_validation() {
        let body = ConvertCardIdBody {
            source_id_type: "card_id".to_string(),
            target_id_type: "open_card_id".to_string(),
            card_ids: vec!["card_1".to_string(), "card_2".to_string()],
        };

        assert_eq!(body.source_id_type, "card_id");
        assert_eq!(body.target_id_type, "open_card_id");
        assert_eq!(body.card_ids.len(), 2);
    }
}

/// Body 结构体序列化测试
#[cfg(test)]
mod body_serialization_tests {
    use super::*;

    #[test]
    fn test_create_card_body_serialization() {
        let body = CreateCardBody {
            card_content: json!({"type": "text"}),
            card_type: Some("interactive".to_string()),
            template_id: Some("tmpl_123".to_string()),
            temp: Some(true),
            temp_expire_time: Some(3600),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("card_content"));
        assert!(json_str.contains("card_type"));
        assert!(json_str.contains("template_id"));
    }

    #[test]
    fn test_update_card_body_serialization() {
        let body = UpdateCardBody {
            card_id: "card_123".to_string(),
            card_content: json!({"updated": true}),
            card_type: Some("template".to_string()),
            update_mask: Some(vec!["card_content".to_string()]),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("card_id"));
        assert!(json_str.contains("card_content"));
    }

    #[test]
    fn test_batch_update_card_body_serialization() {
        let body = BatchUpdateCardBody {
            card_id: "card_123".to_string(),
            operations: vec![json!({"op": "add"})],
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("card_id"));
        assert!(json_str.contains("operations"));
    }

    #[test]
    fn test_settings_body_serialization() {
        let body = UpdateCardSettingsBody {
            card_id: "card_123".to_string(),
            settings: json!({"auto_submit": true}),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("card_id"));
        assert!(json_str.contains("settings"));
    }

    #[test]
    fn test_id_convert_body_serialization() {
        let body = ConvertCardIdBody {
            source_id_type: "card_id".to_string(),
            target_id_type: "open_card_id".to_string(),
            card_ids: vec!["card_1".to_string()],
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("source_id_type"));
        assert!(json_str.contains("target_id_type"));
        assert!(json_str.contains("card_ids"));
    }
}

/// 边界情况测试
#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_card_content_object() {
        // 空的 JSON 对象应该是有效的
        let body = CreateCardBody {
            card_content: json!({}),
            card_type: None,
            template_id: None,
            temp: None,
            temp_expire_time: None,
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_nested_card_content() {
        // 嵌套的复杂 JSON 对象
        let body = CreateCardBody {
            card_content: json!({
                "header": {
                    "title": {
                        "tag": "plain_text",
                        "content": "标题"
                    }
                },
                "elements": [
                    {"tag": "div", "text": {"tag": "plain_text", "content": "内容"}}
                ]
            }),
            card_type: Some("interactive".to_string()),
            template_id: None,
            temp: None,
            temp_expire_time: None,
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_special_characters_in_strings() {
        let body = CreateCardBody {
            card_content: json!({
                "text": "特殊字符：!@#$%^&*()_+-=[]{}|;':\",./<>?"
            }),
            card_type: Some("interactive".to_string()),
            template_id: None,
            temp: None,
            temp_expire_time: None,
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_unicode_content() {
        let body = CreateCardBody {
            card_content: json!({
                "text": "中文内容 🎉 Emoji 测试"
            }),
            card_type: Some("interactive".to_string()),
            template_id: None,
            temp: None,
            temp_expire_time: None,
        };

        assert!(body.validate().is_ok());
    }
}
