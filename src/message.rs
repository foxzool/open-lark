use std::collections::HashMap;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait MessageTrait: Serialize + DeserializeOwned {
    const MESSAGE_TYPE: &'static str;

    fn message_type(&self) -> &'static str;
}

/// 文本 text
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextMessage {
    text: String,
}

impl TextMessage {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl MessageTrait for TextMessage {
    const MESSAGE_TYPE: &'static str = "text";

    fn message_type(&self) -> &'static str {
        Self::MESSAGE_TYPE
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub enum Language {
    #[serde(rename = "zh_cn")]
    ZhCn,
    #[serde(rename = "en_us")]
    EnUs,
    #[serde(rename = "ja_jp")]
    JaJp,
    #[serde(rename = "zh_hk")]
    ZhHk,
    #[serde(rename = "zh_tw")]
    ZhTw,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LanguageContent {
    #[serde(rename = "zh_cn")]
    ZhCn(RichTextContent),
    #[serde(rename = "en_us")]
    EnUs(Value),
}

impl Default for LanguageContent {
    fn default() -> Self {
        Self::ZhCn(RichTextContent::default())
    }
}

/// 自定义机器人用的富文本消息
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CustomRichTextMessage {
    post: LanguageContent,
}

impl MessageTrait for CustomRichTextMessage {
    const MESSAGE_TYPE: &'static str = "post";

    fn message_type(&self) -> &'static str {
        Self::MESSAGE_TYPE
    }
}

impl CustomRichTextMessage {
    pub fn new(title: impl ToString, content: Vec<Vec<RichTextParagraph>>) -> Self {
        Self {
            post: LanguageContent::ZhCn(RichTextContent {
                title: title.to_string(),
                content,
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RichTextContent {
    title: String,
    content: Vec<Vec<RichTextParagraph>>,
}

/// 富文本支持的标签和参数
#[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(transparent)]
#[serde(tag = "tag")]
pub enum RichTextParagraph {
    #[serde(rename = "text")]
    Text {
        text: String,
        un_escape: Option<bool>,
    },
    #[serde(rename = "a")]
    A { text: String, href: String },
    #[serde(rename = "at")]
    At {
        user_id: String,
        user_name: Option<String>,
    },
    #[serde(rename = "img")]
    Img { image_key: String },
}

impl Default for RichTextParagraph {
    fn default() -> Self {
        Self::Text {
            text: "".to_string(),
            un_escape: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImageMessage {
    image_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ShareChatMessage {
    share_chat_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InteractiveMessage {
    TemplateCard(TemplateCard),
    JsonCard(FeishuCard),
}

impl Default for InteractiveMessage {
    fn default() -> Self {
        Self::TemplateCard(TemplateCard::default())
    }
}

/// 卡片模板
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TemplateCard {
    pub template_id: String,
    pub template_variable: HashMap<String, Value>,
}

impl MessageTrait for InteractiveMessage {
    const MESSAGE_TYPE: &'static str = "interactive";

    fn message_type(&self) -> &'static str {
        Self::MESSAGE_TYPE
    }
}

/// 消息卡片
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FeishuCard {
    pub header: Header,
    /// 卡片的正文内容，支持配置多语言。如果需要配置多语言环境时，需要将 elements 替换为 i18n_elements，并且还需要同时对卡片标题进行多语言配置。详情可参见配置多语言内容。
    ///
    /// 在卡片的正文内容中，支持添加以下属性：
    ///
    /// column_set：多列布局，可以横向排布多个列容器，在列内纵向自由组合图文内容，解决多列内容对齐问题，并实现了灵活的图文混排。详情参见多列布局。
    /// div：内容模块，以格式化的文本为主体，支持混合图片、交互组件的富文本内容。详情参见内容模块。
    /// markdown：使用 Markdown 标签构造富文本内容。详情参见 Markdown。
    /// hr：模块之间的分割线。详情参见分割线。
    /// img：用于展示图片的组件。详情参见图片。
    /// note：备注组件，用于展示卡片内的次要信息。详情参见备注。
    /// actions：交互模块。使用交互组件可以实现消息卡片与用户之间的信息交互。详情参见交互模块。
    pub elements: Option<Vec<Elements>>,
    pub i18n_elements: Option<HashMap<Language, Vec<Elements>>>,
    /// 用于配置卡片的属性，包括是否允许被转发、是否为共享卡片等。详情参见配置卡片属性。
    pub config: Option<CardConfig>,
    /// 用于指定卡片整体的跳转链接。详情参见消息卡片跳转链接。
    pub card_link: Option<CarLinkComponent>,
}

/// 标题组件
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Header {
    pub title: Title,
    pub subtitle: Option<Subtitle>,
    pub icon: Option<HeaderIcon>,
    pub template: Option<String>,
    pub text_tag_list: Option<TextTagList>,
    pub i18n_text_tag_list: Option<I18nTextTagList>,
}

/// 配置卡片的主标题信息
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Title {
    tag: String,
    content: Option<String>,
    i18n: Option<Language>,
}

/// 配置卡片的副标题信息。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Subtitle {
    tag: String,
    content: Option<String>,
    i18n: Option<Language>,
}

/// 该对象用于设置标题的前缀图标。一个卡片仅可配置一个标题图标。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HeaderIcon {
    img_key: Option<String>,
}

/// 标题的标签属性。最多可配置 3 个标签内容，如果配置的标签数量超过 3 个，则取前 3 个标签进行展示。标签展示顺序与数组顺序一致。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextTagList {
    tag: Option<String>,
    text: Option<String>,
    color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct I18nTextTagList {
    language_tags: HashMap<Language, TextTagList>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Elements {
    ColumnSet(Box<ColumnComponent>),
    Div(DivComponent),
    DividerLine(DividerLineComponent),
    Image(ImageComponent),
    Note(NotesComponent),
    Actions(ActionComponent),
}

impl Default for Elements {
    fn default() -> Self {
        Self::ColumnSet(Box::default())
    }
}

/// 内容模块（div）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DivComponent {
    tag: String,
    text: TextComponent,
    fields: Option<FieldsComponent>,
    extra: Option<DivExtraComponent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DivExtraComponent {
    Image(ImageComponent),
    Button(ButtonComponent),
    SelectMenu(SelectMenuComponent),
    Overflow(OverflowComponent),
    DatePicker(DatePickerComponent),
}

impl Default for DivExtraComponent {
    fn default() -> Self {
        Self::Button(ButtonComponent::default())
    }
}

/// 文本组件
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TextComponent {
    /// 文本元素的标签。两种模式的固定取值：
    ///
    /// plain_text：普通文本内容。
    /// lark_md：支持部分 Markdown 语法的文本内容。关于 Markdown 语法的详细介绍，可参见 Markdown。
    pub tag: String,
    /// 文本内容。
    pub content: String,
    /// 内容显示行数。该字段仅支持 text 的 plain_text 模式，不支持 lark_md 模式。
    pub lines: Option<i32>,
}

/// 链接元素（url）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UrlComponent {
    /// 默认的跳转链接。
    pub url: Option<String>,
    /// Android 端的跳转链接。
    pub android_url: Option<String>,
    /// iOS 端的跳转链接。
    pub ios_url: Option<String>,
    /// PC 端的跳转链接。
    pub pc_url: Option<String>,
}

/// 消息卡片跳转链接（card_link）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CarLinkComponent {
    /// 默认的跳转链接。
    pub url: String,
    /// Android 端的跳转链接。
    pub android_url: Option<String>,
    /// iOS 端的跳转链接。
    pub ios_url: Option<String>,
    /// PC 端的跳转链接。
    pub pc_url: Option<String>,
}

/// 按钮（button）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ButtonComponent {
    /// 按钮组件的标识。固定取值：button
    pub tag: String,
    /// 按钮中的文本。基于文本组件的数据结构配置文本内容，详情参见文本组件。
    pub text: TextComponent,
    /// 点击按钮后的跳转链接。该字段与 multi_url 字段不可同时设置。
    pub url: Option<String>,
    /// 基于 url 元素配置多端跳转链接，详情参见url 元素。该字段与 url 字段不可同时设置。
    pub multi_url: Option<UrlComponent>,
    /// 配置按钮样式。取值：
    ///
    /// default：默认样式
    /// primary：强调样式
    /// danger：警示样式
    /// 默认值：default
    pub r#type: Option<String>,
    /// 该字段用于交互组件的回传交互方式,当用户点击交互组件后，会将 value 的值返回给接收回调数据的服务器。后续你可以通过服务器接收的 value 值进行业务处理。
    ///
    /// 该字段值仅支持 key-value 形式的 JSON 结构，且 key 为 String 类型。示例值:
    ///
    /// "value":{
    ///     "key-1":Object-1,
    ///     "key-2":Object-2,
    ///     "key-3":Object-3,
    ///     ······
    /// }
    pub value: Option<Value>,
    /// 设置二次确认弹框。confirm 元素的配置方式可参见 confirm。
    pub confirm: Option<ConfirmComponent>,
}

/// 双列文本
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FieldsComponent {
    /// 是否并排布局。取值：
    ///
    /// true：并排
    /// false：不并排
    pub is_short: bool,
    /// 国际化文本内容。使用文本组件的数据结构展示内容，详情参见文本组件。
    pub text: TextComponent,
}

/// 图片元素
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImageComponent {
    /// 元素标签
    pub tag: String,
    /// 图片资源，获取方式：上传图片
    pub img_key: String,
    /// 图片hover说明
    pub alt: TextComponent,
    /// 点击后是否放大图片，缺省为true。在配置 card_link 后可设置为false，使用户点击卡片上的图片也能响应card_link链接跳转
    pub preview: Option<bool>,
}

/// 二次确认（confirm）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConfirmComponent {
    pub title: TextComponent,
    pub text: TextComponent,
}

/// 列表选择器（selectMenu）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SelectMenuComponent {
    /// 列表选择器的标签。在两种模式下的固定取值：
    ///
    /// 自定义选项选择器：select_static
    /// 人员选择器：select_person
    pub tag: String,
    /// 选择器的提示文本。基于文本组件的数据结构填写内容，详情参见文本组件
    pub placeholder: Option<TextComponent>,
    /// 为列表选择器配置默认选项。在人员选择器（select_person）模式中不支持设置该字段。
    pub initial_option: Option<String>,
    /// 列表选择器中的选项。基于 option 元素添加选项内容，详情参见 option。
    ///
    /// 在自定义选项选择器中，你可以通过 option 元素的 text 字段配置选项内容。
    /// 在人员选择器中，你可以通过 option 元素的 value 字段设置用户 open_id 来指定人员，如果 value 字段不传值，则系统会自动获取当前回话内的人员列表作为选项。
    /// 注意：不支持设置 option 元素中的 url、multi_url 字段。
    pub options: Vec<OptionComponent>,
    /// 该字段用于交互组件的回传交互方式，当用户点击交互组件的选项后，会将 value 的值返回给接收回调数据的服务器。后续你可以通过服务器接收的 value 值进行业务处理。
    ///
    /// 该字段值仅支持 key-value 形式的 JSON 结构，且 key 为 String 类型。示例值：
    ///
    /// "value":{
    ///     "key-1":Object-1,
    ///     "key-2":Object-2,
    ///     "key-3":Object-3,
    ///     ······
    /// }
    pub value: Option<Value>,
    /// 设置二次确认弹框。confirm 元素的配置方式可参见 confirm。
    pub confirm: Option<ConfirmComponent>,
}

/// 选项元素（option）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OptionComponent {
    /// 选项显示的内容。
    ///
    /// 基于文本组件的 plain_text 模式设置文本内容，详情参见文本组件。
    /// 当列表选择器（selectMenu）的模式为人员选择器（select_person）时，无需配置 text 字段，除此之外，其他场景中 text 字段必填。
    pub text: Option<TextComponent>,
    /// 回传参数值。当选项选中后，应用会将该值返回至消息卡片请求地址。
    pub value: Option<String>,
    /// 选项的跳转链接，仅支持在折叠按钮组（overflow）中设置。
    ///
    /// 说明：url 和 multi_url 字段必须且仅能填写其中一个。
    pub url: Option<String>,
    /// 选项的跳转链接,仅支持在折叠按钮组（overflow）中设置。支持按操作系统设置不同的链接，参数配置详情参见 链接元素（url）。
    ///
    /// 说明：url 和 multi_url 字段必须且仅能填写其中一个。
    pub multi_url: Option<UrlComponent>,
}

/// 折叠按钮组（overflow）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OverflowComponent {
    /// 折叠按钮组的标签。固定取值：overflow
    pub tag: String,
    /// 折叠按钮组当中的选项按钮。按钮基于 option 元素进行配置，详情参见 option 元素。
    pub options: Vec<OptionComponent>,
    /// 该字段用于交互组件的回传交互方式,当用户点击交互组件的选项后，会将 value 的值返回给接收回调数据的服务器。后续你可以通过服务器接收的 value 值进行业务处理。
    ///
    /// 该字段值仅支持 key-value 形式的 JSON 结构，且 key 为 String 类型。示例值:
    ///
    /// "value":{
    ///     "key-1":Object-1,
    ///     "key-2":Object-2,
    ///     "key-3":Object-3,
    ///     ······
    /// }
    pub value: Option<Value>,
    /// 设置二次确认弹框。confirm 元素的配置方式可参见 confirm。
    pub confirm: Option<ConfirmComponent>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DatePickerComponent {
    /// 日期选择器的标签。在三种模式下的固定取值：
    ///
    /// 日期选择器：date_picker
    /// 时间选择器：picker_time
    /// 日期时间选择器：picker_datetime
    pub tag: String,
    /// 设置日期模式的初始值。格式：yyyy-MM-dd
    pub initial_date: Option<String>,
    /// 设置时间模式的初始值。格式：HH:mm
    pub initial_time: Option<String>,
    /// 设置日期时间模式的初始值。格式：yyyy-MM-dd HH:mm
    pub initial_datetime: Option<String>,
    /// 选择器的提示文案，无初始值时必填。只能设置文本组件中的 "tag": "plain_text" 模式以及 content 参数。
    ///
    /// 示例值:
    ///
    /// "placeholder": {
    ///     "tag": "plain_text",
    ///     "content": "请选择日期"
    /// },
    pub placeholder: Option<TextComponent>,
    /// 该字段用于交互组件的回传交互方式，当用户点击交互组件的选项后，会将 value 的值返回给接收回调数据的服务器。后续你可以通过服务器接收的 value 值进行业务处理。
    ///
    /// 该字段值仅支持 key-value 形式的 JSON 结构，且 key 为 String 类型。示例值：
    ///
    /// "value":{
    ///     "key-1":Object-1,
    ///     "key-2":Object-2,
    ///     "key-3":Object-3,
    ///     ······
    /// }
    pub value: Option<Value>,
    /// 设置二次确认弹框。confirm 元素的配置方式可参见 confirm。
    pub confirm: Option<ConfirmComponent>,
}

/// 交互模块（action）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionComponent {
    /// 交互模块的标识。固定取值：action
    pub tag: String,
    /// 添加可交互的组件。支持添加的组件如下，你可以跳转至对应的组件文档查看参数配置详情。
    ///
    /// 按钮（button）
    /// 列表选择器（selectMenu）
    /// 折叠按钮组（overflow）
    /// 日期选择器（datePicker）
    pub actions: Vec<Actions>,
    /// 设置窄屏自适应布局方式。取值:
    ///
    /// bisected：二等分布局，每行两列交互元素。
    /// trisection：三等分布局，每行三列交互元素。
    /// flow：流式布局，元素会按自身大小横向排列并在空间不够的时候折行。
    pub layout: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Actions {
    Button(ButtonComponent),
    SelectMenu(SelectMenuComponent),
    OverFlow(OverflowComponent),
    Datepicker(DatePickerComponent),
}

impl Default for Actions {
    fn default() -> Self {
        Self::Button(ButtonComponent::default())
    }
}

/// 多列布局
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ColumnSetComponent {
    /// 多列布局容器的标识，固定取值：column_set。
    pub tag: String,
    /// 移动端和 PC 端的窄屏幕下，各列的自适应方式。取值：
    ///
    /// none：不做布局上的自适应，在窄屏幕下按比例压缩列宽度。
    /// stretch：列布局变为行布局，且每列（行）宽度强制拉伸为 100%，所有列自适应为上下堆叠排布。
    /// flow：列流式排布（自动换行），当一行展示不下一列时，自动换至下一行展示。
    /// bisect：两列等分布局。
    /// trisect：三列等分布局。
    /// 默认值：none。
    pub flex_mode: String,
    /// 多列布局的背景色样式。取值：
    ///
    /// default：默认的白底样式，dark mode 下为黑底。
    /// grey：灰底样式。
    /// 当存在多列布局的嵌套时，上层多列布局的颜色覆盖下层多列布局的颜色。
    pub background_style: Option<String>,
    ///  多列布局内，各列之间的水平分栏间距。取值：
    ///
    /// default：默认间距。
    /// small：窄间距。
    pub horizontal_spacing: Option<String>,
    /// 多列布局容器内，各个列容器的配置信息。详情参见下文 column 参数说明。
    pub columns: Option<Vec<ColumnComponent>>,
    /// 设置点击布局容器时的交互配置。当前仅支持跳转交互。如果布局容器内有交互组件，则优先响应交互组件定义的交互。
    ///
    /// 示例配置如下，其中支持内嵌 url 元素，该元素说明参见 url。
    ///
    /// {
    /// "action": {
    ///  "multi_url": {
    ///   "url": "https://open.feishu.cn",
    ///   "pc_url": "https://open.feishu.com",
    ///   "ios_url": "https://developer.apple.com/",
    ///   "android_url": "https://developer.android.com/"
    ///   }
    ///  }
    /// }
    pub action: Option<ActionComponent>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ColumnComponent {
    /// 列容器标识，固定取值：column。
    pub tag: String,
    /// 列宽度属性。取值：
    ///
    /// auto：列宽度与列内元素宽度一致。
    /// weighted：列宽度按 weight 参数定义的权重分布。
    pub width: Option<String>,
    /// 当 width 取值 weighted 时生效，表示当前列的宽度占比。取值范围：1 ~ 5
    pub weight: Option<u32>,
    /// 列内成员垂直对齐方式。取值：
    ///
    /// top：顶对齐。
    /// center：居中对齐。
    /// bottom：底部对齐。
    pub vertical_align: Option<String>,
    /// 需要在列内展示的卡片元素。
    pub elements: Box<ColumnElements>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ColumnElements {
    Elements(Box<Elements>),
    ColumnSet(ColumnSetComponent),
}

impl Default for ColumnElements {
    fn default() -> Self {
        Self::Elements(Box::default())
    }
}

/// 分割线
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DividerLineComponent {
    /// 分割线模块标识，固定取值：hr。
    pub tag: String,
}

/// 备注模块
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotesComponent {
    pub tag: String,
    pub elements: Vec<NotesElements>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NotesElements {
    Text(TextComponent),
    Image(ImageComponent),
}

/// 在消息卡片结构的 config 字段中，支持配置卡片是否可以转发、是否为共享卡片。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardConfig {
    /// 是否允许转发卡片。取值：
    ///
    /// true：允许
    /// false：不允许
    /// 默认值为 true，该字段要求飞书客户端的版本为 V3.31.0 及以上。
    ///
    /// 注意：转发后卡片上的回传交互组件将自动置为禁用态，用户不能在转发后的卡片内进行数据交互。
    pub enable_forward: Option<bool>,
    /// 是否为共享卡片。取值：
    ///
    /// true：是共享卡片，更新卡片的内容对所有收到这张卡片的人员可见。
    /// false：非共享卡片，即独享卡片，仅操作用户可见卡片的更新内容。
    /// 默认值为 false。
    pub update_multi: Option<bool>,
}

#[cfg(test)]
mod test {
    use crate::message::CustomRichTextMessage;

    #[test]
    fn test_serialize() {
        let rich_text = CustomRichTextMessage::new("a", vec![]);

        println!("{:?}", serde_json::to_string(&rich_text))
    }
}
