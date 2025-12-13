/// CCM Drive Permission 响应类型定义
use serde::{Deserialize, Serialize};

/// 成员权限数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPermittedData {
    /// 是否有权限
    pub permitted: bool,
    /// 权限类型
    #[serde(rename = "perm_type")]
    pub perm_type: String,
    /// 权限详情
    pub perm: Option<String>,
}

/// 成员转移数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberTransferData {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
    /// 转移后的拥有者ID
    #[serde(rename = "owner_id")]
    pub owner_id: Option<String>,
}

/// 公开权限数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicData {
    /// 权限类型
    #[serde(rename = "permission_type")]
    pub permission_type: String,
    /// 链接URL
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// 是否需要密码
    #[serde(rename = "need_password")]
    pub need_password: bool,
    /// 过期时间
    #[serde(rename = "expire_time")]
    pub expire_time: Option<i64>,
}