//! 群公告相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 获取群公告信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementResponse {
    pub content: String,
    pub revision: String,
    pub create_time: String,
    pub update_time: String,
    pub owner_id_type: String,
    pub owner_id: String,
    pub modifier_id_type: String,
    pub modifier_id: String,
}

impl ApiResponseTrait for GetChatAnnouncementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新群公告信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchChatAnnouncementBody {
    pub revision: String,
    pub requests: Vec<String>,
}

