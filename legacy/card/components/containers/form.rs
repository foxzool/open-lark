use serde::{Deserialize, Serialize};

use crate::card::components::{interactive_components::input::InputConfirm, CardElement};

/// 表单容器
#[derive(Debug, Serialize, Deserialize)]
pub struct FormContainer {
    /// 表单容器的标签。固定值为 form。
    tag: String,
    /// 表单容器的唯一标识。用于识别用户提交的数据属于哪个表单容器。在同一张卡片内，
    /// 该字段的值全局唯一。
    name: String,
    /// 表单容器的子节点。可内嵌其它容器类组件和展示、交互组件，不支持内嵌表格、图表、
    /// 和表单容器组件。
    elements: Vec<CardElement>,
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
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，
    /// 才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
    ///
    /// 注意：confirm 字段仅在用户点击包含提交属性的按钮时才会触发二次确认弹窗。
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<InputConfirm>,
}

impl Default for FormContainer {
    fn default() -> Self {
        FormContainer {
            tag: "form".to_string(),
            name: "".to_string(),
            elements: vec![],
            r#type: None,
            confirm: None,
        }
    }
}

impl FormContainer {
    pub fn new() -> Self {
        FormContainer::default()
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn elements(mut self, elements: Vec<CardElement>) -> Self {
        self.elements = elements;
        self
    }

    pub fn r#type(mut self, r#type: &str) -> Self {
        self.r#type = Some(r#type.to_string());
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

    use crate::card::components::{
        content_components::plain_text::PlainText,
        interactive_components::{button::FeishuCardButton, input::FeishuCardInput},
        CardElement,
    };

    use super::*;

    #[test]
    fn test_form_container() {
        let form = FormContainer::new().name("form_1").elements(vec![
            CardElement::InputForm(FeishuCardInput::new().name("reason").required(true)),
            CardElement::Button(
                FeishuCardButton::new()
                    .action_type("form_submit")
                    .name("submit")
                    .r#type("primary")
                    .text(PlainText::text("提交").tag("lark_md")),
            ),
        ]);

        let expect = json!( {
          "tag": "form",    // 表单容器的标签。
          "name": "form_1", // 该表单容器的唯一标识。用于识别用户在交互后，提交的是哪个表单容器的数据。
          "elements": [
            {
              "tag": "input",   // 为表单容器内添加一个输入框组件。
              "name": "reason", // 输入框组件的唯一标识。用于识别用户在交互后，提交的是哪个表单项的数据。在表单容器中所有的交互组件中，该字段必填，否则数据会发送失败。
              "required": true  // 是否必填。为 true 时点击按钮后会做必填校验。
            },
            {
              "tag": "button", // 表单容器内的按钮组件。
              "action_type": "form_submit", // 将当前按钮与提交事件绑定。用户点击后，将触发表单容器的提交事件，异步提交所有已填写的表单项内容
              "name": "submit", // 按钮组件的唯一标识，用于识别用户在交互后，点击的是哪个按钮。在表单容器中所有的交互组件中，该字段必填，否则数据会发送失败。
              "text": { // 按钮上的文本。
                "content": "提交",
                "tag": "lark_md"
              },
              "type": "primary", // 按钮的样式类型。
            }
          ]
        });

        assert_eq!(json!(form), expect);
    }
}
