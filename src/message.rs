use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    /// 消息类型
    pub msg_type: MessageType,
    /// 消息内容
    pub content: Value,
}

/// 消息类型枚举
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageType {
    /// 文本消息
    #[serde(rename = "text")]
    Text,
    /// 富文本消息
    #[serde(rename = "post")]
    RichText,
    /// 群名片
    #[serde(rename = "share_chat")]
    ShareChat,
    /// 图片
    #[serde(rename = "image")]
    Image,
    /// 消息卡片
    #[serde(rename = "interactive")]
    Interactive,
}
