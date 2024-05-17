use serde::{Deserialize, Serialize};

use crate::card::components::{
    content_components::plain_text::PlainText,
    interactive_components::{input::InputConfirm, select_static::SelectStaticOption},
};

/// 下拉选择-多选
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiSelectStatic {
    /// 组件的标签。下拉选择-多选组件取固定值 multi_select_static。
    tag: String,
    /// 组件边框样式。可选值：
    ///
    /// default：带边框样式
    /// text：不带边框的纯文本样式
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    /// 表单容器中组件的唯一标识。当多选组件内嵌在表单容器时，该属性生效，
    /// 用于识别用户提交的数据属于哪个组件。
    ///
    /// 注意：当多选组件嵌套在表单容器中时，该字段必填且需在卡片全局内唯一。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 用户未选择选项时，下拉选择组件内的占位文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<PlainText>,
    /// 下拉选择组件的宽度。支持以下枚举值：
    ///
    /// - default：默认宽度：
    ///     - 当组件带边框时（即 "type":"default"），默认宽度值固定为 282 px
    ///     - 当组件不带边框时（即 "type":"text"），组件宽度自适应选择器的内容宽度
    /// - fill：组件宽度将撑满父容器宽度
    /// - [100,∞)px：自定义固定数值宽度，如 200px。最小值为
    ///   100px。超出父容器宽度时，按撑满父容器宽度展示
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 多选组件的选项是否必选。当组件内嵌在表单容器中时，该属性可用。其它情况将报错或不生效。
    /// 可取值：
    ///
    /// - true：多选组件必选。当用户点击表单容器的“提交”时，未选择多选选项，
    ///   则前端提示“有必填项未填写”，不会向开发者的服务端发起回传请求。
    /// - false：多选组件可选。当用户点击表单容器的“提交”时，未选择多选选项，
    ///   仍提交表单容器中的数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// 是否禁用该多选组件。可选值：
    ///
    /// - true：禁用该多选组件，组件展示自定义的占位文本或选项初始值，且终端用户不可修改交互
    /// - false：多选组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 多选组件默认选中的选项。数组项的值需要和 options.value 对应。
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_values: Option<Vec<String>>,
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

impl Default for MultiSelectStatic {
    fn default() -> Self {
        Self {
            tag: "multi_select_static".to_string(),
            r#type: None,
            name: None,
            required: None,
            disabled: None,
            placeholder: None,
            width: None,
            options: None,
            confirm: None,
            selected_values: None,
        }
    }
}

impl MultiSelectStatic {
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

    pub fn placeholder(mut self, placeholder: PlainText) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn values(mut self, values: Vec<&str>) -> Self {
        self.selected_values = Some(values.iter().map(|v| v.to_string()).collect());
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

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::{
        components::{
            content_components::plain_text::PlainText,
            interactive_components::{
                multi_select_static::MultiSelectStatic, select_static::SelectStaticOption,
            },
        },
        icon::FeishuCardTextIcon,
    };

    #[test]
    fn test_select_static() {
        let select_static = MultiSelectStatic::new()
            .r#type("default")
            .name("multi_select_departments")
            .required(true)
            .disabled(false)
            .placeholder(PlainText::text("默认提示文本"))
            .width("default")
            .values(vec![])
            .options(vec![SelectStaticOption::new("我是交互组件", "selectDemo1")
                .icon(
                    FeishuCardTextIcon::new()
                        .token("chat-forbidden_outlined")
                        .color("orange")
                        .img_key("img_v2_38811724"),
                )
                .value("selectDemo1")]);

        let json = json!({
          "tag": "multi_select_static", // 组件标签。
          "type": "default", // 组件边框样式。默认值 default。
          "name":"multi_select_departments", // 表单容器中组件的自定义唯一标识，当多选组件内嵌在表单容器时，该属性生效，用于识别用户提交的是哪个组件的数据。
          "required": true, // 选项是否必填。当多选组件内嵌在表单容器时，该属性可用。其它情况将报错或不生效。
          "disabled": false, // 选项是否禁用。
          "placeholder": {
            // 下拉选择组件内的占位文本。
            "tag": "plain_text",
            "content": "默认提示文本"
          },
          "width": "default",  // 下拉选择组件的宽度。
          "selected_values": [], // 选项初始值。数组项的值需要和 options.value 对应。
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
        });

        assert_eq!(serde_json::to_value(&select_static).unwrap(), json);
    }
}
