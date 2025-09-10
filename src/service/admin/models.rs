use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 分页响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记，下次请求的起点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 数据项目列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<T>>,
}

// ============ 登录密码管理相关结构 ============

/// 重置用户企业邮箱密码请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    /// 用户ID，ID类型与查询参数中的user_id_type对应
    pub user_id: String,
    /// 新密码
    pub password: String,
}

/// 重置用户企业邮箱密码响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetResponse {
    /// 重置结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 操作时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_time: Option<String>,
}

// ============ 数据报表管理相关结构 ============

/// 部门维度数据报表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentDataReportRequest {
    /// 开始日期，格式为YYYY-MM-DD
    pub start_date: String,
    /// 结束日期，格式为YYYY-MM-DD
    pub end_date: String,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 用户维度数据报表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataReportRequest {
    /// 开始日期，格式为YYYY-MM-DD
    pub start_date: String,
    /// 结束日期，格式为YYYY-MM-DD
    pub end_date: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 数据报表信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DataReport {
    /// 日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 用户ID或部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 用户名或部门名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 活跃用户数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_users: Option<i32>,
    /// 新增用户数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_users: Option<i32>,
    /// 消息发送数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_sent: Option<i32>,
    /// 云文档使用数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_usage: Option<i32>,
    /// 会议时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_duration: Option<i32>,
    /// 其他统计数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics: Option<HashMap<String, Value>>,
}

// ============ 企业勋章相关结构 ============

/// 勋章创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeCreateRequest {
    /// 勋章名称
    pub name: String,
    /// 勋章说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 勋章详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_description: Option<String>,
    /// 是否展示详情时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_detail_time: Option<bool>,
    /// 勋章图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 国际化名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<HashMap<String, String>>,
    /// 国际化说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<HashMap<String, String>>,
    /// 国际化详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_detail_description: Option<HashMap<String, String>>,
}

/// 勋章更新请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeUpdateRequest {
    /// 勋章ID
    pub badge_id: String,
    /// 勋章名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 勋章说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 勋章详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_description: Option<String>,
    /// 是否展示详情时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_detail_time: Option<bool>,
    /// 勋章图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 国际化名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<HashMap<String, String>>,
    /// 国际化说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<HashMap<String, String>>,
    /// 国际化详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_detail_description: Option<HashMap<String, String>>,
}

/// 勋章图片上传请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeImageUploadRequest {
    /// 图片文件内容（base64编码或文件流）
    pub image: String,
    /// 图片文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

/// 勋章列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeListRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 勋章名称（模糊搜索）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 勋章查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGetRequest {
    /// 勋章ID
    pub badge_id: String,
}

/// 勋章信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    /// 勋章ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    /// 勋章名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 勋章说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 勋章详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_description: Option<String>,
    /// 是否展示详情时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_detail_time: Option<bool>,
    /// 勋章图片URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// 勋章图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 国际化名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<HashMap<String, String>>,
    /// 国际化说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<HashMap<String, String>>,
    /// 国际化详情描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_detail_description: Option<HashMap<String, String>>,
}

/// 图片上传结果
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeImageUploadResult {
    /// 图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 图片URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

// ============ 勋章授予名单相关结构 ============

/// 勋章授予名单创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantCreateRequest {
    /// 勋章ID
    pub badge_id: String,
    /// 授予名单名称
    pub name: String,
    /// 名单说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 授予用户列表
    pub user_list: Vec<BadgeGrantUser>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    /// 授予时间展示类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// 勋章授予名单更新请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantUpdateRequest {
    /// 授予名单ID
    pub grant_id: String,
    /// 授予名单名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 名单说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 授予用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<BadgeGrantUser>>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    /// 授予时间展示类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// 勋章授予名单删除请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantDeleteRequest {
    /// 授予名单ID
    pub grant_id: String,
}

/// 勋章授予名单列表查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantListRequest {
    /// 勋章ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 授予名单名称（模糊搜索）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 勋章授予名单查询请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantGetRequest {
    /// 授予名单ID
    pub grant_id: String,
}

/// 勋章授予用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrantUser {
    /// 用户ID
    pub user_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 授予理由
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 授予时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_time: Option<String>,
}

/// 勋章授予名单信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeGrant {
    /// 授予名单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// 勋章ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<String>,
    /// 授予名单名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 名单说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 授予用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<BadgeGrantUser>>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    /// 授予时间展示类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}
