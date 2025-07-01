use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::json;
use strum_macros::EnumString;

use crate::{
    card::{
        components::{
            content_components::{plain_text::PlainText, title::FeishuCardTitle},
            CardElement,
        },
        text::CustomTextSize,
    },
    service::im::v1::message::SendMessageTrait,
};

/// 卡片组件模块
/// 
/// 提供各种卡片UI组件，包括内容组件、交互组件、布局组件等
pub mod components;

/// 链接处理模块
/// 
/// 处理卡片中的链接和跳转逻辑
pub mod href;

/// 图标管理模块
/// 
/// 管理卡片中使用的各种图标资源
pub mod icon;

/// 交互处理模块
/// 
/// 处理卡片的用户交互事件和回调
pub mod interactions;

/// 文本样式模块
/// 
/// 定义卡片中文本的样式和格式化
pub mod text;

/// 飞书消息卡片
///
/// 用于创建和发送交互式消息卡片。支持多语言、富文本、交互组件等丰富功能。
/// 卡片可以包含文本、图片、按钮、表单等多种组件，提供丰富的用户交互体验。
///
/// # 主要特性
///
/// - 🌐 多语言支持
/// - 🎨 丰富的UI组件
/// - 🔄 交互式操作
/// - 📱 响应式布局
/// - 🔧 高度可定制
///
/// # 支持的组件
///
/// - **文本组件**: 纯文本、富文本、标题
/// - **媒体组件**: 图片、视频
/// - **交互组件**: 按钮、输入框、选择器
/// - **布局组件**: 分栏、折叠面板
/// - **数据组件**: 表格、图表
///
/// # 示例
///
/// ```no_run
/// use open_lark::card::{FeishuCard, FeishuCardConfig};
/// use open_lark::card::components::content_components::title::FeishuCardTitle;
/// use open_lark::card::components::content_components::title::Title;
/// use open_lark::card::components::CardElement;
///
/// // 创建简单卡片
/// let card = FeishuCard::new()
///     .config(
///         FeishuCardConfig::new()
///             .enable_forward(true)
///             .update_multi(false)
///     )
///     .header("zh_cn",
///         FeishuCardTitle::new()
///             .title(Title::new("欢迎使用飞书卡片"))
///     )?
///     .elements("zh_cn", vec![
///         // 添加卡片元素
///     ])?;
/// # Ok::<(), open_lark::core::error::LarkAPIError>(())
/// ```
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCard {
    /// config 用于配置卡片的全局行为，包括是否允许被转发、是否为共享卡片等。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<FeishuCardConfig>,
    /// 用于配置卡片的标题
    pub i18n_header: HashMap<FeishuCardLanguage, FeishuCardTitle>,
    /// 卡片的多语言正文内容
    pub i18n_elements: HashMap<FeishuCardLanguage, Vec<CardElement>>,
}

impl SendMessageTrait for FeishuCard {
    fn msg_type(&self) -> String {
        "interactive".to_string()
    }

    fn content(&self) -> String {
        json!(self).to_string()
    }
}

impl FeishuCard {
    /// 创建新的飞书卡片
    ///
    /// 默认使用中文语言环境，创建包含默认标题和空元素列表的卡片。
    pub fn new() -> Self {
        let lng = FeishuCardLanguage::ZhCN;
        let mut header = HashMap::new();
        header.insert(lng, FeishuCardTitle::default());
        let mut elements = HashMap::new();
        elements.insert(lng, vec![]);
        Self {
            config: None,
            i18n_header: header,
            i18n_elements: elements,
        }
    }

    /// 设置卡片全局配置
    /// 
    /// # 参数
    /// * `config` - 卡片配置对象
    pub fn config(mut self, config: FeishuCardConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置卡片标题
    /// 
    /// # 参数  
    /// * `lng` - 语言代码 (如 "zh_cn", "en_us")
    /// * `header` - 卡片标题对象
    pub fn header(
        mut self,
        lng: &str,
        header: FeishuCardTitle,
    ) -> Result<Self, crate::core::error::LarkAPIError> {
        let language: FeishuCardLanguage = lng.parse().map_err(|e| {
            crate::core::error::LarkAPIError::illegal_param(format!(
                "unknown language '{lng}': {e}"
            ))
        })?;
        let origin_header = self.i18n_header.entry(language).or_default();
        *origin_header = header;

        Ok(self)
    }

    /// 添加卡片组件
    /// 
    /// # 参数
    /// * `lng` - 语言代码 (如 "zh_cn", "en_us") 
    /// * `elements` - 卡片组件列表
    pub fn elements(
        mut self,
        lng: &str,
        elements: Vec<CardElement>,
    ) -> Result<Self, crate::core::error::LarkAPIError> {
        let language: FeishuCardLanguage = lng.parse().map_err(|e| {
            crate::core::error::LarkAPIError::illegal_param(format!(
                "unknown language '{lng}': {e}"
            ))
        })?;
        let self_elements = self.i18n_elements.entry(language).or_default();
        self_elements.extend(elements);
        Ok(self)
    }
}

/// 卡片全局行为设置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardConfig {
    /// 是否允许转发卡片。取值：
    ///
    /// - true：允许
    /// - false：不允许 默认值为 true，该字段要求飞书客户端的版本为 V3.31.0 及以上。
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
    /// - true：在用户点击消息翻译后，使用 i18n 对应的目标语种作为翻译结果。若 i18n
    ///   取不到，则使用当前内容请求飞书的机器翻译。
    /// - false：不使用自定义翻译数据，直接请求飞书的机器翻译。
    #[serde(skip_serializing_if = "Option::is_none")]
    use_custom_translation: Option<bool>,
    /// 转发的卡片是否仍然支持回传交互。
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_forward_interaction: Option<bool>,
    ///  添加自定义字号和颜色。可应用于组件的 JSON 数据中，设置字号和颜色属性。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<FeishuCardStyle>,
}

impl FeishuCardConfig {
    /// 创建新的卡片配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否允许转发卡片
    /// 
    /// # 参数
    /// * `enable_forward` - true为允许转发，false为不允许
    pub fn enable_forward(mut self, enable_forward: bool) -> Self {
        self.enable_forward = Some(enable_forward);
        self
    }

    /// 设置是否为共享卡片
    /// 
    /// # 参数  
    /// * `update_multi` - true为共享卡片，false为独享卡片
    pub fn update_multi(mut self, update_multi: bool) -> Self {
        self.update_multi = Some(update_multi);
        self
    }

    /// 设置卡片宽度模式
    /// 
    /// # 参数
    /// * `width_mode` - 宽度模式：默认宽度或自适应屏幕宽度
    pub fn width_mode(mut self, width_mode: FeishuCardWidthMode) -> Self {
        self.width_mode = Some(width_mode);
        self
    }

    /// 设置是否使用自定义翻译数据
    /// 
    /// # 参数
    /// * `use_custom_translation` - true为使用自定义翻译，false为使用机器翻译
    pub fn use_custom_translation(mut self, use_custom_translation: bool) -> Self {
        self.use_custom_translation = Some(use_custom_translation);
        self
    }

    /// 设置转发的卡片是否仍然支持回传交互
    /// 
    /// # 参数
    /// * `enable_forward_interaction` - true为支持交互，false为不支持
    pub fn enable_forward_interaction(mut self, enable_forward_interaction: bool) -> Self {
        self.enable_forward_interaction = Some(enable_forward_interaction);
        self
    }

    /// 设置卡片样式
    /// 
    /// # 参数
    /// * `style` - 卡片样式配置，包括字号和颜色
    pub fn style(mut self, style: FeishuCardStyle) -> Self {
        self.style = Some(style);
        self
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

/// 卡片样式配置
/// 
/// 用于定义卡片的字号和颜色样式，支持为不同主题和设备定制样式
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardStyle {
    /// 分别为移动端和桌面端添加自定义字号。用于在普通文本组件和富文本组件 JSON
    /// 中设置字号属性。支持添加多个自定义字号对象。
    #[serde(skip_serializing_if = "Option::is_none")]
    text_size: Option<HashMap<String, CustomTextSize>>,
    /// 分别为飞书客户端浅色主题和深色主题添加 RGBA 语法。用于在组件 JSON
    /// 中设置颜色属性。支持添加多个自定义颜色对象。
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<HashMap<String, String>>,
}

/// 飞书卡片支持的语言类型
/// 
/// 用于卡片的多语言支持，可为不同语言环境提供相应的内容
#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Clone, Copy)]
pub enum FeishuCardLanguage {
    /// 简体中文
    #[serde(rename = "zh_cn")]
    #[default]
    ZhCN,
    /// 英文（美国）
    #[serde(rename = "en_us")]
    EnUS,
    /// 日文
    #[serde(rename = "ja_jp")]
    JaJP,
    /// 繁体中文（香港）
    #[serde(rename = "zh_hk")]
    ZhHK,
    /// 繁体中文（台湾）
    #[serde(rename = "zh_tw")]
    ZhTW,
}

impl FromStr for FeishuCardLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "zh_cn" => Ok(FeishuCardLanguage::ZhCN),
            "en_us" => Ok(FeishuCardLanguage::EnUS),
            "ja_jp" => Ok(FeishuCardLanguage::JaJP),
            "zh_hk" => Ok(FeishuCardLanguage::ZhHK),
            "zh_tw" => Ok(FeishuCardLanguage::ZhTW),
            _ => Err(format!("unknown language: {s}")),
        }
    }
}

/// 标题的标签属性。最多可配置 3 个标签内容，如果配置的标签数量超过 3 个，则取前 3
/// 个标签进行展示。标签展示顺序与数组顺序一致。
#[derive(Debug, Serialize, Deserialize)]
pub struct TextTag {
    /// 标题标签的标识。固定取值：text_tag
    tag: String,
    /// 标题标签的内容。基于文本组件的 plain_text 模式定义内容。
    text: Option<PlainText>,
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

impl TextTag {
    /// 创建新的文本标签
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置标签文本内容
    /// 
    /// # 参数
    /// * `text` - 标签的文本内容
    pub fn text(mut self, text: PlainText) -> Self {
        self.text = Some(text);
        self
    }

    /// 设置标签颜色
    /// 
    /// # 参数
    /// * `color` - 标签的颜色值
    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
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
