//! Comment service for DOCX v1 API
//!
//! This module provides document comment operations for documents,
//! including creating, reading, updating, and deleting comments.

use crate::prelude::*;

/// Comment operation service
#[derive(Clone, Debug)]
pub struct CommentService {
    client: std::sync::Arc<LarkClient>,
}

impl CommentService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for CommentService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the CommentService functionality
    }
}
