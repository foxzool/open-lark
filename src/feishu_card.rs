use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::json;
use strum_macros::EnumString;

use crate::{
    feishu_card::{
        card_components::{
            containers::column_set::FeishuCardColumnSet,
            content_components::{
                plain_text::PlainTextContent, rich_text::FeishuCardMarkdown, title::FeishuCardTitle,
            },
        },
        text::CustomTextSize,
    },
    service::im::v1::message::SendMessageTrait,
};

pub mod card_components;
pub mod color;
pub mod href;
pub mod icon;
pub mod text;

/// 飞书卡片
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCard {
    /// config 用于配置卡片的全局行为，包括是否允许被转发、是否为共享卡片等。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<FeishuCardConfig>,
    /// 用于配置卡片的标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_header: Option<HashMap<FeishuCardLanguage, FeishuCardTitle>>,
    /// 卡片的多语言正文内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_elements: Option<HashMap<FeishuCardLanguage, Vec<FeishuCardElement>>>,
}

impl SendMessageTrait for FeishuCard {
    fn msg_type(&self) -> String {
        "interactive".to_string()
    }

    fn content(&self) -> String {
        json!(self).to_string()
    }
}

pub struct FeishuCardBuilder {
    current_language: FeishuCardLanguage,
    config: Option<FeishuCardConfig>,
    i18n_header: Option<HashMap<FeishuCardLanguage, FeishuCardTitle>>,
    i18n_elements: Option<HashMap<FeishuCardLanguage, Vec<FeishuCardElement>>>,
}

impl FeishuCardBuilder {
    pub fn new() -> Self {
        let lng = FeishuCardLanguage::ZhCN;
        let mut header = HashMap::new();
        header.insert(lng, FeishuCardTitle::default());
        let mut elements = HashMap::new();
        elements.insert(lng, vec![]);
        FeishuCardBuilder {
            current_language: FeishuCardLanguage::ZhCN,
            config: None,
            i18n_header: Some(header),
            i18n_elements: Some(elements),
        }
    }

    pub fn current_language(mut self, language: &str) -> Self {
        self.current_language = language.parse().unwrap();
        self
    }

    pub fn add_language(mut self, language: &str) -> Self {
        let lng: FeishuCardLanguage = language.parse().unwrap();
        let mut header = HashMap::new();
        header.insert(lng, FeishuCardTitle::default());
        let mut elements = HashMap::<FeishuCardLanguage, Vec<FeishuCardElement>>::new();
        elements.insert(lng, vec![]);
        self
    }

    pub fn config(mut self, config: FeishuCardConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn header(mut self, header: FeishuCardTitle) -> Self {
        let mut i18n_header = self.i18n_header.unwrap_or_default();
        let mut origin_header = i18n_header.entry(self.current_language).or_default();
        *origin_header = header;
        self.i18n_header = Some(i18n_header);
        self
    }

    pub fn push_header(mut self, header: FeishuCardTitle) -> Self {
        let mut i18n_header = self.i18n_header.unwrap_or_default();
        let mut origin_header = i18n_header.entry(self.current_language).or_default();
        *origin_header = header;
        self.i18n_header = Some(i18n_header);
        self
    }

    pub fn i18n_element(
        mut self,
        i18n_element: HashMap<FeishuCardLanguage, Vec<FeishuCardElement>>,
    ) -> Self {
        self.i18n_elements = Some(i18n_element);
        self
    }

    /// 向默认语言添加组件
    pub fn push_element(mut self, element: FeishuCardElement) -> Self {
        let mut i18n_element = self.i18n_elements.unwrap_or_default();
        let elements = i18n_element.entry(self.current_language).or_default();
        elements.push(element);
        self.i18n_elements = Some(i18n_element);
        self
    }

    /// 向指定语言添加组件
    pub fn push_language_element(mut self, language: &str, element: FeishuCardElement) -> Self {
        let lng: FeishuCardLanguage = language.parse().unwrap();
        let mut i18n_element = self.i18n_elements.unwrap_or_default();
        let elements = i18n_element.entry(lng).or_default();
        elements.push(element);
        self.i18n_elements = Some(i18n_element);
        self
    }

    pub fn build(self) -> FeishuCard {
        FeishuCard {
            config: self.config,
            i18n_header: self.i18n_header,
            i18n_elements: self.i18n_elements,
        }
    }
}

/// 卡片全局行为设置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardConfig {
    /// 是否允许转发卡片。取值：
    ///
    /// - true：允许
    /// - false：不允许
    /// 默认值为 true，该字段要求飞书客户端的版本为 V3.31.0 及以上。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_forward: Option<bool>,
    /// 是否为共享卡片。取值：
    ///
    /// - true：是共享卡片，更新卡片的内容对所有收到这张卡片的人员可见。
    /// - false：非共享卡片，即独享卡片，仅操作用户可见卡片的更新内容。
    ///
    /// 默认值为 false。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_multi: Option<bool>,
    /// 卡片宽度模式。取值：
    ///
    /// - default：默认宽度。PC 端宽版、iPad 端上的宽度上限为 600px。
    /// - fill：自适应屏幕宽度
    pub width_mode: Option<FeishuCardWidthMode>,
    /// 是否使用自定义翻译数据。取值：
    ///
    /// - true：在用户点击消息翻译后，使用 i18n 对应的目标语种作为翻译结果。若 i18n 取不到，则使用当前内容请求飞书的机器翻译。
    /// - false：不使用自定义翻译数据，直接请求飞书的机器翻译。
    pub use_custom_translation: Option<bool>,
    /// 转发的卡片是否仍然支持回传交互。
    pub enable_forward_interaction: Option<bool>,
    ///  添加自定义字号和颜色。可应用于组件的 JSON 数据中，设置字号和颜色属性。
    pub style: Option<FeishuCardStyle>,
}

pub struct FeishuCardConfigBuilder {
    config: FeishuCardConfig,
}

impl FeishuCardConfigBuilder {
    pub fn new() -> Self {
        FeishuCardConfigBuilder {
            config: FeishuCardConfig::default(),
        }
    }

    pub fn enable_forward(mut self, enable_forward: bool) -> Self {
        self.config.enable_forward = Some(enable_forward);
        self
    }

    pub fn update_multi(mut self, update_multi: bool) -> Self {
        self.config.update_multi = Some(update_multi);
        self
    }

    pub fn width_mode(mut self, width_mode: FeishuCardWidthMode) -> Self {
        self.config.width_mode = Some(width_mode);
        self
    }

    pub fn use_custom_translation(mut self, use_custom_translation: bool) -> Self {
        self.config.use_custom_translation = Some(use_custom_translation);
        self
    }

    pub fn enable_forward_interaction(mut self, enable_forward_interaction: bool) -> Self {
        self.config.enable_forward_interaction = Some(enable_forward_interaction);
        self
    }

    pub fn style(mut self, style: FeishuCardStyle) -> Self {
        self.config.style = Some(style);
        self
    }

    pub fn build(self) -> FeishuCardConfig {
        self.config
    }
}

/// 卡片宽度模式
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FeishuCardWidthMode {
    /// 默认宽度。PC 端宽版、iPad 端上的宽度上限为 600px。
    #[default]
    Default,
    /// 自适应屏幕宽度
    Fill,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardStyle {
    /// 分别为移动端和桌面端添加自定义字号。用于在普通文本组件和富文本组件 JSON 中设置字号属性。支持添加多个自定义字号对象。
    #[serde(skip_serializing_if = "Option::is_none")]
    text_size: Option<HashMap<String, CustomTextSize>>,
    /// 分别为飞书客户端浅色主题和深色主题添加 RGBA 语法。用于在组件 JSON 中设置颜色属性。支持添加多个自定义颜色对象。
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Clone, Copy)]
pub enum FeishuCardLanguage {
    #[serde(rename = "zh_cn")]
    #[default]
    ZhCN,
    #[serde(rename = "en_us")]
    EnUS,
    #[serde(rename = "ja_jp")]
    JaJP,
    #[serde(rename = "zh_hk")]
    ZhHK,
    #[serde(rename = "zh_tw")]
    ZhTW,
}

impl FromStr for FeishuCardLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zh_cn" => Ok(FeishuCardLanguage::ZhCN),
            "en_us" => Ok(FeishuCardLanguage::EnUS),
            "ja_jp" => Ok(FeishuCardLanguage::JaJP),
            "zh_hk" => Ok(FeishuCardLanguage::ZhHK),
            "zh_tw" => Ok(FeishuCardLanguage::ZhTW),
            _ => Err(format!("unknown language: {}", s)),
        }
    }
}

/// 标题的标签属性。最多可配置 3 个标签内容，如果配置的标签数量超过 3 个，则取前 3 个标签进行展示。标签展示顺序与数组顺序一致。
#[derive(Debug, Serialize, Deserialize)]
pub struct TextTag {
    /// 标题标签的标识。固定取值：text_tag
    tag: String,
    /// 标题标签的内容。基于文本组件的 plain_text 模式定义内容。
    text: Option<PlainTextContent>,
    /// 标题标签的颜色，默认为蓝色（blue）
    color: Option<String>,
}

impl Default for TextTag {
    fn default() -> Self {
        TextTag {
            tag: "text_tag".to_string(),
            text: None,
            color: None,
        }
    }
}

/// 标题标签构建器
pub struct TextTagBuilder {
    text_tag: TextTag,
}

impl TextTagBuilder {
    pub fn new() -> Self {
        TextTagBuilder {
            text_tag: TextTag::default(),
        }
    }

    pub fn text(mut self, text: PlainTextContent) -> Self {
        self.text_tag.text = Some(text);
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.text_tag.color = Some(color.to_string());
        self
    }

    pub fn build(self) -> TextTag {
        self.text_tag
    }
}

/// 标题样式表
#[derive(Debug, Serialize, Deserialize, Default, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum FeishuCardHeaderTemplate {
    Blue,
    Wathet,
    Turquoise,
    Green,
    Yellow,
    Orange,
    Red,
    Carmine,
    Violet,
    Purple,
    Indigo,
    Grey,
    #[default]
    Default,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MessageCardColor {
    Neutral,
    #[default]
    Blue,
    Turquoise,
    Lime,
    Orange,
    Violet,
    Indigo,
    Wathet,
    Green,
    Yellow,
    Red,
    Purple,
    Carmine,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FeishuCardElement {
    ColumnSet(FeishuCardColumnSet),
    Hr,
    Div,
    Markdown(FeishuCardMarkdown),
    Img,
    Note,
    Actions,
}

#[cfg(test)]
mod test {
    use crate::feishu_card::card_components::content_components::rich_text::FeishuCardMarkdownBuilder;
    use crate::feishu_card::href::FeishuCardHrefValBuilder;
    use crate::feishu_card::icon::FeishuCardTextIconBuilder;

    #[test]
    fn test_build() {
        use super::*;
        let card = FeishuCardBuilder::new()
            .push_element(FeishuCardElement::Hr)
            .push_element(FeishuCardElement::Markdown(
                FeishuCardMarkdownBuilder::new()
                    .text_size("heading")
                    .text_align("center")
                    .icon(
                        FeishuCardTextIconBuilder::new()
                            .tag("standard_icon")
                            .token("chat-forbidden_outlined")
                            .color("orange")
                            .img_key("img_v2_38811724")
                            .build(),
                    )
                    .href(
                        FeishuCardHrefValBuilder::new("xxx")
                            .pc_url("xxx1")
                            .ios_url("xxx2")
                            .android_url("xxx3")
                            .build(),
                    )
                    .content("notation字号\n标准emoji 😁😢🌞💼🏆❌✅\n*斜体*\n**粗体**\n~~删除线~~\n[差异化跳转]($urlVal)\n<at id=all></at>")
                    .build(),
            ))
            .build();
        let json = serde_json::to_value(&card).unwrap();
    }
}
