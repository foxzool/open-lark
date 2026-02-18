//! URL 预览相关模型（不算 API）

use serde::{Deserialize, Serialize};

/// 更新 URL 预览请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateUrlPreviewBody {
    pub preview_tokens: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_ids: Option<Vec<String>>,
}
