//! # 关联组织服务
//!
//! 关联组织 (Trust Party) 服务提供完整的关联组织管理功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **关联组织管理**：查询可见关联组织、获取组织详情、部门和成员信息
//! - **可搜可见规则管理**：创建、更新、查询和删除可搜可见规则
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`collaboration_organization`] - 关联组织管理模块
//! - [`searchable_visible_rules`] - 可搜可见规则管理模块
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::trust_party::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 获取可见关联组织列表
//!     let organizations = client.trust_party.collaboration_organization.list_organizations(
//!         collaboration_organization::OrganizationListRequest::default(), None
//!     ).await?;
//!     
//!     // 创建可搜可见规则
//!     let rule = client.trust_party.searchable_visible_rules.create_rule(
//!         searchable_visible_rules::RuleCreateRequest {
//!             name: "示例规则".to_string(),
//!             description: Some("示例描述".to_string()),
//!             rule_type: Some("1".to_string()),
//!             org_id: Some("org_123".to_string()),
//!             config: None,
//!         }, None
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod collaboration_organization;
pub mod models;
pub mod searchable_visible_rules;

use crate::{
    core::config::Config,
    service::trust_party::{
        collaboration_organization::CollaborationOrganizationService,
        searchable_visible_rules::SearchableVisibleRulesService,
    },
};

/// 关联组织服务
///
/// 提供完整的关联组织管理功能，包括组织管理和可搜可见规则管理
pub struct TrustPartyService {
    /// 关联组织管理服务
    pub collaboration_organization: CollaborationOrganizationService,
    /// 可搜可见规则管理服务
    pub searchable_visible_rules: SearchableVisibleRulesService,
}

impl TrustPartyService {
    /// 创建关联组织服务实例
    pub fn new(config: Config) -> Self {
        Self {
            collaboration_organization: CollaborationOrganizationService::new(config.clone()),
            searchable_visible_rules: SearchableVisibleRulesService::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_trust_party_service_creation() {
        let config = Config::default();
        let service = TrustPartyService::new(config.clone());

        assert_eq!(
            service.collaboration_organization.config.app_id,
            config.app_id
        );
        assert_eq!(
            service.collaboration_organization.config.app_secret,
            config.app_secret
        );
        assert_eq!(
            service.searchable_visible_rules.config.app_id,
            config.app_id
        );
        assert_eq!(
            service.searchable_visible_rules.config.app_secret,
            config.app_secret
        );
    }

    #[test]
    fn test_trust_party_service_with_custom_config() {
        let config = Config::builder()
            .app_id("trust_party_test_app")
            .app_secret("trust_party_test_secret")
            .req_timeout(Duration::from_secs(150))
            .build();

        let service = TrustPartyService::new(config.clone());

        assert_eq!(
            service.collaboration_organization.config.app_id,
            "trust_party_test_app"
        );
        assert_eq!(
            service.collaboration_organization.config.app_secret,
            "trust_party_test_secret"
        );
        assert_eq!(
            service.collaboration_organization.config.req_timeout,
            Some(Duration::from_secs(150))
        );
        assert_eq!(
            service.searchable_visible_rules.config.app_id,
            "trust_party_test_app"
        );
        assert_eq!(
            service.searchable_visible_rules.config.req_timeout,
            Some(Duration::from_secs(150))
        );
    }

    #[test]
    fn test_trust_party_service_config_independence() {
        let config1 = Config::builder().app_id("trust_party_app_1").build();

        let config2 = Config::builder().app_id("trust_party_app_2").build();

        let service1 = TrustPartyService::new(config1);
        let service2 = TrustPartyService::new(config2);

        assert_eq!(
            service1.collaboration_organization.config.app_id,
            "trust_party_app_1"
        );
        assert_eq!(
            service2.collaboration_organization.config.app_id,
            "trust_party_app_2"
        );
        assert_ne!(
            service1.collaboration_organization.config.app_id,
            service2.collaboration_organization.config.app_id
        );
        assert_ne!(
            service1.searchable_visible_rules.config.app_id,
            service2.searchable_visible_rules.config.app_id
        );
    }

    #[test]
    fn test_trust_party_service_sub_services_accessible() {
        let config = Config::default();
        let service = TrustPartyService::new(config.clone());

        assert_eq!(
            service.collaboration_organization.config.app_id,
            config.app_id
        );
        assert_eq!(
            service.searchable_visible_rules.config.app_id,
            config.app_id
        );
    }

    #[test]
    fn test_trust_party_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = TrustPartyService::new(config.clone());

        assert_eq!(
            service.collaboration_organization.config.app_id,
            "clone_test_app"
        );
        assert_eq!(
            service.collaboration_organization.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(
            service.searchable_visible_rules.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(
            service.searchable_visible_rules.config.app_id,
            "clone_test_app"
        );
    }

    #[test]
    fn test_trust_party_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = TrustPartyService::new(config);

        assert_eq!(
            service.collaboration_organization.config.req_timeout,
            Some(Duration::from_secs(180))
        );
        assert_eq!(
            service.searchable_visible_rules.config.req_timeout,
            Some(Duration::from_secs(180))
        );
    }

    #[test]
    fn test_trust_party_service_multiple_instances() {
        let config = Config::default();

        let service1 = TrustPartyService::new(config.clone());
        let service2 = TrustPartyService::new(config.clone());

        assert_eq!(
            service1.collaboration_organization.config.app_id,
            service2.collaboration_organization.config.app_id
        );
        assert_eq!(
            service1.collaboration_organization.config.app_secret,
            service2.collaboration_organization.config.app_secret
        );
        assert_eq!(
            service1.searchable_visible_rules.config.app_id,
            service2.searchable_visible_rules.config.app_id
        );
        assert_eq!(
            service1.searchable_visible_rules.config.app_secret,
            service2.searchable_visible_rules.config.app_secret
        );
    }

    #[test]
    fn test_trust_party_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(130))
            .build();

        let service = TrustPartyService::new(config);

        assert_eq!(
            service.collaboration_organization.config.app_id,
            "consistency_test"
        );
        assert_eq!(
            service.collaboration_organization.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.collaboration_organization.config.req_timeout,
            Some(Duration::from_secs(130))
        );
        assert_eq!(
            service.searchable_visible_rules.config.app_id,
            "consistency_test"
        );
        assert_eq!(
            service.searchable_visible_rules.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.searchable_visible_rules.config.req_timeout,
            Some(Duration::from_secs(130))
        );
    }
}
