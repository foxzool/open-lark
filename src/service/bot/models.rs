use serde::{Deserialize, Serialize};

/// 机器人信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
    /// 机器人名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 机器人头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 机器人IP白名单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_white_list: Option<Vec<String>>,
    /// 机器人应用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_status: Option<AppStatus>,
    /// 机器人的open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
}

/// 应用状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppStatus {
    /// 未知状态
    #[serde(rename = "unknown")]
    Unknown,
    /// 正常
    #[serde(rename = "normal")]
    Normal,
    /// 停用
    #[serde(rename = "disabled")]
    Disabled,
}

/// 机器人自定义菜单事件内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotMenuEvent {
    /// 事件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// 事件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 租户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 操作者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<EventOperator>,
    /// 菜单事件详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<MenuEventDetail>,
}

/// 事件操作者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOperator {
    /// 操作者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<OperatorId>,
    /// 操作者类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_type: Option<String>,
}

/// 操作者ID信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorId {
    /// Open ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// Union ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 菜单事件详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuEventDetail {
    /// 菜单事件key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_key: Option<String>,
    /// 时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 菜单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_id: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_bot_serialization() {
        let bot = Bot {
            app_name: Some("测试机器人".to_string()),
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            ip_white_list: Some(vec!["192.168.1.0/24".to_string(), "10.0.0.0/8".to_string()]),
            app_status: Some(AppStatus::Normal),
            open_id: Some("ou_123456789".to_string()),
        };
        let serialized = serde_json::to_string(&bot).unwrap();
        let deserialized: Bot = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bot.app_name, deserialized.app_name);
        assert_eq!(bot.avatar_url, deserialized.avatar_url);
        assert_eq!(bot.ip_white_list, deserialized.ip_white_list);
        assert_eq!(bot.open_id, deserialized.open_id);
    }

    #[test]
    fn test_bot_with_none_values() {
        let bot = Bot {
            app_name: Some("简单机器人".to_string()),
            avatar_url: None,
            ip_white_list: None,
            app_status: None,
            open_id: None,
        };
        let serialized = serde_json::to_string(&bot).unwrap();
        assert!(!serialized.contains("avatar_url"));
        assert!(!serialized.contains("ip_white_list"));
        assert!(!serialized.contains("app_status"));
        assert!(!serialized.contains("open_id"));
        let deserialized: Bot = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bot.app_name, deserialized.app_name);
        assert_eq!(bot.avatar_url, deserialized.avatar_url);
    }

    #[test]
    fn test_app_status_serialization() {
        let unknown_status = AppStatus::Unknown;
        let serialized = serde_json::to_string(&unknown_status).unwrap();
        assert_eq!(serialized, "\"unknown\"");
        let deserialized: AppStatus = serde_json::from_str(&serialized).unwrap();
        assert!(matches!(deserialized, AppStatus::Unknown));

        let normal_status = AppStatus::Normal;
        let serialized = serde_json::to_string(&normal_status).unwrap();
        assert_eq!(serialized, "\"normal\"");

        let disabled_status = AppStatus::Disabled;
        let serialized = serde_json::to_string(&disabled_status).unwrap();
        assert_eq!(serialized, "\"disabled\"");
    }

    #[test]
    fn test_bot_menu_event_serialization() {
        let event = BotMenuEvent {
            event_id: Some("event_123456".to_string()),
            event_type: Some("menu_click".to_string()),
            tenant_key: Some("tenant_abc123".to_string()),
            app_id: Some("cli_app456789".to_string()),
            operator: Some(EventOperator {
                operator_id: Some(OperatorId {
                    open_id: Some("ou_user123".to_string()),
                    union_id: Some("on_union456".to_string()),
                    user_id: Some("user789".to_string()),
                }),
                operator_type: Some("user".to_string()),
            }),
            event: Some(MenuEventDetail {
                event_key: Some("menu_action_1".to_string()),
                timestamp: Some("1640995200000".to_string()),
                menu_id: Some("menu_001".to_string()),
            }),
        };
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: BotMenuEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event.event_id, deserialized.event_id);
        assert_eq!(event.event_type, deserialized.event_type);
        assert_eq!(event.tenant_key, deserialized.tenant_key);
        assert_eq!(event.app_id, deserialized.app_id);
    }

    #[test]
    fn test_bot_menu_event_with_none_values() {
        let event = BotMenuEvent {
            event_id: Some("minimal_event".to_string()),
            event_type: None,
            tenant_key: None,
            app_id: None,
            operator: None,
            event: None,
        };
        let serialized = serde_json::to_string(&event).unwrap();
        assert!(!serialized.contains("event_type"));
        assert!(!serialized.contains("tenant_key"));
        assert!(!serialized.contains("app_id"));
        assert!(!serialized.contains("operator"));
        assert!(!serialized.contains("\"event\":"));
        let deserialized: BotMenuEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event.event_id, deserialized.event_id);
        assert_eq!(event.event_type, deserialized.event_type);
    }

    #[test]
    fn test_event_operator_serialization() {
        let operator = EventOperator {
            operator_id: Some(OperatorId {
                open_id: Some("ou_test123".to_string()),
                union_id: Some("on_test456".to_string()),
                user_id: Some("user_test789".to_string()),
            }),
            operator_type: Some("admin".to_string()),
        };
        let serialized = serde_json::to_string(&operator).unwrap();
        let deserialized: EventOperator = serde_json::from_str(&serialized).unwrap();
        assert_eq!(operator.operator_type, deserialized.operator_type);
        if let (Some(orig_id), Some(deser_id)) = (&operator.operator_id, &deserialized.operator_id)
        {
            assert_eq!(orig_id.open_id, deser_id.open_id);
            assert_eq!(orig_id.union_id, deser_id.union_id);
            assert_eq!(orig_id.user_id, deser_id.user_id);
        }
    }

    #[test]
    fn test_operator_id_serialization() {
        let operator_id = OperatorId {
            open_id: Some("ou_complete_123".to_string()),
            union_id: Some("on_complete_456".to_string()),
            user_id: Some("user_complete_789".to_string()),
        };
        let serialized = serde_json::to_string(&operator_id).unwrap();
        let deserialized: OperatorId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(operator_id.open_id, deserialized.open_id);
        assert_eq!(operator_id.union_id, deserialized.union_id);
        assert_eq!(operator_id.user_id, deserialized.user_id);
    }

    #[test]
    fn test_operator_id_partial_data() {
        let operator_id = OperatorId {
            open_id: Some("ou_partial_only".to_string()),
            union_id: None,
            user_id: None,
        };
        let serialized = serde_json::to_string(&operator_id).unwrap();
        assert!(!serialized.contains("union_id"));
        assert!(!serialized.contains("user_id"));
        let deserialized: OperatorId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(operator_id.open_id, deserialized.open_id);
        assert_eq!(operator_id.union_id, deserialized.union_id);
        assert_eq!(operator_id.user_id, deserialized.user_id);
    }

    #[test]
    fn test_menu_event_detail_serialization() {
        let detail = MenuEventDetail {
            event_key: Some("action_button_clicked".to_string()),
            timestamp: Some("1640995200123".to_string()),
            menu_id: Some("main_menu_item_5".to_string()),
        };
        let serialized = serde_json::to_string(&detail).unwrap();
        let deserialized: MenuEventDetail = serde_json::from_str(&serialized).unwrap();
        assert_eq!(detail.event_key, deserialized.event_key);
        assert_eq!(detail.timestamp, deserialized.timestamp);
        assert_eq!(detail.menu_id, deserialized.menu_id);
    }

    #[test]
    fn test_menu_event_detail_minimal() {
        let detail = MenuEventDetail {
            event_key: Some("simple_click".to_string()),
            timestamp: None,
            menu_id: None,
        };
        let serialized = serde_json::to_string(&detail).unwrap();
        assert!(!serialized.contains("timestamp"));
        assert!(!serialized.contains("menu_id"));
        let deserialized: MenuEventDetail = serde_json::from_str(&serialized).unwrap();
        assert_eq!(detail.event_key, deserialized.event_key);
        assert_eq!(detail.timestamp, deserialized.timestamp);
        assert_eq!(detail.menu_id, deserialized.menu_id);
    }

    #[test]
    fn test_complex_nested_bot_menu_event() {
        let complex_event = BotMenuEvent {
            event_id: Some("complex_event_789".to_string()),
            event_type: Some("interactive_menu".to_string()),
            tenant_key: Some("complex_tenant_xyz".to_string()),
            app_id: Some("cli_complex_app".to_string()),
            operator: Some(EventOperator {
                operator_id: Some(OperatorId {
                    open_id: Some("ou_complex_user".to_string()),
                    union_id: None,
                    user_id: Some("complex_user_id".to_string()),
                }),
                operator_type: Some("external_user".to_string()),
            }),
            event: Some(MenuEventDetail {
                event_key: Some("complex_menu_action".to_string()),
                timestamp: Some("1641081600999".to_string()),
                menu_id: Some("sub_menu_complex".to_string()),
            }),
        };
        let serialized = serde_json::to_string(&complex_event).unwrap();
        let deserialized: BotMenuEvent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(complex_event.event_id, deserialized.event_id);
        assert_eq!(complex_event.event_type, deserialized.event_type);
        assert_eq!(complex_event.tenant_key, deserialized.tenant_key);
        assert_eq!(complex_event.app_id, deserialized.app_id);

        // Test nested operator
        if let (Some(orig_op), Some(deser_op)) = (&complex_event.operator, &deserialized.operator) {
            assert_eq!(orig_op.operator_type, deser_op.operator_type);
            if let (Some(orig_id), Some(deser_id)) = (&orig_op.operator_id, &deser_op.operator_id) {
                assert_eq!(orig_id.open_id, deser_id.open_id);
                assert_eq!(orig_id.user_id, deser_id.user_id);
                assert_eq!(orig_id.union_id, deser_id.union_id);
            }
        }

        // Test nested event detail
        if let (Some(orig_event), Some(deser_event)) = (&complex_event.event, &deserialized.event) {
            assert_eq!(orig_event.event_key, deser_event.event_key);
            assert_eq!(orig_event.timestamp, deser_event.timestamp);
            assert_eq!(orig_event.menu_id, deser_event.menu_id);
        }
    }

    #[test]
    fn test_app_status_enum_all_variants() {
        // Test all enum variants serialize and deserialize correctly
        let statuses = vec![
            (AppStatus::Unknown, "\"unknown\""),
            (AppStatus::Normal, "\"normal\""),
            (AppStatus::Disabled, "\"disabled\""),
        ];

        for (status, expected_json) in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            assert_eq!(serialized, expected_json);
            let deserialized: AppStatus = serde_json::from_str(&serialized).unwrap();
            // Use string comparison since we can't directly compare enum variants
            let reserialized = serde_json::to_string(&deserialized).unwrap();
            assert_eq!(reserialized, expected_json);
        }
    }
}
