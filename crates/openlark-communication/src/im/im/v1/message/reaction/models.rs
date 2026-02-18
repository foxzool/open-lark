//! 消息表情回复相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 表情类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionType {
    pub emoji_type: String,
}

/// 操作者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionOperator {
    pub operator_id: String,
    pub operator_type: String,
}

/// 表情回复
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageReaction {
    pub reaction_id: String,
    pub operator: ReactionOperator,
    pub action_time: String,
    pub reaction_type: ReactionType,
}

impl ApiResponseTrait for MessageReaction {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加消息表情回复请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateMessageReactionBody {
    pub reaction_type: ReactionType,
}

/// 获取消息表情回复列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListMessageReactionsResponse {
    #[serde(default)]
    pub items: Option<Vec<MessageReaction>>,
    pub has_more: bool,
    #[serde(default)]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListMessageReactionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        original: &T,
    ) {
        let json = serde_json::to_string(original).expect("序列化失败");
        let deserialized: T = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(original, &deserialized, "roundtrip 后数据不一致");
    }

    #[test]
    fn test_reaction_type_serialization() {
        let reaction_type = ReactionType {
            emoji_type: "OK".to_string(),
        };
        test_roundtrip(&reaction_type);
    }

    #[test]
    fn test_reaction_operator_serialization() {
        let operator = ReactionOperator {
            operator_id: "user123".to_string(),
            operator_type: "user".to_string(),
        };
        test_roundtrip(&operator);
    }

    #[test]
    fn test_message_reaction_serialization() {
        let reaction = MessageReaction {
            reaction_id: "reaction123".to_string(),
            operator: ReactionOperator {
                operator_id: "user123".to_string(),
                operator_type: "user".to_string(),
            },
            action_time: "2024-01-01T00:00:00Z".to_string(),
            reaction_type: ReactionType {
                emoji_type: "LIKE".to_string(),
            },
        };
        test_roundtrip(&reaction);
    }

    #[test]
    fn test_create_message_reaction_body_serialization() {
        let body = CreateMessageReactionBody {
            reaction_type: ReactionType {
                emoji_type: "HEART".to_string(),
            },
        };
        test_roundtrip(&body);
    }

    #[test]
    fn test_list_message_reactions_response_serialization() {
        let response = ListMessageReactionsResponse {
            items: Some(vec![MessageReaction {
                reaction_id: "reaction123".to_string(),
                operator: ReactionOperator {
                    operator_id: "user123".to_string(),
                    operator_type: "user".to_string(),
                },
                action_time: "2024-01-01T00:00:00Z".to_string(),
                reaction_type: ReactionType {
                    emoji_type: "THUMBS_UP".to_string(),
                },
            }]),
            has_more: false,
            page_token: None,
        };
        test_roundtrip(&response);
    }
}
