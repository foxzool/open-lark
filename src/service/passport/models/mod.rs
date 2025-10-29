// passport模块的数据模型定义

use serde::{Deserialize, Serialize};
/// 用户会话信息
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct UserSession {
    /// 用户ID
#[serde(rename = "user_id")],
    pub user_id: String,
    /// 用户名
#[serde(rename = "user_name")],
    pub user_name: Option<String>,
    /// 用户邮箱
#[serde(rename = "email")],
    pub email: Option<String>,
    /// 用户手机号
#[serde(rename = "mobile")],
    pub mobile: Option<String>,
    /// 头像
#[serde(rename = "avatar")],
    pub avatar: Option<String>,
    /// 部门信息
#[serde(rename = "department")],
    pub department: Option<DepartmentInfo>,
    /// 登录时间
#[serde(rename = "login_time")],
    pub login_time: String,
    /// 登录设备
#[serde(rename = "device_info")],
    pub device_info: Option<DeviceInfo>,
    /// 登录IP
#[serde(rename = "login_ip")],
    pub login_ip: Option<String>,
    /// 会话状态
#[serde(rename = "session_status")],
    pub session_status: SessionStatus,
}
/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct DepartmentInfo {
    /// 部门ID
#[serde(rename = "department_id")],
    pub department_id: String,
    /// 部门名称
#[serde(rename = "department_name")],
    pub department_name: String,
    /// 部门路径
#[serde(rename = "department_path")],
    pub department_path: Option<String>,
}
/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct DeviceInfo {
    /// 设备类型
#[serde(rename = "device_type")],
    pub device_type: String,
    /// 设备型号
#[serde(rename = "device_model")],
    pub device_model: Option<String>,
    /// 操作系统
#[serde(rename = "os")],
    pub os: Option<String>,
    /// 浏览器信息
#[serde(rename = "browser")],
    pub browser: Option<String>,
}
/// 会话状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)],
#[serde(rename_all = "snake_case")],
pub enum SessionStatus {,
/// 活跃
    Active,
    /// 已过期
    Expired,
    /// 已登出
    LoggedOut,
}
/// 批量会话查询请求
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct BatchQueryRequest {
    /// 用户ID列表
#[serde(rename = "user_ids")],
    pub user_ids: Vec<String>,
    /// 用户ID类型
#[serde(rename = "user_id_type")],
    pub user_id_type: String,
}
/// 批量会话查询响应
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct BatchQueryResponse {
    /// 用户会话列表
#[serde(rename = "user_sessions")],
    pub user_sessions: Vec<UserSession>,
    /// 失败的用户ID列表
#[serde(rename = "failed_user_ids")],
    pub failed_user_ids: Vec<String>,
}
/// 退出登录请求
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct LogoutRequest {
    /// 用户ID列表
#[serde(rename = "user_ids")],
    pub user_ids: Vec<String>,
    /// 用户ID类型
#[serde(rename = "user_id_type")],
    pub user_id_type: String,
}
/// 退出登录响应
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct LogoutResponse {
    /// 成功退出的用户数量
#[serde(rename = "success_count")],
    pub success_count: i32,
    /// 失败的用户数量
#[serde(rename = "failed_count")],
    pub failed_count: i32,
    /// 失败的用户ID列表
#[serde(rename = "failed_user_ids")],
    pub failed_user_ids: Vec<String>,
}
/// 通用响应结构
#[derive(Debug, Clone, Serialize, Deserialize)],
pub struct PassportResponse<T> {,
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}