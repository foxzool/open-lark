use serde::{Deserialize, Serialize};

use crate::feishu_card::color::FeishuCardColor;
use crate::feishu_card::icon::FeishuCardIcon;
use crate::feishu_card::text_size::FeishuCardTextSize;

/// 文本组件
#[derive(Debug, Serialize, Deserialize)]
pub struct CardPlainText {
    /// 组件的标签。普通文本组件的标签为 div。
    pub tag: String,
    /// 配置卡片的普通文本信息。
    pub text: Option<PlainTextContent>,
    /// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
    pub icon: Option<PlantTextIcon>,
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
    text_color: Option<FeishuCardColor>,
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

/// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
#[derive(Debug, Serialize, Deserialize)]
pub struct PlantTextIcon {
    /// 图标类型的标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<PlaneTextIconType>,
    /// 图标库中图标的 token。当 tag 为 standard_icon 时生效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<FeishuCardIcon>,
    /// 图标的颜色。支持设置线性和面性图标（即 token 末尾为 outlined 或 filled 的图标）的颜色。当 tag 为 standard_icon 时生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<FeishuCardColor>,
    /// 自定义前缀图标的图片 key。当 tag 为 custom_icon 时生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_key: Option<String>,
}

/// 图标类型的标签
#[derive(Debug, Serialize, Deserialize)]
pub enum PlaneTextIconType {
    #[serde(rename = "standard_icon")]
    Standard,
    #[serde(rename = "custom_icon")]
    Custom,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum TextAlign {
    /// 左对齐
    #[serde(rename = "left")]
    Left,
    /// 居中对齐
    #[serde(rename = "center")]
    Center,
    /// 右对齐
    #[serde(rename = "right")]
    Right,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[test]
    fn test_message_card_text() {
        use super::*;
        let text = CardPlainText {
            tag: "div".to_string(),
            text: Some(PlainTextContent {
                tag: TextTag::PlainText,
                content: "这是一段普通文本示例。".to_string(),
                text_size: Some(FeishuCardTextSize::Normal),
                text_color: Some(FeishuCardColor::Default),
                text_align: Some(TextAlign::Center),
                lines: None,
            }),
            icon: Some(PlantTextIcon {
                tag: Some(PlaneTextIconType::Standard),
                token: Some(FeishuCardIcon::AppDefaultFilled),
                color: Some(FeishuCardColor::Blue),
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
