use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::card::components::{
    content_components::plain_text::PlainText, interactive_components::input::InputConfirm,
};

/// 时间选择器
#[derive(Debug, Serialize, Deserialize)]
pub struct PickerTime {
    /// 组件的标签。时间选择器组件取固定值 picker_time。。
    tag: String,
    /// 该时间选择器组件的唯一标识。用于识别用户提交的数据属于哪个组件。
    ///
    /// 注意: 当时间选择器组件嵌套在表单容器中时，该字段必填且需在卡片全局内唯一。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 时间否必选。当组件内嵌在表单容器中时，该属性可用。其它情况将报错或不生效。可取值：
    ///
    /// - true：时间必选。当用户点击表单容器的“提交”时，未填写时间，则前端提示“有必填项未填写”，
    ///   不会向开发者的服务端发起回传请求。
    /// - false：时间选填。当用户点击表单容器的“提交”时，未填写时间，仍提交表单容器中的数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// 是否禁用该时间选择器。该属性仅支持飞书 V7.4 及以上版本的客户端。可选值：
    ///
    /// true：禁用时间选择器组件
    /// false：时间选择器组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 时间选择器组件的初始选项值。格式为 HH:mm。该配置将会覆盖 placeholder 配置的占位文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_time: Option<String>,
    /// 时间选择器组件内的占位文本。
    ///
    /// 注意：
    ///
    /// 未配置 initial_time 字段设置初始选项值时，该字段必填。
    /// 配置 initial_time 字段设置初始选项值后，该字段不再生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<PlainText>,
    /// 时间选择器组件的宽度。支持以下枚举值：
    ///
    /// - default：默认宽度
    /// - fill：卡片最大支持宽度
    /// - [100,∞)px：自定义宽度。超出卡片宽度时将按最大支持宽度展示
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 设置交互的回传数据，当用户点击交互组件的选项后，会将 value
    /// 的值返回给接收回调数据的服务器。后续你可以通过服务器接收的 value 值进行业务处理。
    ///
    /// 该字段值仅支持 key-value 形式的 JSON 结构，且 key 为 String 类型
    value: Value,
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，
    /// 才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
    ///
    /// 注意：confirm 字段仅在用户点击包含提交属性的按钮时才会触发二次确认弹窗。
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<InputConfirm>,
}

impl Default for PickerTime {
    fn default() -> Self {
        Self {
            tag: "picker_time".to_string(),
            name: None,
            required: None,
            disabled: None,
            initial_time: None,
            placeholder: None,
            width: None,
            value: Value::Null,
            confirm: None,
        }
    }
}

impl PickerTime {
    pub fn new() -> Self {
        Self::default()
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

    pub fn initial_time(mut self, initial_time: &str) -> Self {
        self.initial_time = Some(initial_time.to_string());
        self
    }

    pub fn placeholder(mut self, placeholder: PlainText) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn value(mut self, value: Value) -> Self {
        self.value = value;
        self
    }

    pub fn confirm(mut self, confirm: InputConfirm) -> Self {
        self.confirm = Some(confirm);
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[test]
    fn test_time_selector() {
        let picker_time = super::PickerTime::new()
            .name("picker_time1")
            .required(false)
            .disabled(false)
            .initial_time("11:30")
            .placeholder(super::PlainText::text("请选择"))
            .width("default")
            .value(json!({ "key_1": "value_1"}))
            .confirm(super::InputConfirm::new("title", "content"));

        let json = json!({
          "tag": "picker_time",  // 时间选择器组件的标签。
          "name": "picker_time1", // 时间选择器组件的唯一标识。当组件内嵌在表单容器中时，该字段生效且必填，用于识别用户提交的数据属于哪个组件。
          "required": false, // 时间是否必选。默认值 false。当时间选择器内嵌在表单容器时，该属性可用。其它情况将报错或不生效。
          "disabled": false, // 是否禁用该时间选择器组件。默认值 false。
          "width": "default",  // 时间选择器的宽度。
          "initial_time": "11:30", // 时间初始值。
          "placeholder": {
            // 时间选择器组件内的占位文本。
            "tag": "plain_text",
            "content": "请选择"
          },
          "value": {
            // 回传数据
            "key_1": "value_1"
          },
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
          }
        });

        assert_eq!(json!(picker_time), json);
    }
}
