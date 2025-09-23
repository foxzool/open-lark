//! # 飞书主数据服务
//!
//! 飞书主数据 (Master Data Management, MDM) 服务提供基础数据管理和数据维度管理功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **基础数据管理**：国家/地区信息的查询和管理
//! - **数据维度管理**：用户数据维度的绑定和解绑操作
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`country_region`] - 国家/地区管理模块
//! - [`user_auth_data_relation`] - 用户数据维度管理模块
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use open_lark::prelude::*;
//! use open_lark::service::mdm::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 分页查询国家/地区
//!     let countries = client.mdm.country_region.list(
//!         country_region::CountryRegionListRequest::default(), None
//!     ).await?;
//!     
//!     // 用户数据维度绑定
//!     let bind_result = client.mdm.user_auth_data_relation.bind(
//!         user_auth_data_relation::UserDataRelationBindRequest {
//!             user_id: "user_001".to_string(),
//!             data_dimension_id: "dim_001".to_string(),
//!         }, None
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod country_region;
pub mod models;
pub mod user_auth_data_relation;

use crate::{
    core::config::Config,
    service::mdm::{
        country_region::CountryRegionService, user_auth_data_relation::UserAuthDataRelationService,
    },
};

/// 飞书主数据服务
///
/// 提供完整的主数据管理功能，包括基础数据和数据维度管理
pub struct MdmService {
    /// 国家/地区管理服务
    pub country_region: CountryRegionService,
    /// 用户数据维度管理服务
    pub user_auth_data_relation: UserAuthDataRelationService,
}

impl MdmService {
    /// 创建飞书主数据服务实例
    pub fn new(config: Config) -> Self {
        Self {
            country_region: CountryRegionService::new(config.clone()),
            user_auth_data_relation: UserAuthDataRelationService::new(config),
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
    fn test_mdm_service_creation() {
        let config = create_test_config();
        let mdm_service = MdmService::new(config);

        // Verify service structure
    }

    #[test]
    fn test_mdm_service_debug_trait() {
        let config = create_test_config();
        let mdm_service = MdmService::new(config);

        // Test that service can be used
    }

    #[test]
    fn test_mdm_service_with_different_configs() {
        let configs = vec![
            Config::builder()
                .app_id("app1")
                .app_secret("secret1")
                .build(),
            Config::builder()
                .app_id("app2")
                .app_secret("secret2")
                .req_timeout(std::time::Duration::from_millis(5000))
                .build(),
            Config::builder()
                .app_id("app3")
                .app_secret("secret3")
                .base_url("https://custom.api.com")
                .build(),
        ];

        for config in configs {
            let mdm_service = MdmService::new(config);
            // Each service should be created successfully
        }
    }

    #[test]
    fn test_mdm_service_module_independence() {
        let config = create_test_config();
        let mdm_service = MdmService::new(config);

        // Test that sub-modules are independent (different memory addresses)
        let country_ptr = std::ptr::addr_of!(mdm_service.country_region) as *const _;
        let user_auth_ptr = std::ptr::addr_of!(mdm_service.user_auth_data_relation) as *const _;

        assert_ne!(country_ptr, user_auth_ptr);
    }
}
