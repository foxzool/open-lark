//! 组织架构（Directory）服务
//!
//! 提供飞书组织架构的完整功能集，支持员工管理、部门管理、组织架构维护、
//! 人员信息同步等企业级组织管理能力。是企业人力资源和组织管理的核心工具。
//!
//! # 核心功能
//!
//! ## 员工管理
//! - 👥 员工信息创建、更新和删除
//! - 📋 员工列表查询和筛选
//! - 🔍 员工信息搜索和检索
//! - 📊 员工统计和分析
//! - 🏷️ 员工标签和分类
//!
//! ## 部门管理
//! - 🏢 部门结构创建和维护
//! - 📊 部门层级关系管理
//! - 👑 部门负责人设置
//! - 📋 部门信息查询和更新
//! - 🔄 部门合并和调整
//!
//! ## 组织架构
//! - 🌳 组织架构树形结构
//! - 📈 组织层级关系维护
//! - 🔄 组织变更和调整
//! - 📊 组织统计和分析
//! - 🎯 组织目标和KPI
//!
//! ## 数据同步
//! - 🔄 人员信息自动同步
//! - 📊 数据一致性校验
//! - 🔔 变更通知和提醒
//! - 📋 同步日志和审计
//! - 🛠️ 数据修复和维护
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
//! // 获取组织架构服务
//! let directory = &client.directory;
//!
//! // 创建员工
//! // let employee_request = CreateEmployeeRequest::builder()
//! //     .name("张三")
//! //     .email("zhangsan@company.com")
//! //     .mobile("13800138000")
//! //     .department_id("dept_123")
//! //     .job_title("软件工程师")
//! //     .build();
//! // directory.v1.employee.create(employee_request, None).await?;
//!
//! // 查询员工列表
//! // let list_request = ListEmployeesRequest::builder()
//! //     .department_id("dept_123")
//! //     .page_size(20)
//! //     .build();
//! // let employees = directory.v1.employee.list(list_request, None).await?;
//!
//! // 创建部门
//! // let department_request = CreateDepartmentRequest::builder()
//! //     .name("技术部")
//! //     .parent_id("parent_dept_123")
//! //     .leader_id("leader_user_123")
//! //     .build();
//! // directory.v1.department.create(department_request, None).await?;
//!
//! // 更新部门信息
//! // let update_request = UpdateDepartmentRequest::builder()
//! //     .department_id("dept_123")
//! //     .name("产品技术部")
//! //     .description("负责产品技术研发")
//! //     .build();
//! // directory.v1.department.update(update_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v1版本，提供基础的组织架构功能：
//! - 员工全生命周期管理
//! - 部门结构管理
//! - 组织架构维护
//! - 数据同步和校验
//!
//! # 组织管理特性
//!
//! - 🏢 灵活的组织架构设计
//! - 👥 完善的人员管理体系
//! - 📊 实时数据同步更新
//! - 🔐 权限控制和安全保障
//! - 📱 移动端管理支持
//!
//! # 人力资源集成
//!
//! - 💼 HR系统深度集成
//! - 📋 人事流程自动化
//! - 📊 人力资源数据分析
//! - 🎯 绩效考核支持
//! - 📈 组织发展规划

use crate::core::config::Config;

/// 组织架构服务 v1 版本
pub mod v1;

/// 组织架构服务
///
/// 企业级组织管理的统一入口，提供员工管理、部门管理、
/// 组织架构维护、数据同步等完整的组织管理能力。
///
/// # 服务架构
///
/// - **v1**: 组织架构API v1版本，提供基础功能集
///
/// # 核心特性
///
/// - 👥 全面的员工管理功能
/// - 🏢 灵活的部门管理系统
/// - 🌳 完整的组织架构维护
/// - 🔄 智能的数据同步机制
/// - 📊 丰富的统计分析功能
///
/// # 适用场景
///
/// - 企业组织架构管理
/// - 人员信息管理和维护
/// - 部门结构调整和优化
/// - HR系统数据同步
/// - 组织变更管理
///
/// # 最佳实践
///
/// - 建立清晰的组织层级
/// - 定期维护人员信息
/// - 合理设置部门权限
/// - 监控数据同步状态
/// - 保护员工隐私信息
///
/// # 示例用法
///
/// ```rust,ignore
/// use open_lark::LarkClient;
///
/// let client = LarkClient::builder("app_id", "app_secret").build();
///
/// // 创建员工
/// let response = client.directory.v1.employee.create(request, None).await?;
///
/// // 创建部门
/// let response = client.directory.v1.department.create(request, None).await?;
/// ```
pub struct DirectoryService {
    /// v1版本API服务
    pub v1: v1::V1,
}

impl DirectoryService {
    /// 创建新的组织架构服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的组织架构服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }

    /// 使用共享配置创建服务（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            v1: v1::V1::new(shared.as_ref().clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_directory_service_creation() {
        let config = Config::default();
        let service = DirectoryService::new(config.clone());

        assert_eq!(service.v1.employee.config.app_id, config.app_id);
        assert_eq!(service.v1.employee.config.app_secret, config.app_secret);
        assert_eq!(service.v1.department.config.app_id, config.app_id);
        assert_eq!(service.v1.department.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_directory_service_with_custom_config() {
        let config = Config::builder()
            .app_id("directory_test_app")
            .app_secret("directory_test_secret")
            .req_timeout(Duration::from_secs(250))
            .build();

        let service = DirectoryService::new(config.clone());

        assert_eq!(service.v1.employee.config.app_id, "directory_test_app");
        assert_eq!(
            service.v1.employee.config.app_secret,
            "directory_test_secret"
        );
        assert_eq!(
            service.v1.employee.config.req_timeout,
            Some(Duration::from_secs(250))
        );
        assert_eq!(service.v1.department.config.app_id, "directory_test_app");
        assert_eq!(
            service.v1.department.config.req_timeout,
            Some(Duration::from_secs(250))
        );
    }

    #[test]
    fn test_directory_service_config_independence() {
        let config1 = Config::builder().app_id("directory_app_1").build();

        let config2 = Config::builder().app_id("directory_app_2").build();

        let service1 = DirectoryService::new(config1);
        let service2 = DirectoryService::new(config2);

        assert_eq!(service1.v1.employee.config.app_id, "directory_app_1");
        assert_eq!(service2.v1.employee.config.app_id, "directory_app_2");
        assert_ne!(
            service1.v1.employee.config.app_id,
            service2.v1.employee.config.app_id
        );
        assert_ne!(
            service1.v1.department.config.app_id,
            service2.v1.department.config.app_id
        );
    }

    #[test]
    fn test_directory_service_sub_services_accessible() {
        let config = Config::default();
        let service = DirectoryService::new(config.clone());

        assert_eq!(service.v1.employee.config.app_id, config.app_id);
        assert_eq!(service.v1.department.config.app_id, config.app_id);
    }

    #[test]
    fn test_directory_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = DirectoryService::new(config.clone());

        assert_eq!(service.v1.employee.config.app_id, "clone_test_app");
        assert_eq!(service.v1.employee.config.app_secret, "clone_test_secret");
        assert_eq!(service.v1.department.config.app_secret, "clone_test_secret");
        assert_eq!(service.v1.department.config.app_id, "clone_test_app");
    }

    #[test]
    fn test_directory_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(260))
            .build();

        let service = DirectoryService::new(config);

        assert_eq!(
            service.v1.employee.config.req_timeout,
            Some(Duration::from_secs(260))
        );
        assert_eq!(
            service.v1.department.config.req_timeout,
            Some(Duration::from_secs(260))
        );
    }

    #[test]
    fn test_directory_service_multiple_instances() {
        let config = Config::default();

        let service1 = DirectoryService::new(config.clone());
        let service2 = DirectoryService::new(config.clone());

        assert_eq!(
            service1.v1.employee.config.app_id,
            service2.v1.employee.config.app_id
        );
        assert_eq!(
            service1.v1.employee.config.app_secret,
            service2.v1.employee.config.app_secret
        );
        assert_eq!(
            service1.v1.department.config.app_id,
            service2.v1.department.config.app_id
        );
        assert_eq!(
            service1.v1.department.config.app_secret,
            service2.v1.department.config.app_secret
        );
    }

    #[test]
    fn test_directory_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = DirectoryService::new(config);

        assert_eq!(service.v1.employee.config.app_id, "consistency_test");
        assert_eq!(service.v1.employee.config.app_secret, "consistency_secret");
        assert_eq!(
            service.v1.employee.config.req_timeout,
            Some(Duration::from_secs(180))
        );
        assert_eq!(service.v1.department.config.app_id, "consistency_test");
        assert_eq!(
            service.v1.department.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.v1.department.config.req_timeout,
            Some(Duration::from_secs(180))
        );
    }
}
