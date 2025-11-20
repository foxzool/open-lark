//! openlark-docs module for OpenLark SDK
//!
//! This crate provides cloud document functionality for the OpenLark SDK,
//! organized according to meta.Project(s) business logic:
//! - base: base, bitable
//! - ccm: content collaboration management (docs, docx, sheets, wiki, drive)
//! - cardkit: interactive card components
//! - report: reporting and analytics

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

// 导入端点模块
pub mod endpoints;

// Legacy client adapter module
pub mod legacy_client_adapter;

// Services module
pub mod services;

// Prelude module with common imports
pub mod prelude;

// Base modules - base and bitable projects
#[cfg(any(feature = "base", feature = "bitable"))]
pub mod base;

// Content Collaboration Management (CCM) modules
#[cfg(feature = "ccm-core")]
pub mod ccm;

// Interactive card components
#[cfg(feature = "cardkit")]
pub mod cardkit;

// Reporting and analytics
#[cfg(feature = "report")]
pub mod report;

// Re-export endpoints for convenience
pub use endpoints::*;

// Re-export service types for convenience
#[cfg(any(feature = "base", feature = "bitable"))]
pub use base::BaseService;

#[cfg(feature = "ccm-core")]
pub use ccm::CcmService;

#[cfg(feature = "cardkit")]
pub use cardkit::CardKitService;

#[cfg(feature = "report")]
pub use report::ReportService;
