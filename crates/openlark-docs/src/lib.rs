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

// Legacy client adapter module has been removed
// All services now use Config-based architecture directly

// Services module
pub mod services;

// Prelude module with common imports
pub mod prelude;

// Base modules - base project
#[cfg(feature = "base")]
pub mod base;

// Bitable modules - bitable project (now top-level)
#[cfg(feature = "bitable")]
pub mod bitable;

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
#[cfg(feature = "base")]
pub use base::BaseService;

#[cfg(feature = "bitable")]
pub use bitable::BitableService;

#[cfg(feature = "ccm-core")]
pub use ccm::CcmService;

#[cfg(feature = "cardkit")]
pub use cardkit::CardKitService;

#[cfg(feature = "report")]
pub use report::ReportService;
