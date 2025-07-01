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
