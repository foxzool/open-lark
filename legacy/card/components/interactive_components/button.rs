use serde::{Deserialize, Serialize};

use crate::card::{
    components::{
        content_components::plain_text::PlainText, interactive_components::input::InputConfirm,
    },
    icon::FeishuCardTextIcon,
    interactions::Behaviors,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardButton {
    /// 组件的标签。按钮组件的固定值为 button。
    tag: String,
    /// 按钮的类型。可选值：
    ///
    /// - default：黑色字体按钮，有边框
    /// - primary：蓝色字体按钮，有边框
    /// - danger：红色字体按钮，有边框
    /// - text：黑色字体按钮，无边框
    /// - primary_text：蓝色字体按钮，无边框
    /// - danger_text：红色字体按钮，无边框
    /// - primary_filled：蓝底白字按钮
    /// - danger_filled：红底白字按钮
    /// - laser：镭射按钮
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    /// 按钮的尺寸。可选值：
    ///
    /// - tiny：超小尺寸，PC 端为 24 px；移动端为 28 px
    /// - small：小尺寸，PC 端为 28 px；移动端为 28 px
    /// - medium：中尺寸，PC 端为 32 px；移动端为 36 px
    /// - large：大尺寸，PC 端为 40 px；移动端为 48 px
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    /// 按钮的宽度。支持以下枚举值：
    ///
    /// - default：默认宽度
    /// - fill：卡片最大支持宽度
    /// - [100,∞)px：自定义宽度，如 120px。超出卡片宽度时将按最大支持宽度展示
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 按钮上的文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PlainText>,
    /// 添加图标作为文本前缀图标
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<FeishuCardTextIcon>,
    /// 用户在 PC 端将光标悬浮在交互容器上方时的文案提醒。默认为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_tips: Option<PlainText>,
    /// 是否禁按钮。可选值：
    ///
    /// - true：禁用按钮
    /// - false：按钮组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 禁用按钮后，用户触发交互时的弹窗文案提醒。默认为空，即不弹窗
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_tips: Option<PlainText>,
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，
    /// 才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
    ///
    /// 注意：confirm 字段仅在用户点击包含提交属性的按钮时才会触发二次确认弹窗。
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<InputConfirm>,
    /// 配置交互类型和具体交互行为。支持同时生效跳转链接和回传交互。
    #[serde(skip_serializing_if = "Option::is_none")]
    behaviors: Option<Vec<Behaviors>>,
    /// 表单容器内组件的唯一标识。用于识别用户提交的数据属于哪个组件。
    ///
    /// 注意：该字段必填且需在卡片全局内唯一。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 组件的内容是否必填。当组件内嵌在表单容器中时，该属性生效。可取值：
    ///
    /// - true：必填。当用户点击表单容器的“提交”时，未填写该组件，则前端提示“有必填项未填写”，
    ///   不会向开发者的服务端发起回传请求。
    ///
    /// - false：选填。当用户点击表单容器的“提交”时，未填写该组件，仍提交表单容器中的数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// 内嵌在表单容器中的按钮的交互类型。枚举值包括：
    ///
    /// - link：当前按钮仅支持链接跳转
    /// - request：当前按钮仅支持回传交互
    /// - multi：当前按钮同时支持链接跳转和回传交互
    /// - form_submit：将当前按钮与提交事件绑定。用户点击后，将触发表单容器的提交事件，
    ///   异步提交所有已填写的表单项内容
    /// - form_reset：将当前按钮与取消提交事件绑定。用户点击后，将触发表单容器的取消提交事件，
    ///   重置所有表单组件的输入值为初始值
    #[serde(skip_serializing_if = "Option::is_none")]
    action_type: Option<String>,
}

impl Default for FeishuCardButton {
    fn default() -> Self {
        Self {
            tag: "button".to_string(),
            r#type: None,
            size: None,
            width: None,
            text: None,
            icon: None,
            hover_tips: None,
            disabled: None,
            disabled_tips: None,
            confirm: None,
            behaviors: None,
            name: None,
            required: None,
            action_type: None,
        }
    }
}

impl FeishuCardButton {
    pub fn new() -> Self {
        Self::default()
    }

    /// 按钮的类型。默认为 default。
    pub fn r#type(mut self, typ: &str) -> Self {
        self.r#type = Some(typ.to_string());
        self
    }

    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn text(mut self, text: PlainText) -> Self {
        self.text = Some(text);
        self
    }

    pub fn icon(mut self, icon: FeishuCardTextIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn hover_tips(mut self, hover_tips: PlainText) -> Self {
        self.hover_tips = Some(hover_tips);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn disabled_tips(mut self, disabled_tips: PlainText) -> Self {
        self.disabled_tips = Some(disabled_tips);
        self
    }

    pub fn confirm(mut self, confirm: InputConfirm) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn behaviors(mut self, behaviors: Vec<Behaviors>) -> Self {
        self.behaviors = Some(behaviors);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    pub fn action_type(mut self, action_type: &str) -> Self {
        self.action_type = Some(action_type.to_string());
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::{
        components::{
            content_components::plain_text::PlainText,
            interactive_components::{button::FeishuCardButton, input::InputConfirm},
        },
        icon::FeishuCardTextIcon,
        interactions::{Behaviors, CallbackBehavior, FormBehavior, OpenUrlBehavior},
    };

    #[test]
    fn test_button() {
        let button = FeishuCardButton::new()
            .r#type("primary")
            .size("small")
            .width("default")
            .text(PlainText::text("确定"))
            .icon(
                FeishuCardTextIcon::new()
                    .token("chat-forbidden_outlined")
                    .color("orange")
                    .img_key("img_v2_38811724"),
            )
            .disabled(false)
            .confirm(InputConfirm::new("title", "content"))
            .behaviors(vec![
                Behaviors::OpenUrl(
                    OpenUrlBehavior::new("https://www.baidu.com")
                        .android_url("https://developer.android.com/")
                        .ios_url("lark://msgcard/unsupported_action")
                        .pc_url("https://www.windows.com"),
                ),
                Behaviors::Callback(CallbackBehavior::new(json!({"key": "value"}))),
                Behaviors::Form(FormBehavior::new().behavior("submit")),
            ]);

        let json = json!({
          "tag": "button", // 组件的标签。按钮组件的固定值为 button。
          "type": "primary", // 按钮的类型。默认为 default。
          "size": "small", // 按钮的尺寸。默认值 medium。
          "width": "default", // 按钮的宽度。默认为 default。
          "text": {
            // 按钮上的文本。
            "tag": "plain_text",
            "content": "确定"
          },
          "icon": {
            // 前缀图标。
            "tag": "standard_icon", // 图标类型。
            "token": "chat-forbidden_outlined", // 图标的 token。仅在 tag 为 standard_icon 时生效。
            "color": "orange", // 图标颜色。仅在 tag 为 standard_icon 时生效。
            "img_key": "img_v2_38811724" // 图片的 key。仅在 tag 为 custom_icon 时生效。
          },

          "disabled": false, // 是否禁用该按钮。默认值 false。

          "confirm": {
            // 二次确认弹窗配置
            "title": {
              "tag": "plain_text",
              "content": "title"
            },
            "text": {
              "tag": "plain_text",
              "content": "content"
            }
          },
          "behaviors": [
            {
              "type": "open_url", // 声明交互类型是打开链接的跳转交互
              "default_url": "https://www.baidu.com", // 兜底跳转地址
              "android_url": "https://developer.android.com/", // 安卓端跳转地址
              "ios_url": "lark://msgcard/unsupported_action", // iOS 端跳转地址。
              "pc_url": "https://www.windows.com" // 桌面端跳转地址
            },
            {
              "type": "callback", // 声明交互类型是回传数据到服务端的回传交互。
              "value": {
                // 回传交互数据。支持 string 或 object 数据类型。
                "key": "value"
              }
            },
            {
              "type": "form_action", // 声明交互类型为表单事件。
              "behavior": "submit" // 声明表单事件类型。默认为 submit。
            }
          ],
        });

        assert_eq!(serde_json::to_value(&button).unwrap(), json);
    }
}
