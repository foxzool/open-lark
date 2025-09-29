/// Contact v3 API 实现
use crate::core::config::Config;

pub mod custom_attr;
pub mod department;
pub mod employee_type_enum;
pub mod functional_role;
pub mod functional_role_member;
pub mod group;
pub mod group_member;
pub mod job_family;
pub mod job_level;
pub mod job_title;
pub mod scope;
pub mod unit;
pub mod user;
pub mod work_city;

// Contact v3 事件模块
pub mod p2_contact_department_created_v3;
pub mod p2_contact_department_deleted_v3;
pub mod p2_contact_department_updated_v3;
pub mod p2_contact_user_created_v3;
pub mod p2_contact_user_deleted_v3;
pub mod p2_contact_user_updated_v3;

pub use custom_attr::CustomAttrService;
pub use department::DepartmentService;
pub use employee_type_enum::EmployeeTypeEnumService;
pub use functional_role::FunctionalRoleService;
pub use functional_role_member::FunctionalRoleMemberService;
pub use group::GroupService;
pub use group_member::GroupMemberService;
pub use job_family::JobFamilyService;
pub use job_level::JobLevelService;
pub use job_title::JobTitleService;
pub use scope::ScopeService;
pub use unit::UnitService;
pub use user::UserService;
pub use work_city::WorkCityService;

/// Contact v3 API 版本服务
pub struct V3 {
    /// 权限范围服务
    pub scope: ScopeService,
    /// 用户管理服务
    pub user: UserService,
    /// 用户组服务
    pub group: GroupService,
    /// 自定义用户字段服务
    pub custom_attr: CustomAttrService,
    /// 人员类型服务
    pub employee_type_enum: EmployeeTypeEnumService,
    /// 部门管理服务
    pub department: DepartmentService,
    /// 单位管理服务
    pub unit: UnitService,
    /// 用户组成员服务
    pub group_member: GroupMemberService,
    /// 角色管理服务
    pub functional_role: FunctionalRoleService,
    /// 角色成员服务
    pub functional_role_member: FunctionalRoleMemberService,
    /// 职级管理服务
    pub job_level: JobLevelService,
    /// 序列管理服务
    pub job_family: JobFamilyService,
    /// 职务服务
    pub job_title: JobTitleService,
    /// 工作城市服务
    pub work_city: WorkCityService,
}

impl V3 {
    pub fn new(config: Config) -> Self {
        Self {
            scope: ScopeService::new(config.clone()),
            user: UserService::new(config.clone()),
            group: GroupService::new(config.clone()),
            custom_attr: CustomAttrService::new(config.clone()),
            employee_type_enum: EmployeeTypeEnumService::new(config.clone()),
            department: DepartmentService::new(config.clone()),
            unit: UnitService::new(config.clone()),
            group_member: GroupMemberService::new(config.clone()),
            functional_role: FunctionalRoleService::new(config.clone()),
            functional_role_member: FunctionalRoleMemberService::new(config.clone()),
            job_level: JobLevelService::new(config.clone()),
            job_family: JobFamilyService::new(config.clone()),
            job_title: JobTitleService::new(config.clone()),
            work_city: WorkCityService::new(config),
        }
    }
}
