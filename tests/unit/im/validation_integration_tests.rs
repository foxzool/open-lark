//! 验证功能集成测试
//!
//! 测试验证工具模块与构建器的集成

use rstest::*;
use open_lark::service::im::v1::message::*;
use open_lark::core::validation::*;

/// 验证功能集成测试
#[cfg(test)]
mod validation_integration_tests {
    use super::*;

    #[test]
    fn test_create_message_request_body_builder_validates_uuid_length() {
        // 测试UUID长度验证 - 超长UUID应该被截断
        let long_uuid = "a".repeat(100); // 超过50字符限制
        
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(r#"{"text":"test"}"#)
            .uuid(&long_uuid)
            .build();
        
        // UUID应该被截断到50个字符
        assert_eq!(body.uuid.unwrap().len(), 50);
        assert_eq!(body.uuid.unwrap(), &long_uuid[..50]);
    }

    #[test]
    fn test_create_message_request_body_builder_validates_required_fields() {
        // 测试必填字段验证 - 虽然验证只记录日志，但构建应该成功
        let body = CreateMessageRequestBody::builder()
            .receive_id("")  // 空 receive_id
            .msg_type("")    // 空 msg_type
            .content("")     // 空 content
            .build();
        
        // 构建应该成功，但字段为空
        assert_eq!(body.receive_id, "");
        assert_eq!(body.msg_type, "");
        assert_eq!(body.content, "");
    }

    #[test]
    fn test_create_message_request_body_builder_validates_text_message_size() {
        // 测试文本消息大小验证
        let large_content = r#"{"text":""#.to_owned() + &"a".repeat(160 * 1024) + r#""}"#; // 超过150KB
        
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("text")
            .content(&large_content)
            .build();
        
        // 内容应该保持原样（验证只记录日志）
        assert!(body.content.len() > 150 * 1024);
    }

    #[test]
    fn test_create_message_request_body_builder_validates_post_message_size() {
        // 测试富文本消息大小验证
        let large_content = r#"{"post":{"zh_cn":{"title":""#.to_owned() + &"a".repeat(35 * 1024) + r#""}}}"#; // 超过30KB
        
        let body = CreateMessageRequestBody::builder()
            .receive_id("ou_test")
            .msg_type("post")
            .content(&large_content)
            .build();
        
        // 内容应该保持原样（验证只记录日志）
        assert!(body.content.len() > 30 * 1024);
    }

    #[test]
    fn test_validation_functions_directly() {
        // 直接测试验证函数
        
        // 测试字符串长度验证
        let long_string = "a".repeat(100);
        let validated = validate_string_length(long_string.clone(), 50, "test");
        assert_eq!(validated.len(), 50);
        assert_eq!(validated, &long_string[..50]);
        
        // 测试必填字段验证
        assert!(validate_required("non-empty", "test"));
        assert!(!validate_required("", "test"));
        
        // 测试内容大小验证
        assert!(validate_content_size("small content", 100, "test"));
        assert!(!validate_content_size("a".repeat(200), 100, "test"));
    }

    #[test]
    fn test_validation_result_enum() {
        // 测试ValidationResult枚举
        
        let valid = ValidationResult::Valid;
        assert!(valid.is_valid());
        assert!(valid.is_strictly_valid());
        assert!(valid.error().is_none());
        
        let warning = ValidationResult::Warning("test warning".to_string());
        assert!(warning.is_valid());
        assert!(!warning.is_strictly_valid());
        assert_eq!(warning.error(), Some("test warning"));
        
        let invalid = ValidationResult::Invalid("test error".to_string());
        assert!(!invalid.is_valid());
        assert!(!invalid.is_strictly_valid());
        assert_eq!(invalid.error(), Some("test error"));
    }
}