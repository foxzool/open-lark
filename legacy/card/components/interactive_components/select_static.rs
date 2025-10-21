use serde::{Deserialize, Serialize};

use crate::card::{
    components::{
        content_components::plain_text::PlainText, interactive_components::input::InputConfirm,
    },
    icon::FeishuCardTextIcon,
};

/// 下拉选择-单选
#[derive(Debug, Serialize, Deserialize)]
pub struct SelectStatic {
    /// 组件的标签。下拉选择-单选组件取固定值 select_static。
    tag: String,
    /// 组件边框样式。可选值：
    ///
    /// default：带边框样式
    /// text：不带边框的纯文本样式
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    /// 单选组件的唯一标识。当单选组件内嵌在表单容器时，该属性生效，
    /// 用于识别用户提交的文本属于哪个单选组件。
    ///
    /// 注意：当单选组件嵌套在表单容器中时，该字段必填且需在卡片全局内唯一。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 单选组件的内容是否必选。当组件内嵌在表单容器中时，该属性可用。其它情况将报错或不生效。
    /// 可取值：
    ///
    /// - true：单选组件必选。当用户点击表单容器的“提交”时，未填写单选组件，
    ///   则前端提示“有必填项未填写”，不会向开发者的服务端发起回传请求。
    /// - false：单选组件选填。当用户点击表单容器的“提交”时，未填写单选组件，
    ///   仍提交表单容器中的数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// 是否禁用该单选组件。该属性仅支持飞书 V7.4 及以上版本的客户端。可选值：
    ///
    /// true：禁用单选组件组件
    /// false：单选组件组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 下拉选择组件的初始选项值。取值上限为选项的数量。该配置将会覆盖 placeholder 配置的占位文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_index: Option<u32>,
    /// 下拉选择组件内的占位文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<PlainText>,
    /// 单选组件的宽度。支持以下枚举值：
    ///
    /// - default：默认宽度
    /// - fill：卡片最大支持宽度
    /// - [100,∞)px：自定义宽度。超出卡片宽度时将按最大支持宽度展示
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 选项的配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<SelectStaticOption>>,
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，
    /// 才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
    ///
    /// 注意：confirm 字段仅在用户点击包含提交属性的按钮时才会触发二次确认弹窗。
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<InputConfirm>,
}

impl Default for SelectStatic {
    fn default() -> Self {
        Self {
            tag: "select_static".to_string(),
            r#type: None,
            name: None,
            required: None,
            disabled: None,
            initial_index: None,
            placeholder: None,
            width: None,
            options: None,
            confirm: None,
        }
    }
}

impl SelectStatic {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn r#type(mut self, r#type: &str) -> Self {
        self.r#type = Some(r#type.to_string());
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn initial_index(mut self, initial_index: u32) -> Self {
        self.initial_index = Some(initial_index);
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

    pub fn options(mut self, options: Vec<SelectStaticOption>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn confirm(mut self, confirm: InputConfirm) -> Self {
        self.confirm = Some(confirm);
        self
    }
}

/// 选项的配置。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SelectStaticOption {
    /// 选项的名称。
    text: PlainText,
    /// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<FeishuCardTextIcon>,
    /// 自定义选项回调值。当用户点击交互组件的选项后，会将 value
    /// 的值返回给接收回调数据的服务器。后续你可以通过服务器接收的 value 值进行业务处理。
    ///
    /// 注意：同一个选择组件内，各选项的 value 值不可重复，否则将无法识别用户点击的是哪个选项。
    value: String,
}

impl SelectStaticOption {
    pub fn new(text: &str, value: &str) -> Self {
        Self {
            text: PlainText::text(text),
            icon: None,
            value: value.to_string(),
        }
    }

    pub fn text(mut self, text: PlainText) -> Self {
        self.text = text;
        self
    }

    pub fn icon(mut self, icon: FeishuCardTextIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::{
        components::{
            content_components::plain_text::PlainText,
            interactive_components::{
                input::InputConfirm,
                select_static::{SelectStatic, SelectStaticOption},
            },
        },
        icon::FeishuCardTextIcon,
    };

    #[test]
    fn test_select_static() {
        let select_static = SelectStatic::new()
            .r#type("text")
            .name("select_static1")
            .required(false)
            .disabled(false)
            .initial_index(1)
            .placeholder(PlainText::text("默认提示文本"))
            .width("default")
            .options(vec![SelectStaticOption::new("我是交互组件", "selectDemo1")
                .icon(
                    FeishuCardTextIcon::new()
                        .token("chat-forbidden_outlined")
                        .color("orange")
                        .img_key("img_v2_38811724"),
                )])
            .confirm(InputConfirm::new("弹窗标题", "弹窗正文文案"));

        let json = json!({
          "tag": "select_static",  // 下拉选择-单选组件的标签。
          "type": "text", // 组件边框样式。默认值 default。
          "name": "select_static1", // 下拉选择-单选组件的唯一标识。当下拉选择-单选组件内嵌在表单容器时，该属性生效，用于识别用户提交的文本属于哪个下拉选择-单选组件。
          "required": false, // 下拉选择-单选组件的内容是否必填。默认值 false。当下拉选择-单选组件内嵌在表单容器时，该属性可用。其它情况将报错或不生效。
          "disabled": false, // 是否禁用该单选组件。默认值 false。
          "initial_index": 1, // 选项展示的初始值。默认为空。
          "placeholder": {
            // 下拉选择组件内的占位文本。
            "tag": "plain_text",
            "content": "默认提示文本"
          },
          "width": "default",  // 下拉选择组件的宽度。
          "options": [
            // 选项配置
            {
              "text": {
                // 选项名称
                "tag": "plain_text",
                "content": "我是交互组件"
              },
              "icon": {
                // 添加图标作为选项前缀图标。支持自定义或使用图标库中的图标。
                "tag": "standard_icon", // 图标类型。
                "token": "chat-forbidden_outlined", // 图标的 token。仅在 tag 为 standard_icon 时生效。
                "color": "orange", // 图标颜色。仅在 tag 为 standard_icon 时生效。
                "img_key": "img_v2_38811724" // 图片的 key。仅在 tag 为 custom_icon 时生效。
              },
              "value": "selectDemo1" // 选项回调值，支持 string 类型数据。
            }
          ],
          "confirm": {
            // 二次确认弹窗配置
            "title": {
              "tag": "plain_text",
              "content": "弹窗标题"
            },
            "text": {
              "tag": "plain_text",
              "content": "弹窗正文文案"
            }
          }
        });

        assert_eq!(serde_json::to_value(&select_static).unwrap(), json);
    }
}
