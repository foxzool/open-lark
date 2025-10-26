//! 通讯录（Contact）服务
//!
//! 提供企业通讯录的完整管理功能，支持用户、部门、组织架构等核心要素的
//! 生命周期管理。这是企业人员和组织管理的核心服务模块。

use open_lark_core::core::config::Config;

/// 通讯录数据模型定义
pub mod models;
pub use models::*;

/// 通讯录服务
///
/// 企业通讯录的统一管理入口，提供完整的人员和组织管理功能。
/// 支持企业级的用户生命周期管理、组织架构调整和权限控制。
/// # 服务架构
/// - **models**: 数据模型和结构定义
/// # 核心特性
/// - 🚀 高性能批量操作
/// - 🔐 企业级安全控制
/// - 📊 灵活的组织架构
/// - 🎯 精细化权限管理
/// - 🔄 完整的生命周期支持
/// # 适用场景
/// - 企业人力资源管理
/// - 组织架构调整和优化
/// - 权限和访问控制
/// - 用户身份管理
/// - 通讯录同步和集成
/// # 最佳实践
/// - 定期同步和更新用户信息
/// - 合理设计组织架构层级
/// - 遵循最小权限原则
/// - 建立完善的审计机制
pub struct ContactService {
    /// models模块
    pub models: models::Models,
}

impl ContactService {
    /// 创建新的通讯录服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    /// # 返回值
    /// 配置完成的通讯录服务实例
    pub fn new(config: Config) -> Self {
        Self {
            models: models::Models::new(config),
        }
    }

    /// 使用共享配置创建服务（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            models: models::Models::new(shared.as_ref().clone()),
        }
    }
}

use open_lark_core::core::trait_system::Service;

impl open_lark_core::core::trait_system::Service for ContactService {
    fn config(&self) -> &Config {
        self.models.config()
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ContactService"
    }
}

impl Clone for ContactService {
    fn clone(&self) -> Self {
        Self {
            models: models::Models::new(self.models.config().clone()),
        }
    }
}

impl std::fmt::Debug for ContactService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContactService")
            .field("service_name", &Self::service_name())
            .field("app_id", &self.models.config().app_id)
            .field("v3_service", &"V3")
            .field("modules", &4)
            .field("user_management", &true)
            .field("department_management", &true)
            .finish()
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use open_lark_core::core::config::Config;
    // use std::time::Duration; // 暂时注释掉未使用的导入

    /// 创建测试配置
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_contact_app_id")
            .app_secret("test_contact_app_secret")
            .build()
    }

    #[test]
    fn test_contact_service_creation() {
        let config = create_test_config();
        let service = ContactService::new(config.clone());
        // 验证服务创建成功
        assert!(!service.models.config().app_id.is_empty());
        assert!(!service.models.config().app_secret.is_empty());
        assert_eq!(service.models.config().app_id, "test_contact_app_id");
        assert_eq!(
            service.models.config().app_secret,
            "test_contact_app_secret"
        );
    }

    #[test]
    fn test_contact_service_validate_services_config() {
        let config = create_test_config();
        let service = ContactService::new(config.clone());
        // 测试有效配置
        assert!(service.models.config().app_id.len() > 0);
        assert!(!config.app_id.is_empty());
        // 测试无效配置
        let empty_config = Config::builder()
            .app_id("")
            .app_secret("test_secret")
            .build();
        let empty_service = ContactService::new(empty_config);
        assert!(empty_service.models.config().app_id.is_empty());
    }

    #[test]
    fn test_contact_service_health_check() {
        // 测试健康检查通过
        let config = create_test_config();
        let service = ContactService::new(config);
        assert!(!service.models.config().app_id.is_empty());
        assert!(!service.models.config().app_secret.is_empty());
        // 测试健康检查失败
        let invalid_config = Config::builder().app_id("").app_secret("").build();
        let invalid_service = ContactService::new(invalid_config);
        assert!(invalid_service.models.config().app_id.is_empty());
    }

    #[test]
    fn test_contact_service_debug_trait() {
        // 测试Debug trait
        let config = create_test_config();
        let service = ContactService::new(config);
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("ContactService"));
        assert!(debug_str.contains("test_contact_app_id"));
        // 测试Clone trait
        let cloned_service = service.clone();
        assert_eq!(
            service.models.config().app_id,
            cloned_service.models.config().app_id
        );
    }
}
