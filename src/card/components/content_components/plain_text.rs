use serde::{Deserialize, Serialize};

use crate::card::icon::FeishuCardTextIcon;

/// 文本组件
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardText {
    /// 组件的标签。普通文本组件的标签为 div。
    tag: String,
    /// 配置卡片的普通文本信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PlainText>,
    /// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<FeishuCardTextIcon>,
}

impl Default for FeishuCardText {
    fn default() -> Self {
        FeishuCardText {
            tag: "div".to_string(),
            text: None,
            icon: None,
        }
    }
}

impl FeishuCardText {
    pub fn new() -> Self {
        FeishuCardText::default()
    }

    pub fn text(mut self, text: PlainText) -> Self {
        self.text = Some(text);
        self
    }

    pub fn icon(mut self, icon: FeishuCardTextIcon) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// text 结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct PlainText {
    /// 文本类型的标签。可取值：
    ///
    /// plain_text：普通文本内容
    /// lark_md：支持部分 Markdown 语法的文本内容。详情参考下文 lark_md 支持的 Markdown 语法
    tag: String,
    /// 文本内容。当 tag 为 lark_md 时，支持部分 Markdown 语法的文本内容。
    content: String,
    /// 文本大小。可取值如下所示。如果你填写了其它值，卡片将展示为 normal
    /// 字段对应的字号。你也可分别为移动端和桌面端定义不同的字号
    #[serde(skip_serializing_if = "Option::is_none")]
    text_size: Option<String>,
    /// 文本的颜色。仅在 tag 为 plain_text 时生效。可取值：
    ///
    /// default：客户端浅色主题模式下为黑色；客户端深色主题模式下为白色
    /// 颜色的枚举值。详情参考颜色枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    text_color: Option<String>,
    /// 文本对齐方式。可取值：
    ///
    /// - left：左对齐
    /// - center：居中对齐
    /// - right：右对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    text_align: Option<String>,
    /// 内容最大显示行数，超出设置行的内容用 ... 省略。
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<i32>,
}

impl Default for PlainText {
    fn default() -> Self {
        PlainText {
            tag: "plain_text".to_string(),
            content: "".to_string(),
            text_size: None,
            text_color: None,
            text_align: None,
            lines: None,
        }
    }
}

impl PlainText {
    pub fn text(content: &str) -> Self {
        Self {
            tag: "plain_text".to_string(),
            content: content.to_string(),
            text_size: None,
            text_color: None,
            text_align: None,
            lines: None,
        }
    }

    pub fn markdown(content: &str) -> Self {
        Self {
            tag: "lark_md".to_string(),
            content: content.to_string(),
            text_size: None,
            text_color: None,
            text_align: None,
            lines: None,
        }
    }

    /// 文本类型的标签。可取值：
    ///
    /// plain_text：普通文本内容
    /// lark_md：支持部分 Markdown 语法的文本内容
    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = tag.to_string();
        self
    }

    /// 文本内容
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn text_size(mut self, text_size: &str) -> Self {
        self.text_size = Some(text_size.to_string());
        self
    }

    pub fn text_color(mut self, text_color: &str) -> Self {
        self.text_color = Some(text_color.to_string());
        self
    }

    pub fn text_align(mut self, text_align: &str) -> Self {
        self.text_align = Some(text_align.to_string());
        self
    }

    pub fn lines(mut self, lines: i32) -> Self {
        self.lines = Some(lines);
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[test]
    fn test_message_card_text() {
        use super::*;
        let text = FeishuCardText::new()
            .text(
                PlainText::text("这是一段普通文本示例。")
                    .text_size("cus-0")
                    .text_align("center")
                    .text_color("default"),
            )
            .icon(
                FeishuCardTextIcon::new()
                    .token("app-default_filled")
                    .color("blue"),
            );
        let json = json!({
                "tag": "div",
                "text": {
                  "tag": "plain_text",
                  "content": "这是一段普通文本示例。",
                  "text_size": "cus-0",
                  "text_align": "center",
                  "text_color": "default"
                },
                "icon": {
                  "tag": "standard_icon",
                  "token": "app-default_filled",
                  "color": "blue"
                }
              }
        );
        assert_eq!(json, serde_json::to_value(&text).unwrap());
    }
}
