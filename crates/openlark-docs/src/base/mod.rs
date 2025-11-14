//! Base module for OpenLark Docs
//!
//! This module provides base functionality for document operations,
//! including fundamental document types and shared utilities.

use crate::prelude::*;

pub struct BaseService {
    client: std::sync::Arc<LarkClient>,
}

impl BaseService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for BaseService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the BaseService functionality
    }
}