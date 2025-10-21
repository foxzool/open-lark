use serde::{Deserialize, Serialize};

use crate::card::{
    components::{
        content_components::plain_text::PlainText, interactive_components::input::InputConfirm,
        CardElement,
    },
    interactions::Behaviors,
};

/// 交互容器
#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveContainer {
    /// 交互容器的标签。固定值为 interactive_container。
    tag: String,
    /// 交互容器的宽度。可取值：
    ///
    /// - fill：卡片最大支持宽度
    /// - auto：自适应宽度
    /// - [16,999]px：自定义宽度，如 "20px"。最小宽度为 16px
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 交互容器的高度。可取值：
    ///
    /// - auto：自适应高度
    /// - [10,999]px：自定义高度，如 "20px"
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<String>,
    /// 交互容器的背景色样式。可取值：
    ///
    /// - default：默认的白底样式，客户端深色主题下为黑底
    /// - laser：镭射渐变彩色样式
    /// - 卡片支持的颜色枚举值和 RGBA 语法自定义颜色。参考颜色枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    background_style: Option<String>,
    /// 是否展示边框，粗细固定为 1px。
    #[serde(skip_serializing_if = "Option::is_none")]
    has_border: Option<bool>,
    /// 边框的颜色，仅 has_border 为 true 时，此字段生效。枚举值为卡片支持的颜色枚举值和 RGBA
    /// 语法自定义颜色，参考颜色枚举值。
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<String>,
    /// 交互容器的圆角半径，单位是像素（px）或百分比（%）。取值遵循以下格式：
    ///
    /// - [0,∞]px，如 "10px"
    /// - [0,100]%，如 "30%"
    #[serde(skip_serializing_if = "Option::is_none")]
    corner_radius: Option<String>,
    /// 交互容器的内边距。值的取值范围为 [0,28]px。支持填写单值或多值：
    ///
    /// - 单值：如 "10px"，表示容器内四个内边距都为 10px
    /// - 多值：如 "4px 12px 4px 12px"，表示容器内上、右、下、左的内边距分别为
    ///   4px，12px，4px，12px。四个值必填，使用空格间隔
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<String>,
    /// 设置点击交互容器时的交互配置。如果交互容器内有交互组件，则优先响应交互组件定义的交互。
    /// 详情参考配置卡片交互
    behaviors: Vec<Behaviors>,
    /// 用户在 PC 端将光标悬浮在交互容器上方时的文案提醒。默认为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_tips: Option<PlainText>,
    /// 是否禁用交互容器。可选值：
    ///
    /// - true：禁用整个容器
    /// - false：容器组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 禁用交互容器后，用户触发交互时的弹窗文案提醒。默认为空，即不弹窗
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_tips: Option<PlainText>,
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，
    /// 才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
    ///
    /// 注意：confirm 字段仅在用户点击包含提交属性的按钮时才会触发二次确认弹窗。
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<InputConfirm>,
    /// 交互容器内嵌的组件。仅支持内嵌普通文本、富文本、图片、备注、分栏、勾选器、交互容器组件
    elements: Vec<CardElement>,
}

impl Default for InteractiveContainer {
    fn default() -> Self {
        InteractiveContainer {
            tag: "interactive_container".to_string(),
            width: None,
            height: None,
            background_style: None,
            has_border: None,
            border_color: None,
            corner_radius: None,
            padding: None,
            behaviors: vec![],
            hover_tips: None,
            disabled: None,
            disabled_tips: None,
            confirm: None,
            elements: vec![],
        }
    }
}

impl InteractiveContainer {
    pub fn new() -> Self {
        InteractiveContainer::default()
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn height(mut self, height: &str) -> Self {
        self.height = Some(height.to_string());
        self
    }

    pub fn background_style(mut self, background_style: &str) -> Self {
        self.background_style = Some(background_style.to_string());
        self
    }

    pub fn has_border(mut self, has_border: bool) -> Self {
        self.has_border = Some(has_border);
        self
    }

    pub fn border_color(mut self, border_color: &str) -> Self {
        self.border_color = Some(border_color.to_string());
        self
    }

    pub fn corner_radius(mut self, corner_radius: &str) -> Self {
        self.corner_radius = Some(corner_radius.to_string());
        self
    }

    pub fn padding(mut self, padding: &str) -> Self {
        self.padding = Some(padding.to_string());
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

    pub fn confirm(mut self, confirm: InputConfirm) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn elements(mut self, elements: Vec<CardElement>) -> Self {
        self.elements = elements;
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::{
        components::{
            containers::interactive::InteractiveContainer,
            content_components::plain_text::PlainText,
        },
        interactions::{Behaviors, CallbackBehavior, FormBehavior, OpenUrlBehavior},
    };

    #[test]
    fn test_interactive() {
        let interactive = InteractiveContainer::new()
            .width("fill")
            .height("auto")
            .background_style("default")
            .has_border(false)
            .border_color("grey")
            .corner_radius("40px")
            .padding("10px 20px 10px 20px")
            .behaviors(vec![
                Behaviors::OpenUrl(
                    OpenUrlBehavior::new("https://www.baidu.com")
                        .android_url("https://developer.android.com/")
                        .ios_url("lark://msgcard/unsupported_action")
                        .pc_url("https://www.windows.com"),
                ),
                Behaviors::Callback(CallbackBehavior::new(json!({
                    "key": "value"
                }))),
                Behaviors::Form(FormBehavior::new().behavior("submit")),
            ])
            .hover_tips(PlainText::text("demo"))
            .disabled(false)
            .disabled_tips(PlainText::text("demo"))
            .elements(vec![]);

        let expect = json!({
          "tag": "interactive_container", // 交互容器的标签。
          "width": "fill", // 交互容器的宽度。默认值 fill。
          "height": "auto", // 交互容器的高度。默认值 auto。
          "background_style": "default", // 背景色。默认值 default（无背景色）。
          "has_border": false, // 是否展示边框，粗细固定为 1px。默认值 false。
          "border_color": "grey", // 交互容器的边框颜色，仅 has_border 为 true 时生效。
          "corner_radius": "40px", // 交互容器的圆角半径。可选。
          "padding": "10px 20px 10px 20px", // 交互容器的内边距。默认值 "4px 12px 4px 12px"。
          "behaviors": [
            {
              "type": "open_url", // 声明交互类型是打开链接的跳转交互。
              "default_url": "https://www.baidu.com", // 兜底跳转地址。
              "android_url": "https://developer.android.com/", // 安卓端跳转地址。
              "ios_url": "lark://msgcard/unsupported_action", // iOS 端跳转地址。
              "pc_url": "https://www.windows.com" // 桌面端跳转地址。
            },
            {
              "type": "callback", // 声明交互类型是回传数据到服务端的回传交互。
              "value": {
                // 回传交互数据
                "key": "value"
              }
            },
            {
              "type": "form_action", // 声明交互类型为表单事件。
              "behavior": "submit" // 声明表单事件类型。默认为 submit。
            }
          ],
          "disabled": false,
          "disabled_tips": { "tag": "plain_text", "content": "demo" },
          "hover_tips": {
            "tag": "plain_text",
            "content": "demo"
          },
          "elements": [] // 容器子组件，仅支持内嵌普通文本、富文本、图片、备注、分栏、勾选器、交互容器组件。
        });

        assert_eq!(json!(interactive), expect);
    }
}
