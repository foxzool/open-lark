//! ACS (访问控制系统) 数据模型

use serde::{Deserialize, Serialize};

/// 门禁用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
    /// 用户手机号
    pub mobile: Option<String>,
    /// 用户部门ID列表
    pub department_ids: Vec<String>,
    /// 用户状态
    pub status: crate::models::Status,
    /// 人脸是否已录入
    pub face_enrolled: bool,
    /// 门禁权限组ID列表
    pub rule_ids: Vec<String>,
    /// 创建时间
    pub create_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// 用户列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    /// 用户列表
    pub users: Vec<UserInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 用户创建/更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRequest {
    /// 用户姓名
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
    /// 用户手机号
    pub mobile: Option<String>,
    /// 用户部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 用户状态
    pub status: Option<crate::models::Status>,
    /// 门禁权限组ID列表
    pub rule_ids: Option<Vec<String>>,
    /// 扩展信息
    pub extension: Option<crate::models::ExtensionMap>,
}

/// 人脸图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceImageInfo {
    /// 人脸图片URL
    pub face_url: String,
    /// 人脸特征值
    pub face_feature: Option<String>,
    /// 图片质量分数
    pub quality_score: Option<f64>,
    /// 上传时间
    pub upload_time: crate::models::Timestamp,
}

/// 权限组信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRuleInfo {
    /// 权限组ID
    pub rule_id: String,
    /// 权限组名称
    pub name: String,
    /// 权限组描述
    pub description: Option<String>,
    /// 权限组状态
    pub status: crate::models::Status,
    /// 关联的设备ID列表
    pub device_ids: Vec<String>,
    /// 关联的用户ID列表
    pub user_ids: Vec<String>,
    /// 生效开始时间
    pub valid_from: Option<crate::models::Timestamp>,
    /// 生效结束时间
    pub valid_until: Option<crate::models::Timestamp>,
    /// 创建时间
    pub create_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// 权限组创建/更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRuleRequest {
    /// 权限组名称
    pub name: String,
    /// 权限组描述
    pub description: Option<String>,
    /// 关联的设备ID列表
    pub device_ids: Option<Vec<String>>,
    /// 关联的用户ID列表
    pub user_ids: Option<Vec<String>>,
    /// 生效开始时间
    pub valid_from: Option<crate::models::Timestamp>,
    /// 生效结束时间
    pub valid_until: Option<crate::models::Timestamp>,
    /// 扩展信息
    pub extension: Option<crate::models::ExtensionMap>,
}

/// 设备绑定权限组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBindRuleRequest {
    /// 权限组ID
    pub rule_id: String,
    /// 设备ID列表
    pub device_ids: Vec<String>,
    /// 是否覆盖现有绑定
    pub overwrite: Option<bool>,
}

/// 访客信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorInfo {
    /// 访客ID
    pub visitor_id: String,
    /// 访客姓名
    pub name: String,
    /// 访客手机号
    pub mobile: String,
    /// 访客邮箱
    pub email: Option<String>,
    /// 拜访事由
    pub visit_reason: Option<String>,
    /// 被访人信息
    pub host_info: Option<HostInfo>,
    /// 访问权限开始时间
    pub valid_from: crate::models::Timestamp,
    /// 访问权限结束时间
    pub valid_until: crate::models::Timestamp,
    /// 访客状态
    pub status: VisitorStatus,
    /// 权限组ID列表
    pub rule_ids: Vec<String>,
    /// 创建时间
    pub create_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// 被访人信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostInfo {
    /// 被访人ID
    pub user_id: String,
    /// 被访人姓名
    pub name: String,
    /// 被访人手机号
    pub mobile: Option<String>,
}

/// 访客状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VisitorStatus {
    /// 待审核
    Pending,
    /// 已批准
    Approved,
    /// 已拒绝
    Rejected,
    /// 已过期
    Expired,
    /// 已撤销
    Revoked,
}

/// 访客创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorRequest {
    /// 访客姓名
    pub name: String,
    /// 访客手机号
    pub mobile: String,
    /// 访客邮箱
    pub email: Option<String>,
    /// 拜访事由
    pub visit_reason: Option<String>,
    /// 被访人信息
    pub host_info: Option<HostInfo>,
    /// 访问权限开始时间
    pub valid_from: crate::models::Timestamp,
    /// 访问权限结束时间
    pub valid_until: crate::models::Timestamp,
    /// 权限组ID列表
    pub rule_ids: Option<Vec<String>>,
    /// 扩展信息
    pub extension: Option<crate::models::ExtensionMap>,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备ID
    pub device_id: String,
    /// 设备名称
    pub device_name: String,
    /// 设备类型
    pub device_type: DeviceType,
    /// 设备状态
    pub status: DeviceStatus,
    /// 设备位置
    pub location: Option<String>,
    /// 地理位置信息
    pub geo_location: Option<crate::models::GeoLocation>,
    /// 在线状态
    pub online: bool,
    /// 最后在线时间
    pub last_online_time: Option<crate::models::Timestamp>,
    /// 权限组ID列表
    pub rule_ids: Vec<String>,
    /// 创建时间
    pub create_time: crate::models::Timestamp,
    /// 更新时间
    pub update_time: crate::models::Timestamp,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// 设备类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    /// 门禁读卡器
    CardReader,
    /// 人脸识别终端
    FaceRecognition,
    /// 指纹识别终端
    FingerprintReader,
    /// 门锁
    DoorLock,
    /// 闸机
    Turnstile,
    /// 其他设备
    Other(String),
}

/// 设备状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeviceStatus {
    /// 正常
    Normal,
    /// 离线
    Offline,
    /// 故障
    Fault,
    /// 维护中
    Maintenance,
    /// 已禁用
    Disabled,
}

/// 访问记录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRecord {
    /// 记录ID
    pub record_id: String,
    /// 用户ID（如果是用户访问）
    pub user_id: Option<String>,
    /// 访客ID（如果是访客访问）
    pub visitor_id: Option<String>,
    /// 设备ID
    pub device_id: String,
    /// 访问时间
    pub access_time: crate::models::Timestamp,
    /// 访问结果
    pub access_result: AccessResult,
    /// 验证方式
    pub verification_method: VerificationMethod,
    /// 拒绝原因（如果访问被拒绝）
    pub reject_reason: Option<String>,
    /// 访问照片URL
    pub photo_url: Option<String>,
    /// 扩展信息
    pub extension: crate::models::ExtensionMap,
}

/// 访问结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AccessResult {
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 超时
    Timeout,
}

/// 验证方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationMethod {
    /// 人脸识别
    Face,
    /// 门禁卡
    Card,
    /// 指纹
    Fingerprint,
    /// 密码
    Password,
    /// 二维码
    QrCode,
    /// 其他方式
    Other(String),
}
