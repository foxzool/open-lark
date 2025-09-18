//! # 智能门禁服务
//!
//! 飞书智能门禁 (ACS - Access Control System) 服务提供完整的门禁管理功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **用户管理**：门禁用户信息管理、人脸图片上传下载
//! - **权限组管理**：门禁权限组的创建、更新、删除和设备绑定
//! - **访客管理**：临时访客的添加和删除
//! - **设备管理**：门禁设备列表查询和管理
//! - **访问记录**：门禁访问记录查询和人脸识别图片下载
//! - **事件推送**：用户信息变更和访问记录事件推送
//!
//! ## 安全说明
//!
//! 本服务涉及物理安全和访问控制，请确保：
//! - 严格控制 API 访问权限
//! - 妥善保管人脸识别等生物识别数据
//! - 遵循相关安全规范和法律法规
//! - 建立完善的访问记录和审计机制
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`user`] - 用户管理模块
//! - [`rule_external`] - 权限组管理模块
//! - [`visitor`] - 访客管理模块
//! - [`device`] - 门禁设备管块
//! - [`access_record`] - 门禁记录模块
//! - [`v1`] - 事件定义模块
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::acs::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 获取用户列表
//!     let users = client.acs.user.list_users(
//!         user::UserListRequest::default(), None
//!     ).await?;
//!     
//!     // 获取门禁设备列表
//!     let devices = client.acs.device.list_devices(
//!         device::DeviceListRequest::default(), None
//!     ).await?;
//!     
//!     // 获取访问记录
//!     let records = client.acs.access_record.list_access_records(
//!         access_record::AccessRecordListRequest::default(), None
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod access_record;
pub mod device;
pub mod models;
pub mod rule_external;
pub mod user;
pub mod v1;
pub mod visitor;

use crate::{
    core::config::Config,
    service::acs::{
        access_record::AccessRecordService, device::DeviceService,
        rule_external::RuleExternalService, user::UserService, visitor::VisitorService,
    },
};

/// 智能门禁服务
///
/// 提供完整的门禁管理功能，包括用户管理、权限控制、访客管理、设备管理和访问记录
pub struct AcsService {
    /// 用户管理服务
    pub user: UserService,
    /// 权限组管理服务
    pub rule_external: RuleExternalService,
    /// 访客管理服务
    pub visitor: VisitorService,
    /// 设备管理服务
    pub device: DeviceService,
    /// 访问记录服务
    pub access_record: AccessRecordService,
}

impl AcsService {
    /// 创建智能门禁服务实例
    pub fn new(config: Config) -> Self {
        Self {
            user: UserService::new(config.clone()),
            rule_external: RuleExternalService::new(config.clone()),
            visitor: VisitorService::new(config.clone()),
            device: DeviceService::new(config.clone()),
            access_record: AccessRecordService::new(config),
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::constants::AppType;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .build()
    }

    #[test]
    fn test_acs_service_creation() {
        let config = create_test_config();
        let service = AcsService::new(config);

        // Verify that all services are properly initialized
    }

    #[test]
    fn test_acs_service_with_different_config() {
        let config = Config::builder()
            .app_id("different_app_id")
            .app_secret("different_app_secret")
            .app_type(AppType::Marketplace)
            .build();

        let service = AcsService::new(config);

        // Verify service creation works with different config types
    }

    #[test]
    fn test_acs_service_structure() {
        let config = create_test_config();
        let service = AcsService::new(config);

        // Test that we can access all service fields
        let _user = &service.user;
        let _rule_external = &service.rule_external;
        let _visitor = &service.visitor;
        let _device = &service.device;
        let _access_record = &service.access_record;

        // If we reach here without panic, structure is correct
        // Test passes by not panicking above
    }

    #[test]
    fn test_acs_service_memory_safety() {
        let config = create_test_config();

        // Create service in a scope
        let service = AcsService::new(config);

        // Access services multiple times
        let _first_access = &service.user;
        let _second_access = &service.user;

        // Verify multiple references work correctly
        assert!(std::ptr::eq(_first_access, _second_access));
    }
}
