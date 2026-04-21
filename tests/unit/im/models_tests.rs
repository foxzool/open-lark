//! IM 模型单元测试
//!
//! 测试 IM 模块中的所有数据模型，包括序列化、反序列化、枚举转换等

use rstest::*;
use serde_json::{json, Value};
use open_lark::service::im::v1::models::*;

/// 测试消息类型枚举
mod message_type_tests {
    use super::*;
    
    #[rstest]
    #[case(MessageType::Text, r#""text""#)]
    #[case(MessageType::Post, r#""post""#)]
    #[case(MessageType::Image, r#""image""#)]
    #[case(MessageType::File, r#""file""#)]
    #[case(MessageType::Audio, r#""audio""#)]
    #[case(MessageType::Media, r#""media""#)]
    #[case(MessageType::Sticker, r#""sticker""#)]
    #[case(MessageType::Interactive, r#""interactive""#)]
    #[case(MessageType::ShareChat, r#""share_chat""#)]
    #[case(MessageType::ShareUser, r#""share_user""#)]
    #[case(MessageType::System, r#""system""#)]
    fn test_message_type_serialization(#[case] msg_type: MessageType, #[case] expected_json: &str) {
        let serialized = serde_json::to_string(&msg_type).unwrap();
        assert_eq!(serialized, expected_json);
    }

    #[rstest]
    #[case(r#""text""#, MessageType::Text)]
    #[case(r#""post""#, MessageType::Post)]
    #[case(r#""image""#, MessageType::Image)]
    #[case(r#""file""#, MessageType::File)]
    #[case(r#""audio""#, MessageType::Audio)]
    #[case(r#""media""#, MessageType::Media)]
    #[case(r#""sticker""#, MessageType::Sticker)]
    #[case(r#""interactive""#, MessageType::Interactive)]
    #[case(r#""share_chat""#, MessageType::ShareChat)]
    #[case(r#""share_user""#, MessageType::ShareUser)]
    #[case(r#""system""#, MessageType::System)]
    fn test_message_type_deserialization(#[case] json_str: &str, #[case] expected: MessageType) {
        let deserialized: MessageType = serde_json::from_str(json_str).expect("JSON 反序列化失败");
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_message_type_equality() {
        assert_eq!(MessageType::Text, MessageType::Text);
        assert_ne!(MessageType::Text, MessageType::Post);
    }

    #[test]
    fn test_message_type_clone() {
        let original = MessageType::Interactive;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }
}

/// 测试用户ID类型枚举
mod user_id_type_tests {
    use super::*;

    #[rstest]
    #[case(UserIdType::UserId, "user_id")]
    #[case(UserIdType::UnionId, "union_id")]
    #[case(UserIdType::OpenId, "open_id")]
    fn test_user_id_type_as_str(#[case] id_type: UserIdType, #[case] expected: &str) {
        assert_eq!(id_type.as_str(), expected);
    }

    #[rstest]
    #[case(UserIdType::UserId, r#""user_id""#)]
    #[case(UserIdType::UnionId, r#""union_id""#)]
    #[case(UserIdType::OpenId, r#""open_id""#)]
    fn test_user_id_type_serialization(#[case] id_type: UserIdType, #[case] expected: &str) {
        let serialized = serde_json::to_string(&id_type).unwrap();
        assert_eq!(serialized, expected);
    }
}

/// 测试接收ID类型枚举
mod receive_id_type_tests {
    use super::*;

    #[rstest]
    #[case(ReceiveIdType::OpenId, "open_id")]
    #[case(ReceiveIdType::UserId, "user_id")]
    #[case(ReceiveIdType::UnionId, "union_id")]
    #[case(ReceiveIdType::Email, "email")]
    #[case(ReceiveIdType::ChatId, "chat_id")]
    fn test_receive_id_type_as_str(#[case] id_type: ReceiveIdType, #[case] expected: &str) {
        assert_eq!(id_type.as_str(), expected);
    }

    #[test]
    fn test_receive_id_type_comprehensive_coverage() {
        // 确保我们覆盖了所有枚举变体
        let all_types = vec![
            ReceiveIdType::OpenId,
            ReceiveIdType::UserId,
            ReceiveIdType::UnionId,
            ReceiveIdType::Email,
            ReceiveIdType::ChatId,
        ];

        for id_type in all_types {
            // 每个类型都应该有有效的字符串表示
            assert!(!id_type.as_str().is_empty());
            // 每个类型都应该能够序列化
            assert!(serde_json::to_string(&id_type).is_ok());
        }
    }
}

/// 测试批量消息状态枚举
mod batch_message_status_tests {
    use super::*;

    #[rstest]
    #[case(BatchMessageStatus::Processing, 0)]
    #[case(BatchMessageStatus::PartialSuccess, 1)]
    #[case(BatchMessageStatus::Success, 2)]
    #[case(BatchMessageStatus::Failed, 3)]
    fn test_batch_message_status_values(#[case] status: BatchMessageStatus, #[case] expected_value: u8) {
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, expected_value.to_string());
    }

    #[rstest]
    #[case("0", BatchMessageStatus::Processing)]
    #[case("1", BatchMessageStatus::PartialSuccess)]
    #[case("2", BatchMessageStatus::Success)]
    #[case("3", BatchMessageStatus::Failed)]
    fn test_batch_message_status_deserialization(#[case] json_str: &str, #[case] expected: BatchMessageStatus) {
        let deserialized: BatchMessageStatus = serde_json::from_str(json_str).expect("JSON 反序列化失败");
        assert_eq!(deserialized, expected);
    }
}

/// 测试表情类型结构
mod emoji_type_tests {
    use super::*;

    #[test]
    fn test_emoji_type_new() {
        let emoji = EmojiType::new();
        assert_eq!(emoji.emoji_type, None);
    }

    #[test] 
    fn test_emoji_type_default() {
        let emoji = EmojiType::default();
        assert_eq!(emoji.emoji_type, None);
    }

    #[test]
    fn test_emoji_type_with_emoji_type() {
        let emoji = EmojiType::new().with_emoji_type("SMILE");
        assert_eq!(emoji.emoji_type, Some("SMILE".to_string()));
    }

    #[test]
    fn test_emoji_type_with_emoji_type_string() {
        let emoji_string = String::from("HEART");
        let emoji = EmojiType::new().with_emoji_type(emoji_string);
        assert_eq!(emoji.emoji_type, Some("HEART".to_string()));
    }

    #[test]
    fn test_emoji_type_serialization() {
        let emoji = EmojiType::new().with_emoji_type("LAUGH");
        let serialized = serde_json::to_string(&emoji).unwrap();
        let expected = json!({"emoji_type": "LAUGH"});
        let actual: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_emoji_type_serialization_none() {
        let emoji = EmojiType::new();
        let serialized = serde_json::to_string(&emoji).unwrap();
        let expected = json!({});
        let actual: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(actual, expected);
    }
}

/// 测试消息反应结构
mod message_reaction_tests {
    use super::*;

    #[test]
    fn test_message_reaction_serialization_with_all_fields() {
        let reaction_user = ReactionUser {
            user_id: Some("user123".to_string()),
            name: Some("John Doe".to_string()),
            avatar: Some("http://example.com/avatar.jpg".to_string()),
            reaction_time: Some("1640995200".to_string()),
        };

        let reaction = MessageReaction {
            message_id: Some("msg123".to_string()),
            emoji_type: Some(EmojiType::new().with_emoji_type("SMILE")),
            reaction_count: Some(5),
            has_reacted: Some(true),
            reaction_users: Some(vec![reaction_user]),
        };

        let serialized = serde_json::to_string(&reaction).unwrap();
        assert!(serialized.contains("msg123"));
        assert!(serialized.contains("SMILE"));
        assert!(serialized.contains("5"));
        assert!(serialized.contains("true"));
        assert!(serialized.contains("user123"));
    }

    #[test] 
    fn test_message_reaction_serialization_with_none_fields() {
        let reaction = MessageReaction {
            message_id: None,
            emoji_type: None,
            reaction_count: None,
            has_reacted: None,
            reaction_users: None,
        };

        let serialized = serde_json::to_string(&reaction).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(parsed, json!({}));
    }
}

/// 测试反应用户结构
mod reaction_user_tests {
    use super::*;

    #[test]
    fn test_reaction_user_complete_serialization() {
        let user = ReactionUser {
            user_id: Some("user456".to_string()),
            name: Some("Jane Smith".to_string()),
            avatar: Some("http://example.com/jane.jpg".to_string()),
            reaction_time: Some("1640995300".to_string()),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["user_id"], "user456");
        assert_eq!(parsed["name"], "Jane Smith");
        assert_eq!(parsed["avatar"], "http://example.com/jane.jpg");
        assert_eq!(parsed["reaction_time"], "1640995300");
    }

    #[test]
    fn test_reaction_user_partial_serialization() {
        let user = ReactionUser {
            user_id: Some("user789".to_string()),
            name: None,
            avatar: None,
            reaction_time: Some("1640995400".to_string()),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        
        assert_eq!(parsed["user_id"], "user789");
        assert_eq!(parsed["reaction_time"], "1640995400");
        assert!(parsed.get("name").is_none());
        assert!(parsed.get("avatar").is_none());
    }
}

/// 测试Pin消息结构
mod pin_tests {
    use super::*;

    #[test]
    fn test_pin_complete_data() {
        let pin = Pin {
            pin_id: Some("pin123".to_string()),
            message_id: Some("msg456".to_string()),
            chat_id: Some("chat789".to_string()),
            operator_id: Some("user123".to_string()),
            pin_type: Some("message".to_string()),
            create_time: Some("1640995500".to_string()),
        };

        let serialized = serde_json::to_string(&pin).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");

        assert_eq!(parsed["pin_id"], "pin123");
        assert_eq!(parsed["message_id"], "msg456");
        assert_eq!(parsed["chat_id"], "chat789");
        assert_eq!(parsed["operator_id"], "user123");
        assert_eq!(parsed["pin_type"], "message");
        assert_eq!(parsed["create_time"], "1640995500");
    }
}

/// 测试批量消息结构
mod batch_message_tests {
    use super::*;

    #[test]
    fn test_batch_message_with_progress() {
        let batch_msg = BatchMessage {
            batch_message_id: Some("batch123".to_string()),
            message_id: Some("msg789".to_string()),
            status: Some(BatchMessageStatus::PartialSuccess),
            sent_count: Some(150),
            read_count: Some(120),
            total_count: Some(200),
            progress: Some(0.75),
        };

        let serialized = serde_json::to_string(&batch_msg).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");

        assert_eq!(parsed["batch_message_id"], "batch123");
        assert_eq!(parsed["status"], 1); // PartialSuccess = 1
        assert_eq!(parsed["sent_count"], 150);
        assert_eq!(parsed["read_count"], 120);
        assert_eq!(parsed["total_count"], 200);
        assert_eq!(parsed["progress"], 0.75);
    }
}

/// 测试图片信息结构
mod image_info_tests {
    use super::*;

    #[test]
    fn test_image_info_complete() {
        let image_info = ImageInfo {
            image_key: Some("img_key_123".to_string()),
            image_type: Some("png".to_string()),
            image_size: Some(1048576), // 1MB
            width: Some(1920),
            height: Some(1080),
        };

        let serialized = serde_json::to_string(&image_info).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");

        assert_eq!(parsed["image_key"], "img_key_123");
        assert_eq!(parsed["image_type"], "png");
        assert_eq!(parsed["image_size"], 1048576);
        assert_eq!(parsed["width"], 1920);
        assert_eq!(parsed["height"], 1080);
    }

    #[test]
    fn test_image_info_minimal() {
        let image_info = ImageInfo {
            image_key: Some("img_key_456".to_string()),
            image_type: None,
            image_size: None,
            width: None,
            height: None,
        };

        let serialized = serde_json::to_string(&image_info).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");

        assert_eq!(parsed["image_key"], "img_key_456");
        assert!(parsed.get("image_type").is_none());
        assert!(parsed.get("image_size").is_none());
        assert!(parsed.get("width").is_none());
        assert!(parsed.get("height").is_none());
    }
}

/// 测试文件信息结构
mod file_info_tests {
    use super::*;

    #[test]
    fn test_file_info_complete() {
        let file_info = FileInfo {
            file_key: Some("file_key_789".to_string()),
            file_name: Some("document.pdf".to_string()),
            file_type: Some("pdf".to_string()),
            file_size: Some(2097152), // 2MB
            file_token: Some("token_abc123".to_string()),
        };

        let serialized = serde_json::to_string(&file_info).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");

        assert_eq!(parsed["file_key"], "file_key_789");
        assert_eq!(parsed["file_name"], "document.pdf");
        assert_eq!(parsed["file_type"], "pdf");
        assert_eq!(parsed["file_size"], 2097152);
        assert_eq!(parsed["file_token"], "token_abc123");
    }
}

/// 测试加急类型枚举
mod urgent_type_tests {
    use super::*;

    #[rstest]
    #[case(UrgentType::App, r#""app""#)]
    #[case(UrgentType::Sms, r#""sms""#)]
    #[case(UrgentType::Phone, r#""phone""#)]
    fn test_urgent_type_serialization(#[case] urgent_type: UrgentType, #[case] expected: &str) {
        let serialized = serde_json::to_string(&urgent_type).unwrap();
        assert_eq!(serialized, expected);
    }

    #[rstest]
    #[case(r#""app""#, UrgentType::App)]
    #[case(r#""sms""#, UrgentType::Sms)]
    #[case(r#""phone""#, UrgentType::Phone)]
    fn test_urgent_type_deserialization(#[case] json_str: &str, #[case] expected: UrgentType) {
        let deserialized: UrgentType = serde_json::from_str(json_str).expect("JSON 反序列化失败");
        assert_eq!(deserialized, expected);
    }
}

/// 测试加急信息结构
mod urgent_info_tests {
    use super::*;

    #[test]
    fn test_urgent_info_sms_type() {
        let urgent_info = UrgentInfo {
            urgent_type: Some(UrgentType::Sms),
            user_id_list: Some(vec!["user1".to_string(), "user2".to_string()]),
            message: Some("紧急通知：请立即查看".to_string()),
        };

        let serialized = serde_json::to_string(&urgent_info).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");

        assert_eq!(parsed["urgent_type"], "sms");
        assert_eq!(parsed["user_id_list"][0], "user1");
        assert_eq!(parsed["user_id_list"][1], "user2");
        assert_eq!(parsed["message"], "紧急通知：请立即查看");
    }
}

/// 测试URL预览信息
mod url_preview_tests {
    use super::*;

    #[test]
    fn test_url_preview_complete() {
        let url_preview = UrlPreview {
            url: Some("https://example.com".to_string()),
            title: Some("Example Website".to_string()),
            description: Some("This is an example website".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            preview_type: Some("website".to_string()),
        };

        let serialized = serde_json::to_string(&url_preview).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");

        assert_eq!(parsed["url"], "https://example.com");
        assert_eq!(parsed["title"], "Example Website");
        assert_eq!(parsed["description"], "This is an example website");
        assert_eq!(parsed["image_url"], "https://example.com/image.jpg");
        assert_eq!(parsed["preview_type"], "website");
    }
}

/// 测试分页信息
mod page_info_tests {
    use super::*;

    #[test]
    fn test_page_info_with_more_data() {
        let page_info = PageInfo {
            page_token: Some("token_next_page".to_string()),
            has_more: Some(true),
        };

        let serialized = serde_json::to_string(&page_info).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");

        assert_eq!(parsed["page_token"], "token_next_page");
        assert_eq!(parsed["has_more"], true);
    }

    #[test]
    fn test_page_info_no_more_data() {
        let page_info = PageInfo {
            page_token: None,
            has_more: Some(false),
        };

        let serialized = serde_json::to_string(&page_info).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");

        assert_eq!(parsed["has_more"], false);
        assert!(parsed.get("page_token").is_none());
    }
}

/// 属性测试 - 使用 proptest 进行随机化测试
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_message_type_roundtrip_serialization(msg_type in prop::sample::select(vec![
            MessageType::Text,
            MessageType::Post,
            MessageType::Image,
            MessageType::File,
            MessageType::Audio,
            MessageType::Media,
            MessageType::Sticker,
            MessageType::Interactive,
            MessageType::ShareChat,
            MessageType::ShareUser,
            MessageType::System,
        ])) {
            let serialized = serde_json::to_string(&msg_type).unwrap();
        let deserialized: MessageType = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
            prop_assert_eq!(msg_type, deserialized);
        }

        #[test]
        fn test_user_id_type_as_str_not_empty(id_type in prop::sample::select(vec![
            UserIdType::UserId,
            UserIdType::UnionId,
            UserIdType::OpenId,
        ])) {
            prop_assert!(!id_type.as_str().is_empty());
        }

        #[test]
        fn test_batch_message_status_roundtrip(status in prop::sample::select(vec![
            BatchMessageStatus::Processing,
            BatchMessageStatus::PartialSuccess,
            BatchMessageStatus::Success,
            BatchMessageStatus::Failed,
        ])) {
            let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: BatchMessageStatus = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
            prop_assert_eq!(status, deserialized);
        }

        #[test]
        fn test_emoji_type_with_random_string(emoji_str in "\\PC{1,20}") {
            let emoji = EmojiType::new().with_emoji_type(emoji_str.clone());
            prop_assert_eq!(emoji.emoji_type, Some(emoji_str));
        }
    }
}

/// 边界条件和错误处理测试
#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_invalid_message_type_deserialization() {
        let result: Result<MessageType, _> = serde_json::from_str(r#""invalid_type""#);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_user_id_type_deserialization() {
        let result: Result<UserIdType, _> = serde_json::from_str(r#""invalid_id_type""#);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_batch_message_status_deserialization() {
        let result: Result<BatchMessageStatus, _> = serde_json::from_str("999");
        assert!(result.is_err());
    }

    #[test]
    fn test_emoji_type_empty_string() {
        let emoji = EmojiType::new().with_emoji_type("");
        assert_eq!(emoji.emoji_type, Some("".to_string()));
    }

    #[test]
    fn test_very_long_emoji_string() {
        let long_string = "A".repeat(1000);
        let emoji = EmojiType::new().with_emoji_type(long_string.clone());
        assert_eq!(emoji.emoji_type, Some(long_string));
    }

    #[test]
    fn test_special_characters_in_emoji() {
        let special_emoji = "😀🎉💯";
        let emoji = EmojiType::new().with_emoji_type(special_emoji);
        assert_eq!(emoji.emoji_type, Some(special_emoji.to_string()));
    }

    #[test]
    fn test_message_reaction_large_count() {
        let reaction = MessageReaction {
            message_id: Some("msg123".to_string()),
            emoji_type: None,
            reaction_count: Some(i32::MAX),
            has_reacted: Some(false),
            reaction_users: None,
        };

        let serialized = serde_json::to_string(&reaction).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(parsed["reaction_count"], i32::MAX);
    }

    #[test]
    fn test_image_info_zero_dimensions() {
        let image_info = ImageInfo {
            image_key: Some("test".to_string()),
            image_type: Some("png".to_string()),
            image_size: Some(0),
            width: Some(0),
            height: Some(0),
        };

        let serialized = serde_json::to_string(&image_info).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).expect("JSON 反序列化失败");
        assert_eq!(parsed["width"], 0);
        assert_eq!(parsed["height"], 0);
        assert_eq!(parsed["image_size"], 0);
    }
}
