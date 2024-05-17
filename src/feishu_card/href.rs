use serde::{Deserialize, Serialize};

/// URL变量
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardHrefVal {
    /// 默认的链接地址。
    url: String,
    /// Android 端的链接地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    android_url: Option<String>,
    /// iOS 端的链接地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    ios_url: Option<String>,
    /// PC 端的链接地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pc_url: Option<String>,
}

impl FeishuCardHrefVal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url(mut self, url: impl ToString) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn android_url(mut self, url: impl ToString) -> Self {
        self.android_url = Some(url.to_string());
        self
    }

    pub fn ios_url(mut self, url: impl ToString) -> Self {
        self.ios_url = Some(url.to_string());
        self
    }

    pub fn pc_url(mut self, url: impl ToString) -> Self {
        self.pc_url = Some(url.to_string());
        self
    }
}
