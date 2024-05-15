use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// 自定义字号
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomTextSize {
    ///  在无法差异化配置字号的旧版飞书客户端上，生效的字号属性。选填。
    pub default: FeishuCardTextSize,
    /// 桌面端的字号。
    pub pc: FeishuCardTextSize,
    /// 移动端的字号。
    pub mobile: FeishuCardTextSize,
}

/// 文本大小
#[derive(Debug, Serialize, Deserialize, Default, EnumString)]
pub enum FeishuCardTextSize {
    /// 特大标题（30px）
    #[serde(rename = "heading-0")]
    #[strum(serialize = "heading-0")]
    Heading0,
    /// 一级标题（24px）
    #[serde(rename = "heading-1")]
    #[strum(serialize = "heading-1")]
    Heading1,
    /// 二级标题（20px）
    #[serde(rename = "heading-2")]
    #[strum(serialize = "heading-2")]
    Heading2,
    /// 三级标题（18px）
    #[serde(rename = "heading-3")]
    #[strum(serialize = "heading-3")]
    Heading3,
    /// 四级标题（16px）
    #[serde(rename = "heading-4")]
    #[strum(serialize = "heading-4")]
    Heading4,
    /// 标题（16px）
    #[serde(rename = "heading")]
    #[strum(serialize = "heading")]
    Heading,
    /// 正文（14px）
    #[serde(rename = "normal")]
    #[default]
    #[strum(serialize = "normal")]
    Normal,
    /// 辅助性文字（12px）
    #[serde(rename = "notation")]
    #[strum(serialize = "notation")]
    Notation,
    /// 30px
    #[serde(rename = "xxxx-large")]
    #[strum(serialize = "xxxx-large")]
    XxxxLarge,
    /// 24px,
    #[serde(rename = "xxx-large")]
    #[strum(serialize = "xxx-large")]
    XxxLarge,
    /// 20px
    #[serde(rename = "xx-large")]
    #[strum(serialize = "xx-large")]
    XxLarge,
    /// 18px
    #[serde(rename = "x-large")]
    #[strum(serialize = "x-large")]
    XLarge,
    /// 16px
    #[serde(rename = "large")]
    #[strum(serialize = "large")]
    Large,
    /// 14px
    #[serde(rename = "medium")]
    #[strum(serialize = "medium")]
    Medium,
    /// 12px
    #[serde(rename = "small")]
    #[strum(serialize = "small")]
    Small,
    /// 10px
    #[serde(rename = "x-small")]
    #[strum(serialize = "x-small")]
    XSmall,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
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
