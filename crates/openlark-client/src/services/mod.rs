//! 🏗️ OpenLark Client 服务访问层
//!
//! 提供统一的服务访问接口，作为底层crates的薄包装层
//! 集成 CoreError 错误处理系统，提供企业级服务管理

use crate::error::{validation_error, with_context};
use crate::{Config, DefaultServiceRegistry, Result};

// 新的服务运行时与抽象
pub mod context;
pub mod graph;
pub mod loader;
pub mod runtime;
pub mod service;
pub mod typed_registry;

// ============================================================================
// 业务服务模块
// ============================================================================

/// 🔐 认证服务模块
///
/// 提供飞书认证功能，包括令牌管理、OAuth认证、用户信息验证等
/// 集成了完整的错误处理和上下文管理
pub mod auth;

/// 📡 通讯服务模块
///
/// 提供飞书通讯功能，包括IM消息、联系人管理、群组管理等
/// 支持富文本消息、批量操作和实时事件处理
#[cfg(feature = "communication")]
pub mod communication;

/// 📄 文档服务模块
///
/// 提供飞书云文档功能，包括文档、表格、知识库管理等
/// 支持协作编辑、版本控制和权限管理
#[cfg(feature = "docs")]
pub mod docs;

/// 👥 人力资源服务模块
///
/// 提供飞书HR功能，包括考勤管理、招聘、员工信息等
/// 支持数据分析和报表生成
#[cfg(feature = "hr")]
pub mod hr;

/// 🤖 AI服务模块
///
/// 提供飞书AI功能，包括智能助手、自然语言处理等
/// 支持内容生成和智能推荐
#[cfg(feature = "ai")]
pub mod ai;

/// 📋 任务管理服务模块
///
/// 提供飞书任务功能，包括待办事项、项目协作等
/// 支持进度跟踪和团队协作
#[cfg(feature = "task")]
pub mod task;

/// 📅 日历会议服务模块
///
/// 提供飞书日历功能，包括会议安排、日程管理等
/// 支持重复提醒和资源预订
#[cfg(feature = "calendar")]
pub mod calendar;

/// 🔧 管理员工能服务模块
///
/// 提供飞书管理功能，包括应用管理、权限控制等
/// 支持企业级管理和监控
#[cfg(feature = "admin")]
pub mod admin;

/// ✅ 审批流程服务模块
///
/// 提供飞书审批功能，包括流程定义、审批处理等
/// 支持自定义模板和状态跟踪
#[cfg(feature = "approval")]
pub mod approval;

// ============================================================================
// 服务工厂和注册系统
// ============================================================================

/// 🏭 服务工厂
///
/// 负责创建和管理各种服务实例，提供统一的依赖注入和配置管理
#[derive(Debug)]
pub struct ServiceFactory {
    /// 🔧 客户端配置
    config: Config,
    /// 📋 服务注册表
    registry: DefaultServiceRegistry,
}

impl ServiceFactory {
    /// 🆕 创建新的服务工厂实例
    pub fn new(config: Config) -> Result<Self> {
        tracing::info!("初始化服务工厂");

        // 验证基础配置
        if config.app_id.is_empty() {
            return with_context(
                Err(validation_error("app_id", "应用ID不能为空")),
                "component",
                "ServiceFactory",
            );
        }

        if config.app_secret.is_empty() {
            return with_context(
                Err(validation_error("app_secret", "应用密钥不能为空")),
                "component",
                "ServiceFactory",
            );
        }

        let registry = DefaultServiceRegistry::new();

        let factory = Self { config, registry };

        tracing::debug!("服务工厂初始化完成");

        Ok(factory)
    }

    /// 🔐 创建认证服务
    pub fn create_auth_service(&self) -> Result<auth::AuthService> {
        tracing::debug!("创建认证服务");

        let service = auth::AuthService::new(&self.config);
        Ok(service)
    }

    /// 📡 创建通讯服务
    #[cfg(feature = "communication")]
    pub fn create_communication_service(&self) -> Result<communication::CommunicationService> {
        tracing::debug!("创建通讯服务");

        communication::CommunicationService::new(&self.config, &self.registry)
    }

    /// 📄 创建文档服务
    #[cfg(feature = "docs")]
    pub fn create_docs_service(&self) -> Result<docs::DocsService> {
        tracing::debug!("创建文档服务");

        let service = docs::DocsService::new();
        Ok(service)
    }

    /// 👥 创建人力资源服务
    #[cfg(feature = "hr")]
    pub fn create_hr_service(&self) -> Result<hr::HRService> {
        tracing::debug!("创建人力资源服务");

        let service = hr::HRService::new();
        Ok(service)
    }

    /// 🤖 创建AI服务
    #[cfg(feature = "ai")]
    pub fn create_ai_service(&self) -> Result<ai::AIService> {
        tracing::debug!("创建AI服务");

        let service = ai::AIService::new();
        Ok(service)
    }

    /// 📋 创建任务管理服务
    #[cfg(feature = "task")]
    pub fn create_task_service(&self) -> Result<task::TaskService> {
        tracing::debug!("创建任务管理服务");

        let service = task::TaskService::new();
        Ok(service)
    }

    /// 📅 创建日历会议服务
    #[cfg(feature = "calendar")]
    pub fn create_calendar_service(&self) -> Result<calendar::CalendarService> {
        tracing::debug!("创建日历会议服务");

        let service = calendar::CalendarService::new();
        Ok(service)
    }

    /// 🔧 创建管理员工能服务
    #[cfg(feature = "admin")]
    pub fn create_admin_service(&self) -> Result<admin::AdminService> {
        tracing::debug!("创建管理员工能服务");

        let service = admin::AdminService::new();
        Ok(service)
    }

    /// ✅ 创建审批流程服务
    #[cfg(feature = "approval")]
    pub fn create_approval_service(&self) -> Result<approval::ApprovalService> {
        tracing::debug!("创建审批流程服务");

        let service = approval::ApprovalService::new();
        Ok(service)
    }

    /// 📊 获取服务工厂统计信息
    pub fn get_stats(&self) -> ServiceFactoryStats {
        ServiceFactoryStats {
            total_services: self.count_available_services(),
            enabled_features: self
                .get_enabled_features()
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }

    /// 🔢 统计可用服务数量
    fn count_available_services(&self) -> usize {
        let mut count = 1; // auth service is always available

        #[cfg(feature = "communication")]
        {
            count += 1;
        }

        #[cfg(feature = "docs")]
        {
            count += 1;
        }

        #[cfg(feature = "hr")]
        {
            count += 1;
        }

        #[cfg(feature = "ai")]
        {
            count += 1;
        }

        #[cfg(feature = "task")]
        {
            count += 1;
        }

        #[cfg(feature = "calendar")]
        {
            count += 1;
        }

        #[cfg(feature = "admin")]
        {
            count += 1;
        }

        #[cfg(feature = "approval")]
        {
            count += 1;
        }

        count
    }

    /// 🏷️ 获取已启用的功能列表
    fn get_enabled_features(&self) -> Vec<&'static str> {
        let mut features = vec!["auth"];

        #[cfg(feature = "communication")]
        {
            features.push("communication");
        }

        #[cfg(feature = "docs")]
        {
            features.push("docs");
        }

        #[cfg(feature = "hr")]
        {
            features.push("hr");
        }

        #[cfg(feature = "ai")]
        {
            features.push("ai");
        }

        #[cfg(feature = "task")]
        {
            features.push("task");
        }

        #[cfg(feature = "calendar")]
        {
            features.push("calendar");
        }

        #[cfg(feature = "admin")]
        {
            features.push("admin");
        }

        #[cfg(feature = "approval")]
        {
            features.push("approval");
        }

        features
    }
}

/// 📊 服务工厂统计信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceFactoryStats {
    /// 🔢 可用服务总数
    pub total_services: usize,
    /// 🏷️ 已启用的功能列表
    pub enabled_features: Vec<String>,
}

// ============================================================================
// 服务类型重新导出
// ============================================================================

// 基础服务
pub use auth::AuthService;

// 可选服务（基于功能标志）
#[cfg(feature = "communication")]
pub use communication::CommunicationService;

#[cfg(feature = "docs")]
pub use docs::DocsService;

#[cfg(feature = "hr")]
pub use hr::HRService;

#[cfg(feature = "ai")]
pub use ai::AIService;

#[cfg(feature = "task")]
pub use task::TaskService;

#[cfg(feature = "calendar")]
pub use calendar::CalendarService;

#[cfg(feature = "admin")]
pub use admin::AdminService;

#[cfg(feature = "approval")]
pub use approval::ApprovalService;

// ============================================================================
// 服务访问层预导出
// ============================================================================

/// 📦 服务访问层预导出
///
/// 提供常用的服务类型和工厂，简化客户端代码
pub mod prelude {
    // 基础类型
    pub use super::{ServiceFactory, ServiceFactoryStats};

    // 新运行时抽象
    pub use super::context::ServiceContext;
    pub use super::runtime::{ServiceRuntime, ServiceRuntimeBuilder};
    pub use super::service::{Service, ServiceHealth, ServiceKind, ServiceProvider};
    pub use super::typed_registry::TypedServiceRegistry;

    // 基础服务
    pub use super::AuthService;

    // 可选服务
    #[cfg(feature = "communication")]
    pub use super::CommunicationService;

    #[cfg(feature = "docs")]
    pub use super::DocsService;

    #[cfg(feature = "hr")]
    pub use super::HRService;

    #[cfg(feature = "ai")]
    pub use super::AIService;

    #[cfg(feature = "task")]
    pub use super::TaskService;

    #[cfg(feature = "calendar")]
    pub use super::CalendarService;

    #[cfg(feature = "admin")]
    pub use super::AdminService;

    #[cfg(feature = "approval")]
    pub use super::ApprovalService;
}

// ============================================================================
// 错误处理和验证
// ============================================================================

/// 🔧 服务验证器
///
/// 提供服务相关的验证功能
pub struct ServiceValidator;

impl ServiceValidator {
    /// 验证服务配置完整性
    pub fn validate_config(config: &Config) -> Result<()> {
        if config.app_id.is_empty() {
            return Err(validation_error("app_id", "应用ID不能为空"));
        }

        if config.app_secret.is_empty() {
            return Err(validation_error("app_secret", "应用密钥不能为空"));
        }

        if config.base_url.is_empty() {
            return Err(validation_error("base_url", "基础URL不能为空"));
        }

        // 验证URL格式
        if !config.base_url.starts_with("http://") && !config.base_url.starts_with("https://") {
            return Err(validation_error(
                "base_url",
                "基础URL必须以http://或https://开头",
            ));
        }

        Ok(())
    }

    /// 检查功能依赖
    pub fn check_feature_dependencies(feature: &str) -> Result<Vec<&'static str>> {
        match feature {
            "communication" => Ok(vec!["auth"]), // 通讯服务依赖认证服务
            "docs" => Ok(vec!["auth"]),          // 文档服务依赖认证服务
            "hr" => Ok(vec!["auth"]),            // HR服务依赖认证服务
            "ai" => Ok(vec!["auth"]),            // AI服务依赖认证服务
            "task" => Ok(vec!["auth"]),          // 任务服务依赖认证服务
            "calendar" => Ok(vec!["auth"]),      // 日历服务依赖认证服务
            "admin" => Ok(vec!["auth"]),         // 管理服务依赖认证服务
            "approval" => Ok(vec!["auth"]),      // 审批服务依赖认证服务
            _ => Ok(vec![]),                     // 其他功能暂无特殊依赖
        }
    }
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_service_factory_creation_success() {
        let config = create_test_config();

        let result = ServiceFactory::new(config);

        assert!(result.is_ok(), "服务工厂创建应该成功");

        let factory = result.unwrap();
        let stats = factory.get_stats();

        assert!(stats.total_services >= 1); // 至少有认证服务
        assert!(stats.enabled_features.contains(&"auth"));
    }

    #[test]
    fn test_service_factory_with_invalid_config() {
        let mut config = create_test_config();
        config.app_id = "".to_string();

        let result = ServiceFactory::new(config);

        assert!(result.is_err(), "无效配置应该导致服务工厂创建失败");

        if let Err(error) = result {
            assert!(error.is_validation_error());
            assert!(error
                .user_message()
                .unwrap_or("未知错误")
                .contains("应用ID不能为空"));
        }
    }

    #[test]
    fn test_service_validator() {
        let valid_config = create_test_config();
        assert!(ServiceValidator::validate_config(&valid_config).is_ok());

        let mut invalid_config = create_test_config();
        invalid_config.base_url = "invalid_url".to_string();
        assert!(ServiceValidator::validate_config(&invalid_config).is_err());
    }

    #[test]
    fn test_feature_dependencies() {
        let deps = ServiceValidator::check_feature_dependencies("communication").unwrap();
        assert!(deps.contains(&"auth"));

        let deps = ServiceValidator::check_feature_dependencies("docs").unwrap();
        assert!(deps.contains(&"auth"));

        let deps = ServiceValidator::check_feature_dependencies("auth").unwrap();
        assert!(deps.is_empty()); // 认证服务没有依赖
    }

    #[cfg(feature = "communication")]
    #[tokio::test]
    async fn test_create_communication_service() {
        let config = create_test_config();
        let factory = ServiceFactory::new(config).unwrap();

        let result = factory.create_communication_service();

        assert!(result.is_ok(), "通讯服务创建应该成功");
    }

    #[test]
    fn test_service_factory_stats() {
        let config = create_test_config();
        let factory = ServiceFactory::new(config).unwrap();

        let stats = factory.get_stats();

        assert!(stats.total_services >= 1);
        assert!(!stats.enabled_features.is_empty());
        assert!(stats.enabled_features.contains(&"auth"));
    }
}
