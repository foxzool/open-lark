//! 群管理员相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 指定/删除群管理员请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatManagersBody {
    pub manager_ids: Vec<String>,
}

/// 指定/删除群管理员响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatManagersResponse {
    #[serde(default)]
    pub chat_managers: Option<Vec<String>>,
    #[serde(default)]
    pub chat_bot_managers: Option<Vec<String>>,
}

impl ApiResponseTrait for ChatManagersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

