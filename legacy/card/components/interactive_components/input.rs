use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::card::components::content_components::plain_text::PlainText;

/// 输入框组件
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardInput {
    /// 输入框的标签。固定值为 input。
    tag: String,
    /// 输入框的唯一标识。当输入框内嵌在表单容器时，该属性生效，
    /// 用于识别用户提交的文本属于哪个输入框。
    ///
    /// 注意：当输入框组件嵌套在表单容器中时，该字段必填且需在卡片全局内唯一。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 输入框的内容是否必填。当输入框内嵌在表单容器时，该属性可用。其它情况将报错或不生效。
    /// 可取值：
    ///
    /// - true：输入框必填。当用户点击表单容器的“提交”时，未填写输入框，
    ///   则前端提示“有必填项未填写”，不会向开发者的服务端发起回传请求。
    /// - false：输入框选填。当用户点击表单容器的“提交”时，未填写输入框，仍提交表单容器中的数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// 是否禁用该输入框。该属性仅支持飞书 V7.4 及以上版本的客户端。可选值：
    ///
    /// true：禁用输入框组件
    /// false：输入框组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 输入框中的占位文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<PlainText>,
    /// 输入框中为用户预填写的内容。展示为用户在输入框中输入文本后待提交的样式。
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<String>,
    /// 输入框的宽度。支持以下枚举值：
    ///
    /// - default：默认宽度
    /// - fill：卡片最大支持宽度
    /// - [100,∞)px：自定义宽度。超出卡片宽度时将按最大支持宽度展示
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 输入框可容纳的最大文本长度，可取 1~1,000
    /// 范围内的整数。当用户输入的文本字符数超过最大文本长度，组件将报错提示。
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u32>,
    /// 文本标签，即对输入框的描述，用于提示用户要填写的内容。多用于表单容器中内嵌的输入框组件。
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PlainText>,
    /// 文本标签的位置。可取值：
    ///
    /// - top：文本标签位于输入框上方
    /// - left：文本标签位于输入框左边
    ///   注意：在移动端等窄屏幕场景下，文本标签将自适应固定展示在输入框上方。
    #[serde(skip_serializing_if = "Option::is_none")]
    label_position: Option<String>,
    /// 你可在交互事件中自定义回传数据，支持 string 或 object 数据类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Value>,
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，
    /// 才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
    ///
    /// 注意：confirm 字段仅在用户点击包含提交属性的按钮时才会触发二次确认弹窗。
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<InputConfirm>,
    /// 设置输入框组件的降级文案。由于输入框仅支持飞书 V6.8
    /// 及以上版本的客户端，你需选择在低于此版本的客户端上，该组件的降级展示方式：
    ///
    /// 不填写该字段，使用系统默认的降级文案：“请升级至最新版本客户端，以查看内容”
    /// "drop"：填写 "drop"，在旧版本客户端上直接丢弃该输入框组件
    /// 使用 text 文本对象自定义降级文案
    #[serde(skip_serializing_if = "Option::is_none")]
    fallback: Option<InputFallback>,
}

impl Default for FeishuCardInput {
    fn default() -> Self {
        FeishuCardInput {
            tag: "input".to_string(),
            name: None,
            required: None,
            disabled: None,
            placeholder: None,
            default_value: None,
            width: None,
            max_length: None,
            label: None,
            label_position: None,
            value: None,
            confirm: None,
            fallback: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputConfirm {
    title: PlainText,
    text: PlainText,
}

impl InputConfirm {
    pub fn new(title: &str, text: &str) -> Self {
        InputConfirm {
            title: PlainText::text(title),
            text: PlainText::text(text),
        }
    }

    pub fn title(mut self, title: PlainText) -> Self {
        self.title = title;
        self
    }

    pub fn text(mut self, text: PlainText) -> Self {
        self.text = text;
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputFallback {
    /// 降级文案的标签，固定取值为 fallback_text。
    tag: String,
    /// 降级文案的内容。
    text: PlainText,
}

impl Default for InputFallback {
    fn default() -> Self {
        InputFallback {
            tag: "fallback_text".to_string(),
            text: PlainText::default(),
        }
    }
}

impl InputFallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: PlainText) -> Self {
        self.text = text;
        self
    }
}

impl FeishuCardInput {
    pub fn new() -> Self {
        FeishuCardInput::default()
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn placeholder(mut self, placeholder: PlainText) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn default_value(mut self, default_value: &str) -> Self {
        self.default_value = Some(default_value.to_string());
        self
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn max_length(mut self, max_length: u32) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn label(mut self, label: PlainText) -> Self {
        self.label = Some(label);
        self
    }

    pub fn label_position(mut self, label_position: &str) -> Self {
        self.label_position = Some(label_position.to_string());
        self
    }

    pub fn value(mut self, value: Value) -> Self {
        self.value = Some(value);
        self
    }

    pub fn confirm(mut self, confirm: InputConfirm) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn fallback(mut self, fallback: InputFallback) -> Self {
        self.fallback = Some(fallback);
        self
    }

    pub fn build(self) -> FeishuCardInput {
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::card::components::content_components::plain_text::PlainText;

    use super::*;

    #[test]
    fn test_input_builder() {
        let input = FeishuCardInput::new()
            .name("input1")
            .required(false)
            .disabled(false)
            .placeholder(PlainText::text("请输入"))
            .default_value("demo")
            .width("default")
            .max_length(5)
            .label(PlainText::text("请输入文本："))
            .label_position("left")
            .value(json!({"k": "v"}))
            .confirm(InputConfirm::new("title", "content"))
            .fallback(InputFallback::new().text(PlainText::text("自定义声明")));

        let json = json!({
          "tag": "input", // 输入框的标签。
          "name": "input1", // 输入框的唯一标识。当输入框内嵌在表单容器时，该属性生效，用于识别用户提交的文本属于哪个输入框。
          "required": false, // 输入框的内容是否必填。当输入框内嵌在表单容器时，该属性可用。其它情况将报错或不生效。
          "disabled": false, // 是否禁用该输入框组件。默认值 false。
          "placeholder": {
            // 输入框中的占位文本。
            "tag": "plain_text",
            "content": "请输入"
          },
          "default_value": "demo", // 输入框中为用户预填写的内容。
          "width": "default", // 输入框的宽度。
          "max_length": 5, // 输入框可容纳的最大文本长度。默认值 1000。
          "label": {
            // 文本标签，即对输入框的描述，用于提示用户要填写的内容。
            "tag": "plain_text",
            "content": "请输入文本："
          },
          "label_position": "left", // 文本标签的位置。默认值 top。
          "value": {
            // 回传数据，支持 string 或 object 数据类型。
            "k": "v"
          },
          "confirm": {
            // 二次确认弹窗配置。
            "title": {
              "tag": "plain_text",
              "content": "title"
            },
            "text": {
              "tag": "plain_text",
              "content": "content"
            }
          },
          "fallback": {
            // 设置输入框组件的降级文案。
            "tag": "fallback_text", // 降级文案的标签。
            "text": {
              "content": "自定义声明", // 自定义降级文案的具体内容。
              "tag": "plain_text" // 降级文案内容的标签。
            }
          }
        });

        assert_eq!(serde_json::to_value(input).unwrap(), json);
    }
}
