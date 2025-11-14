//! Block service for DOCX v1 API
//!
//! This module provides document block operations for documents,
//! including text blocks, image blocks, table blocks, etc.

use crate::prelude::*;

/// Block operation service
#[derive(Clone)]
pub struct BlockService {
    client: std::sync::Arc<LarkClient>,
}

impl BlockService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for BlockService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the BlockService functionality
    }
}