use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct GetAppResponse {
    pub app_id: String,
    pub app_name: String,
    pub app_type: String,
    pub description: Option<String>,
}
