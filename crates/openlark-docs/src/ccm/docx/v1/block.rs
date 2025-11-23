//! Block service for DOCX v1 API
//!
//! This module provides document block operations for documents,
//! including text blocks, image blocks, table blocks, etc.

use openlark_core::config::Config;

/// Block operation service
#[derive(Clone, Debug)]
pub struct BlockService {
    #[allow(dead_code)]
    config: Config,
}

impl BlockService {
    pub fn new(#[allow(dead_code)]
    config: Config,) -> Self {
        Self { config }
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
