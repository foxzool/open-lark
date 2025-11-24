//! 用户信息相关数据模型

use serde::{Deserialize, Serialize};

/// 用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    /// 用户唯一标识
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 用户邮箱
    pub email: String,
    /// 用户手机号
    pub mobile: String,
    /// 用户头像URL
    pub avatar_url: String,
    /// 用户状态
    pub status: UserStatus,
    /// 用户所属部门信息
    pub department_ids: Vec<String>,
    /// 用户职位
    pub position: String,
    /// 用户工号
    pub employee_no: String,
    /// 用户昵称
    pub nickname: String,
    /// 用户英文名
    pub en_name: String,
    /// 用户性别
    pub gender: Gender,
    /// 用户开放ID
    pub open_id: String,
    /// 用户联合ID
    pub union_id: String,
}

/// 用户状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    /// 未激活
    Unactivated,
    /// 已激活
    Activated,
    /// 已禁用
    Disabled,
    /// 已离职
    Resigned,
}

/// 用户性别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    /// 男性
    Male,
    /// 女性
    Female,
    /// 未知
    Unknown,
}
