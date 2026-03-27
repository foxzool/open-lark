//! 消息类型和内容结构定义

use serde::{Deserialize, Serialize};

/// 消息类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    /// 文本消息
    Text,
    /// 富文本消息
    Post,
    /// 图片消息
    Image,
    /// 文件消息
    File,
    /// 卡片消息（交互式）
    #[cfg(feature = "card")]
    Interactive,
}

/// 文本消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    /// 消息文本
    pub text: String,
}

impl TextContent {
    /// 创建文本消息内容。
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

/// 富文本消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostContent {
    /// 富文本 JSON 字符串
    pub post: String,
}

impl PostContent {
    /// 创建富文本消息内容。
    pub fn new(post: String) -> Self {
        Self { post }
    }
}

/// 图片消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    /// 图片 key
    pub image_key: String,
}

impl ImageContent {
    /// 创建图片消息内容。
    pub fn new(image_key: String) -> Self {
        Self { image_key }
    }
}

/// 文件消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    /// 文件 key
    pub file_key: String,
}

impl FileContent {
    /// 创建文件消息内容。
    pub fn new(file_key: String) -> Self {
        Self { file_key }
    }
}

/// 卡片消息内容（交互式）
#[cfg(feature = "card")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveContent {
    /// 卡片 JSON 对象
    pub card: serde_json::Value,
}

#[cfg(feature = "card")]
impl InteractiveContent {
    /// 创建交互式卡片消息内容。
    pub fn new(card: serde_json::Value) -> Self {
        Self { card }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_text_content_serialization() {
        let content = TextContent::new("Hello, World!".to_string());
        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["text"], "Hello, World!");
    }

    #[test]
    fn test_post_content_serialization() {
        let post_json = r#"{"title":"Test","content":"Content"}"#.to_string();
        let content = PostContent::new(post_json.clone());
        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["post"], post_json);
    }

    #[test]
    fn test_image_content_serialization() {
        let content = ImageContent::new("img_abc123".to_string());
        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["image_key"], "img_abc123");
    }

    #[test]
    fn test_file_content_serialization() {
        let content = FileContent::new("file_xyz789".to_string());
        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["file_key"], "file_xyz789");
    }

    #[test]
    fn test_msg_type_serialization() {
        let text_type = MsgType::Text;
        let json = serde_json::to_value(text_type).unwrap();
        assert_eq!(json, "text");

        let image_type = MsgType::Image;
        let json = serde_json::to_value(image_type).unwrap();
        assert_eq!(json, "image");
    }

    #[cfg(feature = "card")]
    #[test]
    fn test_interactive_content_serialization() {
        let card_json = serde_json::json!({
            "type": "template",
            "data": {
                "template_id": "test_template"
            }
        });
        let content = InteractiveContent::new(card_json.clone());
        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["card"], card_json);
    }

    #[cfg(feature = "card")]
    #[test]
    fn test_msg_type_interactive_serialization() {
        let interactive_type = MsgType::Interactive;
        let json = serde_json::to_value(interactive_type).unwrap();
        assert_eq!(json, "interactive");
    }
}
