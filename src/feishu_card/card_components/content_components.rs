use serde::{Deserialize, Serialize};
use crate::feishu_card::FeishuCardTextSize;

/// 文本组件
#[derive(Debug, Serialize, Deserialize)]
pub struct CardPlainText {
    /// 组件的标签。普通文本组件的标签为 div。
    pub tag: String,
    /// 配置卡片的普通文本信息。
    pub text: Option<PlainTextContent>,
    /// 内容显示行数。该字段仅支持 text 的 plain_text 模式，不支持 lark_md 模式。
    pub lines: Option<i32>,
}

impl Default for CardPlainText {
    fn default() -> Self {
        CardPlainText {
            tag: "div".to_string(),
            text: None,
            lines: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlainTextContent {
    tag: TextTag,
    content: String,
    text_size: Option<FeishuCardTextSize>
}

/// 文本元素的标签
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum TextTag {
    /// 普通文本内容
    #[default]
    #[serde(rename = "plain_text")]
    PlainText,
    /// 支持部分 Markdown 语法的文本内容
    #[serde(rename = "lark_md")]
    LarkMd,
}

#[cfg(test)]
mod test {

    #[test]
    fn test_message_card_text() {
        use super::*;
        let text = CardPlainText {
            tag: "plain_text".to_string(),
            content: "Hello, World!".to_string(),
            lines: Some(1),
        };
        let json = serde_json::to_string(&text).unwrap();
        assert_eq!(
            json,
            r#"{"tag":"plain_text","content":"Hello, World!","lines":1}"#
        );
    }
}
