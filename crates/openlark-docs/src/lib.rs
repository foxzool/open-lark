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

// Prelude module with common imports
pub mod prelude;

// Base modules - base and bitable projects
#[cfg(feature = "base")]
pub mod base;

// Content Collaboration Management (CCM) modules
#[cfg(feature = "ccm")]
pub mod ccm;


// Interactive card components
#[cfg(feature = "cardkit")]
pub mod cardkit;

// Reporting and analytics
#[cfg(feature = "report")]
pub mod report;

// Re-export service types for convenience
#[cfg(feature = "base")]
pub use base::BaseService;

#[cfg(feature = "ccm")]
pub use ccm::CcmService;

#[cfg(feature = "cardkit")]
pub use cardkit::CardKitService;

#[cfg(feature = "report")]
pub use report::ReportService;
