use std::str::FromStr;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use crate::feishu_card::color::Color;

/// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardTextIcon {
    /// 图标类型的标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<TextIconType>,
    /// 图标库中图标的 token。当 tag 为 standard_icon 时生效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 图标的颜色。支持设置线性和面性图标（即 token 末尾为 outlined 或 filled 的图标）的颜色。当 tag 为 standard_icon 时生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// 自定义前缀图标的图片 key。当 tag 为 custom_icon 时生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_key: Option<String>,
}

pub struct FeishuCardTextIconBuilder {
    icon: FeishuCardTextIcon,
}

impl FeishuCardTextIconBuilder {
    pub fn new() -> Self {
        FeishuCardTextIconBuilder {
            icon: FeishuCardTextIcon {
                tag: None,
                token: None,
                color: None,
                img_key: None,
            },
        }
    }

    pub fn tag(mut self, tag: &str) -> Self {
        let tag = TextIconType::from_str(tag).unwrap();
        self.icon.tag = Some(tag);
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.icon.token = Some(token.to_string());
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.icon.color = Some(color.to_string());
        self
    }

    pub fn img_key(mut self, img_key: &str) -> Self {
        self.icon.img_key = Some(img_key.to_string());
        self
    }

    pub fn build(self) -> FeishuCardTextIcon {
        self.icon
    }
}

/// 图标类型的标签
#[derive(Debug, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TextIconType {
    #[serde(rename = "standard_icon")]
    StandardIcon,
    #[serde(rename = "custom_icon")]
    CustomIcon,
}