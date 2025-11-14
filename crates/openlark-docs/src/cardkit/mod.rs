//! CardKit module for OpenLark Docs
//!
//! This module provides CardKit functionality for creating and managing
//! interactive cards and card-based UI components.

use crate::prelude::*;

pub struct CardKitService {
    client: std::sync::Arc<LarkClient>,
}

impl CardKitService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for CardKitService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardkit_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the CardKitService functionality
    }
}