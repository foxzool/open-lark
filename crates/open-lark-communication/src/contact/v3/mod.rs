use open_lark_core::core::{config::Config, trait_system::Service },
use std::sync::Arc;

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
/// 联系人 v3 API 服务
///
/// 提供完整的企业组织架构和人员管理功能，支持用户、部门、角色等全方位管理。
/// 为企业提供完整的组织架构解决方案，包括员工信息、权限管理和组织结构维护。
/// # 主要功能
/// ## 用户管理
/// - 👤 **用户服务**: 员工基本信息的创建、更新、查询
/// - 🏢 **用户组服务**: 员工分组管理，支持项目组、部门组等
/// - 🔧 **自定义字段**: 灵活的员工信息扩展字段管理
/// - 📋 **员工类型**: 全职、兼职、实习生等员工类型管理
/// ## 组织架构
/// - 🏗️ **部门管理**: 部门层级结构创建和维护
/// - 🏢 **单位管理**: 公司、子公司等法人单位管理
/// - 👥 **组成员**: 用户组成员的添加、删除、查询
/// ## 角色管理
/// - 🎭 **角色定义**: 自定义角色的创建和管理
/// - 🔗 **角色成员**: 角色成员的分配和管理
/// - 🏆 **职级体系**: 职级、序列、职务的层级管理
/// ## 工作地点
/// - 🌍 **工作城市**: 员工工作地点的管理和设置
/// - 📍 **位置权限**: 基于地理位置的权限控制
/// # 使用场景
/// - 🏢 **企业组织管理**: 完整的公司组织架构维护
/// - 👥 **员工生命周期**: 入职、调岗、离职的全流程管理
/// - 🎭 **权限管理**: 基于角色和部门的权限分配
/// - 📊 **组织分析**: 组织架构的统计和分析
pub struct V3 {
    /// 权限范围服务
    ///
    /// 管理API访问权限范围和授权控制。
    /// 提供细粒度的权限管理功能。
    pub scope: ScopeService,
    /// 用户管理服务
    /// 负责员工的基本信息管理，包括个人资料、联系方式等。
    /// 支持员工的创建、更新、查询和删除操作。
    pub user: UserService,
    /// 用户组服务
    /// 管理员工分组，支持项目组、部门组等多种分组方式。
    /// 提供灵活的员工组织和管理功能。
    pub group: GroupService,
    /// 自定义用户字段服务
    /// 管理员工的扩展信息字段，支持自定义字段类型。
    /// 提供灵活的员工信息扩展能力。
    pub custom_attr: CustomAttrService,
    /// 人员类型服务
    /// 管理员工类型分类，如全职、兼职、实习生、外包等。
    /// 支持多种用工形式的管理。
    pub employee_type_enum: EmployeeTypeEnumService,
    /// 部门管理服务
    /// 管理公司的部门层级结构，支持多级部门管理。
    /// 提供完整的组织架构维护功能。
    pub department: DepartmentService,
    /// 单位管理服务
    /// 管理法人单位，如总公司、子公司、分公司等。
    /// 支持复杂的企业实体结构管理。
    pub unit: UnitService,
    /// 用户组成员服务
    /// 管理用户组的成员关系，支持成员的添加和移除。
    /// 提供用户组成员的细粒度管理。
    pub group_member: GroupMemberService,
    /// 角色管理服务
    /// 管理自定义角色定义，支持基于角色的权限控制。
    /// 提供灵活的角色管理体系。
    pub functional_role: FunctionalRoleService,
    /// 角色成员服务
    /// 管理角色的成员分配，支持角色成员的增删改查。
    /// 提供角色成员的统一管理。
    pub functional_role_member: FunctionalRoleMemberService,
    /// 职级管理服务
    /// 管理员工的职级体系，支持职级的层级管理。
    /// 提供职级的定义和分配功能。
    pub job_level: JobLevelService,
    /// 序列管理服务
    /// 管理职业序列，如技术序列、管理序列等。
    /// 支持职业发展路径的管理。
    pub job_family: JobFamilyService,
    /// 职务服务
    /// 管理员工职务信息，支持职务的层级和分类管理。
    /// 提供职务的标准化管理。
    pub job_title: JobTitleService,
    /// 工作城市服务
    /// 管理员工的工作地点，支持多地点办公管理。
    /// 提供地理位置的组织管理能力。
    pub work_city: WorkCityService,
}
impl V3 {
    /// 创建新的联系人 v3 服务实例
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    /// # 返回值
    /// 配置完成的 V3 服务实例，包含所有联系人相关子服务
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
    /// 验证所有联系人服务配置的一致性
    /// 检查主要服务的配置是否有效，确保服务间的协调工作。
