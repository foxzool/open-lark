use serde::{Deserialize, Serialize};

/// 认证信息
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationInfo {
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 应用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_status: Option<String>,
    /// 认证状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
    /// 认证类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_type: Option<String>,
    /// 认证时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_time: Option<String>,
    /// 过期时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 权限范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// 租户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_info: Option<TenantInfo>,
}

/// 租户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct TenantInfo {
    /// 租户key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    /// 租户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}
