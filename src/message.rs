use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub trait MessageTrait: Serialize + DeserializeOwned {
    const MESSAGE_TYPE: &'static str;

    fn message_type(&self) -> &'static str;
}

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

    pub fn new_rich_text(title: impl ToString) -> Self {
        Self {
            content: LarkMessage::RichText(RichTextMessage {
                post: LanguageContent::ZhCn(RichTextContent {
                    title: title.to_string(),
                    content: vec![],
                }),
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

/// 文本消息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextMessage {
    text: String,
}

impl TextMessage {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl MessageTrait for TextMessage {
    const MESSAGE_TYPE: &'static str = "text";

    fn message_type(&self) -> &'static str {
        Self::MESSAGE_TYPE
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LanguageContent {
    #[serde(rename = "zh_cn")]
    ZhCn(RichTextContent),
    #[serde(rename = "en_us")]
    EnUs(Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct RichTextMessage {
    post: LanguageContent,
}

impl MessageTrait for RichTextMessage {
    const MESSAGE_TYPE: &'static str = "post";

    fn message_type(&self) -> &'static str {
        Self::MESSAGE_TYPE
    }
}

impl RichTextMessage {
    pub fn new(title: impl ToString, content: Vec<Vec<RichTextParagraph>>) -> Self {
        Self {
            post: LanguageContent::ZhCn(RichTextContent {
                title: title.to_string(),
                content,
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RichTextContent {
    title: String,
    content: Vec<Vec<RichTextParagraph>>,
}

/// 富文本支持的标签和参数
#[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(transparent)]
#[serde(tag = "tag")]
pub enum RichTextParagraph {
    #[serde(rename = "text")]
    Text {
        text: String,
        un_escape: Option<bool>,
    },
    #[serde(rename = "a")]
    A { text: String, href: String },
    #[serde(rename = "at")]
    At {
        user_id: String,
        user_name: Option<String>,
    },
    #[serde(rename = "img")]
    Img { image_key: String },
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

#[cfg(test)]
mod test {
    use crate::message::RichTextMessage;

    #[test]
    fn test_serialize() {
        let rich_text = RichTextMessage::new("a", vec![]);

        println!("{:?}", serde_json::to_string(&rich_text))
    }
}
