use serde::{Deserialize, Serialize};

use crate::card::{
    components::{
        content_components::plain_text::PlainText, interactive_components::input::InputConfirm,
    },
    icon::FeishuCardTextIcon,
    interactions::Behaviors,
};

/// 勾选器
#[derive(Debug, Serialize, Deserialize)]
pub struct Checker {
    /// 组件的标签。勾选器组件的固定值为 checker。
    tag: String,
    /// 勾选器组件的唯一标识。用于识别用户提交的数据属于哪个组件。
    ///
    /// 注意：当勾选器组件嵌套在表单容器中时，该字段必填且需在卡片全局内唯一。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 勾选器的初始勾选状态。可选值：
    ///
    /// - true：已勾选状态
    /// - false：未勾选状态
    #[serde(skip_serializing_if = "Option::is_none")]
    checked: Option<bool>,
    /// 勾选器组件内的普通文本信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PlainText>,
    /// 当光标悬浮在勾选器上时，勾选器整体是否有阴影效果。
    ///
    /// 注意：要取消阴影效果，你需确保 overall_checkable 为 false 且 pc_display_rule 不为
    /// on_hover。
    #[serde(skip_serializing_if = "Option::is_none")]
    overall_checkable: Option<bool>,
    /// 按钮区配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    button_area: Option<ButtonArea>,
    /// 勾选状态样式。
    #[serde(skip_serializing_if = "Option::is_none")]
    checked_style: Option<CheckedStyle>,
    /// 组件整体的外边距，支持填写单值或多值：
    ///
    /// - 单值：如 "4px"，表示组件的四个外边距都为 4px
    /// - 多值：如 "4px 12px 4px 12px"，表示容器内上、右、下、左的内边距分别为
    ///   4px，12px，4px，12px。四个值必填，使用空格间隔
    #[serde(skip_serializing_if = "Option::is_none")]
    margin: Option<String>,
    /// 组件整体的内边距，支持填写单值或多值：
    ///
    /// - 单值：如 "4px"，表示组件内四个内边距都为 4px
    /// - 多值：如 "4px 12px 4px 12px"，表示容器内上、右、下、左的内边距分别为
    ///   4px，12px，4px，12px。四个值必填，使用空格间隔
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<String>,
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，
    /// 才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
    ///
    /// 注意：confirm 字段仅在用户点击包含提交属性的按钮时才会触发二次确认弹窗。
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<InputConfirm>,
    /// 配置交互类型和具体交互行为。未配置 behaviors 时，终端用户可勾选，但仅本地有效。
    behaviors: Vec<Behaviors>,
    /// 用户在 PC 端将光标悬浮在勾选器上方时的文案提醒。
    ///
    /// 注意：当同时配置 hover_tips 和 disabled_tips 时，disabled_tips 将生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_tips: Option<PlainText>,
    /// 是否禁用该勾选器。可选值：
    ///
    /// - true：禁用
    /// - false：勾选器组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 禁用勾选器后，用户在 PC 端将光标悬浮在勾选器上方时的文案提醒。s
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_tips: Option<PlainText>,
}

impl Default for Checker {
    fn default() -> Self {
        Self {
            tag: "checker".to_string(),
            name: None,
            checked: None,
            text: None,
            overall_checkable: None,
            button_area: None,
            checked_style: None,
            margin: None,
            padding: None,
            confirm: None,
            behaviors: vec![],
            hover_tips: None,
            disabled: None,
            disabled_tips: None,
        }
    }
}

impl Checker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn text(mut self, text: PlainText) -> Self {
        self.text = Some(text);
        self
    }

    pub fn overall_checkable(mut self, overall_checkable: bool) -> Self {
        self.overall_checkable = Some(overall_checkable);
        self
    }

    pub fn button_area(mut self, button_area: ButtonArea) -> Self {
        self.button_area = Some(button_area);
        self
    }

    pub fn checked_style(mut self, checked_style: CheckedStyle) -> Self {
        self.checked_style = Some(checked_style);
        self
    }

    pub fn margin(mut self, margin: &str) -> Self {
        self.margin = Some(margin.to_string());
        self
    }

    pub fn padding(mut self, padding: &str) -> Self {
        self.padding = Some(padding.to_string());
        self
    }

    pub fn confirm(mut self, confirm: InputConfirm) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn behaviors(mut self, behaviors: Vec<Behaviors>) -> Self {
        self.behaviors = behaviors;
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
}

/// 按钮区配置
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ButtonArea {
    #[serde(skip_serializing_if = "Option::is_none")]
    pc_display_rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<Buttons>>,
}

impl ButtonArea {
    pub fn new() -> Self {
        Self {
            pc_display_rule: None,
            buttons: None,
        }
    }

    pub fn pc_display_rule(mut self, pc_display_rule: &str) -> Self {
        self.pc_display_rule = Some(pc_display_rule.to_string());
        self
    }

    pub fn buttons(mut self, buttons: Vec<Buttons>) -> Self {
        self.buttons = Some(buttons);
        self
    }
}

/// 勾选器配置按钮
#[derive(Debug, Serialize, Deserialize)]
pub struct Buttons {
    /// 按钮的标签，取固定值 button
    tag: String,
    /// 按钮的类型，可选值：
    ///
    /// - text：黑色字体按钮，无边框
    /// - primary_text：蓝色字体按钮，无边框
    /// - danger_text：红色字体按钮，无边框
    r#type: String,
    /// 按钮的尺寸，可选值：
    ///
    /// - tiny：超小尺寸，PC 端为 24px；移动端为 28px
    /// - small：小尺寸，PC 端为 28 px；移动端为 28 px
    /// - medium：中尺寸，PC 端为 32 px；移动端为 36 px
    /// - large：大尺寸，PC 端为 40 px；移动端为 48 px
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    /// 按钮上的文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PlainText>,
    /// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<FeishuCardTextIcon>,
    /// 是否禁用按钮。可选值：
    ///
    /// - true：禁用按钮
    /// - false：按钮组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 配置交互类型和具体交互行为
    behaviors: Vec<Behaviors>,
}

impl Buttons {
    pub fn new(r#type: &str) -> Self {
        Self {
            tag: "button".to_string(),
            r#type: r#type.to_string(),
            size: None,
            text: None,
            icon: None,
            disabled: None,
            behaviors: vec![],
        }
    }

    pub fn r#type(mut self, r#type: &str) -> Self {
        self.r#type = r#type.to_string();
        self
    }

    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn behaviors(mut self, behaviors: Vec<Behaviors>) -> Self {
        self.behaviors = behaviors;
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CheckedStyle {
    /// 是否展示内容区的贯穿式删除线。
    show_strikethrough: Option<bool>,
    /// 内容区的不透明度。取值范围为 [0,1] 之间的数字，不限小数位数。
    opacity: Option<f32>,
}

impl CheckedStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_strikethrough(mut self, show_strikethrough: bool) -> Self {
        self.show_strikethrough = Some(show_strikethrough);
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = Some(opacity);
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::interactions::CallbackBehavior;

    use super::*;

    #[test]
    fn test_checker() {
        let checker = super::Checker::new()
            .name("check_1")
            .checked(false)
            .text(
                PlainText::text("")
                    .text_size("normal")
                    .text_color("default")
                    .text_align("left"),
            )
            .overall_checkable(true)
            .button_area(ButtonArea::new().pc_display_rule("always").buttons(
                vec![Buttons::new("text")
                    .size("small")
                    .text(PlainText::text("text按钮"))
                    .icon(
                        FeishuCardTextIcon::new()
                            .token("chat-forbidden_outlined")
                            .color("orange")
                            .img_key("img_v2_38811724"),
                    ).disabled(false)],
            ))
            .checked_style(CheckedStyle::new().show_strikethrough(true).opacity(1.0))
            .margin("0px")
            .padding("0px")
            .behaviors(vec![Behaviors::Callback(CallbackBehavior::new(
                json!({"key": "value"}),
            ))])
            .disabled(false);

        let json = json!({
          "tag": "checker",  // 组件的标签。勾选器组件的固定值为 checker。
          "name": "check_1",  // 勾选器组件的唯一标识。用于识别用户提交的数据属于哪个组件。
          "checked": false,  // 勾选器的初始勾选状态。默认值 false。
          "text": {  // 勾选器组件内的普通文本信息。
            "tag": "plain_text", // 文本类型的标签。
            "content": "", // 文本的内容。当 tag 为 lark_md 时，支持部分 Markdown 语法的文本内容。
            "text_size": "normal", // 文本大小。默认值 normal。
            "text_color": "default", // 文本颜色。仅在 tag 为 plain_text 时生效。默认值 default。
            "text_align": "left", // 文本对齐方式。默认值 left。
          },
          "overall_checkable": true,  // 当光标悬浮在勾选器上时，勾选器整体是否有阴影效果。默认值 true。
          "button_area": {  // 按钮区的配置。可选。
            "pc_display_rule": "always",   // PC 端勾选器内按钮的展示规则。默认值 always，即始终显示按钮。
            "buttons": [  // 在勾选器中添加并配置按钮。最多可配置三个按钮。
              {
                "tag": "button",  // 按钮的标签，取固定值 button。
                "type": "text",  // 按钮的类型。必填。
                "size": "small", // 按钮的尺寸。默认值 medium。
                "text": {   // 按钮上的文本。
                  "tag": "plain_text",
                  "content": "text按钮"
                },
                "icon": {   // 添加图标作为按钮文本上的前缀图标。支持自定义或使用图标库中的图标。
                  "tag": "standard_icon", // 图标类型。
                  "token": "chat-forbidden_outlined", // 图标的 token。仅在 tag 为 standard_icon 时生效。
                  "color": "orange", // 图标颜色。仅在 tag 为 standard_icon 时生效。
                  "img_key": "img_v2_38811724" // 图片的 key。仅在 tag 为 custom_icon 时生效。
                },
                "disabled": false,
                "behaviors": []
              }
            ]
          },
          "checked_style": {  // 勾选状态样式。
            "show_strikethrough": true,  // 是否展示内容区的贯穿式删除线。默认值 false。
            "opacity": 1.0  // 内容区的不透明度。默认值 1。
          },
          "margin": "0px",  // 组件整体的外边距，支持填写单值或多值。默认值为 0px。
          "padding": "0px",  // 组件整体的内边距，支持填写单值或多值。默认值为 0px。


          "behaviors": [  // 配置交互类型和具体交互行为。未配置 behaviors 时，终端用户可勾选，但仅本地有效。
            {
              "type": "callback", // 声明交互类型。仅支持 callback 请求回调交互。
              "value": {
                // 回传交互数据
                "key": "value"
              }
            }
          ],

          "disabled": false,  // 是否禁用该勾选器。默认值 false。

        });

        assert_eq!(json!(checker), json);
    }
}
