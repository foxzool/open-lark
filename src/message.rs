use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 消息类型枚举
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "msg_type", content = "content")]
pub enum LarkMessage {
    /// 文本消息
    #[serde(rename = "text")]
    Text(TextMessage),
    /// 富文本消息
    #[serde(rename = "post")]
    RichText(RichTextMessage),
    /// 群名片
    #[serde(rename = "share_chat")]
    ShareChat(ShareChatMessage),
    /// 图片
    #[serde(rename = "image")]
    Image(ImageMessage),
    /// 消息卡片
    #[serde(rename = "interactive")]
    Interactive(InteractiveMessage),
}

pub struct MessageBuilder {
    content: LarkMessage,
}

impl MessageBuilder {
    pub fn new_text() -> Self {
        Self {
            content: LarkMessage::Text(TextMessage {
                text: "".to_string(),
            }),
        }
    }

    pub fn new_rich_text() -> Self {
        Self {
            content: LarkMessage::RichText(RichTextMessage {
                language: Default::default(),
                content: Value::Null,
            }),
        }
    }

    pub fn new_share_chat() -> Self {
        Self {
            content: LarkMessage::ShareChat(ShareChatMessage {
                share_chat_id: "".to_string(),
            }),
        }
    }

    pub fn new_image() -> Self {
        Self {
            content: LarkMessage::Image(ImageMessage {
                image_key: "".to_string(),
            }),
        }
    }

    pub fn new_interactive() -> Self {
        Self {
            content: LarkMessage::Interactive(InteractiveMessage { card: Value::Null }),
        }
    }

    pub fn add_text(mut self, text: &str) -> Self {
        match self.content {
            LarkMessage::Text(ref mut message) => {
                message.text = text.to_string();
                LarkMessage::Text(message.clone())
            }
            _ => unreachable!(),
        };
        self
    }

    pub fn build(self) -> LarkMessage {
        self.content
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextMessage {
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    #[default]
    ZhCn,
    EnUs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RichTextMessage {
    language: Language,
    content: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageMessage {
    image_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShareChatMessage {
    share_chat_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractiveMessage {
    card: Value,
}
