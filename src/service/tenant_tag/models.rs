use serde::{Deserialize, Serialize};

/// 标签类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TagType {
    /// 群标签
    #[serde(rename = "chat")]
    Chat,
}

impl TagType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TagType::Chat => "chat",
        }
    }
}

/// 标签状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TagStatus {
    /// 激活状态
    #[serde(rename = "active")]
    Active,
    /// 失效状态
    #[serde(rename = "inactive")]
    Inactive,
}

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserIdType {
    /// 用户ID
    #[serde(rename = "user_id")]
    UserId,
    /// union_id
    #[serde(rename = "union_id")]
    UnionId,
    /// open_id
    #[serde(rename = "open_id")]
    OpenId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
}

/// 标签信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// 标签ID
    pub tag_id: String,
    /// 标签名称
    pub name: String,
    /// 标签描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 标签类型
    pub tag_type: TagType,
    /// 标签状态
    pub status: TagStatus,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}

/// 标签绑定关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagBinding {
    /// 标签ID
    pub tag_id: String,
    /// 实体ID（如群ID）
    pub entity_id: String,
    /// 实体类型
    pub entity_type: String,
    /// 绑定时间
    pub bind_time: Option<String>,
    /// 绑定者ID
    pub binder_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_type_enum() {
        let chat_type = TagType::Chat;
        assert_eq!(chat_type.as_str(), "chat");

        let serialized = serde_json::to_string(&chat_type).unwrap();
        assert_eq!(serialized, "\"chat\"");

        let deserialized: TagType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, TagType::Chat);
    }

    #[test]
    fn test_tag_type_debug_clone_partial_eq() {
        let tag_type1 = TagType::Chat;
        let tag_type2 = tag_type1.clone();

        assert_eq!(tag_type1, tag_type2);
        let debug_output = format!("{:?}", tag_type1);
        assert!(debug_output.contains("Chat"));
    }

    #[test]
    fn test_tag_status_enum() {
        let active_status = TagStatus::Active;
        let inactive_status = TagStatus::Inactive;

        assert_eq!(active_status, TagStatus::Active);
        assert_eq!(inactive_status, TagStatus::Inactive);
        assert_ne!(active_status, inactive_status);
    }

    #[test]
    fn test_tag_status_serialization() {
        let active = TagStatus::Active;
        let inactive = TagStatus::Inactive;

        let active_json = serde_json::to_string(&active).unwrap();
        let inactive_json = serde_json::to_string(&inactive).unwrap();

        assert_eq!(active_json, "\"active\"");
        assert_eq!(inactive_json, "\"inactive\"");

        let active_deserialized: TagStatus = serde_json::from_str(&active_json).unwrap();
        let inactive_deserialized: TagStatus = serde_json::from_str(&inactive_json).unwrap();

        assert_eq!(active_deserialized, TagStatus::Active);
        assert_eq!(inactive_deserialized, TagStatus::Inactive);
    }

    #[test]
    fn test_user_id_type_enum() {
        let user_id = UserIdType::UserId;
        let union_id = UserIdType::UnionId;
        let open_id = UserIdType::OpenId;

        assert_eq!(user_id.as_str(), "user_id");
        assert_eq!(union_id.as_str(), "union_id");
        assert_eq!(open_id.as_str(), "open_id");
    }

    #[test]
    fn test_user_id_type_serialization() {
        let types = [UserIdType::UserId, UserIdType::UnionId, UserIdType::OpenId];
        let expected = ["\"user_id\"", "\"union_id\"", "\"open_id\""];

        for (i, id_type) in types.iter().enumerate() {
            let json = serde_json::to_string(id_type).unwrap();
            assert_eq!(json, expected[i]);

            let deserialized: UserIdType = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, *id_type);
        }
    }

    #[test]
    fn test_user_id_type_debug_clone_partial_eq() {
        let user_id1 = UserIdType::UserId;
        let user_id2 = user_id1.clone();

        assert_eq!(user_id1, user_id2);
        let debug_output = format!("{:?}", user_id1);
        assert!(debug_output.contains("UserId"));
    }

    #[test]
    fn test_tag_creation() {
        let tag = Tag {
            tag_id: "tag_123".to_string(),
            name: "重要项目".to_string(),
            description: Some("重要项目相关群组".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1640995200000".to_string()),
            update_time: Some("1640995200000".to_string()),
            creator_id: Some("user_456".to_string()),
        };

        assert_eq!(tag.tag_id, "tag_123");
        assert_eq!(tag.name, "重要项目");
        assert_eq!(tag.description, Some("重要项目相关群组".to_string()));
        assert_eq!(tag.tag_type, TagType::Chat);
        assert_eq!(tag.status, TagStatus::Active);
        assert!(tag.create_time.is_some());
        assert!(tag.creator_id.is_some());
    }

    #[test]
    fn test_tag_serialization() {
        let tag = Tag {
            tag_id: "tag_serialization_test".to_string(),
            name: "Test Tag".to_string(),
            description: Some("A test tag".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1642723200000".to_string()),
            update_time: Some("1642723200000".to_string()),
            creator_id: Some("creator_123".to_string()),
        };

        let json = serde_json::to_string(&tag).unwrap();
        assert!(json.contains("tag_serialization_test"));
        assert!(json.contains("Test Tag"));
        assert!(json.contains("A test tag"));
        assert!(json.contains("creator_123"));

        let deserialized: Tag = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tag_id, tag.tag_id);
        assert_eq!(deserialized.name, tag.name);
        assert_eq!(deserialized.description, tag.description);
        assert_eq!(deserialized.tag_type, tag.tag_type);
        assert_eq!(deserialized.status, tag.status);
    }

    #[test]
    fn test_tag_with_minimal_data() {
        let tag = Tag {
            tag_id: "minimal_tag".to_string(),
            name: "Minimal".to_string(),
            description: None,
            tag_type: TagType::Chat,
            status: TagStatus::Inactive,
            create_time: None,
            update_time: None,
            creator_id: None,
        };

        let json = serde_json::to_string(&tag).unwrap();
        assert!(json.contains("minimal_tag"));
        assert!(json.contains("Minimal"));
        assert!(json.contains("inactive"));
        assert!(!json.contains("description"));
        assert!(!json.contains("create_time"));
        assert!(!json.contains("creator_id"));

        let deserialized: Tag = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tag_id, "minimal_tag");
        assert_eq!(deserialized.description, None);
        assert_eq!(deserialized.create_time, None);
        assert_eq!(deserialized.creator_id, None);
    }

    #[test]
    fn test_tag_debug_clone() {
        let tag = Tag {
            tag_id: "debug_tag".to_string(),
            name: "Debug Tag".to_string(),
            description: Some("For debugging".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1600000000000".to_string()),
            update_time: Some("1600000000000".to_string()),
            creator_id: Some("debug_user".to_string()),
        };

        let cloned_tag = tag.clone();
        assert_eq!(tag.tag_id, cloned_tag.tag_id);
        assert_eq!(tag.name, cloned_tag.name);

        let debug_output = format!("{:?}", tag);
        assert!(debug_output.contains("Tag"));
        assert!(debug_output.contains("debug_tag"));
        assert!(debug_output.contains("Debug Tag"));
    }

    #[test]
    fn test_tag_binding_creation() {
        let binding = TagBinding {
            tag_id: "tag_789".to_string(),
            entity_id: "chat_123".to_string(),
            entity_type: "chat".to_string(),
            bind_time: Some("1642723200000".to_string()),
            binder_id: Some("user_456".to_string()),
        };

        assert_eq!(binding.tag_id, "tag_789");
        assert_eq!(binding.entity_id, "chat_123");
        assert_eq!(binding.entity_type, "chat");
        assert!(binding.bind_time.is_some());
        assert!(binding.binder_id.is_some());
    }

    #[test]
    fn test_tag_binding_serialization() {
        let binding = TagBinding {
            tag_id: "binding_tag".to_string(),
            entity_id: "entity_123".to_string(),
            entity_type: "group".to_string(),
            bind_time: Some("1640995200000".to_string()),
            binder_id: Some("binder_user".to_string()),
        };

        let json = serde_json::to_string(&binding).unwrap();
        assert!(json.contains("binding_tag"));
        assert!(json.contains("entity_123"));
        assert!(json.contains("group"));
        assert!(json.contains("binder_user"));

        let deserialized: TagBinding = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tag_id, binding.tag_id);
        assert_eq!(deserialized.entity_id, binding.entity_id);
        assert_eq!(deserialized.entity_type, binding.entity_type);
        assert_eq!(deserialized.bind_time, binding.bind_time);
        assert_eq!(deserialized.binder_id, binding.binder_id);
    }

    #[test]
    fn test_tag_binding_with_none_values() {
        let binding = TagBinding {
            tag_id: "minimal_binding".to_string(),
            entity_id: "minimal_entity".to_string(),
            entity_type: "unknown".to_string(),
            bind_time: None,
            binder_id: None,
        };

        let json = serde_json::to_string(&binding).unwrap();
        assert!(json.contains("minimal_binding"));
        assert!(json.contains("minimal_entity"));
        assert!(json.contains("unknown"));

        let deserialized: TagBinding = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.bind_time, None);
        assert_eq!(deserialized.binder_id, None);
    }

    #[test]
    fn test_tag_binding_debug_clone() {
        let binding = TagBinding {
            tag_id: "debug_binding".to_string(),
            entity_id: "debug_entity".to_string(),
            entity_type: "debug_type".to_string(),
            bind_time: Some("1672531200000".to_string()),
            binder_id: Some("debug_binder".to_string()),
        };

        let cloned_binding = binding.clone();
        assert_eq!(binding.tag_id, cloned_binding.tag_id);
        assert_eq!(binding.entity_id, cloned_binding.entity_id);

        let debug_output = format!("{:?}", binding);
        assert!(debug_output.contains("TagBinding"));
        assert!(debug_output.contains("debug_binding"));
        assert!(debug_output.contains("debug_entity"));
    }

    #[test]
    fn test_tag_timestamp_validation() {
        let tag = Tag {
            tag_id: "timestamp_test".to_string(),
            name: "Timestamp Test".to_string(),
            description: Some("Testing timestamps".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1640995200000".to_string()),
            update_time: Some("1642723200000".to_string()),
            creator_id: Some("timestamp_user".to_string()),
        };

        // Test that timestamps can be parsed as u64
        if let Some(create_time) = &tag.create_time {
            assert!(create_time.parse::<u64>().is_ok());
        }
        if let Some(update_time) = &tag.update_time {
            assert!(update_time.parse::<u64>().is_ok());
        }

        // Test that update_time is after create_time
        let create_timestamp: u64 = tag.create_time.unwrap().parse().unwrap();
        let update_timestamp: u64 = tag.update_time.unwrap().parse().unwrap();
        assert!(update_timestamp >= create_timestamp);
    }

    #[test]
    fn test_tag_edge_cases() {
        // Test with empty strings
        let tag = Tag {
            tag_id: "".to_string(),
            name: "".to_string(),
            description: Some("".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("0".to_string()),
            update_time: Some("0".to_string()),
            creator_id: Some("".to_string()),
        };

        let json = serde_json::to_string(&tag).unwrap();
        let deserialized: Tag = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tag_id, "");
        assert_eq!(deserialized.name, "");
        assert_eq!(deserialized.description, Some("".to_string()));
        assert_eq!(deserialized.creator_id, Some("".to_string()));
    }
}
