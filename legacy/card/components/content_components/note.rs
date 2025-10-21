use serde::{Deserialize, Serialize};

use crate::card::{
    components::content_components::{image::FeishuCardImage, plain_text::PlainText},
    icon::FeishuCardTextIcon,
};

/// 备注
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardNote {
    /// 组件的标签。备注模块组件的固定值为 note。
    tag: String,
    /// 配置卡片的备注模块信息。支持添加图标、图片以及文本。
    elements: Vec<FeishuCardNoteElement>,
}

impl Default for FeishuCardNote {
    fn default() -> Self {
        FeishuCardNote {
            tag: "note".to_string(),
            elements: vec![],
        }
    }
}

/// 备注组件支持添加图标、图片以及文本
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FeishuCardNoteElement {
    Icon(FeishuCardTextIcon),
    Image(Box<FeishuCardImage>),
    Text(PlainText),
}

impl FeishuCardNote {
    pub fn new() -> Self {
        FeishuCardNote::default()
    }

    pub fn icon(mut self, icon: FeishuCardTextIcon) -> Self {
        self.elements.push(FeishuCardNoteElement::Icon(icon));
        self
    }

    pub fn image(mut self, image: FeishuCardImage) -> Self {
        self.elements
            .push(FeishuCardNoteElement::Image(Box::new(image)));
        self
    }

    pub fn text(mut self, text: PlainText) -> Self {
        self.elements.push(FeishuCardNoteElement::Text(text));
        self
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_note() {
        use crate::card::components::content_components::note::*;
        use serde_json::json;

        let note = FeishuCardNote::new()
            .icon(
                FeishuCardTextIcon::new()
                    .tag("custom_icon")
                    .token("chat-forbidden_outlined")
                    .img_key("img_v2_041b28e3-5680-48c2-9af2-497ace79333g"),
            )
            .text(PlainText::text("备注信息1"))
            .image(
                FeishuCardImage::new()
                    .img_key("img_v2_041b28e3-5680-48c2-9af2-497ace79333g")
                    .alt(PlainText::text("这是备注图片"))
                    .build(),
            )
            .text(PlainText::text("备注信息2"));
        let json = json!({
          "tag": "note",
          "elements": [
            {
              "tag": "custom_icon",
              "token": "chat-forbidden_outlined",
              "img_key": "img_v2_041b28e3-5680-48c2-9af2-497ace79333g"
            },
            {
              "tag": "plain_text",
              "content": "备注信息1"
            },
            {
              "tag": "img",
              "img_key": "img_v2_041b28e3-5680-48c2-9af2-497ace79333g",
              "alt": {
                "tag": "plain_text",
                "content": "这是备注图片"
              }
            },
            {
              "tag": "plain_text",
              "content": "备注信息2"
            }
          ]
        });

        assert_eq!(serde_json::to_value(&note).unwrap(), json);
    }
}
