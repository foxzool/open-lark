use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardMultiImageLayout {
    /// 多图混排组件的标签，固定取值：img_combination。
    tag: String,
    /// 多图混排的方式，可取值：
    ///
    /// - double：双图混排，最多可排布两张图。
    /// - triple：三图混排，最多可排布三张图。
    /// - bisect：等分双列图混排，每行两个等大的正方形图，最多可排布三行，即六张图。
    /// - trisect：等分三列图混排，每行三个等大的正方形图，最多可排布三行，即九张图。
    ///
    /// 注意：
    ///
    /// 若上传的图片数量超过混排方式可容纳的上限，则系统将根据图片上传的顺序，
    /// 优先展示排列顺序中靠前的图片。超出上限的图片将不再显示。
    /// 若上传的图片数量未达到混排方式可容纳的上限，则未排布的部分将保留空白。
    combination_mode: String,
    /// 多图混排图片的圆角半径，单位是像素（px）。取值遵循以下格式：
    ///
    /// [0,∞]px
    /// [0,100]%
    #[serde(skip_serializing_if = "Option::is_none")]
    corner_radius: Option<String>,
    /// 图片资源的 img_key 数组，顺序与图片排列顺序一致
    img_list: Vec<RawImage>,
}

impl Default for FeishuCardMultiImageLayout {
    fn default() -> Self {
        FeishuCardMultiImageLayout {
            tag: "img_combination".to_string(),
            combination_mode: "".to_string(),
            corner_radius: None,
            img_list: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RawImage {
    img_key: String,
}

impl FeishuCardMultiImageLayout {
    pub fn new() -> Self {
        FeishuCardMultiImageLayout::default()
    }

    pub fn combination_mode(mut self, combination_mode: &str) -> Self {
        self.combination_mode = combination_mode.to_string();
        self
    }

    pub fn corner_radius(mut self, corner_radius: &str) -> Self {
        self.corner_radius = Some(corner_radius.to_string());
        self
    }

    pub fn img_list(mut self, img_list: Vec<&str>) -> Self {
        self.img_list = img_list
            .iter()
            .map(|img_key| RawImage {
                img_key: img_key.to_string(),
            })
            .collect();
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_multi_image_layout() {
        let multi_image_layout = FeishuCardMultiImageLayout::new()
            .combination_mode("trisect")
            .img_list(vec![
                "img_v2_4c772db0-9aff-4eba-bbf4-6e6121cabcef",
                "img_v2_4c772db0-9aff-4eba-bbf4-6e6121cabcef",
                "img_v2_4c772db0-9aff-4eba-bbf4-6e6121cabcef",
            ]);

        assert_eq!(
            serde_json::to_value(multi_image_layout).unwrap(),
            json!({
              "tag": "img_combination",
              "combination_mode": "trisect",
              "img_list": [
                {
                  "img_key": "img_v2_4c772db0-9aff-4eba-bbf4-6e6121cabcef"
                },
                {
                  "img_key": "img_v2_4c772db0-9aff-4eba-bbf4-6e6121cabcef"
                },
                {
                  "img_key": "img_v2_4c772db0-9aff-4eba-bbf4-6e6121cabcef"
                }
              ]
            })
        );
    }
}
