use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::card::{
    components::{
        content_components::plain_text::PlainText, interactive_components::input::InputConfirm,
    },
    href::FeishuCardHrefVal,
};

/// 折叠按钮组
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardOverflow {
    /// 折叠按钮组的标签。固定值为 overflow。
    tag: String,
    /// 折叠按钮组的宽度。支持以下枚举值：
    ///
    /// - default：默认宽度
    /// - fill：卡片最大支持宽度
    /// - [100,∞)px：自定义宽度。超出卡片宽度时将按最大支持宽度展示
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 折叠按钮组当中的选项按钮。详见下文 options 字段说明。
    options: Vec<OverflowOption>,
    /// 组件整体的回调数据。当用户点击折叠按钮组的折叠按钮后，会将 value
    /// 的值返回给接收回调数据的服务器。后续你可以通过服务器接收的 value 值进行业务处理。
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Value>,
    /// 二次确认弹窗配置。指在用户提交时弹出二次确认弹窗提示；只有用户点击确认后，
    /// 才提交输入的内容。该字段默认提供了确认和取消按钮，你只需要配置弹窗的标题与内容即可。
    ///
    /// 注意：confirm 字段仅在用户点击包含提交属性的按钮时才会触发二次确认弹窗。
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<InputConfirm>,
}

impl Default for FeishuCardOverflow {
    fn default() -> Self {
        Self {
            tag: "overflow".to_string(),
            width: None,
            options: vec![],
            value: None,
            confirm: None,
        }
    }
}

impl FeishuCardOverflow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn options(mut self, options: Vec<OverflowOption>) -> Self {
        self.options = options;
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

    pub fn add_option(mut self, option: OverflowOption) -> Self {
        self.options.push(option);
        self
    }
}

/// options 字段说明
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OverflowOption {
    /// 按钮上的文本。
    text: Option<PlainText>,
    /// 为按钮添加多端的跳转链接。
    multi_url: Option<FeishuCardHrefVal>,
    /// 该按钮的回传参数值。当用户点击选项后，应用会将该值返回至卡片请求地址。
    value: Option<String>,
}

impl OverflowOption {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: PlainText) -> Self {
        self.text = Some(text);
        self
    }

    pub fn multi_url(mut self, multi_url: FeishuCardHrefVal) -> Self {
        self.multi_url = Some(multi_url);
        self
    }

    pub fn value(mut self, value: impl ToString) -> Self {
        self.value = Some(value.to_string());
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
                overflow::{FeishuCardOverflow, OverflowOption},
            },
        },
        href::FeishuCardHrefVal,
    };

    #[test]
    fn test_overflow() {
        let overflow = FeishuCardOverflow::new()
            .width("fill")
            .options(vec![OverflowOption::new()
                .text(PlainText::text("这是一个链接跳转"))
                .multi_url(
                    FeishuCardHrefVal::new().url("https://open.feishu.cn/document/home/index"),
                )
                .value("document")])
            .value(json!({"key_1": "value_1"}))
            .confirm(InputConfirm::new("title", "content"));

        let json = json!({
          "tag": "overflow",
          "width": "fill", // 折叠按钮组的宽度。默认值为 default。
          "options": [
            // 在此添加折叠按钮组当中的选项按钮。
            { // 为按钮添加文本。
              "text": {
                "tag": "plain_text", // 文本的标签。固定值为 plain_text。
                "content": "这是一个链接跳转" // 文本的内容，最多支持 100 个字符。
              },
              "multi_url": { // 为按钮添加跳转链接。
                "url": "https://open.feishu.cn/document/home/index", // 兜底的跳转地址。
              },
              "value": "document" // 该按钮的回传参数值。当用户点击选项后，应用会将该值返回至卡片请求地址。
            }
          ],
          "value": {
            // 组件整体的回调数据。
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

        assert_eq!(serde_json::to_value(&overflow).unwrap(), json);
    }
}
