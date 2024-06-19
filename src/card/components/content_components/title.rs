use std::{collections::HashMap, default::Default, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::card::{FeishuCardHeaderTemplate, FeishuCardLanguage, FeishuCardStyle, TextTag};

/// 标题组件
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardTitle {
    /// 配置卡片的主标题信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
    /// 配置卡片的副标题信息。
    ///
    /// 不允许只配置副标题内容。如果只配置副标题，则实际展示为主标题效果。
    /// 副标题内容最多 1 行，超长文案末尾使用 ... 进行省略。
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<Title>,
    /// 标题的标签属性。最多可配置 3 个标签内容，如果配置的标签数量超过 3 个，则取前 3
    /// 个标签进行展示。标签展示顺序与数组顺序一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    text_tag_list: Option<Vec<TextTag>>,
    /// 标题标签的国际化属性
    #[serde(skip_serializing_if = "Option::is_none")]
    i18n_text_tag_list: Option<HashMap<FeishuCardLanguage, Vec<TextTag>>>,
    /// 标题主题颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<FeishuCardHeaderTemplate>,
    /// 该对象用于设置标题的前缀图标。一个卡片仅可配置一个标题图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<FeishuCardIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ud_icon: Option<FeishuCardUdIcon>,
}

impl FeishuCardTitle {
    pub fn new() -> Self {
        FeishuCardTitle::default()
    }

    /// 设置标题
    pub fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置副标题
    pub fn subtitle(mut self, subtitle: Title) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    /// 设置标题图标
    pub fn icon(mut self, icon: FeishuCardIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// 设置自定义图标
    pub fn ud_icon(mut self, ud_icon: FeishuCardUdIcon) -> Self {
        self.ud_icon = Some(ud_icon);
        self
    }

    /// 设置标题主题颜色
    pub fn template(mut self, template: &str) -> Self {
        let template = FeishuCardHeaderTemplate::from_str(template).expect("invalid template");
        self.template = Some(template);
        self
    }

    /// 设置标题标签
    pub fn text_tag_list(mut self, text_tag_list: Vec<TextTag>) -> Self {
        self.text_tag_list = Some(text_tag_list);
        self
    }

    /// 设置标题标签的国际化属性
    pub fn i18n_text_tag_list(
        mut self,
        i18n_text_tag_list: HashMap<FeishuCardLanguage, Vec<TextTag>>,
    ) -> Self {
        self.i18n_text_tag_list = Some(i18n_text_tag_list);
        self
    }
}

/// 标题信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    /// 文本标识。固定取值：plain_text
    tag: String,
    /// 卡片主标题内容。
    ///
    /// 必须配置 content 或 i18n 两个属性的其中一个。如果同时配置仅生效 i18n。
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// 国际化文本内容，其中：
    ///
    /// zh_cn：简体中文
    /// en_us：英文
    /// ja_jp：日文
    /// zh_hk：繁体中文（中国香港）
    /// zh_tw：繁体中文（中国台湾）
    #[serde(skip_serializing_if = "Option::is_none")]
    i18n: Option<HashMap<FeishuCardLanguage, String>>,
}

impl Default for Title {
    fn default() -> Self {
        Title {
            tag: "plain_text".to_string(),
            content: None,
            i18n: None,
        }
    }
}

impl Title {
    pub fn new(content: &str) -> Self {
        Self::default().content(content)
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    pub fn i18n(mut self, i18n: HashMap<FeishuCardLanguage, String>) -> Self {
        self.i18n = Some(i18n);
        self
    }
}

/// 文件图标
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardIcon {
    tag: String,
    /// 图标 key 的获取方式：调用上传图片接口，上传用于发送消息的图片，并在返回值中获取图片的
    /// image_key。
    #[serde(skip_serializing_if = "Option::is_none")]
    img_key: Option<String>,
}

impl Default for FeishuCardIcon {
    fn default() -> Self {
        FeishuCardIcon {
            tag: "custom_icon".to_string(),
            img_key: None,
        }
    }
}

impl FeishuCardIcon {
    pub fn new() -> Self {
        FeishuCardIcon::default()
    }

    pub fn img_key(mut self, img_key: &str) -> Self {
        self.img_key = Some(img_key.to_string());
        self
    }

    pub fn build(self) -> FeishuCardIcon {
        self
    }
}

/// 图标库图标
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardUdIcon {
    tag: String,
    /// 图标库中图标的 token。
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    /// 图标的样式。支持自定义图标颜色。
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<FeishuCardStyle>,
}

impl FeishuCardUdIcon {
    pub fn new(token: &str) -> Self {
        Self {
            tag: "standard_icon".to_string(),
            token: Some(token.to_string()),
            ..Default::default()
        }
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn style(mut self, style: FeishuCardStyle) -> Self {
        self.style = Some(style);
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::{components::content_components::plain_text::PlainText, TextTag};

    use super::*;

    #[test]
    fn test_title() {
        let title = Title::new("content").i18n(
            vec![
                (FeishuCardLanguage::ZhCN, "中文".to_string()),
                (FeishuCardLanguage::EnUS, "english".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        let json = json!({"tag":"plain_text","content":"content","i18n":{"zh_cn":"中文","en_us":"english"}});

        assert_eq!(serde_json::to_value(&title).unwrap(), json);
    }

    #[test]
    fn test_feishu_card_title() {
        let title = FeishuCardTitle::new()
            .title(Title::new("示例标题"))
            .subtitle(Title::new("示例文本"))
            .template("blue")
            .text_tag_list(vec![
                TextTag::new()
                    .text(PlainText::text("标签 1"))
                    .color("neutral"),
                TextTag::new()
                    .text(PlainText::text("标签 2"))
                    .color("neutral"),
            ])
            .ud_icon(FeishuCardUdIcon::new("meego_colorful"));
        let json = serde_json::to_value(&title).unwrap();

        assert_eq!(
            json,
            json!({
                 "title": {
                    "tag": "plain_text",
                    "content": "示例标题"
                },
                "subtitle": {
                    "tag": "plain_text",
                    "content": "示例文本"
                },
                "text_tag_list": [
                    {
                        "tag": "text_tag",
                        "text": {
                            "tag": "plain_text",
                            "content": "标签 1"
                        },
                        "color": "neutral"
                    },
                    {
                        "tag": "text_tag",
                        "text": {
                            "tag": "plain_text",
                            "content": "标签 2"
                        },
                        "color": "neutral"
                    }
                ],
                "template": "blue",
                "ud_icon": {
                    "tag": "standard_icon",
                    "token": "meego_colorful"
                }
            })
        );
    }
}
