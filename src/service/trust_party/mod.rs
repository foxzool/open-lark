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
//!             ..Default::default()
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
