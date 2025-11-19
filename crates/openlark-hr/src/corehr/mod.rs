//! 飞书人事企业版（CoreHR）服务
//!
//! 提供飞书人事企业版的完整功能集，支持员工管理、组织架构、基础数据、
//! 岗职务管理、生命周期管理等企业级人力资源管理能力。是大型企业人事系统的核心基础。
//!
//! # 核心功能
//!
//! ## 员工信息管理
//! - 👤 员工基本信息管理
//! - 💼 雇佣信息和合同管理
//! - 📋 任职信息和岗位管理
//! - 🔍 员工搜索和筛选
//! - 📊 员工数据统计分析
//!
//! ## 组织架构管理
//! - 🏢 公司和部门管理
//! - 🌳 组织架构树构建
//! - 👑 管理层级关系设置
//! - 📅 组织变更历史追踪
//! - ⚙️ 组织配置和规则管理
//!
//! ## 基础数据管理
//! - 📋 标准化枚举信息管理
//! - 🌍 地理位置信息查询
//! - 🆔 员工ID类型转换
//! - 🏳️ 国籍和语言信息
//! - 📖 数据字典维护
//!
//! ## 岗职务管理
//! - 💼 职位序列和职级管理
//! - 📈 职业发展路径设计
//! - 🎯 岗位职责和要求定义
//! - 📊 岗位评估和分级
//! - 🔄 岗位调整和变更
//!
//! ## 生命周期管理
//! - 🚪 入职流程和手续办理
//! - 🔄 异动申请和审批
//! - 📝 离职流程和交接
//! - ⏰ 试用期管理和转正
//! - 📋 流程节点跟踪
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取人事企业版服务
//! let corehr = &client.corehr;
//!
//! // 查询员工信息
//! // let search_request = SearchEmployeeRequest::builder()
//! //     .employment_status(vec!["active"])
//! //     .fields(vec!["person", "employment", "job_datas"])
//! //     .page_size(50)
//! //     .build();
//! // let employees = corehr.employee.search(search_request, None).await?;
//!
//! // 创建部门
//! // let dept_request = CreateDepartmentRequest::builder()
//! //     .name("技术研发部")
//! //     .parent_department_id("parent_dept_123")
//! //     .manager("manager_456")
//! //     .effective_time("2024-01-01")
//! //     .build();
//! // let department = corehr.organization.create_department(dept_request, None).await?;
//!
//! // 查询基础枚举信息
//! // let enum_request = SearchEnumRequest::builder()
//! //     .enum_type("gender")
//! //     .page_size(50)
//! //     .build();
//! // let enums = corehr.basic_info.search_enum(enum_request, None).await?;
//!
//! // 管理员工生命周期
//! // let onboard_request = CreateOnboardRequest::builder()
//! //     .person_id("person_789")
//! //     .onboard_date("2024-01-15")
//! //     .job_data(job_info)
//! //     .build();
//! // corehr.lifecycle.create_onboard(onboard_request, None).await?;
//! ```
//!
//! # 企业级特性
//!
//! - 🏢 支持大型企业复杂组织架构
//! - 📊 丰富的人事数据分析能力
//! - 🔗 与企业系统深度集成
//! - 📋 完善的合规性管理
//! - 🔄 自动化人事流程支持
//!
//! # 人力资源管理
//!
//! - 👥 全员全周期管理
//! - 📈 人才发展规划
//! - 💼 绩效考核支持
//! - 📊 人事数据洞察
//! - 🎯 组织效能优化

pub mod basic_info;
pub mod employee;
pub mod job_management;
pub mod lifecycle;
pub mod models;
pub mod organization;

use openlark_core::config::Config;

use basic_info::BasicInfoService;
use employee::EmployeeService;
use job_management::JobManagementService;
use lifecycle::LifecycleService;
use organization::OrganizationService;

/// 飞书人事企业版服务
///
/// 飞书人事企业版(CoreHR)是面向大中型企业的专业人力资源管理解决方案，
/// 提供了完整的人事管理功能体系。本服务封装了CoreHR的核心API接口，支持：
///
/// ## 主要功能
///
/// ### 基础数据管理
/// - 枚举信息查询（性别、婚姻状况、员工状态等）
/// - 地理信息查询（国家、省份、城市、区县）
/// - 国籍信息查询
/// - ID类型转换（person_id、employee_id、user_id等互转）
///
/// ### 员工信息管理
/// - 批量查询员工信息
/// - 员工信息搜索和筛选
/// - 个人信息管理（姓名、身份证、地址等）
/// - 雇佣信息管理（入职日期、雇佣类型、试用期等）
/// - 任职信息管理（职位、部门、汇报关系等）
///
/// ### 组织架构管理
/// - 部门创建和管理
/// - 部门架构树查询
/// - 公司创建和管理
/// - 组织层级关系维护
///
/// ### 扩展功能
/// - 岗职务管理（序列、职级、职等、职务）
/// - 人事流程管理（入职、离职、异动、试用期）
/// - 合同管理
/// - 休假管理
/// - 权限管理
/// - 流程审批
/// - 基础薪酬
///
/// ## 使用场景
///
/// - **大型企业人事系统**: 支持复杂的组织架构和人事流程
/// - **人事数据分析**: 提供丰富的员工数据用于分析决策
/// - **第三方系统集成**: 与ERP、CRM等系统的人事数据同步
/// - **合规性管理**: 满足各种人事合规要求
/// - **自动化流程**: 支持入职、离职、异动等自动化流程
///
/// ## 权限要求
///
/// 使用本服务需要相应的应用权限：
/// - `corehr:employee`: 员工信息读取权限
/// - `corehr:organization`: 组织架构管理权限  
/// - `corehr:basic_info`: 基础数据读取权限
/// - 具体权限要求请参考飞书开放平台文档
///
/// ## 示例用法
///
/// ```ignore
/// use open_lark::prelude::*;
/// use open_lark::service::corehr::models::*;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // 创建客户端
/// let client = LarkClient::builder("app_id", "app_secret")
///     .with_app_type(AppType::SelfBuild)
///     .build();
///
/// // 查询枚举信息
/// let enum_request = EnumSearchRequest {
///     enum_type: "gender".to_string(),
///     page_size: Some(50),
///     page_token: None,
/// };
/// let enums = client.corehr.basic_info.search_enum(enum_request, None).await?;
///
/// // 搜索员工信息
/// let employee_request = EmployeeSearchRequest {
///     page_size: Some(50),
///     employment_status: Some(vec!["active".to_string()]),
///     fields: Some(vec![
///         "person".to_string(),
///         "employment".to_string(),
///         "job_datas".to_string(),
///     ]),
///     ..Default::default()
/// };
/// let employees = client.corehr.employee.search(employee_request, None).await?;
///
/// // 创建部门
/// let dept_request = DepartmentCreateRequest {
///     name: I18nText {
///         zh_cn: Some("技术部".to_string()),
///         en_us: Some("Technology Department".to_string()),
///     },
///     parent_department_id: Some("parent_dept_123".to_string()),
///     manager: Some("emp_manager_456".to_string()),
///     code: Some("TECH".to_string()),
///     effective_time: Some("2024-01-01".to_string()),
///     ..Default::default()
/// };
/// let department = client.corehr.organization.create_department(dept_request, None).await?;
/// # Ok(())
/// # }
/// ```
///
/// ## 注意事项
///
/// - CoreHR是企业级产品，API调用频率和数据量都较大，请注意接口限流
/// - 支持多语言，建议在创建数据时提供中英文名称
/// - 时间字段使用ISO 8601格式（YYYY-MM-DD）
/// - 自定义字段需要先在飞书人事后台配置
/// - 组织架构变更会有时间轴版本管理，请注意生效时间
pub struct CoreHRService {
    /// 基础数据服务
    pub basic_info: BasicInfoService,
    /// 员工信息服务
    pub employee: EmployeeService,
    /// 组织管理服务
    pub organization: OrganizationService,
    /// 岗职务管理服务
    pub job_management: JobManagementService,
    /// 员工生命周期服务（入职/离职/异动）
    pub lifecycle: LifecycleService,
}

impl CoreHRService {
    pub fn new(config: Config) -> Self {
        Self {
            basic_info: BasicInfoService::new(config.clone()),
            employee: EmployeeService::new(config.clone()),
            organization: OrganizationService::new(config.clone()),
            job_management: JobManagementService::new(config.clone()),
            lifecycle: LifecycleService::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_corehr_service_creation() {
        let config = Config::default();
        let service = CoreHRService::new(config.clone());

        assert_eq!(service.basic_info.config.app_id, config.app_id);
        assert_eq!(service.basic_info.config.app_secret, config.app_secret);
        assert_eq!(service.employee.config.app_id, config.app_id);
        assert_eq!(service.organization.config.app_id, config.app_id);
        assert_eq!(service.job_management.config.app_id, config.app_id);
        assert_eq!(service.lifecycle.config.app_id, config.app_id);
    }

    #[test]
    fn test_corehr_service_with_custom_config() {
        let config = Config::builder()
            .app_id("corehr_test_app")
            .app_secret("corehr_test_secret")
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = CoreHRService::new(config.clone());

        assert_eq!(service.basic_info.config.app_id, "corehr_test_app");
        assert_eq!(service.basic_info.config.app_secret, "corehr_test_secret");
        assert_eq!(service.employee.config.app_id, "corehr_test_app");
        assert_eq!(service.organization.config.app_id, "corehr_test_app");
        assert_eq!(service.job_management.config.app_id, "corehr_test_app");
        assert_eq!(service.lifecycle.config.app_id, "corehr_test_app");
    }

    #[test]
    fn test_corehr_service_config_independence() {
        let config1 = Config::builder().app_id("corehr_app_1").build();

        let config2 = Config::builder().app_id("corehr_app_2").build();

        let service1 = CoreHRService::new(config1);
        let service2 = CoreHRService::new(config2);

        assert_eq!(service1.basic_info.config.app_id, "corehr_app_1");
        assert_eq!(service2.basic_info.config.app_id, "corehr_app_2");
        assert_ne!(
            service1.basic_info.config.app_id,
            service2.basic_info.config.app_id
        );
        assert_ne!(
            service1.employee.config.app_id,
            service2.employee.config.app_id
        );
        assert_ne!(
            service1.organization.config.app_id,
            service2.organization.config.app_id
        );
        assert_ne!(
            service1.job_management.config.app_id,
            service2.job_management.config.app_id
        );
        assert_ne!(
            service1.lifecycle.config.app_id,
            service2.lifecycle.config.app_id
        );
    }

    #[test]
    fn test_corehr_service_sub_services_accessible() {
        let config = Config::default();
        let service = CoreHRService::new(config.clone());

        assert_eq!(service.basic_info.config.app_id, config.app_id);
        assert_eq!(service.employee.config.app_id, config.app_id);
        assert_eq!(service.organization.config.app_id, config.app_id);
        assert_eq!(service.job_management.config.app_id, config.app_id);
        assert_eq!(service.lifecycle.config.app_id, config.app_id);
    }

    #[test]
    fn test_corehr_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = CoreHRService::new(config.clone());

        assert_eq!(service.basic_info.config.app_id, "clone_test_app");
        assert_eq!(service.basic_info.config.app_secret, "clone_test_secret");
        assert_eq!(service.employee.config.app_id, "clone_test_app");
        assert_eq!(service.organization.config.app_id, "clone_test_app");
        assert_eq!(service.job_management.config.app_id, "clone_test_app");
        assert_eq!(service.lifecycle.config.app_id, "clone_test_app");
    }

    #[test]
    fn test_corehr_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(300))
            .build();

        let service = CoreHRService::new(config);

        assert_eq!(
            service.basic_info.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.employee.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.organization.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.job_management.config.req_timeout,
            Some(Duration::from_secs(300))
        );
        assert_eq!(
            service.lifecycle.config.req_timeout,
            Some(Duration::from_secs(300))
        );
    }

    #[test]
    fn test_corehr_service_multiple_instances() {
        let config = Config::default();

        let service1 = CoreHRService::new(config.clone());
        let service2 = CoreHRService::new(config.clone());

        assert_eq!(
            service1.basic_info.config.app_id,
            service2.basic_info.config.app_id
        );
        assert_eq!(
            service1.basic_info.config.app_secret,
            service2.basic_info.config.app_secret
        );
        assert_eq!(
            service1.employee.config.app_id,
            service2.employee.config.app_id
        );
        assert_eq!(
            service1.organization.config.app_id,
            service2.organization.config.app_id
        );
        assert_eq!(
            service1.job_management.config.app_id,
            service2.job_management.config.app_id
        );
        assert_eq!(
            service1.lifecycle.config.app_id,
            service2.lifecycle.config.app_id
        );
    }

    #[test]
    fn test_corehr_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(240))
            .build();

        let service = CoreHRService::new(config);

        assert_eq!(service.basic_info.config.app_id, "consistency_test");
        assert_eq!(service.basic_info.config.app_secret, "consistency_secret");
        assert_eq!(
            service.basic_info.config.req_timeout,
            Some(Duration::from_secs(240))
        );
        assert_eq!(service.employee.config.app_id, "consistency_test");
        assert_eq!(service.organization.config.app_id, "consistency_test");
        assert_eq!(service.job_management.config.app_id, "consistency_test");
        assert_eq!(service.lifecycle.config.app_id, "consistency_test");
    }
}
