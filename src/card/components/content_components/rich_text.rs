use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::card::{href::FeishuCardHrefVal, icon::FeishuCardTextIcon};

/// Markdown 组件
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardMarkdown {
    /// 组件的标签。富文本组件固定取值为 markdown。
    tag: String,
    /// 设置文本内容的对齐方式。可取值有：
    ///
    /// left：左对齐
    /// center：居中对齐
    /// right：右对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    text_align: Option<String>,
    /// 文本大小。
    #[serde(skip_serializing_if = "Option::is_none")]
    text_size: Option<String>,
    /// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<FeishuCardTextIcon>,
    /// 配置差异化跳转链接，实现“不同设备跳转链接不同”的效果。
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<HashMap<String, FeishuCardHrefVal>>,
    /// Markdown 文本内容
    content: String,
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

impl FeishuCardMarkdown {
    pub fn new(text: &str) -> Self {
        Self {
            content: text.to_string(),
            ..Default::default()
        }
    }

    pub fn text_align(mut self, text_align: &str) -> Self {
        self.text_align = Some(text_align.to_string());
        self
    }

    pub fn text_size(mut self, text_size: &str) -> Self {
        self.text_size = Some(text_size.to_string());
        self
    }

    pub fn icon(mut self, icon: FeishuCardTextIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn href(mut self, href: FeishuCardHrefVal) -> Self {
        match self.href {
            None => {
                let mut map = HashMap::new();
                map.insert("urlVal".to_string(), href);
                self.href = Some(map);
            }
            Some(mut m) => {
                m.insert("urlVal".to_string(), href);
                self.href = Some(m);
            }
        }

        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::{
        components::content_components::rich_text::FeishuCardMarkdown, href::FeishuCardHrefVal,
    };

    #[test]
    fn test_markdown() {
        let markdown = FeishuCardMarkdown::new("普通文本\n标准emoji😁😢🌞💼🏆❌✅\n*斜体*\n**粗体**\n~~删除线~~\n文字链接\n差异化跳转\n<at id=all></at>")
            .href(
                FeishuCardHrefVal::new().url("xxx1")
                    .pc_url("xxx2")
                    .ios_url("xxx3")
                    .android_url("xxx4"),
            )

            ;

        let json = json!( {
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
        });

        assert_eq!(json, serde_json::to_value(&markdown).unwrap());
    }
}
