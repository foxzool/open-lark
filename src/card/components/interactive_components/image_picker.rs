use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::card::components::content_components::plain_text::PlainText;

/// 多图选择
#[derive(Debug, Serialize, Deserialize)]
pub struct ImagePicker {
    /// 组件的标签。多图选择的固定取值为 select_img。
    tag: String,
    /// 图片加载等状态时的组件风格样式。可取值：
    ///
    /// default：默认灰色样式
    /// laser：彩色渐变样式，建议 AI 场景使用
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
    /// 图片是否多选。可选值：
    ///
    /// - true：多选，仅支持异步提交。多图选择组件需内嵌在表单容器中，否则卡片 JSON 报错。
    /// - false：单选。
    ///     - 组件在表单容器内时，图片选项展示为带单选按钮（radio button）的异步提交样式。
    ///     - 组件不在表单容器内时，图片选项展示为不带单选按钮（radio button）的同步提交样式。
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_select: Option<bool>,
    /// 图片选项的布局方式。可选值：
    ///
    /// - stretch：每个选项的图片宽度撑满父容器宽度，高度按图片大小等比例缩放。
    /// - bisect：二等分排布，每个选项图片宽度占父容器的 1/2，高度按图片大小等比例缩放。
    /// - trisect：三等分排布，每个选项图片宽度占父容器的 1/3，高度按图片大小等比例缩放。
    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<String>,
    /// 自定义多图选择组件的名称作为唯一标识。用于识别用户提交的数据属于哪个组件。
    ///
    /// 注意：当多图选择组件嵌套在表单容器中时，该字段生效、必填，且需在卡片全局内唯一。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 多图选择的选项是否必选。当组件内嵌在表单容器中时，该属性可用。其它情况将报错或不生效。
    /// 可取值：
    ///
    /// - true：选项必填。当用户点击表单容器的“提交”时，未选择选项，则前端提示“有必填项未填写”，
    ///   不会向开发者的服务端发起回传请求。
    /// - false：选项选填。当用户点击表单容器的“提交”时，未选择选项，仍提交表单容器中的数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<bool>,
    /// 点击图片选项后是否弹窗放大图片。当多图选择组件嵌套在表单容器中时，该属性生效。
    ///
    /// - true：点击图片后，弹出图片查看器放大查看当前点击的图片。
    /// - false：点击图片后，响应卡片本身的交互事件，不弹出图片查看器。
    #[serde(skip_serializing_if = "Option::is_none")]
    can_preview: Option<bool>,
    /// 选项中图片的宽高比。图片按最短边撑满图片渲染容器，按照居中裁剪的方式自适应裁剪。可取值：
    ///
    /// - 1:1
    /// - 16:9
    /// - 4:3
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<String>,
    /// 是否禁用整个选择组件。可选值：
    ///
    /// - true：禁用整个选择组件
    /// - false：选择组件保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 禁用整个组件后，用户将光标悬浮在整个组件上时展示的禁用提示文案。
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_tips: Option<PlainText>,
    /// 你可在交互事件中自定义回传参数，支持回传字符串，或 "key":"value" 构成的对象结构体。
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Value>,
    /// 选项值配置。按选项数组的顺序展示选项内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<SelectImageOption>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SelectImageOption {
    /// 图片资源的 Key。你可以调用上传图片接口或在搭建工具中上传图片，获取图片的 key。
    img_key: String,
    /// 自定义每个图片选项的回传参数。在回传交互中指定的回传参数将透传至开发者的服务端。
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    /// 是否禁用某个图片选项。可选值：
    ///
    /// - true：禁用该选项
    /// - false：选项保持可用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    /// 禁用某个图片选项后，用户将光标悬浮在选项上或点击选项时展示的禁用提示文案。
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_tips: Option<PlainText>,
    /// 用户在 PC 端将光标悬浮在多图选择上方时的文案提醒。默认为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_tips: Option<PlainText>,
}

impl SelectImageOption {
    pub fn new(img_key: &str) -> Self {
        Self {
            img_key: img_key.to_string(),
            value: None,
            disabled: None,
            disabled_tips: None,
            hover_tips: None,
        }
    }

    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
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

    pub fn hover_tips(mut self, hover_tips: PlainText) -> Self {
        self.hover_tips = Some(hover_tips);
        self
    }
}

impl Default for ImagePicker {
    fn default() -> Self {
        Self {
            tag: "select_img".to_string(),
            style: None,
            multi_select: None,
            layout: None,
            name: None,
            required: None,
            can_preview: None,
            aspect_ratio: None,
            disabled: None,
            disabled_tips: None,
            value: None,
            options: None,
        }
    }
}

impl ImagePicker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style(mut self, style: &str) -> Self {
        self.style = Some(style.to_string());
        self
    }

    pub fn multi_select(mut self, multi_select: bool) -> Self {
        self.multi_select = Some(multi_select);
        self
    }

    pub fn layout(mut self, layout: &str) -> Self {
        self.layout = Some(layout.to_string());
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

    pub fn can_preview(mut self, can_preview: bool) -> Self {
        self.can_preview = Some(can_preview);
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: &str) -> Self {
        self.aspect_ratio = Some(aspect_ratio.to_string());
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

    pub fn value(mut self, value: Value) -> Self {
        self.value = Some(value);
        self
    }

    pub fn options(mut self, options: Vec<SelectImageOption>) -> Self {
        self.options = Some(options);
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_image_picker() {
        let image_picker = ImagePicker::new()
            .style("laser")
            .multi_select(false)
            .layout("bisect")
            .name("choice_123")
            .required(false)
            .can_preview(false)
            .aspect_ratio("16:9")
            .disabled(false)
            .disabled_tips(PlainText::text("用户禁用提示文案"))
            .value(json!({"key": "value"}))
            .options(vec![SelectImageOption::new("xxxxxxxxxxxxxx")
                .value("picture1")
                .disabled(false)
                .disabled_tips(PlainText::text("用户禁用提示文案"))
                .hover_tips(PlainText::text("第一张图"))]);

        let json = json!(  {
          "tag": "select_img", // 组件标签。
          "style": "laser", // 选填，不填为默认样式。声明为 laser 时为镭射样式。
          "multi_select": false, // 是否多选。
          "layout": "bisect", // 选项的布局模式。
          "name": "choice_123", // 自定义多图选择组件的名称作为唯一标识。当组件内嵌在表单容器中时，该字段生效且必填，用于识别用户提交的数据属于哪个组件。
          "required": false, // 多图选择的选项是否必选。当组件内嵌在表单容器中时，该属性可用。其它情况将报错或不生效。
          "can_preview": false, // 点击图片选项后是否弹窗放大图片。当多图选择组件嵌套在表单容器中时，该属性生效。
          "aspect_ratio": "16:9", // 选项中图片的宽高比。
          "disabled": false, // 是否禁用整个选择组件。
          "disabled_tips": { // 指禁用组件后，用户将光标悬浮在整个组件上时展示的禁用提示文案。
            "tag": "plain_text",
            "content": "用户禁用提示文案"
          },
          "value": { // 自定义回传参数，支持回传字符串，或 "key":"value" 构成的对象结构体。
            "key": "value"
          },
          // 选项数组。在此配置多图选择组件中每个图片选项的属性。
          "options": [
            {
              "img_key": "xxxxxxxxxxxxxx", // 图片资源的 Key。
              "value": "picture1", // 自定义每个图片选项的回传参数。
              "disabled": false, // 是否禁用当前图片选项。
              "disabled_tips": { // 禁用当前选项后，用户将光标悬浮在选项上或点击选项时展示的禁用提示文案。
                "tag": "plain_text",
                "content": "用户禁用提示文案"
              },
              "hover_tips": { // 用户在 PC 端将光标悬浮在选项上方时的文案提醒。
                "tag": "plain_text",
                "content": "第一张图"
              },
            }
          ]
        });

        assert_eq!(json!(image_picker), json);
    }
}
