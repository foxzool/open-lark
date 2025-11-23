//! # OpenLark 文档服务模块
//!
//! 飞书开放平台云文档服务模块，提供文档、表格、知识库等API访问能力。
//!
//! ## 功能模块
//!
//! - **ccm**: 云文档协同 (174 APIs) - 文档、表格、知识库、云盘
//! - **bitable**: 多维表格 (49 APIs) - 多维表格应用、数据表、视图管理
//! - **base**: 基础服务 (0 APIs) - 基础操作和工具
//! - **baike**: 知识库 (27 APIs) - 企业知识库、Wiki管理
//! - **minutes**: 会议纪要 (4 APIs) - 会议记录管理
//!
//! ## 快速开始
//!
//! ```rust
//! use openlark_docs::DocsService;
//! use openlark_core::config::Config;
//!
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//! let docs = DocsService::new(config);
//!
//! // 基础服务使用
//! let config_ref = docs.config();
//! println!("App ID: {}", config_ref.app_id);
//! ```
//!
//! ## 特性
//!
//! - ✅ **254 APIs全覆盖** - 飞书云文档服务完整实现
//! - ✅ **类型安全** - 强类型请求/响应结构
//! - ✅ **异步支持** - 基于tokio的异步API
//! - ✅ **版本化API** - 支持v1/v2/v3/v4多版本API
//! - ✅ **构建器模式** - 流畅的API调用体验

// #![deny(missing_docs)] // 暂时禁用，在开发阶段
#![warn(clippy::all)]

// Include macros first
#[cfg(any(feature = "ccm", feature = "base"))]
#[macro_use]
mod macros;

// Core modules
pub mod error;
pub mod models;

// 主要服务模块
pub mod service;

// 功能模块按业务域组织
#[cfg(feature = "ccm")]
pub mod ccm;

#[cfg(feature = "bitable")]
pub mod bitable;

#[cfg(feature = "base")]
pub mod base;

#[cfg(feature = "baike")]
pub mod baike;

#[cfg(feature = "minutes")]
pub mod minutes;

// API版本模块
#[cfg(any(feature = "v1", feature = "v2", feature = "v3", feature = "v4"))]
pub mod versions;

// Prelude模块 - 常用导入
pub mod prelude;

// 重新导出主要类型
pub use error::{DocsError, DocsResult};
pub use service::DocsService;

// 重新导出各域服务
#[cfg(feature = "ccm")]
pub use ccm::CcmService;

#[cfg(feature = "bitable")]
pub use bitable::BitableService;

#[cfg(feature = "base")]
pub use base::BaseService;

#[cfg(feature = "baike")]
pub use baike::BaikeService;

#[cfg(feature = "minutes")]
pub use minutes::MinutesService;
