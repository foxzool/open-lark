use serde::{Deserialize, Serialize};

use crate::card::components::{
    content_components::plain_text::PlainText,
    interactive_components::select_person::SelectPersonOption,
};

/// 人员选择-多选
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiSelectPerson {
    /// 组件的标签。人员选择-多选组件取固定值 multi_select_person。
    tag: String,
    /// 组件边框样式。可选值：
    ///
    /// default：带边框样式
    /// text：不带边框的纯文本样式
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    /// 表单容器中组件的唯一标识。用于识别用户提交的数据属于哪个组件。在同一张卡片内，
    /// 该字段的值全局唯一。
    name: String,
    /// 多选组件的内容是否必选。当组件内嵌在表单容器中时，该属性可用。其它情况将报错或不生效。
    /// 可取值：
    ///
    /// - true：多选组件必选。当用户点击表单容器的“提交”时，未填写多选组件，
    ///   则前端提示“有必填项未填写”，不会向开发者的服务端发起回传请求。
    /// - false：多选组件选填。当用户点击表单容器的“提交”时，未填写多选组件，
    ///   仍提交表单容器中的数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// 是否禁用该多选组件。该属性仅支持飞书 V7.4 及以上版本的客户端。可选值：
    ///
    /// true：禁用多选组件组件
    /// false：多选组件组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 人员选择组件内的占位文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<PlainText>,
    /// 人员选择组件的宽度。支持以下枚举值：
    ///
    /// - default：默认宽度
    /// - fill：卡片最大支持宽度
    /// - [100,∞)px：自定义宽度。超出卡片宽度时将按最大支持宽度展示
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 多选组件默认选中的选项。数组项的值需要和 options.value 对应。
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_values: Option<Vec<String>>,
    /// 选项值配置。按选项数组的顺序展示选项内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<SelectPersonOption>>,
}

impl Default for MultiSelectPerson {
    fn default() -> Self {
        Self {
            tag: "multi_select_person".to_string(),
            r#type: None,
            name: "".to_string(),
            required: None,
            disabled: None,
            placeholder: None,
            width: None,
            selected_values: None,
            options: None,
        }
    }
}

impl MultiSelectPerson {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn r#type(mut self, r#type: &str) -> Self {
        self.r#type = Some(r#type.to_string());
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

    pub fn options(mut self, options: Vec<SelectPersonOption>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn selected_values(mut self, selected_values: Vec<&str>) -> Self {
        self.selected_values = Some(selected_values.iter().map(|s| s.to_string()).collect());
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::components::interactive_components::select_person::SelectPersonOption;

    use super::*;

    #[test]
    fn test_select_person() {
        let select_person = MultiSelectPerson::new()
            .r#type("text")
            .name("multi_select_users")
            .required(true)
            .disabled(false)
            .placeholder(PlainText::text("默认提示文本"))
            .width("default")
            .selected_values(vec!["ou_48d0958ee4b2ab3eaf0b5f6c968xxxxx"])
            .options(vec![
                SelectPersonOption::new("ou_48d0958ee4b2ab3eaf0b5f6c968xxxxx"),
                SelectPersonOption::new("ou_f9d24af786a14340721288cda6axxxxx"),
            ]);

        let json = json!({
          "tag": "multi_select_person", // 组件的标签。
          "type": "text", // 组件边框样式。默认值 default。
          "name":"multi_select_users", // 表单容器中组件的自定义标识，用于识别用户提交的是哪个组件的数据。
          "placeholder": {
            // 人员选择组件内的占位文本。
            "tag": "plain_text",
            "content": "默认提示文本"
          },
          "width": "default",  // 下拉选择组件的宽度。
          "required":true, // 选项是否必填。
          "disabled":false, // 选项是否禁用。
          "selected_values": ["ou_48d0958ee4b2ab3eaf0b5f6c968xxxxx"], // 组件默认选中的选项。数组项的值需要和 options.value 对应。
          "options": [
            // 选项配置，仅支持添加候选用户的 open_id。
            {
              "value": "ou_48d0958ee4b2ab3eaf0b5f6c968xxxxx"
            },
            {
              "value": "ou_f9d24af786a14340721288cda6axxxxx"
            }
          ]
        });

        assert_eq!(serde_json::to_value(&select_person).unwrap(), json);
    }
}
