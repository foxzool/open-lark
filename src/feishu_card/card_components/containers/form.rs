use serde::{Deserialize, Serialize};

use crate::feishu_card::card_components::Element;
use crate::feishu_card::card_components::interactive_components::input::InputConfirm;

/// 表单容器
#[derive(Debug, Serialize, Deserialize)]
pub struct FormContainer {
    /// 表单容器的标签。固定值为 form。
    tag: String,
    /// 表单容器的唯一标识。用于识别用户提交的数据属于哪个表单容器。在同一张卡片内，该字段的值全局唯一。
    name: String,
    /// 表单容器的子节点。可内嵌其它容器类组件和展示、交互组件，不支持内嵌表格、图表、和表单容器组件。
    elements: Vec<Element>,
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
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
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

    pub fn elements(mut self, elements: Vec<Element>) -> Self {
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

    use crate::feishu_card::card_components::containers::column_set::{Column, ColumnSetContainer};
    use crate::feishu_card::card_components::content_components::plain_text::PlainText;
    use crate::feishu_card::card_components::content_components::rich_text::FeishuCardMarkdown;
    use crate::feishu_card::card_components::Element;
    use crate::feishu_card::card_components::interactive_components::button::FeishuCardButton;
    use crate::feishu_card::card_components::interactive_components::input::FeishuCardInput;
    use crate::feishu_card::card_components::interactive_components::select_static::{
        SelectStatic, SelectStaticOption,
    };

    use super::*;

    #[test]
    fn test_form_container() {
        let form = FormContainer::new().name("Form_lvxmxsxf").elements(vec![
            Element::ColumnSet(
                ColumnSetContainer::new()
                    .flex_mode("stretch")
                    .background_style("default")
                    .columns(vec![
                        Column::new()
                            .width("weighted")
                            .weight(1)
                            .vertical_align("top")
                            .elements(vec![Element::Markdown(
                                FeishuCardMarkdown::new().content("请选择："),
                            )]),
                        Column::new()
                            .width("weighted")
                            .weight(1)
                            .vertical_align("top")
                            .elements(vec![Element::SelectStatic(
                                SelectStatic::new()
                                    .name("Select_pj6kw7cxyl")
                                    .placeholder(PlainText::new("这是一个选择菜单"))
                                    .options(vec![
                                        SelectStaticOption::new("选项1", "1"),
                                        SelectStaticOption::new("选项2", "2"),
                                        SelectStaticOption::new("选项3", "3"),
                                        SelectStaticOption::new("选项4", "4"),
                                    ]),
                            )]),
                    ]),
            ),
            Element::InputForm(
                FeishuCardInput::new()
                    .name("Input_fhaty9jktke")
                    .placeholder(PlainText::new("请输入"))
                    .max_length(5)
                    .label(PlainText::new("请输入文本："))
                    .label_position("left")
                    .value(json!({"k":"v"})),
            ),
            Element::Button(
                FeishuCardButton::new()
                    .action_type("form_submit")
                    .name("Button_e4d9u982x5k")
                    .r#type("primary")
                    .text(PlainText::new("提交").tag("lark_md"))
                    .confirm(InputConfirm::new("title", "确认提交吗")),
            ),
        ]);

        let expect = json!( {
            "tag": "form",
            "name": "Form_lvxmxsxf",
            "elements": [
                {
                    "tag": "column_set",
                    "flex_mode": "stretch",
                    "background_style": "default",
                    "columns": [
                        {
                            "tag": "column",
                            "width": "weighted",
                            "weight": 1,
                            "vertical_align": "top",
                            "elements": [
                                {
                                    "tag": "markdown",
                                    "content": "请选择："
                                }
                            ]
                        },
                        {
                            "tag": "column",
                            "width": "weighted",
                            "weight": 1,
                            "vertical_align": "top",
                            "elements": [
                                {
                                    "tag": "select_static",
                                    "name": "Select_pj6kw7cxyl",
                                    "placeholder": {
                                        "tag": "plain_text",
                                        "content": "这是一个选择菜单"
                                    },
                                    "options": [
                                        {
                                            "text": {
                                                "tag": "plain_text",
                                                "content": "选项1"
                                            },
                                            "value": "1"
                                        },
                                        {
                                            "text": {
                                                "tag": "plain_text",
                                                "content": "选项2"
                                            },
                                            "value": "2"
                                        },
                                        {
                                            "text": {
                                                "tag": "plain_text",
                                                "content": "选项3"
                                            },
                                            "value": "3"
                                        },
                                        {
                                            "text": {
                                                "tag": "plain_text",
                                                "content": "选项4"
                                            },
                                            "value": "4"
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                },
                {
                    "tag": "input",
                    "name": "Input_fhaty9jktke",
                    "placeholder": {
                        "tag": "plain_text",
                        "content": "请输入"
                    },
                    "max_length": 5,
                    "label": {
                        "tag": "plain_text",
                        "content": "请输入文本："
                    },
                    "label_position": "left",
                    "value": {
                        "k": "v"
                    }
                },
                {
                    "action_type": "form_submit",
                    "name": "Button_e4d9u982x5k",
                    "tag": "button",
                    "text": {
                        "content": "提交",
                        "tag": "lark_md"
                    },
                    "type": "primary",
                    "confirm": {
                        "title": {
                            "tag": "plain_text",
                            "content": "title"
                        },
                        "text": {
                            "tag": "plain_text",
                            "content": "确认提交吗"
                        }
                    }
                }
            ]
        });

        assert_eq!(json!(form), expect);
    }
}
