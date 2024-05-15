use serde::{Deserialize, Serialize};

/// URL变量
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardHrefVal {
    /// 默认的链接地址。
    pub url: String,
    /// Android 端的链接地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_url: Option<String>,
    /// iOS 端的链接地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_url: Option<String>,
    /// PC 端的链接地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
}

pub struct FeishuCardHrefValBuilder {
    href_val: FeishuCardHrefVal,
}

impl FeishuCardHrefValBuilder {
    pub fn new(url: impl ToString) -> Self {
        FeishuCardHrefValBuilder {
            href_val: FeishuCardHrefVal {
                url: url.to_string(),
                android_url: None,
                ios_url: None,
                pc_url: None,
            },
        }
    }

    pub fn android_url(mut self, url: impl ToString) -> Self {
        self.href_val.android_url = Some(url.to_string());
        self
    }

    pub fn ios_url(mut self, url: impl ToString) -> Self {
        self.href_val.ios_url = Some(url.to_string());
        self
    }

    pub fn pc_url(mut self, url: impl ToString) -> Self {
        self.href_val.pc_url = Some(url.to_string());
        self
    }

    pub fn build(self) -> FeishuCardHrefVal {
        self.href_val
    }
}
