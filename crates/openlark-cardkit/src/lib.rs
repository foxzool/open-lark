//! OpenLark CardKit Module
//!
//! 飞书 CardKit（卡片能力）服务模块，提供卡片实体与组件相关 API。
//!
//! ## 目录组织（strict）
//!
//! 实现文件严格按 `src/bizTag/meta.project/meta.version/meta.resource/meta.name.rs` 组织，
//! 数据源为仓库根目录 `api_list_export.csv`。

#![allow(missing_docs)]

pub mod common;
pub mod endpoints;
pub mod service;

// 业务模块（按 bizTag 组织）
pub mod cardkit;

pub use common::chain::CardkitClient;
pub use endpoints::*;
pub use service::CardkitService;

/// Re-exports from openlark-core for convenience.
pub mod prelude {
    pub use openlark_core::{config::Config, SDKResult};
}
