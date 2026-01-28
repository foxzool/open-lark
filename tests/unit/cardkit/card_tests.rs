//! 卡片实体构建器单元测试
//!
//! 测试卡片实体的构建器模式，包括参数验证、链式调用、默认值等。

use openlark_cardkit::cardkit::cardkit::v1::card::{
    batch_update::{BatchUpdateCardBody, BatchUpdateCardRequestBuilder},
    create::{CreateCardBody, CreateCardRequestBuilder},
    id_convert::{IdConvertBody, IdConvertRequestBuilder},
    settings::{UpdateCardSettingsBody, UpdateCardSettingsRequestBuilder},
    update::{UpdateCardBody, UpdateCardRequestBuilder},
};
use rstest::*;
use serde_json::json;

/// 创建卡片请求构建器测试
#[cfg(test)]
mod create_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = CreateCardRequestBuilder::new(config.clone());
        let request = builder.build();

        // 默认状态应该有空的请求参数
        assert!(request.config.app_id() == "test_app_id");
    }

    #[test]
    fn test_builder_card_content_setting() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let card_content = json!({"type": "text", "content": "test"});

        let builder =
            CreateCardRequestBuilder::new(config.clone()).card_content(card_content.clone());

        // 验证构建器中设置了值
        assert!(builder.card_content.is_some());
    }

    #[test]
    fn test_builder_card_type_setting() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = CreateCardRequestBuilder::new(config.clone()).card_type("interactive");

        assert_eq!(builder.card_type, Some("interactive".to_string()));
    }

    #[test]
    fn test_builder_template_id_setting() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = CreateCardRequestBuilder::new(config.clone()).template_id("tmpl_123");

        assert_eq!(builder.template_id, Some("tmpl_123".to_string()));
    }

    #[test]
    fn test_builder_temp_settings() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = CreateCardRequestBuilder::new(config.clone())
            .temp(true)
            .temp_expire_time(3600);

        assert_eq!(builder.temp, Some(true));
        assert_eq!(builder.temp_expire_time, Some(3600));
    }

    #[test]
    fn test_builder_chaining() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let card_content = json!({"type": "text"});

        let result = CreateCardRequestBuilder::new(config.clone())
            .card_content(card_content.clone())
            .card_type("interactive")
            .template_id("tmpl_123")
            .temp(false)
            .temp_expire_time(86400)
            .build();

        assert!(result.config.app_id() == "test_app_id");
    }

    #[rstest]
    #[case("interactive")]
    #[case("template")]
    #[case("card")]
    fn test_builder_card_type_variants(#[case] card_type: &str) {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = CreateCardRequestBuilder::new(config.clone()).card_type(card_type);

        assert_eq!(builder.card_type, Some(card_type.to_string()));
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
}

/// 更新卡片请求构建器测试
#[cfg(test)]
mod update_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_update_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = UpdateCardRequestBuilder::new(config.clone(), "card_123".to_string());

        assert_eq!(builder.card_id, "card_123");
    }

    #[test]
    fn test_update_builder_settings() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = UpdateCardRequestBuilder::new(config.clone(), "card_123".to_string())
            .card_content(json!({"updated": true}))
            .card_type("updated_type");

        assert_eq!(builder.card_id, "card_123");
        assert!(builder.card_content.is_some());
        assert_eq!(builder.card_type, Some("updated_type".to_string()));
    }
}

/// 批量更新卡片请求构建器测试
#[cfg(test)]
mod batch_update_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_batch_update_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = BatchUpdateCardRequestBuilder::new(config.clone(), "card_123".to_string());

        assert_eq!(builder.card_id, "card_123");
        assert!(builder.operations.is_empty());
    }

    #[test]
    fn test_batch_update_add_operation() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let operation = json!({
            "operation": "add",
            "element_id": "elem_1",
            "element_content": {"type": "text", "content": "new"}
        });

        let builder = BatchUpdateCardRequestBuilder::new(config.clone(), "card_123".to_string())
            .operation(operation.clone());

        assert_eq!(builder.operations.len(), 1);
        assert_eq!(builder.operations[0], operation);
    }

    #[test]
    fn test_batch_update_multiple_operations() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let op1 = json!({"operation": "add", "element_id": "elem_1"});
        let op2 = json!({"operation": "delete", "element_id": "elem_2"});
        let op3 = json!({"operation": "replace", "element_id": "elem_3", "element_content": {}});

        let builder = BatchUpdateCardRequestBuilder::new(config.clone(), "card_123".to_string())
            .operation(op1.clone())
            .operation(op2.clone())
            .operation(op3.clone());

        assert_eq!(builder.operations.len(), 3);
    }
}

/// 更新卡片设置请求构建器测试
#[cfg(test)]
mod update_card_settings_request_builder_tests {
    use super::*;

    #[test]
    fn test_settings_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = UpdateCardSettingsRequestBuilder::new(config.clone(), "card_123".to_string());

        assert_eq!(builder.card_id, "card_123");
    }

    #[test]
    fn test_settings_builder_greeting_settings() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let greeting = json!({
            "type": "text",
            "content": "Welcome!"
        });

        let builder = UpdateCardSettingsRequestBuilder::new(config.clone(), "card_123".to_string())
            .greeting(greeting.clone())
            .auto_freeze(false)
            .share_enabled(true);

        assert!(builder.greeting.is_some());
        assert_eq!(builder.auto_freeze, Some(false));
        assert_eq!(builder.share_enabled, Some(true));
    }
}

/// ID 转换请求构建器测试
#[cfg(test)]
mod id_convert_request_builder_tests {
    use super::*;

    #[test]
    fn test_id_convert_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = IdConvertRequestBuilder::new(
            config.clone(),
            "user_id".to_string(),
            "open_id".to_string(),
        );

        assert_eq!(builder.source_id_type, "user_id");
        assert_eq!(builder.target_id_type, "open_id");
        assert!(builder.card_ids.is_empty());
    }

    #[test]
    fn test_id_convert_add_card_ids() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = IdConvertRequestBuilder::new(
            config.clone(),
            "user_id".to_string(),
            "open_id".to_string(),
        )
        .card_id("card_1")
        .card_id("card_2")
        .card_ids(vec!["card_3".to_string(), "card_4".to_string()]);

        assert_eq!(builder.card_ids.len(), 4);
    }
}
