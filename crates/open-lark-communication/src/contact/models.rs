use serde::{Deserialize, Serialize },

// 导出所有请求/响应结构体
pub use crate::contact::v3::{
    custom_attr::*, department::*, employee_type_enum::*, functional_role::*,
    functional_role_member::*, group::*, group_member::*, job_family::*, job_level::*,
    job_title::*, scope::*, unit::*, user::*, work_city::*,
},
/// 用户信息
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    pub name: Option<String>,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 电话号码
    pub mobile_visible: Option<bool>,
    /// 性别
    pub gender: Option<i32>,
    /// 头像
    pub avatar: Option<Avatar>,
    /// 状态
    pub status: Option<UserStatus>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 直属上级用户ID
    pub leader_user_id: Option<String>,
    /// 城市
    pub city: Option<String>,
    /// 国家
    pub country: Option<String>,
    /// 工位
    pub work_station: Option<String>,
    /// 入职时间
    pub join_time: Option<i64>,
    /// 离职时间
    pub leave_time: Option<i64>,
    /// 员工编号
    pub employee_no: Option<String>,
    /// 员工类型
    pub employee_type: Option<i32>,
    /// 职务
    pub job_title: Option<String>,
    /// 是否是租户超级管理员
    pub is_tenant_manager: Option<bool>,
    /// 自定义字段
    pub custom_attrs: Option<Vec<UserCustomAttr>>,
    /// 企业邮箱
    pub enterprise_email: Option<String>,
    /// 时区
    pub timezone: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 职级ID
    pub job_level_id: Option<String>,
    /// 序列ID
    pub job_family_id: Option<String>,
    /// 工作城市ID
    pub work_city: Option<String>,
}
/// 头像信息
pub struct Avatar {
    /// 头像URL
    pub avatar_72: Option<String>,
    pub avatar_240: Option<String>,
    pub avatar_640: Option<String>,
    /// 头像原图URL
    pub avatar_origin: Option<String>,
/// 用户状态
pub struct UserStatus {
    /// 是否冻结
    pub is_frozen: Option<bool>,
    /// 是否离职
    pub is_resigned: Option<bool>,
    /// 是否激活
    pub is_activated: Option<bool>,
    /// 是否主动离职
    pub is_exited: Option<bool>,
    /// 是否unjoin
    pub is_unjoin: Option<bool>,
/// 用户自定义字段
pub struct UserCustomAttr {
    /// 字段类型
    pub r#type: Option<String>,
    /// 字段ID
    pub id: Option<String>,
    /// 字段值
    pub value: Option<serde_json::Value>,
/// 部门信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Department {
    /// 部门名称
    /// 国际化部门名称
    pub i18n_name: Option<I18nName>,
    /// 父部门ID
    pub parent_department_id: Option<String>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 部门状态
    pub status: Option<DepartmentStatus>,
    /// 部门负责人
    pub leaders: Option<Vec<String>>,
    /// 部门群ID
    pub chat_id: Option<String>,
    /// 排序
    pub order: Option<String>,
    /// 单位绑定
    pub unit_ids: Option<Vec<String>>,
    /// 成员数量
    pub member_count: Option<i32>,
    /// 创建组织架构
    pub create_group_chat: Option<bool>,
/// 国际化名称
pub struct I18nName {
    /// 中文名
    pub zh_cn: Option<String>,
    pub en_us: Option<String>,
    /// 日文名
    pub ja_jp: Option<String>,
/// 部门状态
pub struct DepartmentStatus {
    /// 是否被删除
    pub is_deleted: Option<bool>,
/// 用户组信息
pub struct Group {
    /// 用户组ID
    /// 用户组名称
    /// 用户组描述
    /// 用户组成员数量
    pub member_user_count: Option<i32>,
    /// 用户组部门数量
    pub member_department_count: Option<i32>,
    /// 用户组类型
    pub r#type: Option<i32>,
/// 用户组成员
pub struct GroupMember {
    /// 成员ID
    pub member_id: Option<String>,
    /// 成员类型
    pub member_type: Option<String>,
    /// 成员ID类型
    pub member_id_type: Option<String>,
/// 用户组成员信息
pub struct GroupMemberInfo {
    pub member_id: String,
/// 用户组成员操作结果
pub struct GroupMemberResult {
    /// 操作结果码
    pub code: Option<i32>,
    /// 操作结果消息
    pub msg: Option<String>,
/// 自定义字段
pub struct CustomAttr {
    /// 字段名称
    pub name: Option<I18nName>,
    /// 字段描述
    pub description: Option<I18nName>,
    /// 是否必填
    pub is_required: Option<bool>,
    /// 字段配置
    pub options: Option<serde_json::Value>,
/// 人员类型枚举
pub struct EmployeeTypeEnum {
    /// 枚举ID
    pub enum_id: Option<String>,
    /// 枚举值
    pub enum_value: Option<String>,
    /// 枚举内容
    pub content: Option<String>,
    /// 枚举类型
    pub enum_type: Option<i32>,
    /// 枚举状态
    pub enum_status: Option<i32>,
    /// 国际化内容
    pub i18n_content: Option<Vec<I18nContent>>,
/// 国际化内容
pub struct I18nContent {
    /// 语言代码
    pub locale: Option<String>,
    /// 内容值
    pub value: Option<String>,
/// 单位信息
pub struct Unit {
    /// 单位ID
    pub unit_id: Option<String>,
    /// 单位名称
    /// 单位类型
    pub unit_type: Option<String>,
/// 角色成员信息
pub struct RoleMemberInfo {
    pub user_id: String,
    /// 管理范围
    pub scope_type: Option<String>,
    /// 管理范围ID列表
    pub scope_ids: Option<Vec<String>>,
/// 角色成员
pub struct RoleMember {
    /// 管理范围类型
/// 角色成员管理范围
pub struct RoleMemberScope {
/// 角色成员操作结果
pub struct RoleMemberResult {
/// 职级信息
pub struct JobLevel {
    /// 职级名称
    pub name: Option<Vec<I18nContent>>,
    /// 职级描述
    pub description: Option<Vec<I18nContent>>,
    /// 职级状态
    pub status: Option<bool>,
    /// 职级等级
    pub rank: Option<i32>,
/// 序列信息
pub struct JobFamily {
    /// 序列名称
    /// 序列描述
    /// 序列状态
/// 职务信息
pub struct JobTitle {
    /// 职务ID
    pub job_title_id: Option<String>,
    /// 职务名称
    /// 职务状态
/// 工作城市信息
pub struct WorkCity {
    pub work_city_id: Option<String>,
    /// 工作城市名称
    /// 工作城市状态
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_user_serialization() {
        let user = User {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
            en_name: Some("Zhang San".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            mobile: Some("+86-13812345678".to_string()),
            mobile_visible: Some(true),
            gender: Some(1),
            avatar: Some(Avatar {
                avatar_72: Some("avatar72.jpg".to_string()),
                avatar_240: Some("avatar240.jpg".to_string()),
                avatar_640: Some("avatar640.jpg".to_string()),
                avatar_origin: Some("avatar_origin.jpg".to_string()),
            }),
            status: Some(UserStatus {
                is_frozen: Some(false),
                is_resigned: Some(false),
                is_activated: Some(true),
                is_exited: Some(false),
                is_unjoin: Some(false),
            department_ids: Some(vec!["dept_1".to_string(), "dept_2".to_string()]),
            leader_user_id: Some("leader_456".to_string()),
            city: Some("北京".to_string()),
            country: Some("中国".to_string()),
            work_station: Some("A座101".to_string()),
            join_time: Some(1640995200),
            leave_time: None,
            employee_no: Some("EMP001".to_string()),
            employee_type: Some(1),
            job_title: Some("高级工程师".to_string()),
            is_tenant_manager: Some(false),
            custom_attrs: Some(vec![UserCustomAttr {
                r#type: Some("text".to_string()),
                id: Some("custom_1".to_string()),
                value: Some(serde_json::Value::String("自定义值".to_string())),
            }]),
            enterprise_email: Some("zhangsan@company.com".to_string()),
            timezone: Some("Asia/Shanghai".to_string()),
            description: Some("用户描述".to_string()),
            job_level_id: Some("level_5".to_string()),
            job_family_id: Some("family_tech".to_string()),
            work_city: Some("beijing".to_string()),
        },
        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&serialized).unwrap();
        assert_eq!(user.user_id, deserialized.user_id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.email, deserialized.email);
        assert_eq!(user.mobile, deserialized.mobile);
        assert_eq!(user.gender, deserialized.gender);
    }
