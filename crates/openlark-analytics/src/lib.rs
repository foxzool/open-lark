//! # OpenLark 数据分析模块
//!
//! OpenLark SDK 的数据分析模块，提供飞书数据分析和搜索相关 API 的完整访问。
//!
//! ## 功能特性
//!
//! - **搜索服务**: 全文搜索、智能搜索、内容检索
//! - **数据分析**: 数据报表、统计分析、趋势分析
//!
//! ## 模块组织
//!
//! 本模块按业务域（bizTag）组织：
//! - `search` - 搜索服务相关 API (14 APIs)
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_analytics::AnalyticsService;
//! use openlark_core::prelude::Config;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 使用 builder 模式创建配置
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//!
//! let analytics_service = AnalyticsService::new(config)?;
//!
//! // 搜索场景（需要启用 search 和 v1 feature）
//! # #[cfg(all(feature = "search", feature = "v1"))]
//! let results = analytics_service
//!     .search()
//!     .v1()
//!     .query()
//!     .search()
//!     .search_term("项目文档")
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```

#![allow(missing_docs)]

mod service;

// 通用模块
pub mod common;

// 业务域模块
#[cfg(feature = "search")]
pub mod search;

// Prelude 模块
pub mod prelude;

// 重新导出核心服务
pub use service::AnalyticsService;

// 配置类型
pub use openlark_core::config::Config;

/// 数据分析模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 数据分析服务配置别名
pub type AnalyticsConfig = Config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_module_composition() {
        // 验证模块组织正确
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
