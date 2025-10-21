use serde::{Deserialize, Serialize};

/// 消息流卡片状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeedCardStatus {
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

/// 消息流卡片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedCard {
    /// 卡片ID
    pub card_id: String,
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片内容
    pub content: Option<String>,
    /// 卡片状态
    pub status: Option<FeedCardStatus>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 按钮信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonInfo {
    /// 按钮ID
    pub button_id: String,
    /// 按钮文本
    pub text: String,
    /// 按钮类型
    pub button_type: Option<String>,
    /// 按钮行为
    pub action: Option<String>,
}

/// 即时提醒配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelyNotification {
    /// 提醒类型
    pub notification_type: String,
    /// 提醒消息
    pub message: String,
    /// 目标用户
    pub target_users: Vec<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_feed_card_status_serialization() {
        let active = FeedCardStatus::Active;
        let inactive = FeedCardStatus::Inactive;

        let active_json = serde_json::to_string(&active).unwrap();
        let inactive_json = serde_json::to_string(&inactive).unwrap();

        assert_eq!(active_json, "\"active\"");
        assert_eq!(inactive_json, "\"inactive\"");
    }

    #[test]
    fn test_feed_card_status_deserialization() {
        let active_result: FeedCardStatus = serde_json::from_str("\"active\"").unwrap();
        let inactive_result: FeedCardStatus = serde_json::from_str("\"inactive\"").unwrap();

        assert_eq!(active_result, FeedCardStatus::Active);
        assert_eq!(inactive_result, FeedCardStatus::Inactive);
    }

    #[test]
    fn test_feed_card_status_clone_and_debug() {
        let status = FeedCardStatus::Active;
        let cloned_status = status.clone();

        assert_eq!(status, cloned_status);
        assert!(format!("{:?}", status).contains("Active"));
    }

    #[test]
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
    }

    #[test]
    fn test_user_id_type_serialization() {
        let user_id = UserIdType::UserId;
        let union_id = UserIdType::UnionId;
        let open_id = UserIdType::OpenId;

        let user_id_json = serde_json::to_string(&user_id).unwrap();
        let union_id_json = serde_json::to_string(&union_id).unwrap();
        let open_id_json = serde_json::to_string(&open_id).unwrap();

        assert_eq!(user_id_json, "\"user_id\"");
        assert_eq!(union_id_json, "\"union_id\"");
        assert_eq!(open_id_json, "\"open_id\"");
    }

    #[test]
    fn test_user_id_type_deserialization() {
        let user_id_result: UserIdType = serde_json::from_str("\"user_id\"").unwrap();
        let union_id_result: UserIdType = serde_json::from_str("\"union_id\"").unwrap();
        let open_id_result: UserIdType = serde_json::from_str("\"open_id\"").unwrap();

        assert_eq!(user_id_result, UserIdType::UserId);
        assert_eq!(union_id_result, UserIdType::UnionId);
        assert_eq!(open_id_result, UserIdType::OpenId);
    }

    #[test]
    fn test_user_id_type_clone_debug_and_partial_eq() {
        let id_type = UserIdType::UserId;
        let cloned_type = id_type.clone();

        assert_eq!(id_type, cloned_type);
        assert!(format!("{:?}", id_type).contains("UserId"));
        assert_ne!(UserIdType::UserId, UserIdType::UnionId);
    }

    #[test]
    fn test_feed_card_serialization() {
        let feed_card = FeedCard {
            card_id: "card123".to_string(),
            title: Some("Test Card".to_string()),
            content: Some("Test Content".to_string()),
            status: Some(FeedCardStatus::Active),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&feed_card).unwrap();
        assert!(json.contains("card123"));
        assert!(json.contains("Test Card"));
        assert!(json.contains("Test Content"));
        assert!(json.contains("active"));
    }

    #[test]
    fn test_feed_card_deserialization() {
        let json = r#"{
            "card_id": "card123",
            "title": "Test Card",
            "content": "Test Content",
            "status": "active",
            "create_time": "2023-01-01T00:00:00Z",
            "update_time": "2023-01-02T00:00:00Z"
        }"#;

        let feed_card: FeedCard = serde_json::from_str(json).unwrap();
        assert_eq!(feed_card.card_id, "card123");
        assert_eq!(feed_card.title, Some("Test Card".to_string()));
        assert_eq!(feed_card.content, Some("Test Content".to_string()));
        assert_eq!(feed_card.status, Some(FeedCardStatus::Active));
        assert_eq!(
            feed_card.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            feed_card.update_time,
            Some("2023-01-02T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_feed_card_with_none_values() {
        let feed_card = FeedCard {
            card_id: "card456".to_string(),
            title: None,
            content: None,
            status: None,
            create_time: None,
            update_time: None,
        };

        let json = serde_json::to_string(&feed_card).unwrap();
        let deserialized: FeedCard = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.card_id, "card456");
        assert_eq!(deserialized.title, None);
        assert_eq!(deserialized.content, None);
        assert_eq!(deserialized.status, None);
        assert_eq!(deserialized.create_time, None);
        assert_eq!(deserialized.update_time, None);
    }

    #[test]
    fn test_feed_card_clone_and_debug() {
        let feed_card = FeedCard {
            card_id: "card789".to_string(),
            title: Some("Clone Test".to_string()),
            content: Some("Clone Content".to_string()),
            status: Some(FeedCardStatus::Inactive),
            create_time: Some("2023-01-03T00:00:00Z".to_string()),
            update_time: Some("2023-01-04T00:00:00Z".to_string()),
        };

        let cloned_card = feed_card.clone();
        assert_eq!(feed_card.card_id, cloned_card.card_id);
        assert_eq!(feed_card.title, cloned_card.title);
        assert_eq!(feed_card.status, cloned_card.status);

        let debug_output = format!("{:?}", feed_card);
        assert!(debug_output.contains("card789"));
        assert!(debug_output.contains("Clone Test"));
    }

    #[test]
    fn test_button_info_serialization() {
        let button = ButtonInfo {
            button_id: "btn123".to_string(),
            text: "Click Me".to_string(),
            button_type: Some("primary".to_string()),
            action: Some("submit".to_string()),
        };

        let json = serde_json::to_string(&button).unwrap();
        assert!(json.contains("btn123"));
        assert!(json.contains("Click Me"));
        assert!(json.contains("primary"));
        assert!(json.contains("submit"));
    }

    #[test]
    fn test_button_info_deserialization() {
        let json = r#"{
            "button_id": "btn456",
            "text": "Submit",
            "button_type": "secondary",
            "action": "cancel"
        }"#;

        let button: ButtonInfo = serde_json::from_str(json).unwrap();
        assert_eq!(button.button_id, "btn456");
        assert_eq!(button.text, "Submit");
        assert_eq!(button.button_type, Some("secondary".to_string()));
        assert_eq!(button.action, Some("cancel".to_string()));
    }

    #[test]
    fn test_button_info_with_none_values() {
        let button = ButtonInfo {
            button_id: "btn789".to_string(),
            text: "Basic Button".to_string(),
            button_type: None,
            action: None,
        };

        let json = serde_json::to_string(&button).unwrap();
        let deserialized: ButtonInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.button_id, "btn789");
        assert_eq!(deserialized.text, "Basic Button");
        assert_eq!(deserialized.button_type, None);
        assert_eq!(deserialized.action, None);
    }

    #[test]
    fn test_button_info_clone_and_debug() {
        let button = ButtonInfo {
            button_id: "btn_clone".to_string(),
            text: "Clone Test".to_string(),
            button_type: Some("test".to_string()),
            action: Some("test_action".to_string()),
        };

        let cloned_button = button.clone();
        assert_eq!(button.button_id, cloned_button.button_id);
        assert_eq!(button.text, cloned_button.text);
        assert_eq!(button.button_type, cloned_button.button_type);
        assert_eq!(button.action, cloned_button.action);

        let debug_output = format!("{:?}", button);
        assert!(debug_output.contains("btn_clone"));
        assert!(debug_output.contains("Clone Test"));
    }

    #[test]
    fn test_timely_notification_serialization() {
        let notification = TimelyNotification {
            notification_type: "urgent".to_string(),
            message: "Important notification".to_string(),
            target_users: vec!["user1".to_string(), "user2".to_string()],
        };

        let json = serde_json::to_string(&notification).unwrap();
        assert!(json.contains("urgent"));
        assert!(json.contains("Important notification"));
        assert!(json.contains("user1"));
        assert!(json.contains("user2"));
    }

    #[test]
    fn test_timely_notification_deserialization() {
        let json = r#"{
            "notification_type": "reminder",
            "message": "Meeting in 5 minutes",
            "target_users": ["alice", "bob", "charlie"]
        }"#;

        let notification: TimelyNotification = serde_json::from_str(json).unwrap();
        assert_eq!(notification.notification_type, "reminder");
        assert_eq!(notification.message, "Meeting in 5 minutes");
        assert_eq!(notification.target_users, vec!["alice", "bob", "charlie"]);
    }

    #[test]
    fn test_timely_notification_with_empty_users() {
        let notification = TimelyNotification {
            notification_type: "info".to_string(),
            message: "System maintenance".to_string(),
            target_users: vec![],
        };

        let json = serde_json::to_string(&notification).unwrap();
        let deserialized: TimelyNotification = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.notification_type, "info");
        assert_eq!(deserialized.message, "System maintenance");
        assert!(deserialized.target_users.is_empty());
    }

    #[test]
    fn test_timely_notification_clone_and_debug() {
        let notification = TimelyNotification {
            notification_type: "test".to_string(),
            message: "Test message".to_string(),
            target_users: vec!["test_user".to_string()],
        };

        let cloned_notification = notification.clone();
        assert_eq!(
            notification.notification_type,
            cloned_notification.notification_type
        );
        assert_eq!(notification.message, cloned_notification.message);
        assert_eq!(notification.target_users, cloned_notification.target_users);

        let debug_output = format!("{:?}", notification);
        assert!(debug_output.contains("test"));
        assert!(debug_output.contains("Test message"));
        assert!(debug_output.contains("test_user"));
    }

    #[test]
    fn test_timely_notification_with_unicode() {
        let notification = TimelyNotification {
            notification_type: "提醒".to_string(),
            message: "会议将在5分钟后开始".to_string(),
            target_users: vec!["张三".to_string(), "李四".to_string()],
        };

        let json = serde_json::to_string(&notification).unwrap();
        let deserialized: TimelyNotification = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.notification_type, "提醒");
        assert_eq!(deserialized.message, "会议将在5分钟后开始");
        assert_eq!(deserialized.target_users, vec!["张三", "李四"]);
    }

    #[test]
    fn test_complex_feed_card_status_combinations() {
        let test_cases = vec![
            (FeedCardStatus::Active, "active"),
            (FeedCardStatus::Inactive, "inactive"),
        ];

        for (status, expected_str) in test_cases {
            let json = serde_json::to_string(&status).unwrap();
            assert_eq!(json, format!("\"{}\"", expected_str));

            let deserialized: FeedCardStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, status);
        }
    }

    #[test]
    fn test_all_user_id_types() {
        let types = vec![UserIdType::UserId, UserIdType::UnionId, UserIdType::OpenId];

        for user_type in types {
            let json = serde_json::to_string(&user_type).unwrap();
            let deserialized: UserIdType = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, user_type);

            // Test as_str method
            let str_value = user_type.as_str();
            assert!(!str_value.is_empty());
            assert!(str_value.contains("_id") || str_value == "open_id");
        }
    }
}
