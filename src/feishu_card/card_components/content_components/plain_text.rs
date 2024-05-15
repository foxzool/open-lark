use serde::{Deserialize, Serialize};
use crate::feishu_card::color::Color;

use crate::feishu_card::icon::FeishuCardTextIcon;
use crate::feishu_card::text::FeishuCardTextSize;
use crate::feishu_card::text::TextAlign;

/// 文本组件
#[derive(Debug, Serialize, Deserialize)]
pub struct CardPlainText {
    /// 组件的标签。普通文本组件的标签为 div。
    pub tag: String,
    /// 配置卡片的普通文本信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<PlainTextContent>,
    /// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FeishuCardTextIcon>,
}

impl Default for CardPlainText {
    fn default() -> Self {
        CardPlainText {
            tag: "div".to_string(),
            text: None,
            icon: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlainTextContent {
    /// 文本类型的标签。可取值：
    ///
    /// plain_text：普通文本内容
    /// lark_md：支持部分 Markdown 语法的文本内容。详情参考下文 lark_md 支持的 Markdown 语法
    tag: TextTag,
    /// 文本内容。当 tag 为 lark_md 时，支持部分 Markdown 语法的文本内容。
    content: String,
    /// 文本大小。可取值如下所示。如果你填写了其它值，卡片将展示为 normal 字段对应的字号。你也可分别为移动端和桌面端定义不同的字号
    #[serde(skip_serializing_if = "Option::is_none")]
    text_size: Option<FeishuCardTextSize>,
    /// 文本的颜色。仅在 tag 为 plain_text 时生效。可取值：
    ///
    /// default：客户端浅色主题模式下为黑色；客户端深色主题模式下为白色
    /// 颜色的枚举值。详情参考颜色枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    text_color: Option<Color>,
    /// 文本对齐方式。可取值：
    ///
    /// - left：左对齐
    /// - center：居中对齐
    /// - right：右对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    text_align: Option<TextAlign>,
    /// 内容最大显示行数，超出设置行的内容用 ... 省略。
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<i32>,
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
    use serde_json::json;
    use crate::feishu_card::icon::TextIconType;

    #[test]
    fn test_message_card_text() {
        use super::*;
        let text = CardPlainText {
            tag: "div".to_string(),
            text: Some(PlainTextContent {
                tag: TextTag::PlainText,
                content: "这是一段普通文本示例。".to_string(),
                text_size: Some(FeishuCardTextSize::Normal),
                text_color: Some("default".to_string()),
                text_align: Some(TextAlign::Center),
                lines: None,
            }),
            icon: Some(FeishuCardTextIcon {
                tag: Some(TextIconType::StandardIcon),
                token: Some("app-default_filled".to_string()),
                color: Some("blue".to_string()),
                img_key: None,
            }),
        };
        let json = serde_json::to_value(&text).unwrap();
        assert_eq!(
            json,
            json!({
                    "tag": "div",
                    "text": {
                      "tag": "plain_text",
                      "content": "这是一段普通文本示例。",
                      "text_size": "normal",
                      "text_align": "center",
                      "text_color": "default"
                    },
                    "icon": {
                      "tag": "standard_icon",
                      "token": "app-default_filled",
                      "color": "blue"
                    }
                  }
            )
        );
    }
}
