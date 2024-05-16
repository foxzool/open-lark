use serde::{Deserialize, Serialize};

/// 图片组件
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardImage {

    /// 图标 key 的获取方式：调用上传图片接口，上传用于发送消息的图片，并在返回值中获取图片的 image_key。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_key: Option<String>,
}


