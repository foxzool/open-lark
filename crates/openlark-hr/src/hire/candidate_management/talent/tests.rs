#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod talent_tests {
    use openlark_core::validation::ValidateBuilder;
    use crate::hire::candidate_management::talent::{
        TalentCreateRequestBuilder, TalentListRequestBuilder,
    };

    #[test]
    fn test_talent_create_builder_valid_input() {
        let request = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .with_email("zhangsan@example.com")
            .with_phone("13800138000")
            .with_work_experience(5)
            .with_education("本科")
            .with_current_company("阿里巴巴")
            .with_current_position("高级工程师")
            .with_expected_salary("20-25K")
            .with_tags(vec!["Java".to_string(), "Spring".to_string()])
            .build()
            .expect("Should build valid talent request");

        assert_eq!(request.name, "张三");
        assert_eq!(request.email, Some("zhangsan@example.com".to_string()));
        assert_eq!(request.phone, Some("13800138000".to_string()));
        assert_eq!(request.work_experience, Some(5));
        assert_eq!(request.education, Some("本科".to_string()));
        assert_eq!(request.current_company, Some("阿里巴巴".to_string()));
        assert_eq!(request.current_position, Some("高级工程师".to_string()));
        assert_eq!(request.expected_salary, Some("20-25K".to_string()));
        assert_eq!(request.tags, vec!["java".to_string(), "spring".to_string()]);
    }

    #[test]
    fn test_talent_create_builder_missing_name() {
        let result = TalentCreateRequestBuilder::default()
            .with_email("test@example.com")
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("name is required"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_name_too_short() {
        let result = TalentCreateRequestBuilder::default()
            .with_name("张")
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid name"));
                assert!(msg.contains("must be at least"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_name_too_long() {
        let long_name = "张".repeat(101); // 101 characters
        let result = TalentCreateRequestBuilder::default()
            .with_name(&long_name)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid name"));
                assert!(msg.contains("must not exceed"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_name_special_chars() {
        let result = TalentCreateRequestBuilder::default()
            .with_name("张三@#$%")
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid name"));
                assert!(msg.contains("invalid characters"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_email() {
        let result = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .with_email("invalid-email")
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid email"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_phone() {
        let result = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .with_phone("123") // Too short
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid phone"));
                assert!(msg.contains("must be at least"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_work_experience() {
        let result = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .with_work_experience(60) // Exceeds maximum
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid work experience"));
                assert!(msg.contains("must not exceed"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_birthday_format() {
        let result = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .with_birthday("2023-13-32") // Invalid date
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid birthday"));
                assert!(msg.contains("YYYY-MM-DD"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_expected_salary() {
        let result = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .with_expected_salary("1000K") // Unreasonably high
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid expected salary"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_tag_too_long() {
        let long_tag = "a".repeat(51); // 51 characters
        let result = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .add_tag(&long_tag)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid tag at index 0"));
                assert!(msg.contains("must not exceed"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_too_many_tags() {
        let mut builder = TalentCreateRequestBuilder::default().with_name("张三");

        // Add 21 tags (exceeds maximum of 20)
        for i in 0..21 {
            builder = builder.add_tag(&format!("tag{}", i));
        }

        let result = builder.build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid tags"));
                assert!(msg.contains("maximum number"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_resume_attachment() {
        let result = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .add_resume_attachment("") // Empty attachment ID
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid resume attachment"));
                assert!(msg.contains("cannot be empty"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_custom_field_key() {
        let mut custom_fields = std::collections::HashMap::new();
        custom_fields.insert(
            "".to_string(),
            serde_json::Value::String("value".to_string()),
        ); // Empty key

        let result = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .with_custom_fields(custom_fields)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            openlark_core::error::LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid custom fields"));
                assert!(msg.contains("key cannot be empty"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_talent_create_builder_name_sanitization() {
        let request = TalentCreateRequestBuilder::default()
            .with_name("  张三  ")
            .with_tags(vec!["  Java  ".to_string()])
            .build()
            .expect("Should build valid talent request");

        // Name should be trimmed
        assert_eq!(request.name, "张三");
        // Tags should be trimmed and lowercased
        assert_eq!(request.tags, vec!["java".to_string()]);
    }

    #[test]
    fn test_talent_list_builder() {
        let request = TalentListRequestBuilder::default()
            .with_page_size(50)
            .with_name_keyword("张")
            .with_work_experience(5)
            .with_tags(vec!["Java".to_string()])
            .with_created_start_time("2023-01-01")
            .with_created_end_time("2023-12-31")
            .build();

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.name_keyword, Some("张".to_string()));
        assert_eq!(request.work_experience, Some(5));
        assert_eq!(request.tags, vec!["Java".to_string()]);
        assert_eq!(request.created_start_time, Some("2023-01-01".to_string()));
        assert_eq!(request.created_end_time, Some("2023-12-31".to_string()));
    }

    #[test]
    fn test_talent_create_builder_with_custom_fields() {
        let mut custom_fields = std::collections::HashMap::new();
        custom_fields.insert(
            "source".to_string(),
            serde_json::Value::String("招聘网站".to_string()),
        );
        custom_fields.insert(
            "rating".to_string(),
            serde_json::Value::Number(serde_json::Number::from(5)),
        );

        let request = TalentCreateRequestBuilder::default()
            .with_name("李四")
            .with_custom_fields(custom_fields)
            .build()
            .expect("Should build valid talent request");

        assert!(request.custom_fields.is_some());
        let fields = request.custom_fields.as_ref().unwrap();
        assert_eq!(
            fields.get("source"),
            Some(&serde_json::Value::String("招聘网站".to_string()))
        );
        assert_eq!(
            fields.get("rating"),
            Some(&serde_json::Value::Number(serde_json::Number::from(5)))
        );
    }

    #[test]
    fn test_talent_create_builder_with_resume_attachments() {
        let request = TalentCreateRequestBuilder::default()
            .with_name("王五")
            .add_resume_attachment("attachment_1")
            .add_resume_attachment("attachment_2")
            .with_resume_attachments(vec!["attachment_3".to_string(), "attachment_4".to_string()])
            .build()
            .expect("Should build valid talent request");

        assert_eq!(
            request.resume_attachment_ids,
            vec![
                "attachment_1",
                "attachment_2",
                "attachment_3",
                "attachment_4"
            ]
        );
    }

    #[test]
    fn test_talent_create_builder_with_tags() {
        let request = TalentCreateRequestBuilder::default()
            .with_name("赵六")
            .add_tag("Rust")
            .add_tag("Go")
            .with_tags(vec!["Python".to_string(), "JavaScript".to_string()])
            .build()
            .expect("Should build valid talent request");

        assert_eq!(request.tags, vec!["rust", "go", "python", "javascript"]);
    }

    #[test]
    fn test_validation_trait_implementation() {
        let builder = TalentCreateRequestBuilder::default()
            .with_name("测试")
            .with_email("test@example.com");

        // Test ValidateBuilder trait implementation
        let result = builder.validate();
        match result {
            openlark_core::validation::ValidationResult::Valid => {}
            _ => panic!("Expected valid validation result"),
        }

        // Test invalid case
        let builder = TalentCreateRequestBuilder::default();
        let result = builder.validate();
        match result {
            openlark_core::validation::ValidationResult::Invalid(msg) => {
                assert!(msg.contains("name is required"));
            }
            _ => panic!("Expected invalid validation result"),
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test with minimum valid work experience
        let request = TalentCreateRequestBuilder::default()
            .with_name("测试")
            .with_work_experience(0)
            .build();
        assert!(request.is_ok());

        // Test with maximum valid work experience
        let request = TalentCreateRequestBuilder::default()
            .with_name("测试")
            .with_work_experience(50)
            .build();
        assert!(request.is_ok());

        // Test with minimum valid name length
        let request = TalentCreateRequestBuilder::default()
            .with_name("张三") // 2 characters
            .build();
        assert!(request.is_ok());

        // Test with maximum valid name length
        let name = "张".repeat(100); // 100 characters
        let request = TalentCreateRequestBuilder::default()
            .with_name(&name)
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_partial_updates() {
        // Test that optional fields can be omitted
        let request = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .build()
            .expect("Should build valid talent request with only required fields");

        assert_eq!(request.name, "张三");
        assert_eq!(request.email, None);
        assert_eq!(request.phone, None);
        assert_eq!(request.work_experience, None);
        assert_eq!(request.tags, Vec::<String>::new());
    }
}
