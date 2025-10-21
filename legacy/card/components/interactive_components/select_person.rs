use serde::{Deserialize, Serialize};

use crate::card::components::{
    content_components::plain_text::PlainText, interactive_components::input::InputConfirm,
};

/// 人员选择-单选
#[derive(Debug, Serialize, Deserialize)]
pub struct SelectPerson {
    /// 组件的标签。人员选择-单选组件取固定值 select_person。
    tag: String,
    /// 组件边框样式。可选值：
    ///
    /// default：带边框样式
    /// text：不带边框的纯文本样式
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
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
    /// 选项值配置。按选项数组的顺序展示选项内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<SelectPersonOption>>,
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，
    /// 才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
    ///
    /// 注意：confirm 字段仅在用户点击包含提交属性的按钮时才会触发二次确认弹窗。
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<InputConfirm>,
}

impl Default for SelectPerson {
    fn default() -> Self {
        Self {
            tag: "select_person".to_string(),
            r#type: None,
            required: None,
            disabled: None,
            placeholder: None,
            width: None,
            options: None,
            confirm: None,
        }
    }
}

impl SelectPerson {
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

    pub fn confirm(mut self, confirm: InputConfirm) -> Self {
        self.confirm = Some(confirm);
        self
    }
}

/// 选项的配置。
#[derive(Debug, Serialize, Deserialize)]
pub struct SelectPersonOption {
    /// 选项配置，仅支持添加候选用户的 open_id。
    value: String,
}

impl SelectPersonOption {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_select_person() {
        let select_person = SelectPerson::new()
            .r#type("text")
            .required(true)
            .disabled(false)
            .placeholder(PlainText::text("默认提示文本"))
            .width("default")
            .options(vec![
                SelectPersonOption::new("ou_48d0958ee4b2ab3eaf0b5f6c968xxxxx"),
                SelectPersonOption::new("ou_f9d24af786a14340721288cda6axxxxx"),
            ])
            .confirm(InputConfirm::new("弹窗标题", "弹窗正文文案"));

        let json = json!({
          "tag": "select_person", // 组件的标签
          "type": "text", // 组件边框样式。默认值为 default。
          "required":true, // 选项是否必填。
          "disabled":false, // 选项是否禁用。
          "placeholder": {
            // 人员选择组件内的占位文本
            "tag": "plain_text",
            "content": "默认提示文本"
          },
          "width": "default",  // 下拉选择组件的宽度
          "options": [
            // 选项配置，仅支持添加候选用户的 open_id。
            {
              "value": "ou_48d0958ee4b2ab3eaf0b5f6c968xxxxx" // 候选用户的 open_id
            },
            {
              "value": "ou_f9d24af786a14340721288cda6axxxxx" // 候选用户的 open_id
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

        assert_eq!(serde_json::to_value(&select_person).unwrap(), json);
    }
}
