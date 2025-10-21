use serde::{Deserialize, Serialize};

/// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardTextIcon {
    /// 图标类型的标签。可取值：
    ///
    /// standard_icon：使用图标库中的图标。
    /// custom_icon：使用用自定义图片作为图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    /// 图标库中图标的 token。当 tag 为 standard_icon 时生效
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    /// 图标的颜色。支持设置线性和面性图标（即 token 末尾为 outlined 或 filled 的图标）的颜色。当
    /// tag 为 standard_icon 时生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    /// 自定义前缀图标的图片 key。当 tag 为 custom_icon 时生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    img_key: Option<String>,
    /// 图标的尺寸。支持 "[1,999] [1,999]px"。
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
}

impl Default for FeishuCardTextIcon {
    fn default() -> Self {
        FeishuCardTextIcon {
            tag: Some("standard_icon".to_string()),
            token: None,
            color: None,
            img_key: None,
            size: None,
        }
    }
}

impl FeishuCardTextIcon {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn img_key(mut self, img_key: &str) -> Self {
        self.img_key = Some(img_key.to_string());
        self
    }

    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }
}
