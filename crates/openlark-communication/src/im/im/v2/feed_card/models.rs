//! 消息流卡片相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 即时提醒请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedCardTimeSensitiveBody {
    pub time_sensitive: bool,
    pub user_ids: Vec<String>,
}

/// 单个用户失败原因
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedUserReason {
    pub error_code: i32,
    pub error_message: String,
    pub user_id: String,
}

/// 即时提醒响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedCardActionResponse {
    #[serde(default)]
    pub failed_user_reasons: Option<Vec<FailedUserReason>>,
}

impl ApiResponseTrait for FeedCardActionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

