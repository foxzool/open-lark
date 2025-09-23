//! # 工作台服务
//!
//! 工作台 (Workplace) 服务提供工作台访问数据和应用推荐功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **工作台访问数据**：获取工作台访问数据、定制工作台访问数据、定制工作台小组件访问数据
//! - **我的常用应用**：获取用户自定义常用应用、管理员推荐应用、推荐规则列表
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`workplace_access_data`] - 工作台访问数据模块
//! - [`app_recommend`] - 应用推荐模块
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::workplace::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 获取工作台访问数据
//!     let access_data = client.workplace.workplace_access_data.search(
//!         workplace_access_data::AccessDataSearchRequest::default(), None
//!     ).await?;
//!     
//!     // 获取用户常用应用
//!     let favourite_apps = client.workplace.app_recommend.get_favourite_apps(
//!         app_recommend::FavouriteAppsRequest::default(), None
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod app_recommend;
pub mod models;
pub mod workplace_access_data;

use crate::{
    core::config::Config,
    service::workplace::{
        app_recommend::AppRecommendService, workplace_access_data::WorkplaceAccessDataService,
    },
};

/// 工作台服务
///
/// 提供完整的工作台功能，包括访问数据统计和应用推荐管理
pub struct WorkplaceService {
    /// 工作台访问数据服务
    pub workplace_access_data: WorkplaceAccessDataService,
    /// 应用推荐服务
    pub app_recommend: AppRecommendService,
}

impl WorkplaceService {
    /// 创建工作台服务实例
    pub fn new(config: Config) -> Self {
        Self {
            workplace_access_data: WorkplaceAccessDataService::new(config.clone()),
            app_recommend: AppRecommendService::new(config),
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_workplace_service_creation() {
        let config = create_test_config();
        let workplace_service = WorkplaceService::new(config);

        // Verify service structure
    }

    #[test]
    fn test_workplace_service_debug_trait() {
        let config = create_test_config();
        let workplace_service = WorkplaceService::new(config);

        // Test that service can be used
    }

    #[test]
    fn test_workplace_service_modules_independence() {
        let config = create_test_config();
        let workplace_service = WorkplaceService::new(config);

        // Test that sub-modules are independent
        let access_data_ptr =
            std::ptr::addr_of!(workplace_service.workplace_access_data) as *const _;
        let app_recommend_ptr = std::ptr::addr_of!(workplace_service.app_recommend) as *const _;

        assert_ne!(
            access_data_ptr, app_recommend_ptr,
            "Sub-services should be independent"
        );
    }

    #[test]
    fn test_workplace_service_with_different_configs() {
        let configs = vec![
            Config::builder()
                .app_id("workplace1")
                .app_secret("secret1")
                .build(),
            Config::builder()
                .app_id("workplace2")
                .app_secret("secret2")
                .req_timeout(std::time::Duration::from_millis(12000))
                .build(),
            Config::builder()
                .app_id("workplace3")
                .app_secret("secret3")
                .base_url("https://workplace.custom.com")
                .build(),
        ];

        for config in configs {
            let workplace_service = WorkplaceService::new(config);

            // Each service should be created successfully
        }
    }

    #[test]
    fn test_workplace_service_config_cloning() {
        let config = create_test_config();

        // Test that config cloning works correctly
        let workplace_service1 = WorkplaceService::new(config.clone());
        let workplace_service2 = WorkplaceService::new(config);

        // Both services should be created successfully

        // But should be different instances
        let service1_ptr = std::ptr::addr_of!(workplace_service1) as *const _;
        let service2_ptr = std::ptr::addr_of!(workplace_service2) as *const _;
        assert_ne!(
            service1_ptr, service2_ptr,
            "Services should be independent instances"
        );
    }
}
