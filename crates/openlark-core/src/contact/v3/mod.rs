//! Contact v3 API services
//!
//! This module provides Contact v3 API services for:
//! - User management and operations
//! - Department management
//! - Group management
//! - Job titles and families
//! - Functional roles and permissions
//! - Work cities and units
//! - Custom attributes and scopes

// Basic service modules only for now
pub mod department;
pub mod group;
pub mod user;
pub mod work_city;

// Re-export basic services for convenience
pub use department::*;
pub use group::*;
pub use user::*;
pub use work_city::*;
