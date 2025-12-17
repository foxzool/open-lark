use serde::{Deserialize, Serialize};

/// AppRole 自定义角色
#[derive(Debug, Serialize, Deserialize)]
pub struct AppRole {
    /// 角色ID，以 "custom_" 开头
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 自定义角色的名字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}
