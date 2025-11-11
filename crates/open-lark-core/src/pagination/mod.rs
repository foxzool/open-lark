//! Pagination support for API responses

use serde::{Deserialize, Serialize};

/// Pagination information for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page number
    pub page: i32,
    /// Number of items per page
    pub page_size: i32,
    /// Total number of pages
    pub total_page: i32,
    /// Total number of items
    pub total: i32,
}

/// Cursor-based pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPagination {
    /// Cursor for the next page
    pub page_token: Option<String>,
    /// Whether there are more pages
    pub has_more: bool,
}
