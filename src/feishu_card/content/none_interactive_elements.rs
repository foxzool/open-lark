use serde::{Deserialize, Serialize};

/// 文本组件
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardPlainText {
    /// 文本元素的标签。两种模式的固定取值：
    ///
    /// - plain_text：普通文本内容。
    /// - lark_md：支持部分 Markdown 语法的文本内容。关于 Markdown 语法的详细介绍，可参见
    pub tag: String,
    /// 文本内容。
    pub content: String,
    /// 内容显示行数。该字段仅支持 text 的 plain_text 模式，不支持 lark_md 模式。
    pub lines: Option<i32>,
}

/// 文本元素的标签
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum FeishuCardTextTag {
    #[default]
    #[serde(rename = "plain_text")]
    PlainText,
    #[serde(rename = "lark_md")]
    LarkMd,
}

#[cfg(test)]
mod test {

    #[test]
    fn test_message_card_text() {
        use super::*;
        let text = FeishuCardPlainText {
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
