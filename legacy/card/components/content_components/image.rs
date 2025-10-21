use serde::{Deserialize, Serialize};

use crate::card::components::content_components::plain_text::PlainText;

/// 图片组件
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardImage {
    /// 组件的标签。图片组件固定取值为 img。
    tag: String,
    /// 图片资源的 Key。
    img_key: String,
    /// 悬浮（hover）在图片上时展示的说明文案
    #[serde(skip_serializing_if = "Option::is_none")]
    alt: Option<PlainText>,
    /// 图片标题
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PlainText>,
    /// 图片的圆角半径，单位是像素（px）。取值遵循以下格式：
    ///
    /// [0,∞]px
    /// [0,100]%
    #[serde(skip_serializing_if = "Option::is_none")]
    corner_radius: Option<String>,
    /// 图片的裁剪模式，当 size 字段的比例和图片的比例不一致时会触发裁剪。可取值：
    ///
    /// - crop_center：居中裁剪
    /// - crop_top：顶部裁剪
    /// - fit_horizontal：完整展示不裁剪
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_type: Option<String>,
    /// 图片尺寸。仅在 scale_type 字段为 crop_center 或 crop_top 时生效。可取值：
    ///
    /// - large：大图，尺寸为 160 × 160，适用于多图混排。
    /// - medium：中图，尺寸为 80 × 80，适用于图文混排的封面图。
    /// - small：小图，尺寸为 40 × 40，适用于人员头像。
    /// - tiny：超小图，尺寸为 16 × 16，适用于图标、备注。
    /// - stretch_without_padding：通栏图，适用于高宽比小于 16:9 的图片，图片的宽度将撑满卡片宽度。
    /// - stretch：超大图，适用于高宽比小于 16:9 的图片。
    /// - [1,999]px [1,999]px：自定义图片尺寸，单位为像素，中间用空格分隔。
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    /// 是否为透明底色。默认为 false，即图片为白色底色。
    #[serde(skip_serializing_if = "Option::is_none")]
    transparent: Option<bool>,
    /// 点击后是否放大图片。
    ///
    /// true：点击图片后，弹出图片查看器放大查看当前点击的图片。
    /// false：点击图片后，响应卡片本身的交互事件，不弹出图片查看器。
    /// 提示：如果你为卡片配置了跳转链接card_link参数，可将该参数设置为
    /// false，后续用户点击卡片上的图片也能响应 card_link 链接跳转。
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<bool>,
}

impl Default for FeishuCardImage {
    fn default() -> Self {
        FeishuCardImage {
            tag: "img".to_string(),
            img_key: "".to_string(),
            alt: None,
            title: None,
            corner_radius: None,
            scale_type: None,
            size: None,
            transparent: None,
            preview: None,
        }
    }
}

impl FeishuCardImage {
    pub fn new() -> Self {
        FeishuCardImage::default()
    }

    pub fn img_key(mut self, img_key: &str) -> Self {
        self.img_key = img_key.to_string();
        self
    }

    pub fn alt(mut self, alt: PlainText) -> Self {
        self.alt = Some(alt);
        self
    }

    pub fn title(mut self, title: PlainText) -> Self {
        self.title = Some(title);
        self
    }

    pub fn corner_radius(mut self, corner_radius: &str) -> Self {
        self.corner_radius = Some(corner_radius.to_string());
        self
    }

    pub fn scale_type(mut self, scale_type: &str) -> Self {
        self.scale_type = Some(scale_type.to_string());
        self
    }

    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = Some(transparent);
        self
    }

    pub fn preview(mut self, preview: bool) -> Self {
        self.preview = Some(preview);
        self
    }

    pub fn build(self) -> FeishuCardImage {
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_image() {
        let image = FeishuCardImage::new()
            .img_key("img_v2_ace8a4f2-ae13-420f-9eb3-b3530b4abcef")
            .scale_type("crop_top")
            .size("stretch")
            .preview(true);
        assert_eq!(
            serde_json::to_value(image).unwrap(),
            json!( {
                "tag": "img",
                "img_key": "img_v2_ace8a4f2-ae13-420f-9eb3-b3530b4abcef",
                "preview": true,
                "scale_type": "crop_top",
                "size": "stretch"
            })
        );
    }
}
