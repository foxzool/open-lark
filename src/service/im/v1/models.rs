use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 消息类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// 文本消息
    Text,
    /// 富文本消息
    Post,
    /// 图片消息
    Image,
    /// 文件消息
    File,
    /// 音频消息
    Audio,
    /// 视频消息
    Media,
    /// 表情包消息
    Sticker,
    /// 交互式消息卡片
    Interactive,
    /// 分享群名片
    ShareChat,
    /// 分享用户名片
    ShareUser,
    /// 系统消息
    System,
}

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    UserId,
    UnionId,
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

/// 接收ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveIdType {
    OpenId,
    UserId,
    UnionId,
    Email,
    ChatId,
}

impl ReceiveIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReceiveIdType::OpenId => "open_id",
            ReceiveIdType::UserId => "user_id",
            ReceiveIdType::UnionId => "union_id",
            ReceiveIdType::Email => "email",
            ReceiveIdType::ChatId => "chat_id",
        }
    }
}

/// 批量消息状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BatchMessageStatus {
    /// 处理中
    Processing = 0,
    /// 部分成功
    PartialSuccess = 1,
    /// 全部成功
    Success = 2,
    /// 全部失败
    Failed = 3,
}

/// 表情回复类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmojiType {
    /// 表情类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_type: Option<String>,
}

impl EmojiType {
    pub fn new() -> Self {
        Self { emoji_type: None }
    }

    pub fn with_emoji_type(mut self, emoji_type: impl Into<String>) -> Self {
        self.emoji_type = Some(emoji_type.into());
        self
    }
}

impl Default for EmojiType {
    fn default() -> Self {
        Self::new()
    }
}

/// 表情回复信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReaction {
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 表情类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_type: Option<EmojiType>,
    /// 回复次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_count: Option<i32>,
    /// 是否包含当前用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_reacted: Option<bool>,
    /// 回复用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_users: Option<Vec<ReactionUser>>,
}

/// 表情回复用户信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionUser {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 回复时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_time: Option<String>,
}

/// Pin消息信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pin {
    /// Pin ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_id: Option<String>,
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 聊天ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// Pin创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// Pin类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

/// 批量消息信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchMessage {
    /// 批量消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_message_id: Option<String>,
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BatchMessageStatus>,
    /// 已发送人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_count: Option<i32>,
    /// 已读人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_count: Option<i32>,
    /// 总人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 发送进度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
}

/// 图片信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageInfo {
    /// 图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 图片类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 图片大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<i64>,
    /// 图片宽度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 图片高度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

/// 文件信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key: Option<String>,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
}

/// 消息卡片信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageCard {
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 卡片内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<serde_json::Value>,
    /// 是否仅特定人可见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_multi: Option<bool>,
    /// 特定用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
}

/// 加急类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UrgentType {
    /// 应用内加急
    App,
    /// 短信加急
    Sms,
    /// 电话加急
    Phone,
}

/// 加急信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrgentInfo {
    /// 加急类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent_type: Option<UrgentType>,
    /// 加急用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    /// 自定义消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// URL预览信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlPreview {
    /// URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 图片URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// 预览类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_type: Option<String>,
}

/// 分页信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageInfo {
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 消息已读信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReadInfo {
    /// 已读用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_users: Option<Vec<ReadUser>>,
    /// 是否还有更多
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 已读用户信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadUser {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 已读时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_time: Option<String>,
    /// 租户key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_serialization() {
        let types = vec![
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
        ];

        for msg_type in types {
            let serialized = serde_json::to_string(&msg_type).unwrap();
            let deserialized: MessageType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(msg_type, deserialized);
        }
    }

    #[test]
    fn test_user_id_type_serialization() {
        let types = vec![UserIdType::UserId, UserIdType::UnionId, UserIdType::OpenId];

        for user_type in types {
            let serialized = serde_json::to_string(&user_type).unwrap();
            let deserialized: UserIdType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(user_type, deserialized);
        }
    }

    #[test]
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
    }

    #[test]
    fn test_receive_id_type_serialization() {
        let types = vec![
            ReceiveIdType::OpenId,
            ReceiveIdType::UserId,
            ReceiveIdType::UnionId,
            ReceiveIdType::Email,
            ReceiveIdType::ChatId,
        ];

        for receive_type in types {
            let serialized = serde_json::to_string(&receive_type).unwrap();
            let deserialized: ReceiveIdType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(receive_type, deserialized);
        }
    }

    #[test]
    fn test_receive_id_type_as_str() {
        assert_eq!(ReceiveIdType::OpenId.as_str(), "open_id");
        assert_eq!(ReceiveIdType::UserId.as_str(), "user_id");
        assert_eq!(ReceiveIdType::UnionId.as_str(), "union_id");
        assert_eq!(ReceiveIdType::Email.as_str(), "email");
        assert_eq!(ReceiveIdType::ChatId.as_str(), "chat_id");
    }

    #[test]
    fn test_batch_message_status_serialization() {
        let statuses = vec![
            BatchMessageStatus::Processing,
            BatchMessageStatus::PartialSuccess,
            BatchMessageStatus::Success,
            BatchMessageStatus::Failed,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: BatchMessageStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_batch_message_status_values() {
        assert_eq!(BatchMessageStatus::Processing as u8, 0);
        assert_eq!(BatchMessageStatus::PartialSuccess as u8, 1);
        assert_eq!(BatchMessageStatus::Success as u8, 2);
        assert_eq!(BatchMessageStatus::Failed as u8, 3);
    }

    #[test]
    fn test_emoji_type_creation() {
        let emoji = EmojiType::new();
        assert_eq!(emoji.emoji_type, None);

        let emoji_with_type = EmojiType::new().with_emoji_type("smile");
        assert_eq!(emoji_with_type.emoji_type, Some("smile".to_string()));
    }

    #[test]
    fn test_emoji_type_default() {
        let emoji: EmojiType = Default::default();
        assert_eq!(emoji.emoji_type, None);
    }

    #[test]
    fn test_emoji_type_serialization() {
        let emoji = EmojiType {
            emoji_type: Some("thumbs_up".to_string()),
        };

        let serialized = serde_json::to_string(&emoji).unwrap();
        let deserialized: EmojiType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(emoji.emoji_type, deserialized.emoji_type);
    }

    #[test]
    fn test_message_reaction_serialization() {
        let reaction = MessageReaction {
            message_id: Some("msg_123".to_string()),
            emoji_type: Some(EmojiType {
                emoji_type: Some("heart".to_string()),
            }),
            reaction_count: Some(5),
            has_reacted: Some(true),
            reaction_users: Some(vec![ReactionUser {
                user_id: Some("user_456".to_string()),
                name: Some("张三".to_string()),
                avatar: Some("avatar_url".to_string()),
                reaction_time: Some("2024-01-01T10:00:00Z".to_string()),
            }]),
        };

        let serialized = serde_json::to_string(&reaction).unwrap();
        let deserialized: MessageReaction = serde_json::from_str(&serialized).unwrap();

        assert_eq!(reaction.message_id, deserialized.message_id);
        assert_eq!(reaction.reaction_count, deserialized.reaction_count);
        assert_eq!(reaction.has_reacted, deserialized.has_reacted);
        assert_eq!(
            reaction.reaction_users.as_ref().unwrap().len(),
            deserialized.reaction_users.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_reaction_user_serialization() {
        let user = ReactionUser {
            user_id: Some("user_789".to_string()),
            name: Some("李四".to_string()),
            avatar: Some("https://avatar.example.com/789".to_string()),
            reaction_time: Some("2024-01-01T11:30:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: ReactionUser = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.user_id, deserialized.user_id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.avatar, deserialized.avatar);
        assert_eq!(user.reaction_time, deserialized.reaction_time);
    }

    #[test]
    fn test_pin_serialization() {
        let pin = Pin {
            pin_id: Some("pin_123".to_string()),
            message_id: Some("msg_456".to_string()),
            chat_id: Some("chat_789".to_string()),
            operator_id: Some("operator_101".to_string()),
            pin_type: Some("manual".to_string()),
            create_time: Some("2024-01-01T09:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&pin).unwrap();
        let deserialized: Pin = serde_json::from_str(&serialized).unwrap();

        assert_eq!(pin.pin_id, deserialized.pin_id);
        assert_eq!(pin.message_id, deserialized.message_id);
        assert_eq!(pin.chat_id, deserialized.chat_id);
        assert_eq!(pin.operator_id, deserialized.operator_id);
        assert_eq!(pin.pin_type, deserialized.pin_type);
        assert_eq!(pin.create_time, deserialized.create_time);
    }

    #[test]
    fn test_batch_message_serialization() {
        let batch_msg = BatchMessage {
            batch_message_id: Some("batch_123".to_string()),
            message_id: Some("msg_456".to_string()),
            status: Some(BatchMessageStatus::PartialSuccess),
            sent_count: Some(80),
            read_count: Some(65),
            total_count: Some(100),
            progress: Some(0.8),
        };

        let serialized = serde_json::to_string(&batch_msg).unwrap();
        let deserialized: BatchMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(batch_msg.batch_message_id, deserialized.batch_message_id);
        assert_eq!(batch_msg.message_id, deserialized.message_id);
        assert_eq!(batch_msg.status, deserialized.status);
        assert_eq!(batch_msg.sent_count, deserialized.sent_count);
        assert_eq!(batch_msg.read_count, deserialized.read_count);
        assert_eq!(batch_msg.total_count, deserialized.total_count);
        assert_eq!(batch_msg.progress, deserialized.progress);
    }

    #[test]
    fn test_image_info_serialization() {
        let image = ImageInfo {
            image_key: Some("img_key_123".to_string()),
            image_type: Some("png".to_string()),
            image_size: Some(1024000),
            width: Some(1920),
            height: Some(1080),
        };

        let serialized = serde_json::to_string(&image).unwrap();
        let deserialized: ImageInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(image.image_key, deserialized.image_key);
        assert_eq!(image.image_type, deserialized.image_type);
        assert_eq!(image.image_size, deserialized.image_size);
        assert_eq!(image.width, deserialized.width);
        assert_eq!(image.height, deserialized.height);
    }

    #[test]
    fn test_file_info_serialization() {
        let file = FileInfo {
            file_key: Some("file_key_456".to_string()),
            file_name: Some("document.pdf".to_string()),
            file_type: Some("pdf".to_string()),
            file_size: Some(2048000),
            file_token: Some("token_789".to_string()),
        };

        let serialized = serde_json::to_string(&file).unwrap();
        let deserialized: FileInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(file.file_key, deserialized.file_key);
        assert_eq!(file.file_name, deserialized.file_name);
        assert_eq!(file.file_type, deserialized.file_type);
        assert_eq!(file.file_size, deserialized.file_size);
        assert_eq!(file.file_token, deserialized.file_token);
    }

    #[test]
    fn test_message_card_serialization() {
        let card_content = serde_json::json!({
            "type": "template",
            "data": {
                "template_id": "ctp_123",
                "template_variable": {
                    "title": "测试卡片",
                    "content": "这是一个测试消息卡片"
                }
            }
        });

        let card = MessageCard {
            card_id: Some("card_123".to_string()),
            card: Some(card_content.clone()),
            update_multi: Some(false),
            user_id_list: Some(vec!["user_1".to_string(), "user_2".to_string()]),
        };

        let serialized = serde_json::to_string(&card).unwrap();
        let deserialized: MessageCard = serde_json::from_str(&serialized).unwrap();

        assert_eq!(card.card_id, deserialized.card_id);
        assert_eq!(card.update_multi, deserialized.update_multi);
        assert_eq!(card.user_id_list, deserialized.user_id_list);
        assert!(deserialized.card.is_some());
    }

    #[test]
    fn test_urgent_type_serialization() {
        let types = vec![UrgentType::App, UrgentType::Sms, UrgentType::Phone];

        for urgent_type in types {
            let serialized = serde_json::to_string(&urgent_type).unwrap();
            let deserialized: UrgentType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(urgent_type, deserialized);
        }
    }

    #[test]
    fn test_urgent_info_serialization() {
        let urgent = UrgentInfo {
            urgent_type: Some(UrgentType::Sms),
            user_id_list: Some(vec![
                "urgent_user_1".to_string(),
                "urgent_user_2".to_string(),
            ]),
            message: Some("紧急通知：请立即处理".to_string()),
        };

        let serialized = serde_json::to_string(&urgent).unwrap();
        let deserialized: UrgentInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(urgent.urgent_type, deserialized.urgent_type);
        assert_eq!(urgent.user_id_list, deserialized.user_id_list);
        assert_eq!(urgent.message, deserialized.message);
    }

    #[test]
    fn test_url_preview_serialization() {
        let preview = UrlPreview {
            url: Some("https://example.com".to_string()),
            title: Some("示例网站".to_string()),
            description: Some("这是一个示例网站的描述".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            preview_type: Some("link".to_string()),
        };

        let serialized = serde_json::to_string(&preview).unwrap();
        let deserialized: UrlPreview = serde_json::from_str(&serialized).unwrap();

        assert_eq!(preview.url, deserialized.url);
        assert_eq!(preview.title, deserialized.title);
        assert_eq!(preview.description, deserialized.description);
        assert_eq!(preview.image_url, deserialized.image_url);
        assert_eq!(preview.preview_type, deserialized.preview_type);
    }

    #[test]
    fn test_page_info_serialization() {
        let page_info = PageInfo {
            page_token: Some("token_next_page".to_string()),
            has_more: Some(true),
        };

        let serialized = serde_json::to_string(&page_info).unwrap();
        let deserialized: PageInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(page_info.page_token, deserialized.page_token);
        assert_eq!(page_info.has_more, deserialized.has_more);
    }

    #[test]
    fn test_message_read_info_serialization() {
        let read_info = MessageReadInfo {
            read_users: Some(vec![
                ReadUser {
                    user_id: Some("reader_1".to_string()),
                    read_time: Some("2024-01-01T10:15:00Z".to_string()),
                    tenant_key: Some("tenant_a".to_string()),
                },
                ReadUser {
                    user_id: Some("reader_2".to_string()),
                    read_time: Some("2024-01-01T10:20:00Z".to_string()),
                    tenant_key: Some("tenant_b".to_string()),
                },
            ]),
            has_more: Some(false),
            page_token: Some("end_token".to_string()),
        };

        let serialized = serde_json::to_string(&read_info).unwrap();
        let deserialized: MessageReadInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(read_info.has_more, deserialized.has_more);
        assert_eq!(read_info.page_token, deserialized.page_token);
        assert_eq!(
            read_info.read_users.as_ref().unwrap().len(),
            deserialized.read_users.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_read_user_serialization() {
        let read_user = ReadUser {
            user_id: Some("reader_123".to_string()),
            read_time: Some("2024-01-01T14:30:00Z".to_string()),
            tenant_key: Some("tenant_xyz".to_string()),
        };

        let serialized = serde_json::to_string(&read_user).unwrap();
        let deserialized: ReadUser = serde_json::from_str(&serialized).unwrap();

        assert_eq!(read_user.user_id, deserialized.user_id);
        assert_eq!(read_user.read_time, deserialized.read_time);
        assert_eq!(read_user.tenant_key, deserialized.tenant_key);
    }

    #[test]
    fn test_models_with_none_values() {
        let emoji = EmojiType { emoji_type: None };
        let serialized = serde_json::to_string(&emoji).unwrap();
        let deserialized: EmojiType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(emoji.emoji_type, deserialized.emoji_type);

        let reaction = MessageReaction {
            message_id: None,
            emoji_type: None,
            reaction_count: None,
            has_reacted: None,
            reaction_users: None,
        };
        let serialized = serde_json::to_string(&reaction).unwrap();
        let deserialized: MessageReaction = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.message_id.is_none());
        assert!(deserialized.emoji_type.is_none());
        assert!(deserialized.reaction_count.is_none());
    }

    #[test]
    fn test_complex_message_reaction_with_multiple_users() {
        let reaction = MessageReaction {
            message_id: Some("complex_msg_123".to_string()),
            emoji_type: Some(EmojiType {
                emoji_type: Some("party".to_string()),
            }),
            reaction_count: Some(3),
            has_reacted: Some(true),
            reaction_users: Some(vec![
                ReactionUser {
                    user_id: Some("user_a".to_string()),
                    name: Some("用户A".to_string()),
                    avatar: Some("avatar_a.jpg".to_string()),
                    reaction_time: Some("2024-01-01T09:00:00Z".to_string()),
                },
                ReactionUser {
                    user_id: Some("user_b".to_string()),
                    name: Some("用户B".to_string()),
                    avatar: Some("avatar_b.jpg".to_string()),
                    reaction_time: Some("2024-01-01T09:05:00Z".to_string()),
                },
                ReactionUser {
                    user_id: Some("user_c".to_string()),
                    name: Some("用户C".to_string()),
                    avatar: Some("avatar_c.jpg".to_string()),
                    reaction_time: Some("2024-01-01T09:10:00Z".to_string()),
                },
            ]),
        };

        let serialized = serde_json::to_string(&reaction).unwrap();
        let deserialized: MessageReaction = serde_json::from_str(&serialized).unwrap();

        assert_eq!(reaction.message_id, deserialized.message_id);
        assert_eq!(reaction.reaction_count, deserialized.reaction_count);
        assert_eq!(reaction.has_reacted, deserialized.has_reacted);
        assert_eq!(reaction.reaction_users.as_ref().unwrap().len(), 3);
        assert_eq!(deserialized.reaction_users.as_ref().unwrap().len(), 3);

        let first_user = &reaction.reaction_users.as_ref().unwrap()[0];
        let deserialized_first_user = &deserialized.reaction_users.as_ref().unwrap()[0];
        assert_eq!(first_user.user_id, deserialized_first_user.user_id);
        assert_eq!(first_user.name, deserialized_first_user.name);
    }

    #[test]
    fn test_debug_trait_for_models() {
        let msg_type = MessageType::Interactive;
        let debug_string = format!("{:?}", msg_type);
        assert!(debug_string.contains("Interactive"));

        let user_type = UserIdType::OpenId;
        let debug_string = format!("{:?}", user_type);
        assert!(debug_string.contains("OpenId"));

        let batch_status = BatchMessageStatus::Success;
        let debug_string = format!("{:?}", batch_status);
        assert!(debug_string.contains("Success"));
    }
}
