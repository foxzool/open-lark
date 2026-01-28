//! 卡片组件构建器单元测试
//!
//! 测试卡片组件的构建器模式，包括参数验证、链式调用、默认值等。

use openlark_cardkit::cardkit::cardkit::v1::card::element::{
    content::{UpdateElementContentBody, UpdateElementContentRequestBuilder},
    create::{CreateElementBody, CreateElementRequestBuilder},
    delete::{DeleteElementBody, DeleteElementRequestBuilder},
    patch::{PatchElementBody, PatchElementRequestBuilder},
    update::{UpdateElementBody, UpdateElementRequestBuilder},
};
use rstest::*;
use serde_json::json;

/// 创建卡片组件请求构建器测试
#[cfg(test)]
mod create_element_request_builder_tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = CreateElementRequestBuilder::new(config.clone(), "card_123".to_string());

        assert_eq!(builder.card_id, "card_123");
    }

    #[test]
    fn test_builder_element_content_setting() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let element_content = json!({"type": "text", "content": "hello"});

        let builder = CreateElementRequestBuilder::new(config.clone(), "card_123".to_string())
            .element_content(element_content.clone());

        assert!(builder.element_content.is_some());
        assert_eq!(builder.element_content.unwrap(), element_content);
    }

    #[test]
    fn test_builder_element_id_setting() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = CreateElementRequestBuilder::new(config.clone(), "card_123".to_string())
            .element_id("elem_456");

        assert_eq!(builder.element_id, Some("elem_456".to_string()));
    }

    #[test]
    fn test_builder_root_id_setting() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = CreateElementRequestBuilder::new(config.clone(), "card_123".to_string())
            .root_id("root_789");

        assert_eq!(builder.root_id, Some("root_789".to_string()));
    }

    #[test]
    fn test_builder_chaining() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let element_content = json!({"type": "text"});

        let result = CreateElementRequestBuilder::new(config.clone(), "card_123".to_string())
            .element_id("elem_456")
            .element_content(element_content.clone())
            .root_id("root_789")
            .build();

        assert!(result.config.app_id() == "test_app_id");
    }

    #[rstest]
    #[case("text")]
    #[case("image")]
    #[case("button")]
    #[case("divider")]
    fn test_builder_element_type_variants(#[case] element_type: &str) {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let element_content = json!({"type": element_type});

        let builder = CreateElementRequestBuilder::new(config.clone(), "card_123".to_string())
            .element_content(element_content);

        assert!(builder.element_content.is_some());
    }
}

/// 创建组件体验证测试
#[cfg(test)]
mod create_element_body_validation_tests {
    use super::*;

    #[test]
    fn test_valid_element_body() {
        let body = CreateElementBody {
            card_id: "card_123".to_string(),
            element_content: json!({"type": "text", "content": "hello"}),
            element_id: None,
            root_id: None,
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_empty_card_id_validation() {
        let body = CreateElementBody {
            card_id: "   ".to_string(), // 只有空白字符
            element_content: json!({"type": "text"}),
            element_id: None,
            root_id: None,
        };

        let result = body.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("card_id 不能为空"));
    }

    #[test]
    fn test_null_element_content_validation() {
        let body = CreateElementBody {
            card_id: "card_123".to_string(),
            element_content: serde_json::Value::Null,
            element_id: None,
            root_id: None,
        };

        let result = body.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("element_content 不能为空"));
    }

    #[test]
    fn test_non_object_element_content_validation() {
        let body = CreateElementBody {
            card_id: "card_123".to_string(),
            element_content: json!("string content"),
            element_id: None,
            root_id: None,
        };

        let result = body.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("element_content 必须是 JSON 对象"));
    }
}

/// 更新卡片组件请求构建器测试
#[cfg(test)]
mod update_element_request_builder_tests {
    use super::*;

    #[test]
    fn test_update_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = UpdateElementRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        );

        assert_eq!(builder.card_id, "card_123");
        assert_eq!(builder.element_id, "elem_456");
    }

    #[test]
    fn test_update_builder_settings() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let new_content = json!({"updated": true});

        let builder = UpdateElementRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        )
        .element_content(new_content.clone());

        assert!(builder.element_content.is_some());
    }
}

/// 更新组件体验证测试
#[cfg(test)]
mod update_element_body_validation_tests {
    use super::*;

    #[test]
    fn test_valid_update_body() {
        let body = UpdateElementBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
            element_content: json!({"updated": true}),
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_empty_card_id_validation() {
        let body = UpdateElementBody {
            card_id: "".to_string(),
            element_id: "elem_456".to_string(),
            element_content: json!({}),
        };

        let result = body.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("card_id 不能为空"));
    }

    #[test]
    fn test_empty_element_id_validation() {
        let body = UpdateElementBody {
            card_id: "card_123".to_string(),
            element_id: "  ".to_string(),
            element_content: json!({}),
        };

        let result = body.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("element_id 不能为空"));
    }
}

/// 修补卡片组件请求构建器测试
#[cfg(test)]
mod patch_element_request_builder_tests {
    use super::*;

    #[test]
    fn test_patch_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = PatchElementRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        );

        assert_eq!(builder.card_id, "card_123");
        assert_eq!(builder.element_id, "elem_456");
    }

    #[test]
    fn test_patch_builder_operations() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let operation = json!({
            "operation": "replace",
            "path": "/content",
            "value": {"new": "content"}
        });

        let builder = PatchElementRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        )
        .operation(operation.clone());

        assert_eq!(builder.operations.len(), 1);
    }

    #[test]
    fn test_patch_builder_multiple_operations() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let op1 = json!({"operation": "replace", "path": "/content"});
        let op2 = json!({"operation": "remove", "path": "/extra"});

        let builder = PatchElementRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        )
        .operation(op1.clone())
        .operation(op2.clone());

        assert_eq!(builder.operations.len(), 2);
    }
}

/// 删除卡片组件请求构建器测试
#[cfg(test)]
mod delete_element_request_builder_tests {
    use super::*;

    #[test]
    fn test_delete_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = DeleteElementRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        );

        assert_eq!(builder.card_id, "card_123");
        assert_eq!(builder.element_id, "elem_456");
    }

    #[test]
    fn test_delete_builder_with_topic_id() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let builder = DeleteElementRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        )
        .topic_id("topic_789");

        assert_eq!(builder.topic_id, Some("topic_789".to_string()));
    }
}

/// 删除组件体验证测试
#[cfg(test)]
mod delete_element_body_validation_tests {
    use super::*;

    #[test]
    fn test_valid_delete_body() {
        let body = DeleteElementBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
            topic_id: None,
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_empty_card_id_validation() {
        let body = DeleteElementBody {
            card_id: "".to_string(),
            element_id: "elem_456".to_string(),
            topic_id: None,
        };

        let result = body.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("card_id 不能为空"));
    }

    #[test]
    fn test_empty_element_id_validation() {
        let body = DeleteElementBody {
            card_id: "card_123".to_string(),
            element_id: "   ".to_string(),
            topic_id: None,
        };

        let result = body.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("element_id 不能为空"));
    }
}

/// 更新组件内容请求构建器测试
#[cfg(test)]
mod update_element_content_request_builder_tests {
    use super::*;

    #[test]
    fn test_content_builder_default_state() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = UpdateElementContentRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        );

        assert_eq!(builder.card_id, "card_123");
        assert_eq!(builder.element_id, "elem_456");
    }

    #[test]
    fn test_content_builder_content_setting() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let content = json!({"type": "text", "content": "updated content"});

        let builder = UpdateElementContentRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        )
        .content(content.clone());

        assert!(builder.content.is_some());
    }

    #[test]
    fn test_content_builder_chaining() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let content = json!({"type": "text"});

        let result = UpdateElementContentRequestBuilder::new(
            config.clone(),
            "card_123".to_string(),
            "elem_456".to_string(),
        )
        .content(content.clone())
        .build();

        assert!(result.config.app_id() == "test_app_id");
    }
}
