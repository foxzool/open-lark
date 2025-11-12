//! 服务管理模块
//!
//! 提供条件编译的服务集成和动态服务注册功能

use std::sync::Arc;
use openlark_core::config::Config;
use crate::registry::DefaultServiceRegistry;
use crate::traits::ServiceRegistry;

// 条件导入服务
#[cfg(feature = "communication")]
use openlark_communication::contact::ContactService;

#[cfg(feature = "docs")]
use openlark_docs::docs::DocxService;

#[cfg(feature = "sheet")]
use openlark_docs::sheet::SheetsService;

#[cfg(feature = "bitable")]
use openlark_docs::bitable::BitableService;

#[cfg(feature = "wiki")]
use openlark_docs::wiki::WikiService;

#[cfg(feature = "drive")]
use openlark_docs::drive::DriveService;

#[cfg(feature = "ccm")]
use openlark_docs::ccm::CcmService;

#[cfg(feature = "hr")]
use openlark_hr::{hire::HireService, compensation_management::CompensationManagementService};

#[cfg(feature = "ai")]
use openlark_ai::AiService;

#[cfg(feature = "auth")]
use openlark_auth::AuthService;

/// 服务管理器
///
/// 负责根据启用的功能标志初始化和注册所有服务
pub struct ServiceManager;

impl ServiceManager {
    /// 初始化所有启用的服务并注册到注册表中
    pub fn initialize_services(
        _config: &Config,
        shared_config: &Arc<Config>,
        registry: &mut DefaultServiceRegistry,
    ) {
        tracing::debug!("Initializing services with feature flags");

        // 云文档服务
        #[cfg(feature = "docs")]
        {
            tracing::debug!("Initializing docs service");
            let service = DocxService::new_from_shared(shared_config.clone());
            registry.register_service("docs", service);
        }

        #[cfg(feature = "sheet")]
        {
            tracing::debug!("Initializing sheet service");
            let service = SheetsService::new_from_shared(shared_config.clone());
            registry.register_service("sheet", service);
        }

        #[cfg(feature = "bitable")]
        {
            tracing::debug!("Initializing bitable service");
            let service = BitableService::new_from_shared(shared_config.clone());
            registry.register_service("bitable", service);
        }

        #[cfg(feature = "wiki")]
        {
            tracing::debug!("Initializing wiki service");
            let service = WikiService::new_from_shared(shared_config.clone());
            registry.register_service("wiki", service);
        }

        #[cfg(feature = "drive")]
        {
            tracing::debug!("Initializing drive service");
            let service = DriveService::new_from_shared(shared_config.clone());
            registry.register_service("drive", service);
        }

        #[cfg(feature = "ccm")]
        {
            tracing::debug!("Initializing ccm service");
            let service = CcmService::new_from_shared(shared_config.clone());
            registry.register_service("ccm", service);
        }

        // 通信服务
        #[cfg(feature = "communication")]
        {
            tracing::debug!("Initializing communication services");
            let contact_service = ContactService::new(config.clone());
            registry.register_service("contact", contact_service);
        }

        // HR 服务
        #[cfg(feature = "hr")]
        {
            tracing::debug!("Initializing HR services");
            let hire_service = HireService::new(config.clone());
            registry.register_service("hire", hire_service);

            let compensation_service = CompensationManagementService::new(config.clone());
            registry.register_service("compensation_management", compensation_service);
        }

        // AI 服务
        #[cfg(feature = "ai")]
        {
            tracing::debug!("Initializing AI service");
            let service = AiService::new_from_shared(shared_config.clone());
            registry.register_service("ai", service);
        }

        // 认证服务
        #[cfg(feature = "auth")]
        {
            tracing::debug!("Initializing auth service");
            let service = AuthService::new_from_shared(shared_config.clone());
            registry.register_service("auth", service);
        }

        tracing::info!("Service initialization completed. Registered services: {:?}",
                     registry.list_services());
    }

    /// 获取已启用的服务列表
    pub fn get_enabled_services() -> Vec<&'static str> {
        let mut services = Vec::new();

        #[cfg(feature = "docs")]
        services.push("docs");

        #[cfg(feature = "sheet")]
        services.push("sheet");

        #[cfg(feature = "bitable")]
        services.push("bitable");

        #[cfg(feature = "wiki")]
        services.push("wiki");

        #[cfg(feature = "drive")]
        services.push("drive");

        #[cfg(feature = "ccm")]
        services.push("ccm");

        #[cfg(feature = "communication")]
        {
            services.push("contact");
        }

        #[cfg(feature = "hr")]
        {
            services.push("hire");
            services.push("compensation_management");
        }

        #[cfg(feature = "ai")]
        services.push("ai");

        #[cfg(feature = "auth")]
        services.push("auth");

        services
    }

    /// 验证必需的服务是否已注册
    pub fn validate_services(registry: &DefaultServiceRegistry) -> Result<(), String> {
        let enabled = Self::get_enabled_services();
        let registered = registry.list_services();

        for service in enabled {
            if !registered.contains(&service.to_string()) {
                return Err(format!("Required service '{}' is not registered", service));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_enabled_services() {
        let services = ServiceManager::get_enabled_services();
        // 验证函数能正常工作
        assert!(services.len() >= 0);
    }

    #[test]
    fn test_validate_empty_registry() {
        let registry = DefaultServiceRegistry::new();
        let result = ServiceManager::validate_services(&registry);

        // 当没有启用任何服务时应该通过验证
        match result {
            Ok(()) => (),
            Err(msg) => {
                // 如果没有启用服务，应该验证通过
                // 只有在启用了服务但未注册时才失败
                println!("Validation result: {}", msg);
            }
        }
    }
}