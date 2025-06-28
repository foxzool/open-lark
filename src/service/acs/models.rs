use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 用户类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    /// 员工
    Employee,
    /// 访客
    Visitor,
    /// 承包商
    Contractor,
    /// 临时人员
    Temporary,
}

/// 用户状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    /// 活跃
    Active,
    /// 禁用
    Disabled,
    /// 过期
    Expired,
    /// 待审核
    Pending,
}

/// 门禁用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcsUser {
    /// 用户ID
    pub user_id: String,
    /// 员工工号/员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 用户姓名
    pub name: String,
    /// 用户类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 电话号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 是否有人脸图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_face_image: Option<bool>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 权限组状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleStatus {
    /// 活跃
    Active,
    /// 禁用
    Disabled,
}

/// 权限组信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExternal {
    /// 权限组ID
    pub rule_id: String,
    /// 权限组名称
    pub name: String,
    /// 权限组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限组状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleStatus>,
    /// 关联的设备ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
    /// 关联的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 生效开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 生效结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 访客状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisitorStatus {
    /// 活跃
    Active,
    /// 过期
    Expired,
    /// 已签出
    CheckedOut,
}

/// 访客信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visitor {
    /// 访客ID
    pub visitor_id: String,
    /// 访客姓名
    pub name: String,
    /// 访客电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 访客公司
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 访问目的
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// 接待人员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_user_id: Option<String>,
    /// 接待人员姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    /// 访客状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VisitorStatus>,
    /// 访问开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 访问结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 设备类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    /// 门禁设备
    AccessControl,
    /// 人脸识别设备
    FaceRecognition,
    /// 闸机
    Turnstile,
    /// 电梯控制器
    ElevatorController,
}

/// 设备状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceStatus {
    /// 在线
    Online,
    /// 离线
    Offline,
    /// 故障
    Error,
    /// 维护中
    Maintenance,
}

/// 门禁设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// 设备ID
    pub device_id: String,
    /// 设备名称
    pub name: String,
    /// 设备类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<DeviceType>,
    /// 设备状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeviceStatus>,
    /// 设备位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// 设备描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 设备IP地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// 设备MAC地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 访问类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessType {
    /// 进入
    Entry,
    /// 离开
    Exit,
}

/// 访问方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMethod {
    /// 人脸识别
    FaceRecognition,
    /// 刷卡
    Card,
    /// 密码
    Password,
    /// 指纹
    Fingerprint,
    /// 手动开门
    Manual,
}

/// 访问结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessResult {
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 拒绝
    Denied,
    /// 超时
    Timeout,
}

/// 门禁访问记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRecord {
    /// 记录ID
    pub record_id: String,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 设备ID
    pub device_id: String,
    /// 设备名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// 访问类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<AccessType>,
    /// 访问方式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_method: Option<AccessMethod>,
    /// 访问结果
    pub result: AccessResult,
    /// 是否有人脸识别图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_face_image: Option<bool>,
    /// 访问时间戳
    pub access_time: i64,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

/// 人脸图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceImage {
    /// 图片ID
    pub image_id: String,
    /// 图片内容 (base64编码)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_content: Option<String>,
    /// 图片格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_format: Option<String>,
    /// 图片大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 上传时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<i64>,
}
