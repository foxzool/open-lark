//! 图片相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 图片类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Message,
    Avatar,
}

impl ImageType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Message => "message",
            Self::Avatar => "avatar",
        }
    }
}

/// 上传图片响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageResponse {
    pub image_key: String,
}

impl ApiResponseTrait for CreateImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
