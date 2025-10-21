use serde::{Deserialize, Serialize};

// 导出所有请求/响应结构体
pub use crate::service::contact::v3::{
    custom_attr::*, department::*, employee_type_enum::*, functional_role::*,
    functional_role_member::*, group::*, group_member::*, job_family::*, job_level::*,
    job_title::*, scope::*, unit::*, user::*, work_city::*,
};

/// 用户信息
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
            }),
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
        };
        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&serialized).unwrap();
        assert_eq!(user.user_id, deserialized.user_id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.email, deserialized.email);
        assert_eq!(user.mobile, deserialized.mobile);
        assert_eq!(user.gender, deserialized.gender);
    }

    #[test]
    fn test_user_default() {
        let user = User::default();
        assert_eq!(user.user_id, None);
        assert_eq!(user.name, None);
        assert_eq!(user.email, None);
        assert_eq!(user.mobile, None);
        assert_eq!(user.gender, None);
    }

    #[test]
    fn test_user_minimal_data() {
        let user = User {
            user_id: Some("minimal_user".to_string()),
            name: Some("最小用户".to_string()),
            ..Default::default()
        };
        let serialized = serde_json::to_string(&user).unwrap();
        assert!(!serialized.contains("email"));
        assert!(!serialized.contains("mobile"));
        assert!(!serialized.contains("gender"));
        let deserialized: User = serde_json::from_str(&serialized).unwrap();
        assert_eq!(user.user_id, deserialized.user_id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.email, deserialized.email);
    }

    #[test]
    fn test_avatar_serialization() {
        let avatar = Avatar {
            avatar_72: Some("https://example.com/avatar_72.jpg".to_string()),
            avatar_240: Some("https://example.com/avatar_240.jpg".to_string()),
            avatar_640: Some("https://example.com/avatar_640.jpg".to_string()),
            avatar_origin: Some("https://example.com/avatar_origin.jpg".to_string()),
        };
        let serialized = serde_json::to_string(&avatar).unwrap();
        let deserialized: Avatar = serde_json::from_str(&serialized).unwrap();
        assert_eq!(avatar.avatar_72, deserialized.avatar_72);
        assert_eq!(avatar.avatar_240, deserialized.avatar_240);
        assert_eq!(avatar.avatar_640, deserialized.avatar_640);
        assert_eq!(avatar.avatar_origin, deserialized.avatar_origin);
    }

    #[test]
    fn test_avatar_with_none_values() {
        let avatar = Avatar {
            avatar_72: Some("avatar72.jpg".to_string()),
            avatar_240: None,
            avatar_640: None,
            avatar_origin: None,
        };
        let serialized = serde_json::to_string(&avatar).unwrap();
        assert!(!serialized.contains("avatar_240"));
        assert!(!serialized.contains("avatar_640"));
        assert!(!serialized.contains("avatar_origin"));
        let deserialized: Avatar = serde_json::from_str(&serialized).unwrap();
        assert_eq!(avatar.avatar_72, deserialized.avatar_72);
        assert_eq!(avatar.avatar_240, deserialized.avatar_240);
    }

    #[test]
    fn test_user_status_serialization() {
        let status = UserStatus {
            is_frozen: Some(false),
            is_resigned: Some(true),
            is_activated: Some(true),
            is_exited: Some(false),
            is_unjoin: Some(false),
        };
        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: UserStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status.is_frozen, deserialized.is_frozen);
        assert_eq!(status.is_resigned, deserialized.is_resigned);
        assert_eq!(status.is_activated, deserialized.is_activated);
        assert_eq!(status.is_exited, deserialized.is_exited);
        assert_eq!(status.is_unjoin, deserialized.is_unjoin);
    }

    #[test]
    fn test_user_custom_attr_serialization() {
        let custom_attr = UserCustomAttr {
            r#type: Some("number".to_string()),
            id: Some("attr_123".to_string()),
            value: Some(serde_json::Value::Number(serde_json::Number::from(42))),
        };
        let serialized = serde_json::to_string(&custom_attr).unwrap();
        let deserialized: UserCustomAttr = serde_json::from_str(&serialized).unwrap();
        assert_eq!(custom_attr.r#type, deserialized.r#type);
        assert_eq!(custom_attr.id, deserialized.id);
        assert_eq!(custom_attr.value, deserialized.value);
    }

    #[test]
    fn test_department_serialization() {
        let department = Department {
            name: Some("技术部".to_string()),
            i18n_name: Some(I18nName {
                zh_cn: Some("技术部".to_string()),
                en_us: Some("Technology Department".to_string()),
                ja_jp: Some("技術部".to_string()),
            }),
            parent_department_id: Some("parent_dept_123".to_string()),
            department_id: Some("dept_456".to_string()),
            status: Some(DepartmentStatus {
                is_deleted: Some(false),
            }),
            leaders: Some(vec!["leader_1".to_string(), "leader_2".to_string()]),
            chat_id: Some("chat_789".to_string()),
            order: Some("100".to_string()),
            unit_ids: Some(vec!["unit_1".to_string()]),
            member_count: Some(25),
            create_group_chat: Some(true),
        };
        let serialized = serde_json::to_string(&department).unwrap();
        let deserialized: Department = serde_json::from_str(&serialized).unwrap();
        assert_eq!(department.name, deserialized.name);
        assert_eq!(department.department_id, deserialized.department_id);
        assert_eq!(department.member_count, deserialized.member_count);
    }

    #[test]
    fn test_i18n_name_serialization() {
        let i18n_name = I18nName {
            zh_cn: Some("中文名称".to_string()),
            en_us: Some("English Name".to_string()),
            ja_jp: Some("日本語名前".to_string()),
        };
        let serialized = serde_json::to_string(&i18n_name).unwrap();
        let deserialized: I18nName = serde_json::from_str(&serialized).unwrap();
        assert_eq!(i18n_name.zh_cn, deserialized.zh_cn);
        assert_eq!(i18n_name.en_us, deserialized.en_us);
        assert_eq!(i18n_name.ja_jp, deserialized.ja_jp);
    }

    #[test]
    fn test_department_status_serialization() {
        let status = DepartmentStatus {
            is_deleted: Some(true),
        };
        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: DepartmentStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status.is_deleted, deserialized.is_deleted);
    }

    #[test]
    fn test_group_serialization() {
        let group = Group {
            id: Some("group_123".to_string()),
            name: Some("开发组".to_string()),
            description: Some("负责产品开发的用户组".to_string()),
            member_user_count: Some(15),
            member_department_count: Some(3),
            r#type: Some(1),
        };
        let serialized = serde_json::to_string(&group).unwrap();
        let deserialized: Group = serde_json::from_str(&serialized).unwrap();
        assert_eq!(group.id, deserialized.id);
        assert_eq!(group.name, deserialized.name);
        assert_eq!(group.description, deserialized.description);
        assert_eq!(group.member_user_count, deserialized.member_user_count);
    }

    #[test]
    fn test_group_member_serialization() {
        let member = GroupMember {
            member_id: Some("member_123".to_string()),
            member_type: Some("user".to_string()),
            member_id_type: Some("user_id".to_string()),
        };
        let serialized = serde_json::to_string(&member).unwrap();
        let deserialized: GroupMember = serde_json::from_str(&serialized).unwrap();
        assert_eq!(member.member_id, deserialized.member_id);
        assert_eq!(member.member_type, deserialized.member_type);
        assert_eq!(member.member_id_type, deserialized.member_id_type);
    }

    #[test]
    fn test_group_member_info_serialization() {
        let member_info = GroupMemberInfo {
            member_id: "required_member_123".to_string(),
            member_type: Some("user".to_string()),
            member_id_type: Some("open_id".to_string()),
        };
        let serialized = serde_json::to_string(&member_info).unwrap();
        let deserialized: GroupMemberInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(member_info.member_id, deserialized.member_id);
        assert_eq!(member_info.member_type, deserialized.member_type);
        assert_eq!(member_info.member_id_type, deserialized.member_id_type);
    }

    #[test]
    fn test_group_member_result_serialization() {
        let result = GroupMemberResult {
            member_id: Some("member_456".to_string()),
            code: Some(0),
            msg: Some("操作成功".to_string()),
        };
        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: GroupMemberResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result.member_id, deserialized.member_id);
        assert_eq!(result.code, deserialized.code);
        assert_eq!(result.msg, deserialized.msg);
    }

    #[test]
    fn test_custom_attr_serialization() {
        let custom_attr = CustomAttr {
            id: Some("custom_field_1".to_string()),
            r#type: Some("select".to_string()),
            name: Some(I18nName {
                zh_cn: Some("自定义字段".to_string()),
                en_us: Some("Custom Field".to_string()),
                ja_jp: None,
            }),
            description: Some(I18nName {
                zh_cn: Some("字段描述".to_string()),
                en_us: Some("Field Description".to_string()),
                ja_jp: None,
            }),
            is_required: Some(true),
            options: Some(serde_json::json!({"choices": ["选项1", "选项2"]})),
        };
        let serialized = serde_json::to_string(&custom_attr).unwrap();
        let deserialized: CustomAttr = serde_json::from_str(&serialized).unwrap();
        assert_eq!(custom_attr.id, deserialized.id);
        assert_eq!(custom_attr.r#type, deserialized.r#type);
        assert_eq!(custom_attr.is_required, deserialized.is_required);
    }

    #[test]
    fn test_employee_type_enum_serialization() {
        let employee_type = EmployeeTypeEnum {
            enum_id: Some("type_1".to_string()),
            enum_value: Some("1".to_string()),
            content: Some("正式员工".to_string()),
            enum_type: Some(1),
            enum_status: Some(1),
            i18n_content: Some(vec![
                I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("正式员工".to_string()),
                },
                I18nContent {
                    locale: Some("en_us".to_string()),
                    value: Some("Full-time Employee".to_string()),
                },
            ]),
        };
        let serialized = serde_json::to_string(&employee_type).unwrap();
        let deserialized: EmployeeTypeEnum = serde_json::from_str(&serialized).unwrap();
        assert_eq!(employee_type.enum_id, deserialized.enum_id);
        assert_eq!(employee_type.enum_value, deserialized.enum_value);
        assert_eq!(employee_type.content, deserialized.content);
    }

    #[test]
    fn test_i18n_content_serialization() {
        let content = I18nContent {
            locale: Some("zh_cn".to_string()),
            value: Some("中文内容".to_string()),
        };
        let serialized = serde_json::to_string(&content).unwrap();
        let deserialized: I18nContent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(content.locale, deserialized.locale);
        assert_eq!(content.value, deserialized.value);
    }

    #[test]
    fn test_unit_serialization() {
        let unit = Unit {
            unit_id: Some("unit_123".to_string()),
            name: Some("技术中心".to_string()),
            unit_type: Some("center".to_string()),
        };
        let serialized = serde_json::to_string(&unit).unwrap();
        let deserialized: Unit = serde_json::from_str(&serialized).unwrap();
        assert_eq!(unit.unit_id, deserialized.unit_id);
        assert_eq!(unit.name, deserialized.name);
        assert_eq!(unit.unit_type, deserialized.unit_type);
    }

    #[test]
    fn test_role_member_info_serialization() {
        let role_member = RoleMemberInfo {
            user_id: "role_user_123".to_string(),
            scope_type: Some("department".to_string()),
            scope_ids: Some(vec!["dept_1".to_string(), "dept_2".to_string()]),
        };
        let serialized = serde_json::to_string(&role_member).unwrap();
        let deserialized: RoleMemberInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(role_member.user_id, deserialized.user_id);
        assert_eq!(role_member.scope_type, deserialized.scope_type);
        assert_eq!(role_member.scope_ids, deserialized.scope_ids);
    }

    #[test]
    fn test_role_member_serialization() {
        let role_member = RoleMember {
            user_id: Some("user_789".to_string()),
            scope_type: Some("all".to_string()),
            scope_ids: None,
        };
        let serialized = serde_json::to_string(&role_member).unwrap();
        assert!(!serialized.contains("scope_ids"));
        let deserialized: RoleMember = serde_json::from_str(&serialized).unwrap();
        assert_eq!(role_member.user_id, deserialized.user_id);
        assert_eq!(role_member.scope_type, deserialized.scope_type);
        assert_eq!(role_member.scope_ids, deserialized.scope_ids);
    }

    #[test]
    fn test_role_member_scope_serialization() {
        let scope = RoleMemberScope {
            user_id: "scope_user_456".to_string(),
            scope_type: Some("department".to_string()),
            scope_ids: Some(vec!["dept_tech".to_string()]),
        };
        let serialized = serde_json::to_string(&scope).unwrap();
        let deserialized: RoleMemberScope = serde_json::from_str(&serialized).unwrap();
        assert_eq!(scope.user_id, deserialized.user_id);
        assert_eq!(scope.scope_type, deserialized.scope_type);
        assert_eq!(scope.scope_ids, deserialized.scope_ids);
    }

    #[test]
    fn test_role_member_result_serialization() {
        let result = RoleMemberResult {
            user_id: Some("result_user_123".to_string()),
            code: Some(200),
            msg: Some("角色分配成功".to_string()),
        };
        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: RoleMemberResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result.user_id, deserialized.user_id);
        assert_eq!(result.code, deserialized.code);
        assert_eq!(result.msg, deserialized.msg);
    }

    #[test]
    fn test_job_level_serialization() {
        let job_level = JobLevel {
            job_level_id: Some("level_senior".to_string()),
            name: Some(vec![
                I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("高级".to_string()),
                },
                I18nContent {
                    locale: Some("en_us".to_string()),
                    value: Some("Senior".to_string()),
                },
            ]),
            description: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("高级职级".to_string()),
            }]),
            status: Some(true),
            rank: Some(5),
        };
        let serialized = serde_json::to_string(&job_level).unwrap();
        let deserialized: JobLevel = serde_json::from_str(&serialized).unwrap();
        assert_eq!(job_level.job_level_id, deserialized.job_level_id);
        assert_eq!(job_level.status, deserialized.status);
        assert_eq!(job_level.rank, deserialized.rank);
    }

    #[test]
    fn test_job_family_serialization() {
        let job_family = JobFamily {
            job_family_id: Some("family_tech".to_string()),
            name: Some(vec![
                I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("技术序列".to_string()),
                },
                I18nContent {
                    locale: Some("en_us".to_string()),
                    value: Some("Technology Track".to_string()),
                },
            ]),
            description: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("技术发展序列".to_string()),
            }]),
            status: Some(true),
        };
        let serialized = serde_json::to_string(&job_family).unwrap();
        let deserialized: JobFamily = serde_json::from_str(&serialized).unwrap();
        assert_eq!(job_family.job_family_id, deserialized.job_family_id);
        assert_eq!(job_family.status, deserialized.status);
    }

    #[test]
    fn test_job_title_serialization() {
        let job_title = JobTitle {
            job_title_id: Some("title_engineer".to_string()),
            name: Some(vec![
                I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("软件工程师".to_string()),
                },
                I18nContent {
                    locale: Some("en_us".to_string()),
                    value: Some("Software Engineer".to_string()),
                },
            ]),
            status: Some(true),
        };
        let serialized = serde_json::to_string(&job_title).unwrap();
        let deserialized: JobTitle = serde_json::from_str(&serialized).unwrap();
        assert_eq!(job_title.job_title_id, deserialized.job_title_id);
        assert_eq!(job_title.status, deserialized.status);
    }

    #[test]
    fn test_work_city_serialization() {
        let work_city = WorkCity {
            work_city_id: Some("city_beijing".to_string()),
            name: Some(vec![
                I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("北京".to_string()),
                },
                I18nContent {
                    locale: Some("en_us".to_string()),
                    value: Some("Beijing".to_string()),
                },
            ]),
            status: Some(true),
        };
        let serialized = serde_json::to_string(&work_city).unwrap();
        let deserialized: WorkCity = serde_json::from_str(&serialized).unwrap();
        assert_eq!(work_city.work_city_id, deserialized.work_city_id);
        assert_eq!(work_city.status, deserialized.status);
    }

    #[test]
    fn test_work_city_with_none_values() {
        let work_city = WorkCity {
            work_city_id: Some("city_minimal".to_string()),
            name: None,
            status: None,
        };
        let serialized = serde_json::to_string(&work_city).unwrap();
        assert!(!serialized.contains("name"));
        assert!(!serialized.contains("status"));
        let deserialized: WorkCity = serde_json::from_str(&serialized).unwrap();
        assert_eq!(work_city.work_city_id, deserialized.work_city_id);
        assert_eq!(work_city.name, deserialized.name);
        assert_eq!(work_city.status, deserialized.status);
    }
}
