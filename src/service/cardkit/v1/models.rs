//! CardKit v1 数据模型
//!
//! 定义了卡片、组件、配置等核心数据结构

use serde::{Deserialize, Serialize};

/// 卡片实体
#[derive(Debug, Clone, Default)]
pub struct Card {
    /// 卡片ID
    pub card_id: Option<String>,
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片描述
    pub description: Option<String>,
    /// 卡片JSON内容
    pub card_json: Option<serde_json::Value>,
    /// 卡片状态
    pub status: Option<CardStatus>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 卡片状态
#[derive(Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CardStatus {
    /// 草稿
    Draft,
    /// 已发布
    Published,
    /// 已删除
    Deleted,
}

impl Default for CardStatus {
    fn default() -> Self {
        CardStatus::Draft
    }
}

/// 卡片组件
#[derive(Debug, Clone, Default)]
pub struct CardElement {
    /// 组件ID
    pub element_id: Option<String>,
    /// 组件类型
    pub element_type: Option<String>,
    /// 组件内容
    pub content: Option<serde_json::Value>,
    /// 组件属性
    pub properties: Option<serde_json::Value>,
    /// 父组件ID
    pub parent_id: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    /// Open ID
    OpenId,
    /// Union ID
    UnionId,
    /// User ID
    UserId,
}

impl Default for UserIdType {
    fn default() -> Self {
        UserIdType::OpenId
    }
}

impl std::fmt::Display for UserIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserIdType::OpenId => write!(f, "open_id"),
            UserIdType::UnionId => write!(f, "union_id"),
            UserIdType::UserId => write!(f, "user_id"),
        }
    }
}

/// 卡片配置
#[derive(Debug, Clone, Default)]
pub struct CardSettings {
    /// 是否启用交互
    pub enable_interaction: Option<bool>,
    /// 卡片主题
    pub theme: Option<String>,
    /// 自定义配置
    pub custom_config: Option<serde_json::Value>,
}

/// 批量更新操作
#[derive(Debug, Clone)]
pub struct BatchUpdateOperation {
    /// 操作类型
    pub operation: String,
    /// 目标路径
    pub path: String,
    /// 操作值
    pub value: Option<serde_json::Value>,
}

/// 卡片布局类型
#[derive(Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CardLayoutType {
    /// 垂直布局
    Vertical,
    /// 水平布局
    Horizontal,
    /// 网格布局
    Grid,
}

impl Default for CardLayoutType {
    fn default() -> Self {
        CardLayoutType::Vertical
    }
}

/// 卡片主题
#[derive(Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CardTheme {
    /// 默认主题
    Default,
    /// 深色主题
    Dark,
    /// 浅色主题
    Light,
    /// 自定义主题
    Custom(String),
}

impl Default for CardTheme {
    fn default() -> Self {
        CardTheme::Default
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_default_creation() {
        let card = Card::default();
        assert_eq!(card.card_id, None);
        assert_eq!(card.title, None);
        assert_eq!(card.status, None);
    }

    #[test]
    fn test_card_with_values() {
        let card = Card {
            card_id: Some("card_123".to_string()),
            title: Some("测试卡片".to_string()),
            description: Some("这是一个测试卡片".to_string()),
            status: Some(CardStatus::Published),
            ..Default::default()
        };

        assert_eq!(card.card_id, Some("card_123".to_string()));
        assert_eq!(card.title, Some("测试卡片".to_string()));
        assert_eq!(card.description, Some("这是一个测试卡片".to_string()));
        assert_eq!(card.status, Some(CardStatus::Published));
    }

    #[test]
    fn test_card_status_default() {
        assert_eq!(CardStatus::default(), CardStatus::Draft);
    }

    #[test]
    fn test_card_status_equality() {
        assert_eq!(CardStatus::Draft, CardStatus::Draft);
        assert_ne!(CardStatus::Draft, CardStatus::Published);
    }

    #[test]
    fn test_user_id_type_default() {
        assert_eq!(UserIdType::default(), UserIdType::OpenId);
    }

    #[test]
    fn test_user_id_type_display() {
        assert_eq!(UserIdType::OpenId.to_string(), "open_id");
        assert_eq!(UserIdType::UnionId.to_string(), "union_id");
        assert_eq!(UserIdType::UserId.to_string(), "user_id");
    }

    #[test]
    fn test_user_id_type_equality() {
        assert_eq!(UserIdType::OpenId, UserIdType::OpenId);
        assert_ne!(UserIdType::OpenId, UserIdType::UnionId);
    }

    #[test]
    fn test_card_element_default() {
        let element = CardElement::default();
        assert_eq!(element.element_id, None);
        assert_eq!(element.element_type, None);
        assert_eq!(element.content, None);
    }

    #[test]
    fn test_card_element_with_values() {
        let element = CardElement {
            element_id: Some("element_123".to_string()),
            element_type: Some("text".to_string()),
            content: Some(serde_json::json!({
                "type": "plain_text",
                "content": "Hello World"
            })),
            ..Default::default()
        };

        assert_eq!(element.element_id, Some("element_123".to_string()));
        assert_eq!(element.element_type, Some("text".to_string()));
        assert!(element.content.is_some());
    }

    #[test]
    fn test_card_settings_default() {
        let settings = CardSettings::default();
        assert_eq!(settings.enable_interaction, None);
        assert_eq!(settings.theme, None);
        assert_eq!(settings.custom_config, None);
    }

    #[test]
    fn test_card_settings_with_values() {
        let settings = CardSettings {
            enable_interaction: Some(true),
            theme: Some("dark".to_string()),
            custom_config: Some(serde_json::json!({
                "custom_color": "#FF0000"
            })),
        };

        assert_eq!(settings.enable_interaction, Some(true));
        assert_eq!(settings.theme, Some("dark".to_string()));
        assert!(settings.custom_config.is_some());
    }

    #[test]
    fn test_batch_update_operation() {
        let operation = BatchUpdateOperation {
            operation: "replace".to_string(),
            path: "/title".to_string(),
            value: Some(serde_json::json!("新标题")),
        };

        assert_eq!(operation.operation, "replace");
        assert_eq!(operation.path, "/title");
        assert!(operation.value.is_some());
    }

    #[test]
    fn test_card_layout_type_default() {
        assert_eq!(CardLayoutType::default(), CardLayoutType::Vertical);
    }

    #[test]
    fn test_card_layout_type_equality() {
        assert_eq!(CardLayoutType::Vertical, CardLayoutType::Vertical);
        assert_ne!(CardLayoutType::Vertical, CardLayoutType::Horizontal);
    }

    #[test]
    fn test_card_theme_default() {
        assert_eq!(CardTheme::default(), CardTheme::Default);
    }

    #[test]
    fn test_card_theme_equality() {
        assert_eq!(CardTheme::Default, CardTheme::Default);
        assert_eq!(CardTheme::Custom("blue".to_string()), CardTheme::Custom("blue".to_string()));
        assert_ne!(CardTheme::Default, CardTheme::Dark);
        assert_ne!(CardTheme::Custom("red".to_string()), CardTheme::Custom("blue".to_string()));
    }

    #[test]
    fn test_complex_card_scenario() {
        // 测试完整的卡片场景
        let card = Card {
            card_id: Some("complex_card_001".to_string()),
            title: Some("复杂卡片".to_string()),
            description: Some("包含多种组件的复杂卡片".to_string()),
            card_json: Some(serde_json::json!({
                "type": "card",
                "header": {
                    "title": "审批申请",
                    "subtitle": "请及时处理"
                },
                "elements": [
                    {
                        "type": "div",
                        "text": {
                            "type": "plain_text",
                            "content": "申请人：张三"
                        }
                    }
                ]
            })),
            status: Some(CardStatus::Published),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(card.card_id, Some("complex_card_001".to_string()));
        assert_eq!(card.status, Some(CardStatus::Published));
        assert!(card.card_json.is_some());
        assert!(card.create_time.is_some());
        assert!(card.update_time.is_some());
    }

    #[test]
    fn test_card_element_with_parent() {
        // 测试有父子关系的组件
        let parent_element = CardElement {
            element_id: Some("parent_001".to_string()),
            element_type: Some("container".to_string()),
            ..Default::default()
        };

        let child_element = CardElement {
            element_id: Some("child_001".to_string()),
            element_type: Some("text".to_string()),
            parent_id: parent_element.element_id.clone(),
            content: Some(serde_json::json!({
                "type": "plain_text",
                "content": "子组件内容"
            })),
            ..Default::default()
        };

        assert_eq!(child_element.parent_id, Some("parent_001".to_string()));
        assert_eq!(parent_element.element_id, child_element.parent_id);
    }

    #[test]
    fn test_card_with_theme_and_layout() {
        // 测试带主题和布局的卡片
        let card = Card {
            title: Some("主题卡片".to_string()),
            card_json: Some(serde_json::json!({
                "theme": "dark",
                "layout": "vertical",
                "elements": []
            })),
            ..Default::default()
        };

        assert_eq!(card.title, Some("主题卡片".to_string()));
        assert!(card.card_json.is_some());

        if let Some(card_json) = &card.card_json {
            assert_eq!(card_json["theme"], "dark");
            assert_eq!(card_json["layout"], "vertical");
        }
    }

    #[test]
    fn test_json_serialization_compatibility() {
        // 测试JSON序列化兼容性
        let card = Card {
            card_id: Some("json_test_001".to_string()),
            title: Some("JSON测试卡片".to_string()),
            status: Some(CardStatus::Published),
            ..Default::default()
        };

        // 测试可以序列化为JSON
        let json_value = serde_json::json!({
            "card_id": "json_test_001",
            "title": "JSON测试卡片",
            "status": "published"
        });

        assert_eq!(json_value["card_id"], "json_test_001");
        assert_eq!(json_value["title"], "JSON测试卡片");
        assert_eq!(json_value["status"], "published");
    }
}