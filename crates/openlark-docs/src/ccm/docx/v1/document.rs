//! Document service for DOCX v1 API
//!
//! This module provides document-level operations for documents,
//! including creation, reading, updating, and deletion.

use crate::prelude::*;

/// Document operation service
#[derive(Clone)]
pub struct DocumentService {
    client: std::sync::Arc<LarkClient>,
}

impl DocumentService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for DocumentService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the DocumentService functionality
    }
}
