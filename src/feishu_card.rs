use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use color::*;

use crate::feishu_card::content::{FeishuCardColumnSet, FeishuCardPlainText};

mod color;
pub mod content;

/// 飞书卡片
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCard {
    /// config 用于配置卡片的全局行为，包括是否允许被转发、是否为共享卡片等。
    config: Option<FeishuCardConfig>,
    /// 用于配置卡片的标题
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<FeishuCardHeader>,
    /// 卡片的正文内容，支持配置多语言
    #[serde(skip_serializing_if = "Option::is_none")]
    elements: Option<Vec<FeishuCardElement>>,
    /// 卡片的多语言正文内容
    #[serde(skip_serializing_if = "Option::is_none")]
    i18n_element: Option<HashMap<FeishuCardLanguage, Vec<FeishuCardElement>>>,
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
    enable_forward: Option<bool>,
    /// 是否为共享卡片。取值：
    ///
    /// - true：是共享卡片，更新卡片的内容对所有收到这张卡片的人员可见。
    /// - false：非共享卡片，即独享卡片，仅操作用户可见卡片的更新内容。
    ///
    /// 默认值为 false。
    #[serde(skip_serializing_if = "Option::is_none")]
    update_multi: Option<bool>,
    /// 卡片宽度模式。取值：
    ///
    /// - default：默认宽度。PC 端宽版、iPad 端上的宽度上限为 600px。
    /// - fill：自适应屏幕宽度
    width_mode: Option<FeishuCardWidthMode>,
    /// 是否使用自定义翻译数据。取值：
    ///
    /// - true：在用户点击消息翻译后，使用 i18n 对应的目标语种作为翻译结果。若 i18n 取不到，则使用当前内容请求飞书的机器翻译。
    /// - false：不使用自定义翻译数据，直接请求飞书的机器翻译。
    use_custom_translation: Option<bool>,
    /// 转发的卡片是否仍然支持回传交互。
    enable_forward_interaction: Option<bool>,
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

pub struct FeishuCardStyle {
    text_size: Option<FeishuCardTextSize>,
    color: Option<FeishuCardColor>,
}

pub enum FeishuCardTextSize {}

/// 标题组件
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardHeader {
    /// 配置卡片的主标题信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<FeishuCardTitle>,
    /// 配置卡片的副标题信息。
    ///
    /// 不允许只配置副标题内容。如果只配置副标题，则实际展示为主标题效果。
    /// 副标题内容最多 1 行，超长文案末尾使用 ... 进行省略。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<FeishuCardTitle>,
    /// 该对象用于设置标题的前缀图标。一个卡片仅可配置一个标题图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FeishuCardImage>,
    /// 标题主题颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<FeishuCardHeaderTemplate>,
    /// 标题的标签属性。最多可配置 3 个标签内容，如果配置的标签数量超过 3 个，则取前 3 个标签进行展示。标签展示顺序与数组顺序一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_tag_list: Option<TextTagList>,
    /// 标题标签的国际化属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_text_tag_list: Option<HashMap<FeishuCardLanguage, Vec<TextTagList>>>,
}

/// 标题信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardTitle {
    /// 文本标识。固定取值：plain_text
    pub tag: Option<String>,
    /// 卡片主标题内容。
    ///
    /// 必须配置 content 或 i18n 两个属性的其中一个。如果同时配置仅生效 i18n。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 国际化文本内容，其中：
    ///
    /// zh_cn：简体中文
    /// en_us：英文
    /// ja_jp：日文
    /// zh_hk：繁体中文（中国香港）
    /// zh_tw：繁体中文（中国台湾）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n: Option<HashMap<FeishuCardLanguage, String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
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

/// 图标
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardImage {
    /// 图标 key 的获取方式：调用上传图片接口，上传用于发送消息的图片，并在返回值中获取图片的 image_key。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_key: Option<String>,
}

/// 标题的标签属性。最多可配置 3 个标签内容，如果配置的标签数量超过 3 个，则取前 3 个标签进行展示。标签展示顺序与数组顺序一致。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextTagList {
    /// 标题标签的标识。固定取值：text_tag
    pub tag: Option<String>,
    /// 标题标签的内容。基于文本组件的 plain_text 模式定义内容。
    pub text: Option<FeishuCardPlainText>,
    /// 标题标签的颜色，默认为蓝色（blue）
    pub color: Option<FeishuCardColor>,
}

/// 标题样式表
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
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
pub enum FeishuCardElement {
    ColumnSet(FeishuCardColumnSet),
    Div,
    Markdown,
    Hr,
    Img,
    Note,
    Actions,
}
