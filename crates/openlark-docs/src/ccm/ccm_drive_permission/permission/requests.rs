/// CCM Drive Permission 请求类型定义
use serde::{Deserialize, Serialize};

/// 成员权限请求
#[derive(Debug, Clone)]
pub struct MemberPermittedRequest {
    /// 文件/文件夹token
    pub obj_token: String,
    /// 成员ID
    pub member_id: String,
    /// 权限类型
    pub perm_type: String,
}

impl MemberPermittedRequest {
    /// 创建新的成员权限请求
    pub fn new(obj_token: &str, member_id: &str, perm_type: &str) -> Self {
        Self {
            obj_token: obj_token.to_string(),
            member_id: member_id.to_string(),
            perm_type: perm_type.to_string(),
        }
    }
}

/// 成员转移请求
#[derive(Debug, Clone)]
pub struct MemberTransferRequest {
    /// 文件/文件夹token
    pub obj_token: String,
    /// 目标成员ID
    pub target_id: String,
    /// 目标类型
    pub target_type: Option<String>,
}

impl MemberTransferRequest {
    /// 创建新的成员转移请求
    pub fn new(obj_token: &str, target_id: &str) -> Self {
        Self {
            obj_token: obj_token.to_string(),
            target_id: target_id.to_string(),
            target_type: None,
        }
    }

    /// 设置目标类型
    pub fn target_type(mut self, target_type: &str) -> Self {
        self.target_type = Some(target_type.to_string());
        self
    }
}

/// 公开权限请求
#[derive(Debug, Clone)]
pub struct PublicRequest {
    /// 文件/文件夹token
    pub obj_token: String,
    /// 权限类型
    pub permission_type: Option<String>,
}

impl PublicRequest {
    /// 创建新的公开权限请求
    pub fn new(obj_token: &str) -> Self {
        Self {
            obj_token: obj_token.to_string(),
            permission_type: None,
        }
    }

    /// 设置权限类型
    pub fn permission_type(mut self, permission_type: &str) -> Self {
        self.permission_type = Some(permission_type.to_string());
        self
    }
}