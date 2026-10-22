//! contact 域 typed 事件（`contact.*`）。
//!
//! 字段权威：Go SDK v3.12.2 `service/contact/v3/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/contact/v3/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `contact.user.created_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactUserCreatedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactUserCreatedV3Event,
}

/// `contact.user.created_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactUserCreatedV3Event {
    /// 变更后的用户信息，只包含有变更的字段数据。
    #[serde(default)]
    pub object: Option<UserEvent>,
}

/// `contact.user.updated_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactUserUpdatedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactUserUpdatedV3Event,
}

/// `contact.user.updated_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactUserUpdatedV3Event {
    /// 变更后的用户信息，只包含有变更的字段数据。
    #[serde(default)]
    pub object: Option<UserEvent>,
    /// 变更前的用户信息，只包含有变更的字段数据。
    #[serde(default)]
    pub old_object: Option<UserEvent>,
}

/// `contact.user.deleted_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactUserDeletedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactUserDeletedV3Event,
}

/// `contact.user.deleted_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactUserDeletedV3Event {
    /// 删除时的用户信息。
    #[serde(default)]
    pub object: Option<UserEvent>,
    /// 删除前信息。
    #[serde(default)]
    pub old_object: Option<OldUserObject>,
}

/// `contact.department.created_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactDepartmentCreatedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactDepartmentCreatedV3Event,
}

/// `contact.department.created_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactDepartmentCreatedV3Event {
    /// 部门信息。
    #[serde(default)]
    pub object: Option<DepartmentEvent>,
}

/// `contact.department.updated_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactDepartmentUpdatedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactDepartmentUpdatedV3Event,
}

/// `contact.department.updated_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactDepartmentUpdatedV3Event {
    /// 更新后的部门信息。
    #[serde(default)]
    pub object: Option<DepartmentEvent>,
    /// 更新前的部门信息。
    #[serde(default)]
    pub old_object: Option<DepartmentEvent>,
}

/// `contact.department.deleted_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactDepartmentDeletedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactDepartmentDeletedV3Event,
}

/// `contact.department.deleted_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactDepartmentDeletedV3Event {
    /// 删除时的部门信息。
    #[serde(default)]
    pub object: Option<DepartmentEvent>,
    /// 部门被删除前的信息。
    #[serde(default)]
    pub old_object: Option<OldDepartmentObject>,
}

/// `contact.custom_attr_event.updated_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactCustomAttrEventUpdatedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactCustomAttrEventUpdatedV3Event,
}

/// `contact.custom_attr_event.updated_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactCustomAttrEventUpdatedV3Event {
    /// 变更后信息。
    #[serde(default)]
    pub object: Option<CustomAttrEvent>,
    /// 变更前信息。
    #[serde(default)]
    pub old_object: Option<CustomAttrEvent>,
}

/// `contact.scope.updated_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactScopeUpdatedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactScopeUpdatedV3Event,
}

/// `contact.scope.updated_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactScopeUpdatedV3Event {
    /// 通讯录权限范围新增的对象信息。
    #[serde(default)]
    pub added: Option<Scope>,
    /// 通讯录权限范围移除的对象信息。
    #[serde(default)]
    pub removed: Option<Scope>,
}

/// `contact.employee_type_enum.created_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumCreatedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactEmployeeTypeEnumCreatedV3Event,
}

/// `contact.employee_type_enum.created_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumCreatedV3Event {
    /// 新建的人员类型信息。
    #[serde(default)]
    pub new_enum: Option<EmployeeTypeEnum>,
}

/// `contact.employee_type_enum.updated_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumUpdatedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactEmployeeTypeEnumUpdatedV3Event,
}

/// `contact.employee_type_enum.updated_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumUpdatedV3Event {
    /// 更新前的人员类型信息。
    #[serde(default)]
    pub old_enum: Option<EmployeeTypeEnum>,
    /// 更新后的人员类型信息。
    #[serde(default)]
    pub new_enum: Option<EmployeeTypeEnum>,
}

/// `contact.employee_type_enum.deleted_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumDeletedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactEmployeeTypeEnumDeletedV3Event,
}

/// `contact.employee_type_enum.deleted_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumDeletedV3Event {
    /// 删除前的人员类型信息。
    #[serde(default)]
    pub old_enum: Option<EmployeeTypeEnum>,
}

/// `contact.employee_type_enum.actived_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumActivedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactEmployeeTypeEnumActivedV3Event,
}

/// `contact.employee_type_enum.actived_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumActivedV3Event {
    /// 激活前的人员类型信息。
    #[serde(default)]
    pub old_enum: Option<EmployeeTypeEnum>,
    /// 激活后的人员类型信息。
    #[serde(default)]
    pub new_enum: Option<EmployeeTypeEnum>,
}

/// `contact.employee_type_enum.deactivated_v3` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumDeactivatedV3 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContactEmployeeTypeEnumDeactivatedV3Event,
}

/// `contact.employee_type_enum.deactivated_v3` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactEmployeeTypeEnumDeactivatedV3Event {
    /// 停用前的人员类型信息。
    #[serde(default)]
    pub old_enum: Option<EmployeeTypeEnum>,
    /// 停用后的人员类型信息。
    #[serde(default)]
    pub new_enum: Option<EmployeeTypeEnum>,
}

/// 事件中的用户 ID 集合（Go `UserId`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserId {
    /// user_id。
    #[serde(default)]
    pub user_id: Option<String>,
    /// open_id。
    #[serde(default)]
    pub open_id: Option<String>,
    /// union_id。
    #[serde(default)]
    pub union_id: Option<String>,
}

/// 自定义通讯录字段变更（Go `CustomAttrEvent`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct CustomAttrEvent {
    /// 通讯录字段键值。
    #[serde(default)]
    pub contact_field_key: Vec<String>,
    /// 开关是否打开。
    #[serde(default)]
    pub allow_open_query: Option<bool>,
}

/// 事件中的部门信息（Go `DepartmentEvent`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentEvent {
    /// 部门名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 父部门的部门 open_department_id。
    #[serde(default)]
    pub parent_department_id: Option<String>,
    /// 当前部门的 department_id。
    #[serde(default)]
    pub department_id: Option<String>,
    /// 当前部门的 open_department_id。
    #[serde(default)]
    pub open_department_id: Option<String>,
    /// 部门主管的用户 open_id。
    #[serde(default)]
    pub leader_user_id: Option<String>,
    /// 部门群 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 当前部门在同级部门的排序。取值越小排序越靠前。
    #[serde(default)]
    pub order: Option<i64>,
    /// 部门单位自定义 ID 列表。
    #[serde(default)]
    pub unit_ids: Vec<String>,
    /// 部门状态。
    #[serde(default)]
    pub status: Option<DepartmentStatus>,
    /// 部门负责人信息。
    #[serde(default)]
    pub leaders: Vec<DepartmentLeader>,
    /// 部门 HRBP 的用户 ID 列表。
    #[serde(default)]
    pub department_hrbps: Vec<UserId>,
}

/// 部门被删除前的信息（Go `OldDepartmentObject`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OldDepartmentObject {
    /// 部门状态。
    #[serde(default)]
    pub status: Option<DepartmentStatus>,
    /// 当前部门的 open_department_id。
    #[serde(default)]
    pub open_department_id: Option<String>,
}

/// 部门状态（Go `DepartmentStatus`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct DepartmentStatus {
    /// 是否被删除。
    #[serde(default)]
    pub is_deleted: Option<bool>,
}

/// 部门负责人（Go `DepartmentLeader`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct DepartmentLeader {
    /// 负责人类型。
    #[serde(default, rename = "leaderType")]
    pub leader_type: Option<i64>,
    /// 负责人的用户 open_id。
    #[serde(default, rename = "leaderID")]
    pub leader_id: Option<String>,
}

/// 人员类型选项（Go `EmployeeTypeEnum`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeTypeEnum {
    /// 人员类型的选项 ID。
    #[serde(default)]
    pub enum_id: Option<String>,
    /// 人员类型的选项编号。
    #[serde(default)]
    pub enum_value: Option<String>,
    /// 人员类型的选项内容。
    #[serde(default)]
    pub content: Option<String>,
    /// 人员类型的选项类型。
    #[serde(default)]
    pub enum_type: Option<i64>,
    /// 人员类型的选项激活状态。
    #[serde(default)]
    pub enum_status: Option<i64>,
    /// 选项内容的国际化配置。
    #[serde(default)]
    pub i18n_content: Vec<I18nContent>,
}

/// 人员类型国际化内容（Go `I18nContent`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct I18nContent {
    /// 语言版本，例如 `zh_cn` / `en_us` / `ja_jp`。
    #[serde(default)]
    pub locale: Option<String>,
    /// 语言版本对应的内容。
    #[serde(default)]
    pub value: Option<String>,
}

/// 通讯录权限范围（Go `Scope`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scope {
    /// 部门对象信息。
    #[serde(default)]
    pub departments: Vec<Department>,
    /// 用户对象信息。
    #[serde(default)]
    pub users: Vec<User>,
    /// 用户组对象。
    #[serde(default)]
    pub user_groups: Vec<UserGroup>,
}

/// 事件中的用户信息（Go `UserEvent`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserEvent {
    /// 用户的 open_id。
    #[serde(default)]
    pub open_id: Option<String>,
    /// 用户的 union_id。
    #[serde(default)]
    pub union_id: Option<String>,
    /// 用户的 user_id。
    #[serde(default)]
    pub user_id: Option<String>,
    /// 用户名。
    #[serde(default)]
    pub name: Option<String>,
    /// 用户英文名。
    #[serde(default)]
    pub en_name: Option<String>,
    /// 用户别名。
    #[serde(default)]
    pub nickname: Option<String>,
    /// 邮箱。
    #[serde(default)]
    pub email: Option<String>,
    /// 企业邮箱。
    #[serde(default)]
    pub enterprise_email: Option<String>,
    /// 职务。
    #[serde(default)]
    pub job_title: Option<String>,
    /// 手机号。
    #[serde(default)]
    pub mobile: Option<String>,
    /// 手机号码可见性。
    #[serde(default)]
    pub mobile_visible: Option<bool>,
    /// 性别。
    #[serde(default)]
    pub gender: Option<i64>,
    /// 头像信息。
    #[serde(default)]
    pub avatar: Option<AvatarInfo>,
    /// 用户状态。
    #[serde(default)]
    pub status: Option<UserStatus>,
    /// 用户所属部门的 ID 列表。
    #[serde(default)]
    pub department_ids: Vec<String>,
    /// 用户直属主管的用户 open_id。
    #[serde(default)]
    pub leader_user_id: Option<String>,
    /// 城市。
    #[serde(default)]
    pub city: Option<String>,
    /// 国家。
    #[serde(default)]
    pub country: Option<String>,
    /// 工位。
    #[serde(default)]
    pub work_station: Option<String>,
    /// 入职时间，秒级时间戳。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_i64")]
    pub join_time: Option<i64>,
    /// 是否是租户管理员。
    #[serde(default)]
    pub is_tenant_manager: Option<bool>,
    /// 工号。
    #[serde(default)]
    pub employee_no: Option<String>,
    /// 员工类型。
    #[serde(default)]
    pub employee_type: Option<i64>,
    /// 职位信息。
    #[serde(default)]
    pub positions: Vec<UserPosition>,
    /// 用户排序信息。
    #[serde(default)]
    pub orders: Vec<UserOrder>,
    /// 时区。
    #[serde(default)]
    pub time_zone: Option<String>,
    /// 自定义字段信息。
    #[serde(default)]
    pub custom_attrs: Vec<UserCustomAttr>,
    /// 职级 ID。
    #[serde(default)]
    pub job_level_id: Option<String>,
    /// 序列 ID。
    #[serde(default)]
    pub job_family_id: Option<String>,
    /// 虚线上级的用户 ID。
    #[serde(default)]
    pub dotted_line_leader_user_ids: Vec<String>,
}

/// 用户被删除前的信息（Go `OldUserObject`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OldUserObject {
    /// 用户所属部门的 ID 列表（open_department_id）。
    #[serde(default)]
    pub department_ids: Vec<String>,
    /// 用户的 open_id。
    #[serde(default)]
    pub open_id: Option<String>,
}

/// 通讯录权限范围中的部门（Go `Department`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Department {
    /// 部门名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 国际化的部门名称。
    #[serde(default)]
    pub i18n_name: Option<DepartmentI18nName>,
    /// 父部门的 ID。取值为 0 表示父部门为根部门。
    #[serde(default)]
    pub parent_department_id: Option<String>,
    /// 当前部门的自定义部门 ID。
    #[serde(default)]
    pub department_id: Option<String>,
    /// 部门的 open_department_id。
    #[serde(default)]
    pub open_department_id: Option<String>,
    /// 部门主管的 open_id。
    #[serde(default)]
    pub leader_user_id: Option<String>,
    /// 部门群 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 部门的排序，取值越小排序越靠前。
    #[serde(default)]
    pub order: Option<String>,
    /// 部门单位的自定义 ID 列表。
    #[serde(default)]
    pub unit_ids: Vec<String>,
    /// 部门下用户的个数。
    #[serde(default)]
    pub member_count: Option<i64>,
    /// 部门状态。
    #[serde(default)]
    pub status: Option<DepartmentStatus>,
    /// 是否创建部门群。
    #[serde(default)]
    pub create_group_chat: Option<bool>,
    /// 部门负责人信息。
    #[serde(default)]
    pub leaders: Vec<DepartmentLeader>,
    /// 部门群雇员类型限制。
    #[serde(default)]
    pub group_chat_employee_types: Vec<i64>,
    /// 部门 HRBP。
    #[serde(default)]
    pub department_hrbps: Vec<String>,
    /// 部门下主属用户的个数。
    #[serde(default)]
    pub primary_member_count: Option<i64>,
}

/// 通讯录权限范围中的用户（Go `User`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    /// 用户的 union_id。
    #[serde(default)]
    pub union_id: Option<String>,
    /// 用户的 user_id。
    #[serde(default)]
    pub user_id: Option<String>,
    /// 用户的 open_id。
    #[serde(default)]
    pub open_id: Option<String>,
    /// 用户名。
    #[serde(default)]
    pub name: Option<String>,
    /// 英文名。
    #[serde(default)]
    pub en_name: Option<String>,
    /// 别名。
    #[serde(default)]
    pub nickname: Option<String>,
    /// 邮箱。
    #[serde(default)]
    pub email: Option<String>,
    /// 手机号。
    #[serde(default)]
    pub mobile: Option<String>,
    /// 手机号码可见性。
    #[serde(default)]
    pub mobile_visible: Option<bool>,
    /// 性别。
    #[serde(default)]
    pub gender: Option<i64>,
    /// 头像的文件 Key。
    #[serde(default)]
    pub avatar_key: Option<String>,
    /// 头像信息。
    #[serde(default)]
    pub avatar: Option<AvatarInfo>,
    /// 用户状态。
    #[serde(default)]
    pub status: Option<UserStatus>,
    /// 用户所属部门的 ID 列表。
    #[serde(default)]
    pub department_ids: Vec<String>,
    /// 用户的直接主管的 open_id。
    #[serde(default)]
    pub leader_user_id: Option<String>,
    /// 工作城市。
    #[serde(default)]
    pub city: Option<String>,
    /// 国家或地区 Code 缩写。
    #[serde(default)]
    pub country: Option<String>,
    /// 工位。
    #[serde(default)]
    pub work_station: Option<String>,
    /// 入职时间，秒级时间戳。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_i64")]
    pub join_time: Option<i64>,
    /// 是否是租户超级管理员。
    #[serde(default)]
    pub is_tenant_manager: Option<bool>,
    /// 工号。
    #[serde(default)]
    pub employee_no: Option<String>,
    /// 员工类型。
    #[serde(default)]
    pub employee_type: Option<i64>,
    /// 职位信息。
    #[serde(default)]
    pub positions: Vec<UserPosition>,
    /// 用户排序信息。
    #[serde(default)]
    pub orders: Vec<UserOrder>,
    /// 自定义字段。
    #[serde(default)]
    pub custom_attrs: Vec<UserCustomAttr>,
    /// 企业邮箱。
    #[serde(default)]
    pub enterprise_email: Option<String>,
    /// 租户类型。
    #[serde(default)]
    pub idp_type: Option<String>,
    /// 用户时区。
    #[serde(default)]
    pub time_zone: Option<String>,
    /// 用户签名。
    #[serde(default)]
    pub description: Option<String>,
    /// 职务。
    #[serde(default)]
    pub job_title: Option<String>,
    /// 是否发送提示消息。
    #[serde(default)]
    pub need_send_notification: Option<bool>,
    /// 提示消息配置。
    #[serde(default)]
    pub notification_option: Option<NotificationOption>,
    /// 是否为暂停状态的用户。
    #[serde(default)]
    pub is_frozen: Option<bool>,
    /// 数据驻留地。
    #[serde(default)]
    pub geo: Option<String>,
    /// 职级 ID。
    #[serde(default)]
    pub job_level_id: Option<String>,
    /// 序列 ID。
    #[serde(default)]
    pub job_family_id: Option<String>,
    /// 分配给用户的席位 ID 列表。
    #[serde(default)]
    pub subscription_ids: Vec<String>,
    /// 用户席位列表。
    #[serde(default)]
    pub assign_info: Vec<UserAssignInfo>,
    /// 部门路径。
    #[serde(default)]
    pub department_path: Vec<DepartmentDetail>,
    /// 虚线上级 ID。
    #[serde(default)]
    pub dotted_line_leader_user_ids: Vec<String>,
}

/// 通讯录权限范围中的用户组（Go `UserGroup`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGroup {
    /// 用户组的自定义 ID。
    #[serde(default)]
    pub user_group_id: Option<String>,
    /// 用户组的名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 用户组的类型。
    #[serde(default)]
    pub r#type: Option<i64>,
    /// 成员数量。
    #[serde(default)]
    pub member_count: Option<i64>,
    /// 用户组状态。
    #[serde(default)]
    pub status: Option<i64>,
}

/// 头像信息（Go `AvatarInfo`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct AvatarInfo {
    /// 72*72 像素头像链接。
    #[serde(default)]
    pub avatar_72: Option<String>,
    /// 240*240 像素头像链接。
    #[serde(default)]
    pub avatar_240: Option<String>,
    /// 640*640 像素头像链接。
    #[serde(default)]
    pub avatar_640: Option<String>,
    /// 原始头像链接。
    #[serde(default)]
    pub avatar_origin: Option<String>,
}

/// 用户状态（Go `UserStatus`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserStatus {
    /// 是否为暂停状态。
    #[serde(default)]
    pub is_frozen: Option<bool>,
    /// 是否为离职状态。
    #[serde(default)]
    pub is_resigned: Option<bool>,
    /// 是否为激活状态。
    #[serde(default)]
    pub is_activated: Option<bool>,
    /// 是否为主动退出状态。
    #[serde(default)]
    pub is_exited: Option<bool>,
    /// 是否为未加入状态。
    #[serde(default)]
    pub is_unjoin: Option<bool>,
}

/// 用户职位（Go `UserPosition`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserPosition {
    /// 岗位 Code。
    #[serde(default)]
    pub position_code: Option<String>,
    /// 岗位名称。
    #[serde(default)]
    pub position_name: Option<String>,
    /// 岗位部门 ID。
    #[serde(default)]
    pub department_id: Option<String>,
    /// 上级领导 ID。
    #[serde(default)]
    pub leader_user_id: Option<String>,
    /// 本岗位领导的岗位 code。
    #[serde(default)]
    pub leader_position_code: Option<String>,
    /// 是否主岗，只允许一个主岗。
    #[serde(default)]
    pub is_major: Option<bool>,
}

/// 用户排序信息（Go `UserOrder`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserOrder {
    /// 排序信息对应的部门 ID。
    #[serde(default)]
    pub department_id: Option<String>,
    /// 用户在其直属部门内的排序，数值越大排序越靠前。
    #[serde(default)]
    pub user_order: Option<i64>,
    /// 用户所属的多个部门间的排序，数值越大排序越靠前。
    #[serde(default)]
    pub department_order: Option<i64>,
    /// 标识部门是否为用户的唯一主部门。
    #[serde(default)]
    pub is_primary_dept: Option<bool>,
}

/// 用户自定义字段（Go `UserCustomAttr`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserCustomAttr {
    /// 自定义字段类型。
    #[serde(default)]
    pub r#type: Option<String>,
    /// 自定义字段 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 自定义字段取值。
    #[serde(default)]
    pub value: Option<UserCustomAttrValue>,
}

/// 用户自定义字段取值（Go `UserCustomAttrValue`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserCustomAttrValue {
    /// 文本值或网页标题。
    #[serde(default)]
    pub text: Option<String>,
    /// 字段类型为 HREF 时的默认 URL。
    #[serde(default)]
    pub url: Option<String>,
    /// 字段类型为 HREF 时的 PC 端 URL。
    #[serde(default)]
    pub pc_url: Option<String>,
    /// 选项 ID。
    #[serde(default)]
    pub option_id: Option<String>,
    /// 选项类型的值。
    #[serde(default)]
    pub option_value: Option<String>,
    /// 选项类型为图片时的图片名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 选项类型为图片时的图片链接。
    #[serde(default)]
    pub picture_url: Option<String>,
    /// 字段类型为 `GENERIC_USER` 时的引用人员信息。
    #[serde(default)]
    pub generic_user: Option<CustomAttrGenericUser>,
}

/// 自定义字段引用人员（Go `CustomAttrGenericUser`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct CustomAttrGenericUser {
    /// 引用人员的 user_id。
    #[serde(default)]
    pub id: Option<String>,
    /// 用户类型。目前固定取值为 1，表示用户。
    #[serde(default)]
    pub r#type: Option<i64>,
}

/// 国际化部门名称（Go `DepartmentI18nName`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct DepartmentI18nName {
    /// 部门的中文名。
    #[serde(default)]
    pub zh_cn: Option<String>,
    /// 部门的日文名。
    #[serde(default)]
    pub ja_jp: Option<String>,
    /// 部门的英文名。
    #[serde(default)]
    pub en_us: Option<String>,
}

/// 提示消息配置（Go `NotificationOption`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct NotificationOption {
    /// 通道列表。
    #[serde(default)]
    pub channels: Vec<String>,
    /// 语言类型。
    #[serde(default)]
    pub language: Option<String>,
}

/// 用户席位（Go `UserAssignInfo`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserAssignInfo {
    /// 席位 ID。
    #[serde(default)]
    pub subscription_id: Option<String>,
    /// 席位的许可证（license plan key）。
    #[serde(default)]
    pub license_plan_key: Option<String>,
    /// 席位名称。
    #[serde(default)]
    pub product_name: Option<String>,
    /// 国际化名称。
    #[serde(default)]
    pub i18n_name: Option<ProductI18nName>,
    /// 席位起始时间。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub start_time: Option<String>,
    /// 席位结束时间。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub end_time: Option<String>,
}

/// 席位国际化名称（Go `ProductI18nName`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ProductI18nName {
    /// 席位中文名。
    #[serde(default)]
    pub zh_cn: Option<String>,
    /// 席位日文名。
    #[serde(default)]
    pub ja_jp: Option<String>,
    /// 席位英文名。
    #[serde(default)]
    pub en_us: Option<String>,
}

/// 部门路径节点（Go `DepartmentDetail`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentDetail {
    /// 部门 ID。
    #[serde(default)]
    pub department_id: Option<String>,
    /// 部门名称信息。
    #[serde(default)]
    pub department_name: Option<DepartmentPathName>,
    /// 部门路径。
    #[serde(default)]
    pub department_path: Option<DepartmentPath>,
}

/// 部门路径名称（Go `DepartmentPathName`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentPathName {
    /// 部门名。
    #[serde(default)]
    pub name: Option<String>,
    /// 国际化的部门名称。
    #[serde(default)]
    pub i18n_name: Option<DepartmentI18nName>,
}

/// 部门路径（Go `DepartmentPath`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentPath {
    /// 部门路径 ID 列表。
    #[serde(default)]
    pub department_ids: Vec<String>,
    /// 部门名称信息。
    #[serde(default)]
    pub department_path_name: Option<DepartmentPathName>,
}

typed_events! {
    /// `contact.user.created_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.user.created_v3" | CONTACT_USER_CREATED_V3 | ContactUserCreatedV3 | ContactUserCreatedV3Handler | register_contact_user_created_v3;

    /// `contact.user.updated_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.user.updated_v3" | CONTACT_USER_UPDATED_V3 | ContactUserUpdatedV3 | ContactUserUpdatedV3Handler | register_contact_user_updated_v3;

    /// `contact.user.deleted_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.user.deleted_v3" | CONTACT_USER_DELETED_V3 | ContactUserDeletedV3 | ContactUserDeletedV3Handler | register_contact_user_deleted_v3;

    /// `contact.department.created_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.department.created_v3" | CONTACT_DEPARTMENT_CREATED_V3 | ContactDepartmentCreatedV3 | ContactDepartmentCreatedV3Handler | register_contact_department_created_v3;

    /// `contact.department.updated_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.department.updated_v3" | CONTACT_DEPARTMENT_UPDATED_V3 | ContactDepartmentUpdatedV3 | ContactDepartmentUpdatedV3Handler | register_contact_department_updated_v3;

    /// `contact.department.deleted_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.department.deleted_v3" | CONTACT_DEPARTMENT_DELETED_V3 | ContactDepartmentDeletedV3 | ContactDepartmentDeletedV3Handler | register_contact_department_deleted_v3;

    /// `contact.custom_attr_event.updated_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.custom_attr_event.updated_v3" | CONTACT_CUSTOM_ATTR_EVENT_UPDATED_V3 | ContactCustomAttrEventUpdatedV3 | ContactCustomAttrEventUpdatedV3Handler | register_contact_custom_attr_event_updated_v3;

    /// `contact.scope.updated_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.scope.updated_v3" | CONTACT_SCOPE_UPDATED_V3 | ContactScopeUpdatedV3 | ContactScopeUpdatedV3Handler | register_contact_scope_updated_v3;

    /// `contact.employee_type_enum.created_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.employee_type_enum.created_v3" | CONTACT_EMPLOYEE_TYPE_ENUM_CREATED_V3 | ContactEmployeeTypeEnumCreatedV3 | ContactEmployeeTypeEnumCreatedV3Handler | register_contact_employee_type_enum_created_v3;

    /// `contact.employee_type_enum.updated_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.employee_type_enum.updated_v3" | CONTACT_EMPLOYEE_TYPE_ENUM_UPDATED_V3 | ContactEmployeeTypeEnumUpdatedV3 | ContactEmployeeTypeEnumUpdatedV3Handler | register_contact_employee_type_enum_updated_v3;

    /// `contact.employee_type_enum.deleted_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.employee_type_enum.deleted_v3" | CONTACT_EMPLOYEE_TYPE_ENUM_DELETED_V3 | ContactEmployeeTypeEnumDeletedV3 | ContactEmployeeTypeEnumDeletedV3Handler | register_contact_employee_type_enum_deleted_v3;

    /// `contact.employee_type_enum.actived_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.employee_type_enum.actived_v3" | CONTACT_EMPLOYEE_TYPE_ENUM_ACTIVED_V3 | ContactEmployeeTypeEnumActivedV3 | ContactEmployeeTypeEnumActivedV3Handler | register_contact_employee_type_enum_actived_v3;

    /// `contact.employee_type_enum.deactivated_v3` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "contact.employee_type_enum.deactivated_v3" | CONTACT_EMPLOYEE_TYPE_ENUM_DEACTIVATED_V3 | ContactEmployeeTypeEnumDeactivatedV3 | ContactEmployeeTypeEnumDeactivatedV3Handler | register_contact_employee_type_enum_deactivated_v3;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const USER_CREATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c1","event_type":"contact.user.created_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "object":{"open_id":"on_new","union_id":"un_new","name":"张三","email":"zhang@ex.com","gender":1,
          "status":{"is_activated":true,"is_resigned":false},
          "department_ids":["od_1"],
          "employee_type":1,
          "positions":[{"position_code":"P1","position_name":"工程师","department_id":"od_1","is_major":true}],
          "custom_attrs":[{"type":"TEXT","id":"attr_1","value":{"text":"值"}}]}
      }
    }"#;

    const USER_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c2","event_type":"contact.user.updated_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "object":{"open_id":"on_u","name":"新名","job_title":"TL"},
        "old_object":{"open_id":"on_u","name":"旧名","job_title":"工程师"}
      }
    }"#;

    const USER_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c3","event_type":"contact.user.deleted_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "object":{"open_id":"on_gone","name":"离职者"},
        "old_object":{"department_ids":["od_1","od_2"],"open_id":"on_gone"}
      }
    }"#;

    const DEPARTMENT_CREATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c4","event_type":"contact.department.created_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "object":{"name":"新部门","parent_department_id":"od_parent","department_id":"d_1","open_department_id":"od_1",
          "leader_user_id":"on_leader","chat_id":"oc_dept","order":1,"unit_ids":["unit_1"],
          "status":{"is_deleted":false},
          "leaders":[{"leaderType":1,"leaderID":"on_l"}],
          "department_hrbps":[{"open_id":"on_hrbp"}]}
      }
    }"#;

    const DEPARTMENT_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c5","event_type":"contact.department.updated_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "object":{"name":"新部门名","open_department_id":"od_1"},
        "old_object":{"name":"旧部门名","open_department_id":"od_1"}
      }
    }"#;

    const DEPARTMENT_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c6","event_type":"contact.department.deleted_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "object":{"open_department_id":"od_del"},
        "old_object":{"status":{"is_deleted":true},"open_department_id":"od_del"}
      }
    }"#;

    const CUSTOM_ATTR_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c7","event_type":"contact.custom_attr_event.updated_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "object":{"contact_field_key":["k1","k2"],"allow_open_query":true},
        "old_object":{"contact_field_key":["k1"],"allow_open_query":false}
      }
    }"#;

    const SCOPE_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c8","event_type":"contact.scope.updated_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "added":{
          "departments":[{"name":"新增部门","open_department_id":"od_add","member_count":10,"create_group_chat":true,"group_chat_employee_types":[1,2]}],
          "users":[{"open_id":"on_in","name":"张三","avatar":{"avatar_72":"https://a/72.png"}}],
          "user_groups":[{"user_group_id":"ug_1","name":"用户组","type":1,"member_count":3,"status":1}]
        },
        "removed":{"users":[{"open_id":"on_out"}]}
      }
    }"#;

    const EMPLOYEE_TYPE_ENUM_CREATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c9","event_type":"contact.employee_type_enum.created_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "new_enum":{"enum_id":"enum_1","enum_value":"1","content":"正式","enum_type":1,"enum_status":1,
          "i18n_content":[{"locale":"zh_cn","value":"正式"},{"locale":"en_us","value":"Regular"}]}
      }
    }"#;

    const EMPLOYEE_TYPE_ENUM_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c10","event_type":"contact.employee_type_enum.updated_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "old_enum":{"enum_id":"enum_1","enum_value":"2","content":"旧实习生","enum_type":2,"enum_status":1,"i18n_content":[]},
        "new_enum":{"enum_id":"enum_1","enum_value":"2","content":"实习生","enum_type":2,"enum_status":1,"i18n_content":[]}
      }
    }"#;

    const EMPLOYEE_TYPE_ENUM_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c11","event_type":"contact.employee_type_enum.deleted_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"old_enum":{"enum_id":"enum_9","enum_value":"9","content":"废弃","enum_type":9,"enum_status":0,"i18n_content":[]}}
    }"#;

    const EMPLOYEE_TYPE_ENUM_ACTIVED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c12","event_type":"contact.employee_type_enum.actived_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "old_enum":{"enum_id":"enum_1","enum_status":0,"i18n_content":[]},
        "new_enum":{"enum_id":"enum_1","enum_status":1,"i18n_content":[]}
      }
    }"#;

    const EMPLOYEE_TYPE_ENUM_DEACTIVATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"c13","event_type":"contact.employee_type_enum.deactivated_v3","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "old_enum":{"enum_id":"enum_2","enum_status":1,"i18n_content":[]},
        "new_enum":{"enum_id":"enum_2","enum_status":0,"i18n_content":[]}
      }
    }"#;

    /// 分发 fixture 并把闭包断言中捕获的关键值取出（`seen` 写入 / `seen` 读出）。
    macro_rules! assert_seen {
        ($register:ident, $fixture:ident, $envelope:ty, $capture:expr, $expected:expr) => {{
            let seen = Arc::new(Mutex::new(None));
            let seen_h = Arc::clone(&seen);
            let handler = EventDispatcherHandler::builder()
                .$register(move |event: $envelope| {
                    let captured = $capture(&event);
                    *seen_h.lock().expect("lock") = captured;
                    Ok(())
                })
                .expect("register")
                .build();
            handler
                .do_without_validation($fixture.as_bytes())
                .expect("dispatch");
            assert_eq!(*seen.lock().expect("lock"), $expected);
        }};
    }

    #[test]
    fn typed_contact_user_created_deserializes_object() {
        assert_seen!(
            register_contact_user_created_v3,
            USER_CREATED_FIXTURE,
            ContactUserCreatedV3,
            |event: &ContactUserCreatedV3| {
                let obj = event.event.object.as_ref().expect("object");
                assert_eq!(event.header.event_type, "contact.user.created_v3");
                assert_eq!(obj.open_id.as_deref(), Some("on_new"));
                assert_eq!(obj.name.as_deref(), Some("张三"));
                assert_eq!(obj.status.as_ref().and_then(|s| s.is_activated), Some(true));
                assert_eq!(obj.employee_type, Some(1));
                let pos = obj.positions.first().expect("position");
                assert_eq!(pos.position_name.as_deref(), Some("工程师"));
                assert_eq!(pos.is_major, Some(true));
                let attr = obj.custom_attrs.first().expect("custom_attr");
                assert_eq!(attr.r#type.as_deref(), Some("TEXT"));
                Some(obj.department_ids.clone())
            },
            Some(vec!["od_1".to_string()])
        );
    }

    #[test]
    fn typed_contact_user_updated_deserializes_old_object() {
        assert_seen!(
            register_contact_user_updated_v3,
            USER_UPDATED_FIXTURE,
            ContactUserUpdatedV3,
            |event: &ContactUserUpdatedV3| {
                let old = event.event.old_object.as_ref().expect("old_object");
                assert_eq!(old.name.as_deref(), Some("旧名"));
                event.event.object.as_ref().expect("object").name.clone()
            },
            Some("新名".to_string())
        );
    }

    #[test]
    fn typed_contact_user_deleted_deserializes_old_department_ids() {
        assert_seen!(
            register_contact_user_deleted_v3,
            USER_DELETED_FIXTURE,
            ContactUserDeletedV3,
            |event: &ContactUserDeletedV3| {
                let old = event.event.old_object.as_ref().expect("old_object");
                Some(old.department_ids.clone())
            },
            Some(vec!["od_1".to_string(), "od_2".to_string()])
        );
    }

    #[test]
    fn typed_contact_department_created_deserializes_leaders() {
        assert_seen!(
            register_contact_department_created_v3,
            DEPARTMENT_CREATED_FIXTURE,
            ContactDepartmentCreatedV3,
            |event: &ContactDepartmentCreatedV3| {
                let obj = event.event.object.as_ref().expect("object");
                assert_eq!(obj.open_department_id.as_deref(), Some("od_1"));
                assert_eq!(obj.order, Some(1));
                assert_eq!(obj.status.as_ref().and_then(|s| s.is_deleted), Some(false));
                let leader = obj.leaders.first().expect("leader");
                assert_eq!(leader.leader_type, Some(1));
                assert_eq!(leader.leader_id.as_deref(), Some("on_l"));
                assert_eq!(
                    obj.department_hrbps
                        .first()
                        .and_then(|id| id.open_id.clone()),
                    Some("on_hrbp".to_string())
                );
                Some(obj.chat_id.clone().expect("chat_id"))
            },
            Some("oc_dept".to_string())
        );
    }

    #[test]
    fn typed_contact_department_updated_deserializes_names() {
        assert_seen!(
            register_contact_department_updated_v3,
            DEPARTMENT_UPDATED_FIXTURE,
            ContactDepartmentUpdatedV3,
            |event: &ContactDepartmentUpdatedV3| {
                let obj = event.event.object.as_ref().expect("object");
                let old = event.event.old_object.as_ref().expect("old_object");
                assert_eq!(old.name.as_deref(), Some("旧部门名"));
                Some(obj.name.clone().expect("name"))
            },
            Some("新部门名".to_string())
        );
    }

    #[test]
    fn typed_contact_department_deleted_deserializes_status() {
        assert_seen!(
            register_contact_department_deleted_v3,
            DEPARTMENT_DELETED_FIXTURE,
            ContactDepartmentDeletedV3,
            |event: &ContactDepartmentDeletedV3| {
                let old = event.event.old_object.as_ref().expect("old_object");
                assert_eq!(old.status.as_ref().and_then(|s| s.is_deleted), Some(true));
                old.open_department_id.clone()
            },
            Some("od_del".to_string())
        );
    }

    #[test]
    fn typed_contact_custom_attr_updated_deserializes_keys() {
        assert_seen!(
            register_contact_custom_attr_event_updated_v3,
            CUSTOM_ATTR_UPDATED_FIXTURE,
            ContactCustomAttrEventUpdatedV3,
            |event: &ContactCustomAttrEventUpdatedV3| {
                let obj = event.event.object.as_ref().expect("object");
                assert_eq!(obj.allow_open_query, Some(true));
                assert_eq!(
                    event
                        .event
                        .old_object
                        .as_ref()
                        .and_then(|o| o.allow_open_query),
                    Some(false)
                );
                Some(obj.contact_field_key.clone())
            },
            Some(vec!["k1".to_string(), "k2".to_string()])
        );
    }

    #[test]
    fn typed_contact_scope_updated_deserializes_added_and_removed() {
        assert_seen!(
            register_contact_scope_updated_v3,
            SCOPE_UPDATED_FIXTURE,
            ContactScopeUpdatedV3,
            |event: &ContactScopeUpdatedV3| {
                let added = event.event.added.as_ref().expect("added");
                let dept = added.departments.first().expect("department");
                assert_eq!(dept.open_department_id.as_deref(), Some("od_add"));
                assert_eq!(dept.member_count, Some(10));
                assert_eq!(dept.group_chat_employee_types, vec![1, 2]);
                assert_eq!(
                    added.users.first().and_then(|u| u.name.clone()),
                    Some("张三".to_string())
                );
                let group = added.user_groups.first().expect("user_group");
                assert_eq!(group.user_group_id.as_deref(), Some("ug_1"));
                let removed = event.event.removed.as_ref().expect("removed");
                assert_eq!(removed.users.len(), 1);
                Some(added.users.len())
            },
            Some(1_usize)
        );
    }

    #[test]
    fn typed_contact_employee_type_enum_created_deserializes_new_enum() {
        assert_seen!(
            register_contact_employee_type_enum_created_v3,
            EMPLOYEE_TYPE_ENUM_CREATED_FIXTURE,
            ContactEmployeeTypeEnumCreatedV3,
            |event: &ContactEmployeeTypeEnumCreatedV3| {
                let en = event.event.new_enum.as_ref().expect("new_enum");
                assert_eq!(en.enum_value.as_deref(), Some("1"));
                assert_eq!(en.content.as_deref(), Some("正式"));
                let i18n = en.i18n_content.first().expect("i18n");
                assert_eq!(i18n.locale.as_deref(), Some("zh_cn"));
                en.enum_id.clone()
            },
            Some("enum_1".to_string())
        );
    }

    #[test]
    fn typed_contact_employee_type_enum_updated_deserializes_both_enums() {
        assert_seen!(
            register_contact_employee_type_enum_updated_v3,
            EMPLOYEE_TYPE_ENUM_UPDATED_FIXTURE,
            ContactEmployeeTypeEnumUpdatedV3,
            |event: &ContactEmployeeTypeEnumUpdatedV3| {
                let old = event.event.old_enum.as_ref().expect("old_enum");
                assert_eq!(old.content.as_deref(), Some("旧实习生"));
                event
                    .event
                    .new_enum
                    .as_ref()
                    .and_then(|e| e.content.clone())
            },
            Some("实习生".to_string())
        );
    }

    #[test]
    fn typed_contact_employee_type_enum_deleted_deserializes_old_enum() {
        assert_seen!(
            register_contact_employee_type_enum_deleted_v3,
            EMPLOYEE_TYPE_ENUM_DELETED_FIXTURE,
            ContactEmployeeTypeEnumDeletedV3,
            |event: &ContactEmployeeTypeEnumDeletedV3| {
                event
                    .event
                    .old_enum
                    .as_ref()
                    .and_then(|e| e.enum_id.clone())
            },
            Some("enum_9".to_string())
        );
    }

    #[test]
    fn typed_contact_employee_type_enum_actived_deserializes_status() {
        assert_seen!(
            register_contact_employee_type_enum_actived_v3,
            EMPLOYEE_TYPE_ENUM_ACTIVED_FIXTURE,
            ContactEmployeeTypeEnumActivedV3,
            |event: &ContactEmployeeTypeEnumActivedV3| {
                assert_eq!(
                    event.event.old_enum.as_ref().and_then(|e| e.enum_status),
                    Some(0)
                );
                event.event.new_enum.as_ref().and_then(|e| e.enum_status)
            },
            Some(1)
        );
    }

    #[test]
    fn typed_contact_employee_type_enum_deactivated_deserializes_status() {
        assert_seen!(
            register_contact_employee_type_enum_deactivated_v3,
            EMPLOYEE_TYPE_ENUM_DEACTIVATED_FIXTURE,
            ContactEmployeeTypeEnumDeactivatedV3,
            |event: &ContactEmployeeTypeEnumDeactivatedV3| {
                assert_eq!(
                    event.event.old_enum.as_ref().and_then(|e| e.enum_status),
                    Some(1)
                );
                event.event.new_enum.as_ref().and_then(|e| e.enum_status)
            },
            Some(0)
        );
    }

    /// 事件串常量与票面清单逐字一致。
    #[test]
    fn typed_contact_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::CONTACT_USER_CREATED_V3,
            "contact.user.created_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_USER_UPDATED_V3,
            "contact.user.updated_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_USER_DELETED_V3,
            "contact.user.deleted_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_DEPARTMENT_CREATED_V3,
            "contact.department.created_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_DEPARTMENT_UPDATED_V3,
            "contact.department.updated_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_DEPARTMENT_DELETED_V3,
            "contact.department.deleted_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_CUSTOM_ATTR_EVENT_UPDATED_V3,
            "contact.custom_attr_event.updated_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_SCOPE_UPDATED_V3,
            "contact.scope.updated_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_EMPLOYEE_TYPE_ENUM_CREATED_V3,
            "contact.employee_type_enum.created_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_EMPLOYEE_TYPE_ENUM_UPDATED_V3,
            "contact.employee_type_enum.updated_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_EMPLOYEE_TYPE_ENUM_DELETED_V3,
            "contact.employee_type_enum.deleted_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_EMPLOYEE_TYPE_ENUM_ACTIVED_V3,
            "contact.employee_type_enum.actived_v3"
        );
        assert_eq!(
            EventDispatcherHandler::CONTACT_EMPLOYEE_TYPE_ENUM_DEACTIVATED_V3,
            "contact.employee_type_enum.deactivated_v3"
        );
    }

    /// typed 注册后再 `register_raw` 同 key 返回 Err；反方向同理。
    #[test]
    fn typed_contact_duplicate_register_fails() {
        struct Noop;
        impl crate::ws_client::EventHandler for Noop {
            fn handle(
                &self,
                _payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                Ok(())
            }
        }
        let err = EventDispatcherHandler::builder()
            .register_contact_user_created_v3(|_event: ContactUserCreatedV3| Ok(()))
            .expect("first")
            .register_raw("contact.user.created_v3", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("contact.scope.updated_v3", Noop)
            .expect("raw first")
            .register_contact_scope_updated_v3(|_event: ContactScopeUpdatedV3| Ok(()))
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }
}
