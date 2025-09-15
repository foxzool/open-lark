use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::service::im::v1::message::SendMessageTrait;

/// 文本消息
///
/// 用于发送纯文本消息，支持@用户、换行等功能。
/// 是最常用的消息类型之一。
///
/// # 特殊功能
///
/// - 支持@用户：`<at user_id="xxx"></at>`
/// - 支持@所有人：`<at user_id="all">name="全体成员"</at>`
/// - 支持换行：使用`\n`或调用`line()`方法
pub struct MessageText {
    text: String,
}

impl SendMessageTrait for MessageText {
    fn msg_type(&self) -> String {
        "text".to_string()
    }

    fn content(&self) -> String {
        json!({ "text": self.text }).to_string()
    }
}

impl MessageText {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    pub fn add_text(mut self, text: &str) -> Self {
        self.text.push_str(text);
        self
    }

    pub fn text_line(mut self, text: &str) -> Self {
        self.text.push_str(text);
        self.text.push('\n');
        self
    }

    pub fn at_user(mut self, user_id: &str) -> Self {
        self.text
            .push_str(&format!("<at user_id=\"{}\"></at>", user_id));
        self
    }

    pub fn at_all(mut self, name: &str) -> Self {
        self.text
            .push_str(&format!("<at user_id=\"all\">{}</at>", name));
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

/// 富文本参数
#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePost {
    /// 默认的语言
    #[serde(skip)]
    default_language: String,
    post: HashMap<String, MessagePostContent>,
}

impl SendMessageTrait for MessagePost {
    fn msg_type(&self) -> String {
        "post".to_string()
    }

    fn content(&self) -> String {
        json!(self).to_string()
    }
}

impl MessagePost {
    pub fn new(lng: &str) -> Self {
        let post = HashMap::new();
        Self {
            default_language: lng.to_string(),
            post,
        }
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        let post = self
            .post
            .entry(self.default_language.clone())
            .or_insert(MessagePostContent {
                title: title.to_string(),
                content: vec![],
            });
        post.title = title.to_string();
        self
    }

    /// 追加富文本内容
    pub fn append_content(mut self, contents: Vec<MessagePostNode>) -> Self {
        let post = self
            .post
            .entry(self.default_language.clone())
            .or_insert(MessagePostContent {
                title: "".to_string(),
                content: vec![],
            });
        post.content.push(contents);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessagePostContent {
    /// 富文本消息的标题。
    pub title: String,
    /// 富文本消息内容，由多个段落组成，每个段落为一个 node 列表。支持的 node 标签类型及对应参数
    pub content: Vec<Vec<MessagePostNode>>,
}

/// 富文本消息内容
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum MessagePostNode {
    /// 文本内容。
    #[serde(rename = "text")]
    Text(TextNode),
    #[serde(rename = "a")]
    A(ANode),
    #[serde(rename = "at")]
    At(AtNode),
    #[serde(rename = "img")]
    Img(ImgNode),
    #[serde(rename = "media")]
    Media(MediaNode),
    #[serde(rename = "emotion")]
    Emotion(EmotionNode),
}

/// 文本node
#[derive(Debug, Serialize, Deserialize)]
pub struct TextNode {
    text: String,
    /// 表示是不是 unescape 解码，默认为 false ，不用可以不填。
    #[serde(skip_serializing_if = "Option::is_none")]
    un_escape: Option<bool>,
    /// 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为bold、underline、
    /// lineThrough与italic，非可选值将被忽略。
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Vec<String>>,
}

impl TextNode {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            un_escape: None,
            style: None,
        }
    }

    pub fn un_escape(mut self, un_escape: bool) -> Self {
        self.un_escape = Some(un_escape);
        self
    }

    pub fn style(mut self, style: Vec<&str>) -> Self {
        self.style = Some(style.iter().map(|s| s.to_string()).collect());
        self
    }
}

/// a Node
#[derive(Debug, Serialize, Deserialize)]
pub struct ANode {
    /// 文本内容
    text: String,
    /// 默认的链接地址，请确保链接地址的合法性，否则消息会发送失败。
    href: String,
    /// 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为bold、underline、
    /// lineThrough与italic，非可选值将被忽略。
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Vec<String>>,
}

impl ANode {
    pub fn new(text: &str, href: &str) -> Self {
        Self {
            text: text.to_string(),
            href: href.to_string(),
            style: None,
        }
    }

    pub fn style(mut self, style: Vec<&str>) -> Self {
        self.style = Some(style.iter().map(|s| s.to_string()).collect());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtNode {
    /// 用户的open_id，union_id 或 user_id，请参考如何获取 User ID、Open ID 和 Union ID？
    /// 注意: @单个用户时，user_id字段必须是有效值；@所有人填"all"。
    user_id: String,
    /// 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为bold、underline、
    /// lineThrough与italic，非可选值将被忽略。
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Vec<String>>,
}

impl AtNode {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            style: None,
        }
    }

    pub fn style(mut self, style: Vec<&str>) -> Self {
        self.style = Some(style.iter().map(|s| s.to_string()).collect());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImgNode {
    /// 图片的唯一标识，可通过 上传图片 接口获取image_key。
    image_key: String,
}

impl ImgNode {
    pub fn new(image_key: &str) -> Self {
        Self {
            image_key: image_key.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaNode {
    /// 视频文件的唯一标识，可通过 上传文件 接口获取file_key
    file_key: String,
    /// 视频封面图片的唯一标识，可通过 上传图片 接口获取image_key。
    #[serde(skip_serializing_if = "Option::is_none")]
    image_key: Option<String>,
}

impl MediaNode {
    pub fn new(file_key: &str, image_key: Option<&str>) -> Self {
        Self {
            file_key: file_key.to_string(),
            image_key: image_key.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmotionNode {
    /// 表情包的类型，目前支持：static（静态表情）、dynamic（动态表情）
    emotion_type: String,
    /// 表情包的 emoji 值，参考 表情包列表
    emoji: String,
}

impl EmotionNode {
    pub fn new(emotion_type: &str, emoji: &str) -> Self {
        Self {
            emotion_type: emotion_type.to_string(),
            emoji: emoji.to_string(),
        }
    }
}

/// 图片消息
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageImage {
    /// 图片的唯一标识，可通过 上传图片 接口获取image_key
    image_key: String,
    /// 图片的高度，单位像素，默认值 0
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i32>,
    /// 图片的宽度，单位像素，默认值 0
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i32>,
}

impl SendMessageTrait for MessageImage {
    fn msg_type(&self) -> String {
        "image".to_string()
    }

    fn content(&self) -> String {
        json!(self).to_string()
    }
}

impl MessageImage {
    pub fn new(image_key: &str) -> Self {
        Self {
            image_key: image_key.to_string(),
            height: None,
            width: None,
        }
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }
}

/// 卡片模板消息
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCardTemplate {
    /// 卡片类型，目前支持：template（模板卡片）、text_only（纯文本卡片）、internal_contact（人员信息卡片）
    #[serde(rename = "type")]
    card_type: String,
    /// 卡片的数据和样式配置，详见各卡片类型示例。
    data: serde_json::Value,
}

impl SendMessageTrait for MessageCardTemplate {
    fn msg_type(&self) -> String {
        "interactive".to_string()
    }

    fn content(&self) -> String {
        json!(self).to_string()
    }
}

impl MessageCardTemplate {
    pub fn new(card_type: &str, data: serde_json::Value) -> Self {
        Self {
            card_type: card_type.to_string(),
            data,
        }
    }
}
