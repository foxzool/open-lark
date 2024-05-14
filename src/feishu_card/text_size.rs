use serde::{Deserialize, Serialize};

/// 文本大小
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum FeishuCardTextSize {
    /// 特大标题（30px）
    #[serde(rename = "heading-0")]
    Heading0,
    /// 一级标题（24px）
    #[serde(rename = "heading-1")]
    Heading1,
    /// 二级标题（20px）
    #[serde(rename = "heading-2")]
    Heading2,
    /// 三级标题（18px）
    #[serde(rename = "heading-3")]
    Heading3,
    /// 四级标题（16px）
    #[serde(rename = "heading-4")]
    Heading4,
    /// 标题（16px）
    #[serde(rename = "heading")]
    Heading,
    /// 正文（14px）
    #[serde(rename = "normal")]
    #[default]
    Normal,
    /// 辅助性文字（12px）
    #[serde(rename = "notation")]
    Notation,
    /// 30px
    #[serde(rename = "xxxx-large")]
    XxxxLarge,
    /// 24px,
    #[serde(rename = "xxx-large")]
    XxxLarge,
    /// 20px
    #[serde(rename = "xx-large")]
    XxLarge,
    /// 18px
    #[serde(rename = "x-large")]
    XLarge,
    /// 16px
    #[serde(rename = "large")]
    Large,
    /// 14px
    #[serde(rename = "medium")]
    Medium,
    /// 12px
    #[serde(rename = "small")]
    Small,
    /// 10px
    #[serde(rename = "x-small")]
    XSmall,
}
