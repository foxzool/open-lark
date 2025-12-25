use serde::{Deserialize, Serialize};

/// 成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户类型
    pub user_type: String,
}

impl MemberInfo {
    /// 创建成员信息
    pub fn new(user_id: impl Into<String>, user_type: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            user_type: user_type.into(),
        }
    }
}

/// 成员权限信息（用于批量添加）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberPermission {
    /// 成员信息
    pub member: MemberInfo,
    /// 权限类型
    #[serde(rename = "type")]
    pub r#type: String,
}

impl MemberPermission {
    /// 创建成员权限
    pub fn new(member: MemberInfo, r#type: impl Into<String>) -> Self {
        Self {
            member,
            r#type: r#type.into(),
        }
    }
}

