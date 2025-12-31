//! 批量消息相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 批量消息推送与阅读情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMessageReadUser {
    pub read_count: String,
    pub total_count: String,
}

/// 查询批量消息推送和阅读人数响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMessageReadUserResponse {
    pub read_user: BatchMessageReadUser,
}

impl ApiResponseTrait for BatchMessageReadUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

