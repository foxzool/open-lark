use serde::{Deserialize, Serialize};

use crate::card::{
    components::{content_components::plain_text::PlainText, CardElement},
    icon::FeishuCardTextIcon,
};

/// 折叠面板
#[derive(Debug, Serialize, Deserialize)]
pub struct CollapsiblePanel {
    /// 组件的标签。折叠面板取固定值为 collapsible_panel。
    tag: String,
    /// 面板是否展开。可选值：
    ///
    /// - true：面板为展开状态
    /// - false：面板为折叠状态。默认为折叠状态
    #[serde(skip_serializing_if = "Option::is_none")]
    expanded: Option<bool>,
    /// 折叠面板的背景色，默认为透明
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    /// 折叠面板的标题设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<CollapsibleHeader>,
    /// 边框设置。默认不显示边框
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<CollapsibleBorder>,
    /// 面板内元素垂直边距设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_spacing: Option<String>,
    /// 内容区的内边距。值的取值范围为 [0,28]px。支持填写单值或多值：
    ///
    /// - 单值：如 "4px"，表示组件内四个内边距都为 4px
    /// - 多值：如 "4px 12px 4px 12px"，表示容器内上、右、下、左的内边距分别为
    ///   4px，12px，4px，12px。四个值必填，使用空格间隔
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<String>,
    /// 各个组件的 JSON 结构。暂不支持表单（form）组件。
    elements: Vec<CardElement>,
}

impl Default for CollapsiblePanel {
    fn default() -> Self {
        Self {
            tag: "collapsible_panel".to_string(),
            expanded: None,
            background_color: None,
            header: None,
            border: None,
            vertical_spacing: None,
            padding: None,
            elements: vec![],
        }
    }
}

impl CollapsiblePanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    pub fn background_color(mut self, background_color: &str) -> Self {
        self.background_color = Some(background_color.to_string());
        self
    }

    pub fn header(mut self, header: CollapsibleHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn border(mut self, color: &str, corner_radius: &str) -> Self {
        self.border = Some(CollapsibleBorder {
            color: Some(color.to_string()),
            corner_radius: Some(corner_radius.to_string()),
        });
        self
    }

    pub fn vertical_spacing(mut self, vertical_spacing: &str) -> Self {
        self.vertical_spacing = Some(vertical_spacing.to_string());
        self
    }

    pub fn padding(mut self, padding: &str) -> Self {
        self.padding = Some(padding.to_string());
        self
    }

    pub fn elements(mut self, elements: Vec<CardElement>) -> Self {
        self.elements = elements;
        self
    }
}

/// 折叠面板的标题设置。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CollapsibleHeader {
    /// 标题文本设置。
    title: PlainText,
    /// 折叠面板标题区域的背景颜色设置，默认为透明色。枚举值参见颜色枚举值。
    ///
    /// 注意：如果你未设置此字段，则折叠面板的标题区域的背景色由 background_color 字段决定。
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
    /// 标题区域的垂直居中方式。可取值：
    ///
    /// - top：标题区域垂直居中于面板区域的顶部
    /// - center：标题区域垂直居中于面板区域的中间
    /// - bottom：标题区域垂直居中于面板区域的底部
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align: Option<String>,
    /// 标题区域的内边距。值的取值范围为 [0,28]px。支持填写单值或多值：
    ///
    /// - 单值：如 "4px"，表示组件内四个内边距都为 4px
    /// - 多值：如 "4px 12px 4px 12px"，表示容器内上、右、下、左的内边距分别为
    ///   4px，12px，4px，12px。四个值必填，使用空格间隔
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<String>,
    /// 添加图标作为标题前缀或后缀图标
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<FeishuCardTextIcon>,
    /// 图标的位置。可选值：
    ///
    /// - left：图标在标题区域最左侧
    /// - right：图标在标题区域最右侧
    /// - follow_text：图标在文本右侧
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_position: Option<String>,
    /// 折叠面板展开时图标旋转的角度，正值为顺时针，负值为逆时针。可选值：
    ///
    /// -180：逆时针旋转 180 度
    /// -90：逆时针旋转 90 度
    /// 90：顺时针旋转 90 度
    /// 180：顺时针旋转 180 度
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_expanded_angle: Option<i32>,
}

impl CollapsibleHeader {
    pub fn new(title: &str) -> Self {
        Self {
            title: PlainText::text(title),
            background_color: None,
            vertical_align: None,
            padding: None,
            icon: None,
            icon_position: None,
            icon_expanded_angle: None,
        }
    }

    pub fn title(mut self, title: PlainText) -> Self {
        self.title = title;
        self
    }

    pub fn background_color(mut self, background_color: &str) -> Self {
        self.background_color = Some(background_color.to_string());
        self
    }

    pub fn vertical_align(mut self, vertical_align: &str) -> Self {
        self.vertical_align = Some(vertical_align.to_string());
        self
    }

    pub fn padding(mut self, padding: &str) -> Self {
        self.padding = Some(padding.to_string());
        self
    }

    pub fn icon(mut self, icon: FeishuCardTextIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_position(mut self, icon_position: &str) -> Self {
        self.icon_position = Some(icon_position.to_string());
        self
    }

    pub fn icon_expanded_angle(mut self, icon_expanded_angle: i32) -> Self {
        self.icon_expanded_angle = Some(icon_expanded_angle);
        self
    }
}

/// 边框设置。默认不显示边框
#[derive(Debug, Serialize, Deserialize, Default)]
struct CollapsibleBorder {
    /// 边框颜色设置
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    /// 圆角设置
    #[serde(skip_serializing_if = "Option::is_none")]
    corner_radius: Option<String>,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::{
        components::{
            containers::collapsible_panel::{CollapsibleHeader, CollapsiblePanel},
            content_components::{plain_text::PlainText, rich_text::FeishuCardMarkdown},
            CardElement,
        },
        icon::FeishuCardTextIcon,
    };

    #[test]
    fn test_collapsible() {
        let panel = CollapsiblePanel::new()
            .expanded(true)
            .background_color("grey")
            .header(
                CollapsibleHeader::default()
                    .title(PlainText::text("**面板标题文本**").tag("markdown"))
                    .vertical_align("center")
                    .padding("4px 0px 4px 8px")
                    .icon(
                        FeishuCardTextIcon::new()
                            .token("chat-forbidden_outlined")
                            .color("orange")
                            .img_key("img_v2_38811724")
                            .size("16px 16px"),
                    )
                    .icon_position("follow_text")
                    .icon_expanded_angle(-180),
            )
            .border("grey", "5px")
            .vertical_spacing("8px")
            .padding("8px 8px 8px 8px")
            .elements(vec![CardElement::Markdown(FeishuCardMarkdown::new(
                "很长的文本",
            ))]);

        let expect = json!({
          "tag": "collapsible_panel", // 折叠面板的标签。
          "expanded": true, // 面板是否展开。默认值 false。
          "background_color": "grey", // 折叠面板的背景色，默认为透明
          "header": {
            // 折叠面板的标题设置。
            "title": {
              // 标题文本设置。支持 plain_text 和 markdown。
              "tag": "markdown",
              "content": "**面板标题文本**"
            },
            "vertical_align": "center", // 标题区的垂直居中方式。
            "padding": "4px 0px 4px 8px", // 标题区的内边距。
            "icon": {
              // 标题前缀图标
              "tag": "standard_icon", // 图标类型.
              "token": "chat-forbidden_outlined", // 图标库中图标的 token。当 tag 为 standard_icon 时生效。
              "color": "orange", // 图标的颜色。当 tag 为 standard_icon 时生效。
              "img_key": "img_v2_38811724", // 自定义前缀图标的图片 key。当 tag 为 custom_icon 时生效。
              "size": "16px 16px" // 图标的尺寸。默认值为 10px 10px。
            },
            "icon_position": "follow_text", // 图标的位置。默认值为 right。
            "icon_expanded_angle": -180 // 折叠面板展开时图标旋转的角度，正值为顺时针，负值为逆时针。默认值为 180。
          },
          "border": {
            // 边框设置。默认不显示边框。
            "color": "grey", // 边框的颜色。
            "corner_radius": "5px" // 圆角设置。
          },
          "vertical_spacing": "8px", // 面板内元素垂直边距设置。默认值为 8px。
          "padding": "8px 8px 8px 8px", // 内容区的内边距。
          "elements": [
            // 此处可添加各个组件的 JSON 结构。暂不支持表单（form）组件。
            {
              "tag": "markdown",
              "content": "很长的文本"
            }
          ]
        });

        assert_eq!(serde_json::to_value(&panel).unwrap(), expect);
    }
}
