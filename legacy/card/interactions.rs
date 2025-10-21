use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 卡片交互行为类型
///
/// 定义卡片组件支持的各种交互行为
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Behaviors {
    /// 跳转URL行为
    OpenUrl(OpenUrlBehavior),
    /// 回调行为
    Callback(CallbackBehavior),
    /// 表单行为
    Form(FormBehavior),
}

/// 跳转链接交互
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenUrlBehavior {
    /// 声明交互类型。要配置跳转链接交互，取固定值 open_url。
    r#type: String,
    /// 兜底跳转地址。
    default_url: String,
    /// Android 端的链接地址。可配置为 lark://msgcard/unsupported_action 声明当前端不允许跳转。
    #[serde(skip_serializing_if = "Option::is_none")]
    android_url: Option<String>,
    /// iOS 端的链接地址。可配置为 lark://msgcard/unsupported_action 声明当前端不允许跳转。
    #[serde(skip_serializing_if = "Option::is_none")]
    ios_url: Option<String>,
    /// PC 端的链接地址。可配置为 lark://msgcard/unsupported_action 声明当前端不允许跳转。
    #[serde(skip_serializing_if = "Option::is_none")]
    pc_url: Option<String>,
}

impl OpenUrlBehavior {
    pub fn new(default_url: &str) -> Self {
        Self {
            r#type: "open_url".to_string(),
            default_url: default_url.to_string(),
            android_url: None,
            ios_url: None,
            pc_url: None,
        }
    }

    pub fn default_url(mut self, url: &str) -> Self {
        self.default_url = url.to_string();
        self
    }

    pub fn android_url(mut self, url: &str) -> Self {
        self.android_url = Some(url.to_string());
        self
    }

    pub fn ios_url(mut self, url: &str) -> Self {
        self.ios_url = Some(url.to_string());
        self
    }

    pub fn pc_url(mut self, url: &str) -> Self {
        self.pc_url = Some(url.to_string());
        self
    }
}

/// 服务端回传交互
#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackBehavior {
    r#type: String,
    value: Value,
}

impl CallbackBehavior {
    pub fn new(value: Value) -> Self {
        Self {
            r#type: "callback".to_string(),
            value,
        }
    }
}

/// 表单事件交互
#[derive(Debug, Serialize, Deserialize)]
pub struct FormBehavior {
    /// 声明交互类型。要配置表单事件交互，取固定值 form_action。
    r#type: String,
    /// 表单事件类型。可取值：
    ///
    /// - submit：提交整个表单
    /// - reset：重置整个表单
    #[serde(skip_serializing_if = "Option::is_none")]
    behavior: Option<String>,
}

impl Default for FormBehavior {
    fn default() -> Self {
        Self {
            r#type: "form_action".to_string(),
            behavior: None,
        }
    }
}

impl FormBehavior {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn behavior(mut self, behavior: &str) -> Self {
        self.behavior = Some(behavior.to_string());
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::interactions::{Behaviors, CallbackBehavior, FormBehavior, OpenUrlBehavior};

    #[test]
    fn test_open_url() {
        let jump_behavior = Behaviors::OpenUrl(OpenUrlBehavior::new("xxx"));
        let json = json!({
            "type": "open_url",
            "default_url": "xxx",
        });

        assert_eq!(serde_json::to_value(&jump_behavior).unwrap(), json)
    }

    #[test]
    fn test_callback() {
        let callback_behavior = Behaviors::Callback(CallbackBehavior::new(json!("hello")));
        let json = json!({
            "type": "callback",
            "value": "hello"
        });

        assert_eq!(serde_json::to_value(&callback_behavior).unwrap(), json)
    }

    #[test]
    fn test_form() {
        let form_behavior = Behaviors::Form(FormBehavior::new());
        let json = json!({
           "type": "form_action",
        });

        assert_eq!(serde_json::to_value(&form_behavior).unwrap(), json)
    }
}
