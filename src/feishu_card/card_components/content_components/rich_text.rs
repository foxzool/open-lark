use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::feishu_card::href::FeishuCardHrefVal;
use crate::feishu_card::icon::FeishuCardTextIcon;
use crate::feishu_card::text::{FeishuCardTextSize, TextAlign};

/// Markdown 组件
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardMarkdown {
    /// 组件的标签。富文本组件固定取值为 markdown。
    pub tag: String,
    /// 设置文本内容的对齐方式。可取值有：
    ///
    /// left：左对齐
    /// center：居中对齐
    /// right：右对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<TextAlign>,
    /// 文本大小。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<FeishuCardTextSize>,
    /// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FeishuCardTextIcon>,
    /// 配置差异化跳转链接，实现“不同设备跳转链接不同”的效果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<HashMap<String, FeishuCardHrefVal>>,
    /// Markdown 文本内容
    pub content: String,
}

impl Default for FeishuCardMarkdown {
    fn default() -> Self {
        Self {
            tag: "markdown".to_string(),
            text_align: None,
            text_size: None,
            icon: None,
            href: None,
            content: "".to_string(),
        }
    }
}

pub struct FeishuCardMarkdownBuilder {
    markdown: FeishuCardMarkdown,
}

impl FeishuCardMarkdownBuilder {
    pub fn new() -> Self {
        FeishuCardMarkdownBuilder {
            markdown: FeishuCardMarkdown::default(),
        }
    }

    pub fn text_align(mut self, text_align: &str) -> Self {
        let text_align = TextAlign::from_str(text_align).unwrap();
        self.markdown.text_align = Some(text_align);
        self
    }

    pub fn text_size(mut self, text_size: &str) -> Self {
        let text_size =  FeishuCardTextSize::from_str(text_size).unwrap();
        self.markdown.text_size = Some(text_size);
        self
    }

    pub fn icon(mut self, icon: FeishuCardTextIcon) -> Self {
        self.markdown.icon = Some(icon);
        self
    }

    pub fn href(mut self, href: FeishuCardHrefVal) -> Self {
        match self.markdown.href {
            None => {
                let mut map = HashMap::new();
                map.insert("urlVal".to_string(), href);
                self.markdown.href = Some(map);
            }
            Some(mut m) => {
                m.insert("urlVal".to_string(), href);
                self.markdown.href = Some(m);
            }
        }

        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.markdown.content = content.to_string();
        self
    }

    pub fn build(self) -> FeishuCardMarkdown {
        self.markdown
    }
}

#[cfg(test)]
mod test {
    use crate::feishu_card::card_components::content_components::rich_text::FeishuCardMarkdownBuilder;
    use crate::feishu_card::href::{ FeishuCardHrefValBuilder};

    #[test]
    fn test_markdown() {
        let markdown = FeishuCardMarkdownBuilder::new()
            .href(
                FeishuCardHrefValBuilder::new("xxx1")
                    .pc_url("xxx2")
                    .ios_url("xxx3")
                    .android_url("xxx4").build(),
            )
            .content("普通文本\n标准emoji😁😢🌞💼🏆❌✅\n*斜体*\n**粗体**\n~~删除线~~\n文字链接\n差异化跳转\n<at id=all></at>")
           .build();

        let json = serde_json::to_value(&markdown).unwrap();

        assert_eq!(
            json,
            serde_json::json!( {
              "tag": "markdown",
              "href": {
                "urlVal": {
                  "url": "xxx1",
                  "pc_url": "xxx2",
                  "ios_url": "xxx3",
                  "android_url": "xxx4"
                }
              },
              "content": "普通文本\n标准emoji😁😢🌞💼🏆❌✅\n*斜体*\n**粗体**\n~~删除线~~\n文字链接\n差异化跳转\n<at id=all></at>"
            })
        );
    }
}
