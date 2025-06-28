use serde::{Deserialize, Serialize};

// 导出所有请求/响应结构体
pub use crate::service::contact::v3::{
    custom_attr::*, department::*, employee_type_enum::*, functional_role::*,
    functional_role_member::*, group::*, group_member::*, job_family::*, job_level::*,
    job_title::*, scope::*, unit::*, user::*, work_city::*,
};

/// 用户信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 英文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 电话号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_visible: Option<bool>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Avatar>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 直属上级用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 国家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 工位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_station: Option<String>,
    /// 入职时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<i64>,
    /// 离职时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_time: Option<i64>,
    /// 员工编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 员工类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<i32>,
    /// 职务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 是否是租户超级管理员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tenant_manager: Option<bool>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attrs: Option<Vec<UserCustomAttr>>,
    /// 企业邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 职级ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 工作城市ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_city: Option<String>,
}

/// 头像信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Avatar {
    /// 头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    /// 头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    /// 头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    /// 头像原图URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
}

/// 用户状态
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatus {
    /// 是否冻结
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    /// 是否离职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resigned: Option<bool>,
    /// 是否激活
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_activated: Option<bool>,
    /// 是否主动离职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_exited: Option<bool>,
    /// 是否unjoin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unjoin: Option<bool>,
}

/// 用户自定义字段
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCustomAttr {
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 字段ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// 部门信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Department {
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 国际化部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<I18nName>,
    /// 父部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DepartmentStatus>,
    /// 部门负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaders: Option<Vec<String>>,
    /// 部门群ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// 单位绑定
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_ids: Option<Vec<String>>,
    /// 成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// 创建组织架构
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_group_chat: Option<bool>,
}

/// 国际化名称
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nName {
    /// 中文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 日文名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

/// 部门状态
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentStatus {
    /// 是否被删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
}

/// 用户组信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Group {
    /// 用户组ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 用户组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 用户组成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_user_count: Option<i32>,
    /// 用户组部门数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_department_count: Option<i32>,
    /// 用户组类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}

/// 用户组成员
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupMember {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 成员ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
}

/// 用户组成员信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupMemberInfo {
    /// 成员ID
    pub member_id: String,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 成员ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
}

/// 用户组成员操作结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupMemberResult {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 操作结果码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 操作结果消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

/// 自定义字段
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomAttr {
    /// 字段ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
    /// 字段描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nName>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// 字段配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

/// 人员类型枚举
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeTypeEnum {
    /// 枚举ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_id: Option<String>,
    /// 枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<String>,
    /// 枚举内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 枚举类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<i32>,
    /// 枚举状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_status: Option<i32>,
    /// 国际化内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_content: Option<Vec<I18nContent>>,
}

/// 国际化内容
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nContent {
    /// 语言代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// 内容值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 单位信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Unit {
    /// 单位ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 单位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 单位类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
}

/// 角色成员信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleMemberInfo {
    /// 成员ID
    pub user_id: String,
    /// 管理范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 管理范围ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<Vec<String>>,
}

/// 角色成员
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleMember {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 管理范围类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 管理范围ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<Vec<String>>,
}

/// 角色成员管理范围
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleMemberScope {
    /// 成员ID
    pub user_id: String,
    /// 管理范围类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 管理范围ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<Vec<String>>,
}

/// 角色成员操作结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleMemberResult {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 操作结果码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 操作结果消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

/// 职级信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevel {
    /// 职级ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 职级名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nContent>>,
    /// 职级描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nContent>>,
    /// 职级状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    /// 职级等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// 序列信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobFamily {
    /// 序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 序列名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nContent>>,
    /// 序列描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nContent>>,
    /// 序列状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

/// 职务信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobTitle {
    /// 职务ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title_id: Option<String>,
    /// 职务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nContent>>,
    /// 职务状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

/// 工作城市信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkCity {
    /// 工作城市ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_city_id: Option<String>,
    /// 工作城市名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nContent>>,
    /// 工作城市状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}
