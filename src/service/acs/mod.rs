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
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_acs_service_creation() {
        let config = Config::default();
        let service = AcsService::new(config.clone());

        assert_eq!(service.user.config.app_id, config.app_id);
        assert_eq!(service.user.config.app_secret, config.app_secret);
        assert_eq!(service.rule_external.config.app_id, config.app_id);
        assert_eq!(service.visitor.config.app_id, config.app_id);
        assert_eq!(service.device.config.app_id, config.app_id);
        assert_eq!(service.access_record.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_acs_service_with_custom_config() {
        let config = Config::builder()
            .app_id("acs_test_app")
            .app_secret("acs_test_secret")
            .req_timeout(Duration::from_secs(200))
            .build();

        let service = AcsService::new(config.clone());

        assert_eq!(service.user.config.app_id, "acs_test_app");
        assert_eq!(service.user.config.app_secret, "acs_test_secret");
        assert_eq!(
            service.user.config.req_timeout,
            Some(Duration::from_secs(200))
        );
        assert_eq!(service.rule_external.config.app_id, "acs_test_app");
        assert_eq!(
            service.visitor.config.req_timeout,
            Some(Duration::from_secs(200))
        );
        assert_eq!(service.device.config.app_id, "acs_test_app");
        assert_eq!(
            service.access_record.config.req_timeout,
            Some(Duration::from_secs(200))
        );
    }

    #[test]
    fn test_acs_service_config_independence() {
        let config1 = Config::builder().app_id("acs_app_1").build();

        let config2 = Config::builder().app_id("acs_app_2").build();

        let service1 = AcsService::new(config1);
        let service2 = AcsService::new(config2);

        assert_eq!(service1.user.config.app_id, "acs_app_1");
        assert_eq!(service2.user.config.app_id, "acs_app_2");
        assert_ne!(service1.user.config.app_id, service2.user.config.app_id);
        assert_ne!(
            service1.rule_external.config.app_id,
            service2.rule_external.config.app_id
        );
        assert_ne!(
            service1.visitor.config.app_id,
            service2.visitor.config.app_id
        );
        assert_ne!(service1.device.config.app_id, service2.device.config.app_id);
        assert_ne!(
            service1.access_record.config.app_id,
            service2.access_record.config.app_id
        );
    }

    #[test]
    fn test_acs_service_sub_services_accessible() {
        let config = Config::default();
        let service = AcsService::new(config.clone());

        assert_eq!(service.user.config.app_id, config.app_id);
        assert_eq!(service.rule_external.config.app_id, config.app_id);
        assert_eq!(service.visitor.config.app_id, config.app_id);
        assert_eq!(service.device.config.app_id, config.app_id);
        assert_eq!(service.access_record.config.app_id, config.app_id);
    }

    #[test]
    fn test_acs_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = AcsService::new(config.clone());

        assert_eq!(service.user.config.app_id, "clone_test_app");
        assert_eq!(service.user.config.app_secret, "clone_test_secret");
        assert_eq!(service.rule_external.config.app_secret, "clone_test_secret");
        assert_eq!(service.visitor.config.app_id, "clone_test_app");
        assert_eq!(service.device.config.app_secret, "clone_test_secret");
        assert_eq!(service.access_record.config.app_id, "clone_test_app");
    }

    #[test]
    fn test_acs_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(210))
            .build();

        let service = AcsService::new(config);

        assert_eq!(
            service.user.config.req_timeout,
            Some(Duration::from_secs(210))
        );
        assert_eq!(
            service.rule_external.config.req_timeout,
            Some(Duration::from_secs(210))
        );
        assert_eq!(
            service.visitor.config.req_timeout,
            Some(Duration::from_secs(210))
        );
        assert_eq!(
            service.device.config.req_timeout,
            Some(Duration::from_secs(210))
        );
        assert_eq!(
            service.access_record.config.req_timeout,
            Some(Duration::from_secs(210))
        );
    }

    #[test]
    fn test_acs_service_multiple_instances() {
        let config = Config::default();

        let service1 = AcsService::new(config.clone());
        let service2 = AcsService::new(config.clone());

        assert_eq!(service1.user.config.app_id, service2.user.config.app_id);
        assert_eq!(
            service1.user.config.app_secret,
            service2.user.config.app_secret
        );
        assert_eq!(
            service1.rule_external.config.app_id,
            service2.rule_external.config.app_id
        );
        assert_eq!(
            service1.visitor.config.app_secret,
            service2.visitor.config.app_secret
        );
        assert_eq!(service1.device.config.app_id, service2.device.config.app_id);
        assert_eq!(
            service1.access_record.config.app_secret,
            service2.access_record.config.app_secret
        );
    }

    #[test]
    fn test_acs_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(160))
            .build();

        let service = AcsService::new(config);

        assert_eq!(service.user.config.app_id, "consistency_test");
        assert_eq!(service.user.config.app_secret, "consistency_secret");
        assert_eq!(
            service.user.config.req_timeout,
            Some(Duration::from_secs(160))
        );
        assert_eq!(service.rule_external.config.app_id, "consistency_test");
        assert_eq!(
            service.rule_external.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.rule_external.config.req_timeout,
            Some(Duration::from_secs(160))
        );
        assert_eq!(service.visitor.config.app_id, "consistency_test");
        assert_eq!(service.visitor.config.app_secret, "consistency_secret");
        assert_eq!(
            service.visitor.config.req_timeout,
            Some(Duration::from_secs(160))
        );
        assert_eq!(service.device.config.app_id, "consistency_test");
        assert_eq!(service.device.config.app_secret, "consistency_secret");
        assert_eq!(
            service.device.config.req_timeout,
            Some(Duration::from_secs(160))
        );
        assert_eq!(service.access_record.config.app_id, "consistency_test");
        assert_eq!(
            service.access_record.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.access_record.config.req_timeout,
            Some(Duration::from_secs(160))
        );
    }
}
